//! Change requests: Andrew writes what to change (on a game plan item or on
//! a built game), presses Send, and each request goes to the agent best
//! suited to it. Pressing Send approves every request at the cap shown, like
//! a chat message. Up to MAX_AT_ONCE agents work at the same time, one per
//! agent kind (two Frames never edit the same game at once). Plan changes are
//! written by the agent as change.json and merged into plan.json by the app,
//! one at a time, so parallel agents never overwrite each other.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const MAX_AT_ONCE: usize = 3;
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Change {
    pub id: String,
    /// "plan:<planner run id>" or "project:<name>"
    pub target: String,
    /// Plan item id, for plan changes.
    #[serde(default)]
    pub item: Option<String>,
    pub text: String,
    pub agent: String,
    /// queued | working | paused | done | failed
    pub status: String,
    pub run_id: String,
    pub created_at: String,
    #[serde(default)]
    pub finished_at: Option<String>,
    /// What the agent says it changed (first lines of change.md).
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub checks: Vec<crate::queue::GateCheck>,
    #[serde(default)]
    pub cost_usd: Option<f64>,
    /// "change" (one tweak) or "task" (a full job for an autopilot milestone).
    #[serde(default = "kind_change")]
    pub kind: String,
    /// The autopilot (game slug) this belongs to, if any.
    #[serde(default)]
    pub autopilot: Option<String>,
    #[serde(default)]
    pub milestone: Option<String>,
    /// Same key = same piece of work; the latest attempt counts.
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub attempt: u32,
    /// "andrew", "autopilot" or "agent:<name>".
    #[serde(default = "source_andrew")]
    pub source: String,
    /// The run whose agent asked for this job (its folder is readable).
    #[serde(default)]
    pub parent_run: Option<String>,
    /// Jobs started from the same first run share a group.
    #[serde(default)]
    pub group: Option<String>,
    /// How many agent hand-offs led here (limits chains).
    #[serde(default)]
    pub depth: u32,
    /// Wait for this change to finish first.
    #[serde(default)]
    pub after: Option<String>,
    /// Do not start before this time (RFC 3339): a retry waiting it out.
    #[serde(default)]
    pub not_before: Option<String>,
}

fn kind_change() -> String {
    "change".into()
}
fn source_andrew() -> String {
    "andrew".into()
}

/// A piece of work an autopilot (or an agent through it) asks for.
pub struct Spec {
    pub target: String,
    pub text: String,
    pub prompt: String,
    pub agent: String,
    pub kind: String,
    pub autopilot: Option<String>,
    pub milestone: Option<String>,
    pub key: Option<String>,
    pub attempt: u32,
    pub source: String,
    /// A stronger model for a hard retry.
    pub model: Option<String>,
    pub parent_run: Option<String>,
    pub group: Option<String>,
    pub depth: u32,
    pub after: Option<String>,
}

/// Queue one approved run for a spec. The caller has checked its budget.
pub fn enqueue(root: &Path, spec: Spec) -> Result<Change, String> {
    let mut req = crate::runs::prepare(root, &spec.agent, &spec.prompt)?;
    if let Some(m) = spec.model.filter(|m| *m != req.estimate.model && crate::settings::load().engine != "ollama") {
        let agents = crate::cost::load_agents(root)?;
        let pricing = crate::cost::load_pricing(root)?;
        if let Some(mut prof) = agents.agents.get(&spec.agent).cloned() {
            prof.model = m;
            if let Ok(e) = crate::cost::estimate(&spec.agent, &prof, &spec.prompt, &pricing) {
                req.estimate = e;
            }
        }
    }
    let id = format!("chg-{}", req.run_id.trim_start_matches("run-"));
    req.change = Some(id.clone());
    crate::runs::save(root, &req)?;
    if let Err(e) = crate::runs::approve(root, &req.run_id, req.estimate.cap_usd) {
        let _ = crate::runs::cancel(root, &req.run_id);
        return Err(e);
    }
    let c = Change {
        id,
        target: spec.target,
        item: None,
        text: spec.text,
        agent: spec.agent,
        status: "queued".into(),
        run_id: req.run_id,
        created_at: crate::now_rfc3339(),
        finished_at: None,
        summary: None,
        checks: vec![],
        cost_usd: None,
        kind: spec.kind,
        autopilot: spec.autopilot,
        milestone: spec.milestone,
        key: spec.key,
        attempt: spec.attempt,
        source: spec.source,
        parent_run: spec.parent_run,
        group: spec.group,
        depth: spec.depth,
        after: spec.after,
        not_before: None,
    };
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut all = list(root);
    all.push(c.clone());
    save_all(root, &all)?;
    Ok(c)
}

