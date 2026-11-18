//! Pipelines: several agents in a fixed order, approved once at a total cap.
//!
//! - `prepare` creates every stage's run request up front, with the exact
//!   prompts (earlier stages' output paths filled in), so the estimate covers
//!   the real prompts. Nothing runs.
//! - `approve` requires the total cap that was shown, then approves each
//!   stage at its own cap.
//! - `run` runs stages in order. After each stage the studio checks the
//!   result itself (`evaluate_stage`); it does not take the agent's word.
//!   A stage that produces no output stops the pipeline.
//! - The end result goes into the review queue as one item. It is never
//!   published.

use crate::bridge;
use crate::cost::{self, AgentProfile};
use crate::queue::{self, Gate, GateCheck, QueueItem};
use crate::runs;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageDef {
    pub agent: String,
    pub instruction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDef {
    pub description: String,
    pub queue_kind: String,
    #[serde(default)]
    pub needs_project: bool,
    pub stages: Vec<StageDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelinesConfig {
    pub pipelines: BTreeMap<String, PipelineDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageState {
    pub agent: String,
    pub run_id: String,
    pub output: String,
    /// pending | paused | done | failed | skipped
    pub status: String,
    #[serde(default)]
    pub gate: Option<Gate>,
    #[serde(default)]
    pub cost_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineRun {
    pub pipeline_id: String,
    pub pipeline: String,
    pub input: String,
    #[serde(default)]
    pub project: Option<String>,
    pub stages: Vec<StageState>,
    pub expected_usd: f64,
    pub high_usd: f64,
    pub cap_usd: f64,
    /// awaiting-approval | approved | running | paused | done | stopped | cancelled
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub approved_at: Option<String>,
    #[serde(default)]
    pub finished_at: Option<String>,
    #[serde(default)]
    pub queue_item: Option<String>,
    #[serde(default)]
    pub actual_cost_usd: Option<f64>,
    /// Plain-language reason the pipeline stopped early (None when it did not).
    #[serde(default)]
    pub stopped_reason: Option<String>,
}

pub fn load_config(root: &Path) -> Result<PipelinesConfig, String> {
    let p = root.join("studio/pipelines.json");
    let t = fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
    serde_json::from_str(&t).map_err(|e| format!("parse {}: {e}", p.display()))
}

fn pipe_dir(root: &Path, id: &str) -> Result<PathBuf, String> {
    runs::run_dir(root, id)
}

pub fn save(root: &Path, p: &PipelineRun) -> Result<(), String> {
    let d = pipe_dir(root, &p.pipeline_id)?;
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    let tmp = d.join("pipeline.json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(p).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    fs::rename(&tmp, d.join("pipeline.json")).map_err(|e| e.to_string())
}

pub fn load(root: &Path, id: &str) -> Result<PipelineRun, String> {
    let p = pipe_dir(root, id)?.join("pipeline.json");
    let t = fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
    serde_json::from_str(&t).map_err(|e| format!("parse {}: {e}", p.display()))
}

pub fn list(root: &Path) -> Vec<PipelineRun> {
    let mut out = vec![];
    if let Ok(entries) = fs::read_dir(runs::runs_dir(root)) {
        for e in entries.flatten() {
            if let Ok(t) = fs::read_to_string(e.path().join("pipeline.json")) {
                if let Ok(p) = serde_json::from_str::<PipelineRun>(&t) {
                    out.push(p);
                }
            }
        }
    }
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    out
}

/// Fill `{input}`, `{project}`, and `{out:<agent>}` placeholders.
pub fn render(template: &str, input: &str, project: Option<&str>, outputs: &BTreeMap<String, PathBuf>) -> String {
    let mut s = template.replace("{input}", input);
    s = s.replace("{project}", project.unwrap_or("(no project)"));
    for (agent, path) in outputs {
        s = s.replace(&format!("{{out:{agent}}}"), &path.display().to_string());
    }
    s
}

fn validate_project(root: &Path, project: &str) -> Result<(), String> {
    let ok = project.starts_with("projects/")
        && !project.contains("..")
        && project[9..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if !ok {
        return Err(format!("project must look like projects/<name>, got {project:?}"));
    }
    if !root.join(project).is_dir() {
        return Err(format!("project folder {project} does not exist"));
    }
    Ok(())
}

pub fn prepare(root: &Path, name: &str, input: &str, project: Option<&str>) -> Result<PipelineRun, String> {
    let cfg = load_config(root)?;
    let def = cfg
        .pipelines
        .get(name)
        .ok_or_else(|| format!("unknown pipeline {name:?}"))?;
    if def.needs_project {
        let p = project.ok_or("this pipeline needs a project (projects/<name>)")?;
        validate_project(root, p)?;
    }
    let agents = cost::load_agents(root)?;
    let pid = crate::new_id(&format!("pipe-{name}"));
    let mut outputs: BTreeMap<String, PathBuf> = BTreeMap::new();
    let mut stages = vec![];
    let (mut expected, mut high, mut cap) = (0.0, 0.0, 0.0);
    for (i, st) in def.stages.iter().enumerate() {
        let prof: &AgentProfile = agents
            .agents
            .get(&st.agent)
            .ok_or_else(|| format!("stage {i}: unknown agent {}", st.agent))?;
        let out_name = prof
            .output
            .clone()
            .ok_or_else(|| format!("agent {} has no output file in studio/agents.json", st.agent))?;
        let run_id = format!("{pid}--{}-{}", i + 1, st.agent);
        let prompt = render(&st.instruction, input, project, &outputs);
        let req = runs::prepare_with_id(root, &st.agent, &prompt, &run_id, Some(&pid))?;
        expected += req.estimate.expected_usd;
        high += req.estimate.high_usd;
        cap += req.estimate.cap_usd;
        outputs.insert(st.agent.clone(), runs::run_dir(root, &run_id)?.join(&out_name));
        stages.push(StageState {
            agent: st.agent.clone(),
            run_id,
            output: out_name,
            status: "pending".into(),
            gate: None,
            cost_usd: None,
        });
    }
    let p = PipelineRun {
        pipeline_id: pid,
        pipeline: name.to_string(),
        input: input.to_string(),
        project: project.map(|s| s.to_string()),
        stages,
        expected_usd: round4(expected),
        high_usd: round4(high),
        cap_usd: (cap * 100.0).round() / 100.0,
        status: "awaiting-approval".into(),
        created_at: crate::now_rfc3339(),
        approved_at: None,
        finished_at: None,
        queue_item: None,
        actual_cost_usd: None,
        stopped_reason: None,
    };
    save(root, &p)?;
    Ok(p)
}

pub fn approve(root: &Path, id: &str, confirmed_cap_usd: f64) -> Result<PipelineRun, String> {
    let _g = crate::budget::APPROVE.lock().unwrap_or_else(|e| e.into_inner());
    let mut p = load(root, id)?;
    if p.status != "awaiting-approval" {
        return Err(format!("pipeline {id} is {}, not awaiting approval", p.status));
    }
    if (p.cap_usd - confirmed_cap_usd).abs() > 1e-9 {
        return Err(format!(
            "confirmed cap ${confirmed_cap_usd} does not match the pipeline cap ${}",
            p.cap_usd
        ));
    }
    crate::budget::check(root, p.cap_usd)?;
    let mut done: Vec<String> = vec![];
    for s in &p.stages {
        let res = runs::load(root, &s.run_id).and_then(|req| runs::approve_unchecked(root, &s.run_id, req.estimate.cap_usd));
        if let Err(e) = res {
            // Put the stages already approved back, so nothing stays reserved.
            for id in &done {
                if let Ok(mut r) = runs::load(root, id) {
                    r.status = "awaiting-approval".into();
                    let _ = runs::save(root, &r);
                }
            }
            return Err(e);
        }
        done.push(s.run_id.clone());
    }
    p.status = "approved".into();
    p.approved_at = Some(crate::now_rfc3339());
    save(root, &p)?;
    Ok(p)
}

pub fn cancel(root: &Path, id: &str) -> Result<PipelineRun, String> {
    let mut p = load(root, id)?;
    if p.status != "awaiting-approval" {
        return Err(format!("pipeline {id} is {}, cannot cancel", p.status));
    }
    for s in &p.stages {
        let _ = runs::cancel(root, &s.run_id);
    }
    p.status = "cancelled".into();
    save(root, &p)?;
    Ok(p)
}

/// Pipelines left running or approved when the app closed are stopped,
/// and their unfinished stage runs marked failed.
pub fn recover_stale(root: &Path) -> usize {
    let mut n = 0;
    for e in fs::read_dir(root.join("runs")).into_iter().flatten().flatten() {
        let id = e.file_name().to_string_lossy().to_string();
        if !id.starts_with("pipe-") || id.contains("--") {
            continue;
        }
        // Cut off mid-run: pause it, so it continues at the same stage.
        if let Ok(mut p) = load(root, &id) {
            if p.status == "running" {
                for s in &p.stages {
                    if let Ok(mut r) = runs::load(root, &s.run_id) {
                        if r.status == "running" {
                            r.status = "paused".into();
                            r.resume_at = Some(chrono::Utc::now().to_rfc3339());
                            r.prior_cost_usd = r.actual_cost_usd.unwrap_or(r.prior_cost_usd); // already includes earlier attempts
                            let _ = runs::save(root, &r);
                        }
                    }
                }
                p.status = "paused".into();
                let _ = save(root, &p);
                n += 1;
                continue;
            }
        }
        stop_unfinished(root, &id, "BrewixGames closed before this pipeline finished.");
        if load(root, &id).map(|p| p.status == "awaiting-approval").unwrap_or(false) {
            let _ = cancel(root, &id); // an estimate nobody approved
        }
        n += 1;
    }
    n
}

/// Mark a pipeline that did not finish as stopped (no-op otherwise).
pub fn stop_unfinished(root: &Path, id: &str, why: &str) {
    let Ok(mut p) = load(root, id) else { return };
    if p.status != "running" && p.status != "approved" {
        return;
    }
    for s in &p.stages {
        runs::fail_if_unfinished(root, &s.run_id, why);
    }
    p.status = "stopped".into();
    p.finished_at = Some(crate::now_rfc3339());
    let _ = save(root, &p);
}

/// Make a paused pipeline ready to run again from where it stopped.
pub fn ready_to_continue(root: &Path, id: &str) -> Result<PipelineRun, String> {
    let mut p = load(root, id)?;
    if p.status != "paused" {
        return Err(format!("pipeline {id} is {}, not paused", p.status));
    }
    p.status = "approved".into();
    save(root, &p)?;
    Ok(p)
}

/// Paused pipelines whose paused stage is due to continue.
pub fn due(root: &Path) -> Vec<PipelineRun> {
    let now = chrono::Utc::now();
    list(root)
        .into_iter()
        .filter(|p| p.status == "paused")
        .filter(|p| {
            p.stages.iter().filter_map(|s| runs::load(root, &s.run_id).ok()).filter(|r| r.status == "paused").all(|r| {
                r.resume_at.as_deref().and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()).map(|t| t <= now).unwrap_or(true)
            })
        })
        .collect()
}

/// Whether the pipeline should stop before the next stage. A stage that
/// exited fine but failed its own check still stops the run, so later
/// stages do not spend their budget on bad input.
pub fn should_stop(output_exists: bool, process_ok: bool, gate_passed: bool) -> bool {
    !output_exists || !process_ok || !gate_passed
}

/// A short, human reason for stopping, using the gate's own failure details.
fn stop_reason(agent: &str, gate: &Gate, output_exists: bool, process_ok: bool, next: Option<&str>) -> String {
    let mut why: Vec<String> = gate
        .checks
        .iter()
        .filter(|c| !c.passed)
        .map(|c| if c.detail.trim().is_empty() { c.name.clone() } else { format!("{} ({})", c.name, c.detail.trim()) })
        .collect();
    if why.is_empty() {
        why.push(if !output_exists { "no output file was written".into() } else if !process_ok { "the run did not finish cleanly".into() } else { "its check failed".into() });
    }
    let tail = match next {
        Some(n) => format!(" {} was not started, so no budget was spent on it.", cap_first(n)),
        None => String::new(),
    };
    format!("Stopped after {agent}: its check failed, {}.{tail} Files so far are kept.", why.join("; "))
}

/// Events reported while a pipeline runs.
pub enum Event<'a> {
    StageStart { index: usize, agent: &'a str, run_id: &'a str },
    Line { run_id: &'a str, line: &'a str },
    StageDone { index: usize, agent: &'a str, gate: &'a Gate },
}

pub fn run<F: FnMut(Event)>(root: &Path, id: &str, claude_bin: &Path, mut on: F) -> Result<PipelineRun, String> {
    let mut p = load(root, id)?;
    if p.status != "approved" {
        return Err(format!("pipeline {id} is {}; approve it first", p.status));
    }
    let agents = cost::load_agents(root)?;
    p.status = "running".into();
    save(root, &p)?;
    let mut prior_dirs: Vec<PathBuf> = vec![];
    let mut stopped = false;
    let mut total = 0.0;
    for i in 0..p.stages.len() {
        if stopped {
            p.stages[i].status = "skipped".into();
            let _ = runs::load(root, &p.stages[i].run_id).and_then(|mut r| {
                r.status = "cancelled".into();
                runs::save(root, &r)
            });
            continue;
        }
        let (agent, run_id) = (p.stages[i].agent.clone(), p.stages[i].run_id.clone());
        // Continuing after a pause: stages that already finished are kept.
        if p.stages[i].status == "done" || p.stages[i].status == "failed" {
            total += p.stages[i].cost_usd.unwrap_or(0.0);
            prior_dirs.push(runs::run_dir(root, &run_id)?);
            continue;
        }
        if runs::load(root, &run_id).map(|r| r.status == "paused").unwrap_or(false) {
            runs::ready_to_continue(root, &run_id)?;
        }
        on(Event::StageStart { index: i, agent: &agent, run_id: &run_id });
        let outcome = bridge::run_approved_with(root, &run_id, claude_bin, &prior_dirs, |line| {
            on(Event::Line { run_id: &run_id, line })
        });
        let run_dir = runs::run_dir(root, &run_id)?;
        // Usage limit hit: pause here; the rest stays approved for later.
        if runs::load(root, &run_id).map(|r| r.status == "paused").unwrap_or(false) {
            p.stages[i].status = "paused".into();
            p.status = "paused".into();
            p.actual_cost_usd = Some(round4(total));
            save(root, &p)?;
            return Ok(p);
        }
        let profile = agents.agents.get(&agent).cloned();
        let mut gate = evaluate_stage(root, &run_dir, &agent, profile.as_ref(), &p.stages[i].output, &prior_dirs, p.project.as_deref());
        match &outcome {
            Ok(o) => {
                p.stages[i].cost_usd = o.actual_cost_usd;
                total += o.actual_cost_usd.unwrap_or(0.0);
                gate.checks.insert(0, check("claude run finished without error", o.ok, &format!(
                    "exit {}, turns {}",
                    o.exit_code.map(|c| c.to_string()).unwrap_or_else(|| "none".into()),
                    o.num_turns.map(|t| t.to_string()).unwrap_or_else(|| "unknown".into())
                )));
            }
            Err(e) => gate.checks.insert(0, check("claude run finished without error", false, e)),
        }
        gate.passed = gate.checks.iter().all(|c| c.passed);
        let output_exists = run_dir.join(&p.stages[i].output).is_file();
        p.stages[i].status = if gate.passed { "done".into() } else { "failed".into() };
        p.stages[i].gate = Some(gate.clone());
        on(Event::StageDone { index: i, agent: &agent, gate: &gate });
        save(root, &p)?;
        // Stop when nothing usable came out, or the run itself failed (cap hit,
        // error): later stages would spend their caps on bad input.
        let process_ok = matches!(&outcome, Ok(o) if o.ok);
        // A failed check on the very last stage has nothing after it to
        // protect; it is reported on the stage itself and the run ends as usual.
        let last = i + 1 == p.stages.len();
        if should_stop(output_exists, process_ok, gate.passed) && !(last && output_exists && process_ok) {
            stopped = true;
            let next = p.stages.get(i + 1).map(|s| s.agent.clone());
            p.stopped_reason = Some(stop_reason(&agent, &gate, output_exists, process_ok, next.as_deref()));
        }
        prior_dirs.push(run_dir);
    }
    p.actual_cost_usd = Some(round4(total));
    p.status = if stopped { "stopped".into() } else { "done".into() };
    p.finished_at = Some(crate::now_rfc3339());
    save(root, &p)?; // final status first, before the queue item and notifications

    // One review item for the whole pipeline.
    let def_kind = load_config(root)?
        .pipelines
        .get(&p.pipeline)
        .map(|d| d.queue_kind.clone())
        .unwrap_or_else(|| "build".into());
    let mut checks = vec![];
    let mut artifacts = vec![];
    for s in &p.stages {
        let rel = format!("runs/{}/{}", s.run_id, s.output);
        if root.join(&rel).is_file() {
            artifacts.push(rel);
        }
        match &s.gate {
            Some(g) => checks.extend(g.checks.iter().map(|c| GateCheck {
                name: format!("{}: {}", s.agent, c.name),
                passed: c.passed,
                detail: c.detail.clone(),
            })),
            None => checks.push(check(&format!("{}: stage ran", s.agent), false, "skipped because an earlier stage produced no output")),
        }
    }
    if let Some(proj) = built_project(root, &p) {
        artifacts.push(proj);
    }
    let passed = !checks.is_empty() && checks.iter().all(|c| c.passed);
    let failed: Vec<&str> = p.stages.iter().filter(|s| s.status != "done").map(|s| s.agent.as_str()).collect();
    let item = QueueItem {
        id: format!("q-{}", p.pipeline_id),
        kind: def_kind,
        title: title_for(root, &p),
        summary: format!(
            "Pipeline {} ({}). Input: {}. Stages: {}. {}Actual cost ${:.4} (cap ${:.2}). Approving records your yes only; nothing is published.",
            p.pipeline,
            p.pipeline_id,
            p.input.chars().take(160).collect::<String>().trim_end_matches('.'),
            p.stages.iter().map(|s| format!("{} {}", s.agent, s.status)).collect::<Vec<_>>().join(", "),
            if failed.is_empty() { String::new() } else { format!("Failed or skipped: {}. ", failed.join(", ")) },
            total,
            p.cap_usd
        ),
        created_at: crate::now_rfc3339(),
        created_by: format!("pipeline:{}", p.pipeline),
        run_id: Some(p.pipeline_id.clone()),
        artifacts,
        gate: Gate { passed, checks },
        status: "pending".into(),
        decision: None,
    };
    queue::add(root, &item)?;
    notify_item(&item, &p.status, total);
    p.queue_item = Some(item.id);
    save(root, &p)?;
    Ok(p)
}

fn notify_item(item: &QueueItem, status: &str, cost: f64) {
    use crate::notify::{send_quiet, Kind};
    let failed = item.gate.checks.iter().filter(|c| !c.passed).count();
    let gate = if item.gate.passed { "passed".to_string() } else { format!("failed ({failed} checks)") };
    let lines = vec![
        format!("**{}**", item.title),
        format!("Status: {status} · gate {gate} · cost ${cost:.2}"),
        if item.gate.passed { "Waiting for your approval in the review queue.".into() } else { "Gate failed: it can only be rejected. Open the review queue.".into() },
    ];
    let kind = if status == "done" { Kind::Finished } else { Kind::Failed };
    send_quiet(kind, if status == "done" { "Build finished" } else { "Run stopped" }, &lines);
    send_quiet(Kind::NeedsApproval, "Needs your review", &lines);
}

/// After a single (non-pipeline) agent run: check its work the same way a
/// pipeline stage is checked and put it in the review queue. Agents without
/// an output file (bridge-test) do not create items.
/// Re-run the studio's checks on a finished single run and replace its
/// pending review item (used after a checker fix). Decided items are kept.
pub fn recheck_single(root: &Path, run_id: &str) -> Result<Option<QueueItem>, String> {
    let qid = format!("q-{run_id}");
    if let Ok(existing) = queue::get(root, &qid) {
        if existing.status != "pending" {
            return Err(format!("{qid} was already {}; not changing it", existing.status));
        }
        fs::remove_file(queue::items_dir(root).join(format!("{qid}.json"))).map_err(|e| e.to_string())?;
    }
    finish_single(root, run_id)
}

pub fn finish_single(root: &Path, run_id: &str) -> Result<Option<QueueItem>, String> {
    let req = runs::load(root, run_id)?;
    if req.status == "paused" {
        return Ok(None); // reviewed once it finishes
    }
    let agents = cost::load_agents(root)?;
    let Some(prof) = agents.agents.get(&req.agent) else { return Ok(None) };
    let Some(output) = prof.output.clone() else { return Ok(None) };
    let dir = runs::run_dir(root, run_id)?;
    let mut gate = evaluate_stage(root, &dir, &req.agent, Some(prof), &output, &[], None);
    gate.checks.insert(0, check("claude run finished without error", req.status == "done", &req.status));
    gate.passed = gate.checks.iter().all(|c| c.passed);
    let mut artifacts = vec![];
    let rel = format!("runs/{run_id}/{output}");
    if root.join(&rel).is_file() {
        artifacts.push(rel);
    }
    // Librarian's sound sheet lives in the project, not the run folder.
    if req.agent == "librarian" {
        if let Ok(t) = fs::read_to_string(dir.join(&output)) {
            if let Some(i) = t.find("projects/") {
                let name: String = t[i + 9..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
                let rel = format!("projects/{name}/sounds.json");
                if !name.is_empty() && root.join(&rel).is_file() {
                    artifacts.push(rel);
                }
            }
        }
    }
    for extra in ["thumbnails"] {
        let p = dir.join(extra);
        if p.is_dir() {
            if let Ok(entries) = fs::read_dir(&p) {
                let mut names: Vec<String> = entries.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
                names.sort();
                for n in names {
                    artifacts.push(format!("runs/{run_id}/{extra}/{n}"));
                }
            }
        }
    }
    let item = QueueItem {
        id: format!("q-{run_id}"),
        kind: kind_for(&req.agent).into(),
        title: format!("{}: {}", cap_first(&req.agent), req.prompt.chars().take(80).collect::<String>()),
        summary: req.result_text.clone().unwrap_or_default().chars().take(400).collect(),
        created_at: crate::now_rfc3339(),
        created_by: req.agent.clone(),
        run_id: Some(run_id.to_string()),
        artifacts,
        gate,
        status: "pending".into(),
        decision: None,
    };
    queue::add(root, &item)?;
    notify_item(&item, &req.status, req.actual_cost_usd.unwrap_or(0.0));
    Ok(Some(item))
}

fn kind_for(agent: &str) -> &'static str {
    match agent {
        "pulse" => "pitch",
        "architect" => "design",
        "planner" => "plan",
        "builder" | "scenic" | "frame" | "motion" | "crowd" | "tuner" => "build",
        "modeler" => "asset",
        "librarian" => "asset",
        "splash" => "thumbnail",
        "playtest" => "playtest-report",
        "merchant" => "shop",
        "liveops" | "producer" => "update-plan",
        _ => "other",
    }
}

fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn title_for(root: &Path, p: &PipelineRun) -> String {
    // Use the first heading of the design doc if there is one.
    for s in &p.stages {
        if s.agent == "architect" {
            if let Ok(t) = fs::read_to_string(root.join("runs").join(&s.run_id).join(&s.output)) {
                if let Some(h) = t.lines().find(|l| l.starts_with("# ")) {
                    return format!("{}: {}", p.pipeline, h.trim_start_matches("# ").trim());
                }
            }
        }
    }
    format!("{}: {}", p.pipeline, p.input.chars().take(80).collect::<String>())
}

/// The project folder Builder declared (`Project: projects/<name>`).
fn built_project(root: &Path, p: &PipelineRun) -> Option<String> {
    let s = p.stages.iter().find(|s| s.agent == "builder")?;
    let t = fs::read_to_string(root.join("runs").join(&s.run_id).join(&s.output)).ok()?;
    let line = t.lines().find(|l| l.trim_start().starts_with("Project:"))?;
    let proj = line.trim_start().trim_start_matches("Project:").trim().trim_matches('`').to_string();
    if validate_project(root, &proj).is_ok() {
        Some(proj)
    } else {
        None
    }
}

fn check(name: &str, passed: bool, detail: &str) -> GateCheck {
    GateCheck { name: name.into(), passed, detail: detail.into() }
}

/// The studio's own checks on a finished stage. The agent's gate.json is
/// included, but it is one input among several, not the verdict.
pub fn evaluate_stage(
    root: &Path,
    run_dir: &Path,
    agent: &str,
    profile: Option<&AgentProfile>,
    output: &str,
    prior_dirs: &[PathBuf],
    project: Option<&str>,
) -> Gate {
    let mut checks = vec![];
    let out = run_dir.join(output);
    let body = fs::read_to_string(&out).unwrap_or_default();
    checks.push(check(
        &format!("wrote {output}"),
        !body.trim().is_empty(),
        if body.trim().is_empty() { "missing or empty" } else { "present" },
    ));

    // Agent's own gate: must parse in the required shape and say passed.
    match fs::read_to_string(run_dir.join("gate.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<Gate>(&t).ok())
    {
        Some(g) => {
            let failed: Vec<String> = g.checks.iter().filter(|c| !c.passed).map(|c| c.name.clone()).collect();
            checks.push(check(
                "agent's own gate passed",
                g.passed && failed.is_empty() && !g.checks.is_empty(),
                &if failed.is_empty() { format!("{} checks", g.checks.len()) } else { format!("failed: {}", failed.join("; ")) },
            ));
        }
        None => checks.push(check("agent's own gate passed", false, "gate.json missing or not in the required shape")),
    }

    // Loaded only allowed vault sections.
    if let Some(prof) = profile {
        let (ok, detail) = loads_within(root, run_dir, agent, &prof.allowed_sections, prior_dirs);
        checks.push(check("loaded only allowed vault sections", ok, &detail));
    }

    // Agent-specific checks, built into the app (gates.rs).
    checks.extend(crate::gates::check_stage(root, agent, &out, project));
    let passed = checks.iter().all(|c| c.passed);
    Gate { passed, checks }
}

/// A vault section is a top-level folder named like `07-Monetization`.
fn is_section(vault: &Path, name: &str) -> bool {
    name.len() > 3 && name.as_bytes()[..2].iter().all(|b| b.is_ascii_digit()) && name.as_bytes()[2] == b'-' && vault.join(name).is_dir()
}

/// Compare the hook's load log with the agent's allowed sections.
pub fn loads_within(root: &Path, run_dir: &Path, agent: &str, allowed: &[String], prior_dirs: &[PathBuf]) -> (bool, String) {
    let log = fs::read_to_string(run_dir.join("loads.jsonl")).unwrap_or_default();
    let vault = root.join("vault");
    let mut seen: Vec<String> = vec![];
    let mut bad: Vec<String> = vec![];
    for line in log.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else { continue };
        let tool = v.get("tool").and_then(|t| t.as_str()).unwrap_or("");
        let mut target = v.get("target").and_then(|t| t.as_str()).unwrap_or("").to_string();
        if tool == "Glob" || tool == "Grep" {
            let path = v.pointer("/input/path").and_then(|t| t.as_str()).unwrap_or("");
            let pat_abs = v.pointer("/input/pattern").and_then(|t| t.as_str()).unwrap_or("");
            if path.is_empty() && pat_abs.starts_with('/') && !pat_abs.starts_with(&format!("{}/", vault.display())) {
                // An absolute pattern outside the vault: judge it by the folder it searches.
                target = pat_abs.split(['*', '?', '{', '[']).next().unwrap_or("").trim_end_matches('/').to_string();
            } else if path.is_empty() || Path::new(path) == vault {
                let pat_raw = v.pointer("/input/pattern").and_then(|t| t.as_str()).unwrap_or("");
                // An absolute pattern inside the vault is a search of that section.
                let vault_prefix = format!("{}/", vault.display());
                let pat = pat_raw.strip_prefix(&vault_prefix).unwrap_or(pat_raw);
                let first = pat.split('/').next().unwrap_or("");
                if is_section(&vault, first) {
                    target = first.to_string();
                } else {
                    bad.push(format!("{tool} over the vault root ({pat_raw})"));
                    continue;
                }
            } else {
                target = path.to_string();
            }
        }
        // Normalize absolute vault paths to vault-relative.
        let tp = Path::new(&target);
        let rel = if tp.is_absolute() {
            match tp.strip_prefix(&vault) {
                Ok(r) => r.to_string_lossy().to_string(),
                Err(_) => {
                    let allowed_abs = tp.starts_with(run_dir)
                        || tp.starts_with(root.join("projects"))
                        || prior_dirs.iter().any(|d| tp.starts_with(d));
                    if !allowed_abs {
                        bad.push(format!("{tool} outside allowed folders: {target}"));
                    }
                    continue;
                }
            }
        } else {
            target.clone()
        };
        let first = rel.split('/').next().unwrap_or("").to_string();
        if is_section(&vault, &first) {
            if !seen.contains(&first) {
                seen.push(first.clone());
            }
            if !allowed.iter().any(|a| a == &first) {
                bad.push(format!("section {first} not allowed for {agent}"));
            }
        } else if rel.starts_with(&format!(".claude/skills/{agent}")) || rel == "CLAUDE.md" || rel.is_empty() || rel.starts_with("attachments/") {
            // own skill, the vault's tiny CLAUDE.md, or shared pictures
            // (example sheets and game reference pictures every agent may study)
        } else {
            bad.push(format!("{tool} {rel}"));
        }
    }
    if seen.first().map(|s| s.as_str()) != Some("00-Start-Here") && !seen.is_empty() {
        bad.push("did not read the router first".into());
    }
    if bad.is_empty() {
        (true, format!("sections: {}", if seen.is_empty() { "none".into() } else { seen.join(", ") }))
    } else {
        (false, bad.join("; "))
    }
}

fn round4(x: f64) -> f64 {
    (x * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_fills_placeholders() {
        let mut outs = BTreeMap::new();
        outs.insert("pulse".to_string(), PathBuf::from("/r/runs/a/pitch.md"));
        let s = render("Do {input} in {project} using {out:pulse}", "X", Some("projects/g"), &outs);
        assert_eq!(s, "Do X in projects/g using /r/runs/a/pitch.md");
    }

    #[test]
    fn reading_shared_pictures_is_allowed() {
        let d = std::env::temp_dir().join(format!("loads-att-{}", std::process::id()));
        let root = d.join("studio");
        fs::create_dir_all(root.join("vault/00-Start-Here")).unwrap();
        fs::create_dir_all(root.join("vault/attachments/game-refs/adopt-me")).unwrap();
        let run = root.join("runs/r1");
        fs::create_dir_all(&run).unwrap();
        let v = root.join("vault");
        fs::write(run.join("loads.jsonl"), format!(
            "{{\"tool\":\"Read\",\"target\":\"{0}/00-Start-Here/index.md\"}}\n{{\"tool\":\"Read\",\"target\":\"{0}/attachments/examples/good-hud.svg\"}}\n{{\"tool\":\"Read\",\"target\":\"{0}/attachments/game-refs/adopt-me/01-icon.png\"}}\n",
            v.display()
        )).unwrap();
        let (ok, detail) = loads_within(&root, &run, "planner", &["00-Start-Here".into()], &[]);
        assert!(ok, "{detail}");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn loads_check_flags_wrong_section_and_root_search() {
        let root = std::env::temp_dir().join(format!("studio-p-{}", std::process::id()));
        let run = root.join("runs/x");
        fs::create_dir_all(&run).unwrap();
        let v = root.join("vault");
        for sec in ["00-Start-Here", "02-Game-Design", "13-Luau-Engineering", "19-Project-Memory"] {
            fs::create_dir_all(v.join(sec)).unwrap();
        }
        let log = [
            format!(r#"{{"tool":"Read","target":"00-Start-Here/index.md"}}"#),
            format!(r#"{{"tool":"Read","target":"02-Game-Design/index.md"}}"#),
            format!(r#"{{"tool":"Read","target":"{}"}}"#, run.join("in.md").display()),
        ]
        .join("\n");
        fs::write(run.join("loads.jsonl"), &log).unwrap();
        let allowed = vec!["00-Start-Here".to_string(), "02-Game-Design".to_string()];
        assert!(loads_within(&root, &run, "architect", &allowed, &[]).0);
        let bad = format!("{log}\n{}\n{}", r#"{"tool":"Read","target":"13-Luau-Engineering/index.md"}"#, format!(r#"{{"tool":"Glob","target":"x","input":{{"pattern":"**/*.md","path":"{}"}}}}"#, v.display()));
        fs::write(run.join("loads.jsonl"), bad).unwrap();
        let (ok, d) = loads_within(&root, &run, "architect", &allowed, &[]);
        assert!(!ok);
        assert!(d.contains("13-Luau-Engineering") && d.contains("vault root"), "{d}");
        // Absolute patterns: inside an allowed section, and inside projects/, are fine.
        let ok_log = format!(
            "{}\n{}\n{}",
            r#"{"tool":"Read","target":"00-Start-Here/index.md"}"#,
            format!(r#"{{"tool":"Glob","target":"(vault root)","input":{{"pattern":"{}/02-Game-Design/*.md"}}}}"#, v.display()),
            format!(r#"{{"tool":"Glob","target":"(vault root)","input":{{"pattern":"{}/projects/g/**/*"}}}}"#, root.display())
        );
        fs::write(run.join("loads.jsonl"), ok_log).unwrap();
        let (ok, d) = loads_within(&root, &run, "architect", &allowed, &[]);
        assert!(ok, "{d}");
    }
}

#[cfg(all(test, unix))]
mod pause_tests {
    use super::*;
    #[test]
    fn a_paused_pipeline_continues_at_the_stopped_stage() {
        let d = std::env::temp_dir().join(format!("pipe-pause-t-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("studio")).unwrap();
        for f in ["agents.json", "pricing.json", "pipelines.json"] {
            fs::copy(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../studio").join(f), d.join("studio").join(f)).unwrap();
        }
        fs::create_dir_all(d.join("vault/.claude/agents")).unwrap();
        for a in ["pulse", "lead", "architect", "builder"] {
            fs::write(d.join(format!("vault/.claude/agents/{a}.md")), format!("---\nname: {a}\n---\n")).unwrap();
        }
        // A stand-in for claude: writes the stage outputs, counts calls, and
        // hits the usage limit once on the second call.
        let fake = d.join("fake-claude");
        fs::write(&fake, format!(r#"#!/bin/sh
echo x >> "{c}"
dir=""; prev=""
for a in "$@"; do if [ "$prev" = "--add-dir" ]; then [ -z "$dir" ] && case "$a" in */runs/*) dir="$a";; esac; fi; prev="$a"; done
n=$(wc -l < "{c}")
if [ "$n" = "2" ]; then echo '{{"type":"result","is_error":true,"result":"Claude AI usage limit reached|4102444800","total_cost_usd":0.1}}'; exit 1; fi
for f in pitch.md plan.md design.md build.md; do cat > "$dir/$f" <<'DOC'
# Loaded
none
# Data sources
https://example.com
# Ranked pitches
1. x
# Style kit
# Core loop
# First 60 seconds
0-5 a, 5-10 b, 10-30 c, 30-60 d
# Will not do
nothing
# Blocked
none
| a | b |
| - | - |
| 1 | 2 |
| 3 | 4 |
Ends in the review queue.
DOC
done
echo '{{"passed":true,"checks":[{{"name":"x","passed":true,"detail":""}}]}}' > "$dir/gate.json"
echo '{{"type":"result","is_error":false,"result":"done","total_cost_usd":0.2,"num_turns":1}}'
"#, c = d.join("calls").display())).unwrap();
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        let p = prepare(&d, "pitch-to-build", "a game", None).unwrap();
        approve(&d, &p.pipeline_id, p.cap_usd).unwrap();
        let after = run(&d, &p.pipeline_id, &fake, |_| {}).unwrap();
        assert_eq!(after.status, "paused", "{:?}", after.stopped_reason);
        assert_ne!(after.stages[0].status, "pending"); // finished (its own gate needs gate.json)
        assert_eq!(after.stages[1].status, "paused");
        assert!(after.stages[2..].iter().all(|s| s.status == "pending"));
        let r1 = runs::load(&d, &after.stages[1].run_id).unwrap();
        assert_eq!(r1.status, "paused");
        assert!(due(&d).is_empty(), "waits for the reset");
        // the app closing while paused keeps everything for later
        crate::runs::recover_stale(&d);
        assert_eq!(load(&d, &p.pipeline_id).unwrap().status, "paused");
        assert_eq!(runs::load(&d, &after.stages[2].run_id).unwrap().status, "approved");
        ready_to_continue(&d, &p.pipeline_id).unwrap();
        let done = run(&d, &p.pipeline_id, &fake, |_| {}).unwrap();
        assert_eq!(done.status, "done", "{:?}", done.stages.iter().map(|s| &s.status).collect::<Vec<_>>());
        assert!(done.stages.iter().all(|s| s.status == "done" || s.status == "failed"));
        // 4 stages + 1 limited attempt: stage one was not run again
        assert_eq!(fs::read_to_string(d.join("calls")).unwrap().lines().count(), 5);
        let r1 = runs::load(&d, &after.stages[1].run_id).unwrap();
        assert_eq!(r1.continued, 1);
        assert!((r1.actual_cost_usd.unwrap() - 0.3).abs() < 1e-9, "{:?}", r1.actual_cost_usd);
        assert!((done.actual_cost_usd.unwrap() - 0.9).abs() < 1e-6, "{:?}", done.actual_cost_usd);
        let _ = fs::remove_dir_all(&d);
    }
}

#[cfg(test)]
mod gate_stop_tests {
    use super::*;

    #[test]
    fn stops_when_the_gate_fails_even_if_the_process_was_fine() {
        assert!(!should_stop(true, true, true));
        assert!(should_stop(true, true, false));
        assert!(should_stop(true, false, true));
        assert!(should_stop(false, true, true));
    }

    #[test]
    fn the_stop_reason_is_plain_and_uses_the_gate_details() {
        let g = Gate { passed: false, checks: vec![check("design has every section", false, "missing Risks"), check("ok", true, "")] };
        let r = stop_reason("architect", &g, true, true, Some("builder"));
        assert!(r.starts_with("Stopped after architect: its check failed, design has every section (missing Risks)."), "{r}");
        assert!(r.contains("Builder was not started") && r.contains("Files so far are kept"), "{r}");
        assert!(!r.contains('\u{2014}'));
        let last = stop_reason("builder", &g, true, true, None);
        assert!(!last.contains("not started"), "{last}");
    }
}