/// Cancel queued (not yet started) work, e.g. when an agent asks another
/// to stop. Returns how many were cancelled.
pub fn cancel_queued<F: Fn(&Change) -> bool>(root: &Path, pick: F) -> usize {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut all = list(root);
    let mut n = 0;
    for c in all.iter_mut().filter(|c| c.status == "queued" && pick(c)) {
        c.status = "cancelled".into();
        c.finished_at = Some(crate::now_rfc3339());
        if let Ok(mut r) = crate::runs::load(root, &c.run_id) {
            if r.status == "approved" {
                r.status = "cancelled".into();
                let _ = crate::runs::save(root, &r);
            }
        }
        n += 1;
    }
    if n > 0 {
        let _ = save_all(root, &all);
    }
    n
}

fn path(root: &Path) -> PathBuf {
    root.join("runs").join("changes.json")
}

pub fn list(root: &Path) -> Vec<Change> {
    crate::paths::read_json::<Vec<Change>>(&path(root)).unwrap_or_default()
}

fn save_all(root: &Path, all: &[Change]) -> Result<(), String> {
    let _ = fs::create_dir_all(root.join("runs"));
    crate::paths::write_atomic(&path(root), serde_json::to_string_pretty(all).map_err(|e| e.to_string())?.as_bytes())
}

fn update<F: FnOnce(&mut Change)>(root: &Path, id: &str, f: F) -> Option<Change> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut all = list(root);
    let c = all.iter_mut().find(|c| c.id == id)?;
    f(c);
    let out = c.clone();
    let _ = save_all(root, &all);
    Some(out)
}

/// Which agent a plan section belongs to.
pub fn agent_for_section(section: &str) -> &'static str {
    match section {
        "ui" | "accessibility" => "frame",
        "world" => "scenic",
        "characters" | "cosmetics" => "modeler",
        "animations" => "motion",
        "sounds" => "librarian",
        "monetization" => "merchant",
        "retention" => "liveops",
        "core-loop" | "style-kit" | "tech" => "architect",
        _ => "planner",
    }
}

/// Which agent a free-text request on a built game belongs to.
pub fn agent_for_text(text: &str) -> &'static str {
    let t = format!(" {} ", text.to_lowercase());
    let any = |ws: &[&str]| ws.iter().any(|w| t.contains(w));
    if any(&["lag", "slow", "fps", "performance", "memory", "stutter", "frame rate"]) {
        "tuner"
    } else if any(&["thumbnail", "game icon", "cover art", "store image"]) {
        "splash"
    } else if any(&["sound", "music", "sfx", "audio", "volume"]) {
        "librarian"
    } else if any(&["animation", "animate", " vfx", "effect", "particle", "tween", "juice", "shake", "bounce", "glow", "sparkle", "shine", "glitter", "flash", "trail", "pop "]) {
        "motion"
    } else if any(&["button", " ui", "ui ", "menu", "hud", "screen", "font", "popup", "pop-up", "text size", "layout", "icon"]) {
        "frame"
    } else if any(&["map", "world", "lighting", "terrain", "zone", "island", " sky", "fog", "biome", "spawn area", "decor"]) {
        "scenic"
    } else if any(&["model", "mesh", " hat", "cape", "prop", "pet look", "character look"]) {
        "modeler"
    } else if any(&["price", "gamepass", "game pass", "robux", "dev product", "monetiz"]) {
        "merchant"
    } else if any(&["npc", "enemy ai", "pathfind", "crowd", "mob "]) {
        "crowd"
    } else {
        "builder"
    }
}

/// The plan, its item and the item's section, for a plan change.
fn plan_item(root: &Path, plan_run: &str, item: &str) -> Result<(Value, Value, String), String> {
    let plan = crate::plan::list(root).into_iter().find(|p| p["run_id"] == plan_run).ok_or("plan not found")?;
    for s in plan["sections"].as_array().into_iter().flatten() {
        for it in s["items"].as_array().into_iter().flatten() {
            if it["id"] == item {
                return Ok((plan.clone(), it.clone(), s["id"].as_str().unwrap_or("").to_string()));
            }
        }
    }
    Err(format!("no item {item} in that plan"))
}

fn check_target(root: &Path, target: &str) -> Result<(), String> {
    if let Some(p) = target.strip_prefix("project:") {
        if !crate::paths::valid_id(p) || !root.join("projects").join(p).join("default.project.json").is_file() {
            return Err(format!("no game project named {p}"));
        }
        Ok(())
    } else if let Some(g) = target.strip_prefix("game:") {
        if !crate::paths::valid_id(g) {
            return Err("bad game id".into());
        }
        Ok(())
    } else if let Some(r) = target.strip_prefix("plan:") {
        if !crate::paths::valid_id(r) {
            return Err("bad plan id".into());
        }
        Ok(())
    } else {
        Err("a change needs a game or a plan".into())
    }
}

/// The instruction an agent gets for one change.
pub fn prompt_for(root: &Path, target: &str, item: Option<&str>, text: &str) -> Result<String, String> {
    let head = "This is ONE change request from Andrew, not a full job. Do only this change and keep everything else as it is. Skip your usual full report.";
    let tail = "Then write change.md in your run folder: what you changed and why, in 3 to 8 short lines (first line: one-sentence summary), and gate.json with your checks.";
    if let Some(p) = target.strip_prefix("project:") {
        let notes = root.join("projects").join(p).join("docs/notes-from-andrew.md");
        return Ok(format!(
            "{head}\n\nGame: projects/{p}\nChange request: {text}\n\nEdit the files in projects/{p} directly.{} If you touch Luau code, make sure the game still builds (`brewix gate build projects/{p}` when you have it). Never publish or upload anything.\n\n{tail}",
            if notes.is_file() { format!(" Read projects/{p}/docs/notes-from-andrew.md first.") } else { String::new() }
        ));
    }
    let plan_run = target.strip_prefix("plan:").unwrap_or("");
    let item = item.ok_or("a plan change needs an item")?;
    let (plan, it, sec) = plan_item(root, plan_run, item)?;
    Ok(format!(
        "{head}\n\nGame plan: runs/{plan_run}/plan.json (\"{}\"), section \"{sec}\", item \"{item}\": {}\n{}Change request: {text}\n\nDo NOT edit plan.json; the app merges your answer. Write change.json in your run folder, exactly: {{\"item\": \"{item}\", \"text\": \"<the new item text, one or two sentences>\", \"mock\": null}}. If the change affects how the item looks{}, draw a new SVG mockup at mockups/{item}.svg in your run folder and set \"mock\": \"mockups/{item}.svg\".\n\n{tail}",
        plan["title"].as_str().unwrap_or(""),
        it["text"].as_str().unwrap_or(""),
        it["mock"].as_str().map(|m| format!("Its current mockup: runs/{plan_run}/{m}\n")).unwrap_or_default(),
        if it["mock"].is_string() { " (it has a mockup)" } else { "" }
    ))
}

/// One request as Andrew writes it.
#[derive(Deserialize, Clone, Debug)]
pub struct Request {
    #[serde(default)]
    pub item: Option<String>,
    pub text: String,
    /// Empty or "auto" picks the agent.
    #[serde(default)]
    pub agent: Option<String>,
}

fn pick_agent(root: &Path, target: &str, r: &Request) -> Result<String, String> {
    if let Some(a) = r.agent.as_deref().filter(|a| !a.is_empty() && *a != "auto") {
        return Ok(a.to_string());
    }
    if let (Some(p), Some(it)) = (target.strip_prefix("plan:"), r.item.as_deref()) {
        let (_, _, sec) = plan_item(root, p, it)?;
        return Ok(agent_for_section(&sec).to_string());
    }
    Ok(agent_for_text(&r.text).to_string())
}

/// Who would do each request and what it may cost, before sending.
pub fn preview(root: &Path, target: &str, reqs: &[Request]) -> Result<Value, String> {
    check_target(root, target)?;
    let agents = crate::cost::load_agents(root)?;
    let pricing = crate::cost::load_pricing(root)?;
    let free = crate::settings::load().engine == "ollama";
    let mut rows = vec![];
    let mut total = 0.0;
    for r in reqs {
        let agent = pick_agent(root, target, r)?;
        let prof = agents.agents.get(&agent).ok_or_else(|| format!("unknown agent {agent}"))?;
        let prompt = prompt_for(root, target, r.item.as_deref(), &r.text)?;
        let e = crate::cost::estimate(&agent, prof, &prompt, &pricing)?;
        let cap = if free { 0.0 } else { e.cap_usd };
        total += cap;
        rows.push(json!({ "agent": agent, "cap_usd": cap, "expected_usd": if free { 0.0 } else { e.expected_usd } }));
    }
    Ok(json!({ "rows": rows, "total_cap_usd": total }))
}

/// Send requests: each becomes an approved run for its agent, queued.
/// Pressing Send is the approval of the caps shown by `preview`.
pub fn submit(root: &Path, target: &str, reqs: &[Request]) -> Result<Vec<Change>, String> {
    check_target(root, target)?;
    let reqs: Vec<&Request> = reqs.iter().filter(|r| !r.text.trim().is_empty()).collect();
    if reqs.is_empty() {
        return Err("Write a change request first.".into());
    }
    let mut made = vec![];
    for r in reqs {
        let agent = pick_agent(root, target, r)?;
        let prompt = prompt_for(root, target, r.item.as_deref(), r.text.trim())?;
        let mut req = crate::runs::prepare(root, &agent, &prompt)?;
        let id = format!("chg-{}", req.run_id.trim_start_matches("run-"));
        req.change = Some(id.clone());
        crate::runs::save(root, &req)?;
        if let Err(e) = crate::runs::approve(root, &req.run_id, req.estimate.cap_usd) {
            let _ = crate::runs::cancel(root, &req.run_id);
            return Err(if made.is_empty() { e } else { format!("Sent {} of the changes, then: {e}", made.len()) });
        }
        let c = Change {
            id,
            target: target.into(),
            item: r.item.clone(),
            text: r.text.trim().into(),
            agent,
            status: "queued".into(),
            run_id: req.run_id.clone(),
            created_at: crate::now_rfc3339(),
            finished_at: None,
            summary: None,
            checks: vec![],
            cost_usd: None,
            kind: "change".into(),
            autopilot: None,
            milestone: None,
            key: None,
            attempt: 0,
            source: "andrew".into(),
            parent_run: None,
            group: None,
            depth: 0,
            after: None,
            not_before: None,
        };
        {
            let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let mut all = list(root);
            all.push(c.clone());
            save_all(root, &all)?;
        }
        made.push(c);
    }
    Ok(made)
}

/// Changes to start now: queued ones, oldest first, while fewer than
/// MAX_AT_ONCE are working and no change for the same agent and target is.
/// The picked ones are marked working before this returns.
pub fn take_next(root: &Path) -> Vec<Change> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut all = list(root);
    let mut busy: Vec<(String, String)> = all.iter().filter(|c| c.status == "working").map(|c| (c.agent.clone(), c.target.clone())).collect();
    let mut out = vec![];
    let finished: std::collections::HashSet<String> = all.iter().filter(|c| ["done", "failed", "cancelled"].contains(&c.status.as_str())).map(|c| c.id.clone()).collect();
    for c in all.iter_mut().filter(|c| c.status == "queued") {
        if busy.len() >= MAX_AT_ONCE {
            break;
        }
        if c.after.as_ref().map(|a| !finished.contains(a)).unwrap_or(false) {
            continue;
        }
        if c.not_before.as_deref().and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()).map(|t| t > chrono::Utc::now()).unwrap_or(false) {
            continue;
        }
        if busy.iter().any(|(a, t)| *a == c.agent && *t == c.target) {
            continue;
        }
        c.status = "working".into();
        busy.push((c.agent.clone(), c.target.clone()));
        out.push(c.clone());
    }
    if !out.is_empty() {
        let _ = save_all(root, &all);
    }
    out
}

/// Folders the agent may read besides its own: the plan's run folder.
pub fn extra_dirs(root: &Path, c: &Change) -> Vec<PathBuf> {
    if let Some(a) = &c.autopilot {
        return crate::autopilot::context_dirs(root, a);
    }
    if let Some(p) = &c.parent_run {
        // The agent who asked, and every finished job of the same hand-off.
        let mut v = vec![root.join("runs").join(p)];
        if let Some(g) = &c.group {
            v.push(root.join("runs").join(g));
            for x in list(root).into_iter().filter(|x| x.group.as_ref() == Some(g) && x.status == "done") {
                v.push(root.join("runs").join(&x.run_id));
            }
        }
        v.dedup();
        return v;
    }
    c.target.strip_prefix("plan:").map(|p| vec![root.join("runs").join(p)]).unwrap_or_default()
}

/// After the run: check the work, merge plan changes, record the result.
pub fn finish(root: &Path, change_id: &str) -> Option<Change> {
    let c = list(root).into_iter().find(|c| c.id == change_id)?;
    let req = crate::runs::load(root, &c.run_id).ok()?;
    if req.status == "paused" {
        return update(root, change_id, |x| x.status = "paused".into());
    }
    if req.status == "cancelled" {
        return update(root, change_id, |x| {
            x.status = "cancelled".into();
            x.summary = Some("Stopped by you.".into());
            x.finished_at = Some(crate::now_rfc3339());
        });
    }
    let dir = crate::runs::run_dir(root, &c.run_id).ok()?;
    let note = fs::read_to_string(dir.join("change.md")).unwrap_or_default();
    let mut checks = vec![
        crate::queue::GateCheck { name: "the agent finished".into(), passed: req.status == "done", detail: req.error.clone().unwrap_or_else(|| req.status.clone()) },
        crate::queue::GateCheck { name: "wrote change.md".into(), passed: !note.trim().is_empty(), detail: String::new() },
    ];
    let agents = crate::cost::load_agents(root).ok();
    if let Some(prof) = agents.as_ref().and_then(|a| a.agents.get(&c.agent)) {
        let (ok, detail) = crate::pipeline::loads_within(root, &dir, &c.agent, &prof.allowed_sections, &extra_dirs(root, &c));
        checks.push(crate::queue::GateCheck { name: "loaded only allowed vault sections".into(), passed: ok, detail });
    }
    if c.kind == "task" {
        // A full job: the agent's own output and checks, like a pipeline stage.
        let slug = c.autopilot.clone().or_else(|| c.target.strip_prefix("game:").map(String::from)).unwrap_or_default();
        let project = root.join("projects").join(&slug).join("default.project.json").is_file().then(|| format!("projects/{slug}"));
        if let Some(prof) = agents.as_ref().and_then(|a| a.agents.get(&c.agent)) {
            if let Some(out) = prof.output.clone() {
                let g = crate::pipeline::evaluate_stage(root, &dir, &c.agent, Some(prof), &out, &extra_dirs(root, &c), project.as_deref());
                checks.retain(|k| k.name == "the agent finished");
                checks.extend(g.checks);
            }
        }
    } else if let Some(p) = c.target.strip_prefix("project:").or_else(|| c.target.strip_prefix("game:").filter(|p| root.join("projects").join(p).join("default.project.json").is_file())) {
        // Code changed: the game must still build.
        if ["builder", "motion", "tuner", "frame", "crowd", "merchant"].contains(&c.agent.as_str()) && req.status == "done" {
            checks.extend(crate::gates::build_gate(root, &format!("projects/{p}")));
        }
    } else if let (Some(plan_run), Some(item)) = (c.target.strip_prefix("plan:"), c.item.as_deref()) {
        let applied = if req.status == "done" { crate::plan::apply_change(root, plan_run, item, &dir, change_id) } else { Err("the agent did not finish".into()) };
        checks.push(crate::queue::GateCheck { name: "merged into the plan".into(), passed: applied.is_ok(), detail: applied.err().unwrap_or_default() });
    }
    let ok = checks.iter().all(|x| x.passed);
    let note = if note.trim().is_empty() && c.kind == "task" {
        agents.as_ref().and_then(|a| a.agents.get(&c.agent)).and_then(|p| p.output.clone()).and_then(|o| fs::read_to_string(dir.join(o)).ok()).unwrap_or_default()
    } else {
        note
    };
    let summary = note.lines().map(|l| l.trim_start_matches('#').trim()).find(|l| !l.is_empty()).map(|l| l.chars().take(240).collect());
    update(root, change_id, |x| {
        x.status = if ok { "done".into() } else { "failed".into() };
        x.finished_at = Some(crate::now_rfc3339());
        x.summary = summary.or_else(|| req.error.clone());
        x.checks = checks;
        x.cost_usd = req.actual_cost_usd;
    })
}

pub const MAX_REQUESTS_PER_RUN: usize = 8;
pub const MAX_DEPTH: u32 = 3;

/// After any run (outside an autopilot, which handles its own): start the
/// agents its agent asked for in requests.json. Returns the jobs started,
/// and notes on anything skipped.
pub fn ingest_requests(root: &Path, run_id: &str, parent: Option<&Change>) -> (Vec<Change>, Vec<String>) {
    let mut notes = vec![];
    let Ok(req) = crate::runs::load(root, run_id) else { return (vec![], notes) };
    if req.status != "done" {
        return (vec![], notes);
    }
    let Ok(dir) = crate::runs::run_dir(root, run_id) else { return (vec![], notes) };
    let Some(v) = fs::read_to_string(dir.join("requests.json")).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else { return (vec![], notes) };
    let asks: Vec<Value> = if v.is_array() { v.as_array().cloned().unwrap_or_default() } else { vec![v] };
    let depth = parent.map(|p| p.depth + 1).unwrap_or(1);
    if depth > MAX_DEPTH {
        notes.push(format!("{} asked for more agents, but this hand-off chain is already {MAX_DEPTH} deep; skipped.", req.agent));
        return (vec![], notes);
    }
    let group = parent.and_then(|p| p.group.clone()).unwrap_or_else(|| run_id.to_string());
    let agents = crate::cost::load_agents(root).ok();
    let mut made: Vec<Change> = vec![];
    for ask in asks.iter().take(MAX_REQUESTS_PER_RUN) {
        if let Some(stop) = ask["stop"].as_str() {
            let n = cancel_queued(root, |x| x.group.as_deref() == Some(&group) && x.agent == stop);
            notes.push(format!("{} stopped {n} queued job(s) of {stop}.", req.agent));
            continue;
        }
        let (Some(to), Some(task)) = (ask["to"].as_str(), ask["task"].as_str().map(str::trim).filter(|t| !t.is_empty())) else { continue };
        if ["assistant", "bridge-test"].contains(&to) || !agents.as_ref().map(|a| a.agents.contains_key(to)).unwrap_or(false) {
            notes.push(format!("{} asked for \"{to}\", which is not an agent; skipped.", req.agent));
            continue;
        }
        let game = ask["game"].as_str().or_else(|| ask["project"].as_str().map(|p| p.trim_start_matches("projects/"))).filter(|g| crate::paths::valid_id(g)).map(String::from)
            .or_else(|| parent.and_then(|p| p.target.strip_prefix("game:").or_else(|| p.target.strip_prefix("project:")).map(String::from)));
        let target = format!("game:{}", game.clone().unwrap_or_else(|| "studio".into()));
        let after = ask["after"].as_str().and_then(|a| made.iter().rev().find(|m| m.agent == a).map(|m| m.id.clone()));
        let out = agents.as_ref().and_then(|a| a.agents.get(&req.agent)).and_then(|p| p.output.clone()).unwrap_or_else(|| "its output".into());
        let prompt = format!(
            "{from} handed you this job (from runs/{run_id}/{out}; read it first, and the rest of that folder as needed):\n{task}\nWhy: {why}\n{game_line}\nDo your job as your skill says: your usual output in your run folder plus gate.json. Also write change.md: what you did in 3 to 8 lines (first line a one-sentence summary).",
            from = cap_first(&req.agent),
            why = ask["why"].as_str().unwrap_or("not given"),
            game_line = game.as_ref().map(|g| format!("Game: projects/{g} (it may not exist yet; create it there if your job builds it).")).unwrap_or_default(),
        );
        match enqueue(root, Spec {
            target,
            text: task.chars().take(300).collect(),
            prompt,
            agent: to.into(),
            kind: "task".into(),
            autopilot: None,
            milestone: None,
            key: None,
            attempt: 0,
            source: format!("agent:{}", req.agent),
            model: None,
            parent_run: Some(run_id.into()),
            group: Some(group.clone()),
            depth,
            after,
        }) {
            Ok(c) => made.push(c),
            Err(e) => {
                notes.push(format!("Could not start {to}: {e}"));
                break; // usually the monthly budget
            }
        }
    }
    if asks.len() > MAX_REQUESTS_PER_RUN {
        notes.push(format!("{} asked for {} agents; started the first {MAX_REQUESTS_PER_RUN}.", req.agent, asks.len()));
    }
    (made, notes)
}

fn cap_first(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default()
}

/// When a job failed because of the computer, the network or the Claude
/// sign-in (not the agent's work): which one. Such jobs are retried by
/// themselves and never use up the agent's tries.
pub fn env_problem(c: &Change) -> Option<&'static str> {
    if c.status != "failed" {
        return None;
    }
    let text = c.checks.iter().filter(|k| !k.passed).map(|k| k.detail.to_lowercase()).chain(c.summary.iter().map(|s| s.to_lowercase())).collect::<Vec<_>>().join(" ");
    let any = |ws: &[&str]| ws.iter().any(|w| text.contains(w));
    if any(&["not logged in", "please run /login", "invalid api key", "authentication_error", "oauth token has expired", "token expired", "401 "]) {
        Some("signin")
    } else if any(&["could not start", "os error", "claude code was not found", "no such file or directory"]) {
        Some("launch")
    } else if any(&["no user query found"]) {
        // Ollama cut the request off (window too small); not the network.
        Some("localmodel")
    } else if any(&["connection error", "econnrefused", "econnreset", "enotfound", "getaddrinfo", "network error", "timed out", "overloaded", "api error: 5", "internal server error", "service unavailable", "bad gateway"]) {
        Some("network")
    } else {
        None
    }
}

/// True when the job failed for a reason outside the agent's work.
pub fn launch_failed(c: &Change) -> bool {
    env_problem(c).is_some()
}

/// Andrew stops or pauses one agent's job. A working job ends its Claude
/// process now (files are kept); a waiting one never starts. Paused jobs
/// continue from where they stopped with Resume (continue_now on the run).
pub fn stop_or_pause(root: &Path, id: &str, pause: bool) -> Result<Change, String> {
    let c = list(root).into_iter().find(|c| c.id == id || c.run_id == id).ok_or("no such job")?;
    match c.status.as_str() {
        "working" => {
            if !crate::bridge::stop_run(&c.run_id, pause) {
                return Err("That agent is just starting or finishing; try again in a moment.".into());
            }
            Ok(c)
        }
        "queued" => {
            if pause {
                let mut r = crate::runs::load(root, &c.run_id)?;
                r.status = "paused".into();
                r.resume_at = Some("9999-12-31T00:00:00Z".into());
                r.error = Some("Paused by you before it started.".into());
                crate::runs::save(root, &r)?;
            }
            update(root, &c.id, |x| {
                x.status = if pause { "paused" } else { "cancelled" }.into();
                if !pause {
                    x.summary = Some("Stopped by you before it started.".into());
                    x.finished_at = Some(crate::now_rfc3339());
                }
            })
            .ok_or_else(|| "no such job".into())
        }
        s => Err(format!("That job is {s}; there is nothing to stop.")),
    }
}

/// Hold a queued change until `minutes` from now.
pub fn delay(root: &Path, id: &str, minutes: i64) {
    update(root, id, |x| x.not_before = Some((chrono::Utc::now() + chrono::Duration::minutes(minutes)).to_rfc3339()));
}

/// Outside an autopilot: a job whose Claude could not start is queued
/// again by itself, waiting a little longer each time (1, 5, 15, then
/// every 30 minutes). Returns the new job.
pub fn retry_launch(root: &Path, c: &Change) -> Option<Change> {
    if !launch_failed(c) || c.autopilot.is_some() {
        return None;
    }
    let tries = list(root).iter().filter(|x| x.text == c.text && x.target == c.target && x.agent == c.agent && launch_failed(x)).count();
    let prompt = crate::runs::load(root, &c.run_id).ok()?.prompt;
    let n = enqueue(root, Spec {
        target: c.target.clone(),
        text: c.text.clone(),
        prompt,
        agent: c.agent.clone(),
        kind: c.kind.clone(),
        autopilot: None,
        milestone: None,
        key: c.key.clone(),
        attempt: c.attempt,
        source: c.source.clone(),
        model: None,
        parent_run: c.parent_run.clone(),
        group: c.group.clone(),
        depth: c.depth,
        after: None,
    }).ok()?;
    delay(root, &n.id, backoff_minutes(tries));
    list(root).into_iter().find(|x| x.id == n.id)
}

pub fn backoff_minutes(tries: usize) -> i64 {
    [1, 5, 15, 30][tries.saturating_sub(1).min(3)]
}

/// Record a result directly (tests, and changes whose run cannot start).
pub fn set_result(root: &Path, id: &str, status: &str, checks: Vec<crate::queue::GateCheck>) {
    update(root, id, |x| {
        x.status = status.into();
        x.checks = checks;
        x.finished_at = Some(crate::now_rfc3339());
    });
}

/// A change whose run was paused by the usage limit goes back to working.
pub fn resumed(root: &Path, change_id: &str) {
    update(root, change_id, |x| x.status = "working".into());
}

/// At startup: changes left working had their run cut off (the run itself
/// is paused or failed by runs::recover_stale); follow the run's state.
pub fn recover(root: &Path) {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut all = list(root);
    let mut changed = false;
    for c in all.iter_mut().filter(|c| c.status == "working" || c.status == "queued") {
        let st = crate::runs::load(root, &c.run_id).map(|r| r.status).unwrap_or_else(|_| "failed".into());
        let new = match st.as_str() {
            "paused" => "paused",
            "approved" => "queued",
            "running" => continue,
            _ if c.status == "queued" => continue,
            _ => "failed",
        };
        if c.status != new {
            c.status = new.into();
            changed = true;
        }
    }
    if changed {
        let _ = save_all(root, &all);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routes_requests_to_the_right_agent() {
        assert_eq!(agent_for_text("make the buy button bigger"), "frame");
        assert_eq!(agent_for_text("the spawn island needs more fog and warmer lighting"), "scenic");
        assert_eq!(agent_for_text("add a little bounce animation when a coin is picked up"), "motion");
        assert_eq!(agent_for_text("the game lags on phones"), "tuner");
        assert_eq!(agent_for_text("Make the stars sparkle when they are ready to harvest"), "motion");
        assert_eq!(agent_for_text("lower the price of the VIP gamepass"), "merchant");
        assert_eq!(agent_for_text("double jump should reset on walls"), "builder");
        assert_eq!(agent_for_section("ui"), "frame");
        assert_eq!(agent_for_section("world"), "scenic");
        assert_eq!(agent_for_section("risks"), "planner");
    }

    fn change(id: &str, agent: &str, target: &str, status: &str) -> Change {
        Change { id: id.into(), target: target.into(), item: None, text: "x".into(), agent: agent.into(), status: status.into(), run_id: format!("run-{id}"), created_at: id.into(), finished_at: None, summary: None, checks: vec![], cost_usd: None, kind: "change".into(), autopilot: None, milestone: None, key: None, attempt: 0, source: "andrew".into(), parent_run: None, group: None, depth: 0, after: None, not_before: None }
    }

    #[test]
    fn runs_up_to_three_at_once_and_one_per_agent_and_game() {
        let d = std::env::temp_dir().join(format!("chg-t-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("runs")).unwrap();
        let all = vec![
            change("1", "frame", "project:a", "queued"),
            change("2", "frame", "project:a", "queued"),
            change("3", "scenic", "project:a", "queued"),
            change("4", "frame", "project:b", "queued"),
            change("5", "motion", "project:a", "queued"),
        ];
        save_all(&d, &all).unwrap();
        let first: Vec<String> = take_next(&d).into_iter().map(|c| c.id).collect();
        assert_eq!(first, vec!["1", "3", "4"], "two Frames on the same game never run together; at most 3");
        assert!(take_next(&d).is_empty(), "all three slots busy");
        update(&d, "1", |c| c.status = "done".into());
        let next: Vec<String> = take_next(&d).into_iter().map(|c| c.id).collect();
        assert_eq!(next, vec!["2"], "the second Frame change waits for the first, then goes before later ones");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn lead_starts_the_team_in_order_and_chains_stop() {
        let d = std::env::temp_dir().join(format!("handoff-t-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("studio")).unwrap();
        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for f in ["agents.json", "pricing.json", "pipelines.json"] {
            fs::copy(src.join("studio").join(f), d.join("studio").join(f)).unwrap();
        }
        fs::create_dir_all(d.join("vault/.claude/agents")).unwrap();
        for e in fs::read_dir(src.join("vault/.claude/agents")).unwrap().flatten() {
            fs::copy(e.path(), d.join("vault/.claude/agents").join(e.file_name())).unwrap();
        }
        // Lead finished and asked for Architect, then Builder after it.
        let lead = crate::runs::prepare(&d, "lead", "Make a tycoon").unwrap();
        let mut r = crate::runs::load(&d, &lead.run_id).unwrap();
        r.status = "done".into();
        crate::runs::save(&d, &r).unwrap();
        let ld = crate::runs::run_dir(&d, &lead.run_id).unwrap();
        fs::write(ld.join("requests.json"), r#"[{"to": "architect", "task": "Design doc from plan.md", "why": "needed first", "game": "candy-tycoon"},
            {"to": "builder", "task": "Build it", "why": "the game", "after": "architect", "game": "candy-tycoon"},
            {"to": "wizard", "task": "x"}]"#).unwrap();
        let (made, notes) = ingest_requests(&d, &lead.run_id, None);
        assert_eq!(made.iter().map(|c| c.agent.as_str()).collect::<Vec<_>>(), vec!["architect", "builder"]);
        assert!(notes.iter().any(|n| n.contains("wizard")), "{notes:?}");
        assert_eq!(made[0].target, "game:candy-tycoon");
        assert_eq!(made[1].after.as_deref(), Some(made[0].id.as_str()));
        assert_eq!(made[0].source, "agent:lead");
        assert!(extra_dirs(&d, &made[0]).iter().any(|p| p.ends_with(&lead.run_id)), "the architect may read Lead's folder");
        let prompt = crate::runs::load(&d, &made[0].run_id).unwrap().prompt;
        assert!(prompt.contains(&format!("runs/{}/plan.md", lead.run_id)), "{prompt}");
        // Builder waits for Architect.
        let first: Vec<String> = take_next(&d).into_iter().map(|c| c.agent).collect();
        assert_eq!(first, vec!["architect"]);
        set_result(&d, &made[0].id, "done", vec![]);
        let next: Vec<String> = take_next(&d).into_iter().map(|c| c.agent).collect();
        assert_eq!(next, vec!["builder"]);
        // A chain three hand-offs deep asks for more: skipped.
        let mut deep = made[1].clone();
        deep.depth = MAX_DEPTH;
        let mut br = crate::runs::load(&d, &deep.run_id).unwrap();
        br.status = "done".into();
        crate::runs::save(&d, &br).unwrap();
        fs::write(crate::runs::run_dir(&d, &deep.run_id).unwrap().join("requests.json"), r#"{"to": "frame", "task": "more"}"#).unwrap();
        let (m2, n2) = ingest_requests(&d, &deep.run_id, Some(&deep));
        assert!(m2.is_empty() && n2[0].contains("deep"), "{n2:?}");
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn tells_computer_network_and_signin_problems_from_bad_work() {
        let mut c = Change { id: "x".into(), target: "game:g".into(), item: None, text: "t".into(), agent: "frame".into(), status: "failed".into(), run_id: "r".into(), created_at: "1".into(), finished_at: None, summary: None, checks: vec![], cost_usd: None, kind: "task".into(), autopilot: None, milestone: None, key: None, attempt: 0, source: "andrew".into(), parent_run: None, group: None, depth: 0, after: None, not_before: None };
        let with = |c: &mut Change, d: &str| c.checks = vec![crate::queue::GateCheck { name: "the agent finished".into(), passed: false, detail: d.into() }];
        with(&mut c, "could not start D:\\claude.exe: The filename or extension is too long. (os error 206)");
        assert_eq!(env_problem(&c), Some("launch"));
        with(&mut c, "claude reported an error (exit Some(1)): API Error: Connection error.");
        assert_eq!(env_problem(&c), Some("network"));
        with(&mut c, "claude reported an error (exit Some(1)): Invalid API key · Please run /login");
        assert_eq!(env_problem(&c), Some("signin"));
        with(&mut c, "first 60 seconds covers 0-5, 5-10, 10-30, 30-60");
        assert_eq!(env_problem(&c), None, "bad work is the agent's to fix");
    }
}
