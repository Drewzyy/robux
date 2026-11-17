//! Autopilot: once Andrew approves an idea (with one budget for the whole
//! game), the agents make it milestone by milestone without him pressing
//! anything. Andrew supervises, chimes in with notes whenever he likes, and
//! only approves finished milestones (or sends notes for a revision).
//!
//! - Work runs through the change queue (changes.rs): up to 3 agents at once,
//!   paused by Claude's usage limit and picked up again after a restart.
//! - A job that fails its checks is retried with the failures as feedback:
//!   twice with the agent's model, twice with a stronger one, then it is
//!   marked stuck (Andrew is told) while everything else keeps going.
//! - Agents talk to each other through requests.json in their run folder:
//!   {"to": "<agent>", "task": "...", "why": "..."} starts another agent,
//!   {"stop": "<agent>", "why": "..."} cancels that agent's queued work.
//! - While a milestone waits for review the next one already runs (except
//!   after the plan, which everything else depends on).
//! - Nothing is ever published or uploaded.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

use crate::changes::{self, Change, Spec};

pub const MAX_ATTEMPTS: u32 = 4;
pub const STRONG_MODEL: &str = "claude-opus-5-5";
/// Extra jobs agents may start per milestone (stops runaway chains).
pub const MAX_AGENT_REQUESTS: usize = 6;
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The milestones every game goes through: (id, title, what Andrew reviews,
/// waves of (agent, job)). A wave starts when the one before is done.
pub fn template() -> Vec<(&'static str, &'static str, &'static str, Vec<Vec<(&'static str, &'static str)>>)> {
    vec![
        ("plan", "Game plan", "The full plan with mockups (Game Plan tab).", vec![vec![("planner", "Write the full game plan with mockups (plan.json, plan.md, mockups/) for the idea below.")]]),
        ("design", "Design and player walkthrough", "The design doc, the identity check and a written walkthrough of a player's first 15 minutes.", vec![
            vec![("architect", "Turn the approved plan into the one-page game design doc (design.md): style kit, core loop, first 60 seconds, what we will not do.")],
            vec![
                ("director", "Check the plan and design for one clear hook and a consistent look; cut anything generic or off-theme (direction.md)."),
                ("simulator", "No build exists yet: from the plan and design, write a text walkthrough of the first 15 minutes for players of different ages (what they see, do and feel minute by minute, where they would quit), with a rating and fixes (simulation.md)."),
            ],
        ]),
        ("ui", "UI drafts", "Every screen as a spec and SVG mockups.", vec![vec![("frame", "Draft the whole UI from the plan and design: every screen in ui.md and an SVG mockup per screen in mockups/ in your run folder. No code yet; the build milestone builds it.")]]),
        ("world", "World, characters and sounds", "Lighting and world layout, character and prop models, and the sound list.", vec![vec![
            ("scenic", "Draft the world: map layout, zones, lighting and mood from the plan and design (lighting.md). No code yet unless the project already exists."),
            ("modeler", "Make the characters, pets and props the plan needs: low-poly Blender scripts or CC0 models into projects/<game>/assets/models (models.md)."),
        ]]),
        ("build", "Rough build to test in Roblox Studio", "A playable rough build. Open it in Roblox Studio and play it.", vec![
            vec![("builder", "Build the game as a Rojo project in projects/<game> from the approved plan, design, UI drafts and world: the full core loop playable end to end. Declare 'Project: projects/<game>' in build.md.")],
            vec![
                ("frame", "Build the UI in projects/<game> from your UI drafts (phone first)."),
                ("motion", "Add the animation and feedback that make every action feel good in projects/<game>."),
                ("librarian", "Write projects/<game>/sounds.json: every sound the game needs, with a prompt for each."),
                ("merchant", "Write the launch shop for projects/<game>: products and prices, fair to kids."),
            ],
            vec![("playtest", "Playtest projects/<game> (code walk and headless checks; say what you could not test) and list every problem with a severity.")],
        ]),
        ("polish", "Polished and ready to publish", "Fixed problems, phone performance, simulated players, thumbnails and the release checklist. You publish it yourself.", vec![
            vec![
                ("builder", "Fix every blocker and major problem in the latest playtest report for projects/<game>."),
                ("tuner", "Make projects/<game> fast on phones within the studio's performance budgets."),
            ],
            vec![
                ("simulator", "Simulate players of different ages on projects/<game> (simulation.md)."),
                ("splash", "Make the icon and thumbnail comps for projects/<game>."),
            ],
            vec![("release", "Write the release checklist and notes for projects/<game>. Never publish or upload anything.")],
        ]),
    ]
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    /// waiting | working | review | revising | approved
    pub status: String,
    #[serde(default)]
    pub wave: usize,
    #[serde(default)]
    pub ready_at: Option<String>,
    #[serde(default)]
    pub approved_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Msg {
    pub at: String,
    /// "andrew", "autopilot" or an agent name
    pub from: String,
    #[serde(default)]
    pub to: Option<String>,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Autopilot {
    pub slug: String,
    pub title: String,
    pub idea: String,
    pub budget_usd: f64,
    /// running | paused | budget | done | stopped
    pub status: String,
    #[serde(default)]
    pub why: Option<String>,
    #[serde(default)]
    pub plan_run: Option<String>,
    pub milestones: Vec<Milestone>,
    #[serde(default)]
    pub board: Vec<Msg>,
    #[serde(default)]
    pub notes: Vec<String>,
    pub created_at: String,
}

fn dir(root: &Path) -> PathBuf {
    root.join("runs").join("autopilot")
}
fn file(root: &Path, slug: &str) -> PathBuf {
    dir(root).join(format!("{slug}.json"))
}

pub fn load(root: &Path, slug: &str) -> Result<Autopilot, String> {
    if !crate::paths::valid_id(slug) {
        return Err("bad game id".into());
    }
    crate::paths::read_json(&file(root, slug)).ok_or_else(|| format!("no autopilot for {slug}"))
}

fn save(root: &Path, a: &Autopilot) -> Result<(), String> {
    crate::paths::write_atomic(&file(root, &a.slug), serde_json::to_string_pretty(a).map_err(|e| e.to_string())?.as_bytes())
}

pub fn list(root: &Path) -> Vec<Autopilot> {
    let mut v: Vec<Autopilot> = fs::read_dir(dir(root)).into_iter().flatten().flatten().filter_map(|e| {
        let n = e.file_name().to_string_lossy().to_string();
        n.strip_suffix(".json").and_then(|s| load(root, s).ok())
    }).collect();
    v.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    v
}


fn post(a: &mut Autopilot, from: &str, to: Option<&str>, text: &str) {
    a.board.push(Msg { at: crate::now_rfc3339(), from: from.into(), to: to.map(String::from), text: text.chars().take(600).collect() });
    if a.board.len() > 400 {
        a.board.drain(..a.board.len() - 400);
    }
}

pub fn slugify(title: &str) -> String {
    let mut s: String = title.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    while s.contains("--") {
        s = s.replace("--", "-");
    }
    let s = s.trim_matches('-').chars().take(40).collect::<String>();
    if s.is_empty() { "game".into() } else { s }
}

/// What the whole game may cost: every planned job at its cap, doubled for
/// retries (some on a stronger model), notes and agents' own requests.
pub fn estimate(root: &Path, idea: &str) -> Result<Value, String> {
    let agents = crate::cost::load_agents(root)?;
    let pricing = crate::cost::load_pricing(root)?;
    let free = crate::settings::load().engine == "ollama";
    let mut per = vec![];
    let mut total = 0.0;
    for (id, title, _, waves) in template() {
        let mut m = 0.0;
        for (agent, job) in waves.into_iter().flatten() {
            let prof = agents.agents.get(agent).ok_or_else(|| format!("agent {agent} is missing"))?;
            m += crate::cost::estimate(agent, prof, &format!("{job}\n{idea}"), &pricing)?.cap_usd;
        }
        let m = if free { 0.0 } else { m };
        total += m;
        per.push(json!({ "id": id, "title": title, "cap_usd": m }));
    }
    let budget = (total * 2.0 * 100.0).ceil() / 100.0;
    Ok(json!({ "milestones": per, "jobs_usd": total, "budget_usd": budget }))
}

/// Andrew approves an idea: the autopilot starts with this budget.
pub fn start(root: &Path, title: &str, idea: &str, budget_usd: f64, slug: Option<&str>) -> Result<Autopilot, String> {
    if idea.trim().is_empty() {
        return Err("Write the idea first.".into());
    }
    let slug = slug.map(String::from).unwrap_or_else(|| slugify(title));
    if !crate::paths::valid_id(&slug) {
        return Err("bad game name".into());
    }
    if file(root, &slug).is_file() {
        return Err(format!("{slug} already has an autopilot"));
    }
    let a = Autopilot {
        slug: slug.clone(),
        title: title.trim().into(),
        idea: idea.trim().into(),
        budget_usd,
        status: "running".into(),
        why: None,
        plan_run: None,
        milestones: template().into_iter().map(|(id, t, _, _)| Milestone { id: id.into(), title: t.into(), status: "waiting".into(), wave: 0, ready_at: None, approved_at: None }).collect(),
        board: vec![Msg { at: crate::now_rfc3339(), from: "andrew".into(), to: None, text: format!("Approved the idea: {}", title.trim()) }],
        notes: vec![],
        created_at: crate::now_rfc3339(),
    };
    {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        save(root, &a)?;
    }
    advance(root, &slug);
    load(root, &slug)
}

fn mine(root: &Path, slug: &str) -> Vec<Change> {
    changes::list(root).into_iter().filter(|c| c.autopilot.as_deref() == Some(slug)).collect()
}

/// Run folders of this game's finished work, so agents can read each
/// other's drafts (and the checks allow those reads).
pub fn context_dirs(root: &Path, slug: &str) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = vec![];
    if let Ok(a) = load(root, slug) {
        if let Some(p) = a.plan_run {
            v.push(root.join("runs").join(p));
        }
    }
    for c in mine(root, slug).into_iter().filter(|c| c.status == "done") {
        v.push(root.join("runs").join(&c.run_id));
    }
    v
}

/// What every job is told: the game, the plan, earlier drafts, Andrew's notes
/// and how to talk to other agents.
fn prompt(root: &Path, a: &Autopilot, milestone: &str, job: &str, feedback: Option<&str>) -> String {
    let project = format!("projects/{}", a.slug);
    let job = job.replace("<game>", &a.slug);
    let mut drafts = vec![];
    for c in mine(root, &a.slug).into_iter().filter(|c| c.status == "done") {
        let out = crate::cost::load_agents(root).ok().and_then(|ag| ag.agents.get(&c.agent).and_then(|p| p.output.clone())).unwrap_or_else(|| "change.md".into());
        drafts.push(format!("- {} ({}): runs/{}/{} {}", c.agent, c.milestone.as_deref().unwrap_or(""), c.run_id, if c.kind == "task" { out } else { "change.md".into() }, if c.kind == "task" { "" } else { "(a change)" }));
    }
    let drafts = if drafts.is_empty() { String::new() } else { format!("\nEarlier work by the team (read what your job needs):\n{}\n", drafts.into_iter().rev().take(25).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n")) };
    let plan = a.plan_run.as_ref().map(|p| format!("The approved plan: runs/{p}/plan.json (readable: plan.md, mockups in mockups/).\n")).unwrap_or_default();
    let notes = if a.notes.is_empty() { String::new() } else { format!("\nAndrew's notes so far (follow them):\n{}\n", a.notes.iter().rev().take(12).map(|n| format!("- {n}")).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n")) };
    let fb = feedback.map(|f| format!("\nYOUR LAST ATTEMPT DID NOT PASS. Fix exactly this, keep what was fine:\n{f}\n")).unwrap_or_default();
    format!(
        "Autopilot job for the game \"{}\" (slug {}, folder {project}), milestone \"{milestone}\".\nThe idea Andrew approved: {}\n{plan}{drafts}{notes}{fb}\nYour job: {job}\n\nWork on your own; nobody can answer questions mid-run. If another agent must do something (or redo something) for the game to come out right, write requests.json in your run folder: [{{\"to\": \"<agent name>\", \"task\": \"<what to do>\", \"why\": \"<why>\"}}]. To stop another agent's queued work that would be wasted, add {{\"stop\": \"<agent name>\", \"why\": \"...\"}}. Agents: architect, builder, frame, scenic, motion, modeler, librarian, merchant, tuner, crowd, splash, playtest, simulator, director. Never publish or upload anything. Also write change.md: what you did in 3 to 8 lines (first line a one-sentence summary).",
        a.title, a.slug, a.idea
    )
}

fn spent(root: &Path, slug: &str) -> (f64, f64) {
    let mut spent = 0.0;
    let mut reserved = 0.0;
    for c in mine(root, slug) {
        match c.status.as_str() {
            "done" | "failed" | "paused" => spent += c.cost_usd.unwrap_or(0.0),
            "queued" | "working" => reserved += crate::runs::load(root, &c.run_id).map(|r| r.estimate.cap_usd).unwrap_or(0.0),
            _ => {}
        }
    }
    (spent, reserved)
}

/// Queue one job, unless it would go over this game's budget.
fn queue(root: &Path, a: &mut Autopilot, milestone: &str, agent: &str, job: &str, key: &str, attempt: u32, source: &str, feedback: Option<&str>, kind: &str) -> Result<Change, String> {
    let model = if attempt >= 2 { Some(STRONG_MODEL.to_string()) } else { None };
    let p = prompt(root, a, milestone, job, feedback);
    let (spent_usd, reserved) = spent(root, &a.slug);
    let agents = crate::cost::load_agents(root)?;
    let pricing = crate::cost::load_pricing(root)?;
    let mut prof = agents.agents.get(agent).cloned().ok_or_else(|| format!("unknown agent {agent}"))?;
    if let Some(m) = &model {
        prof.model = m.clone();
    }
    let cap = if crate::settings::load().engine == "ollama" { 0.0 } else { crate::cost::estimate(agent, &prof, &p, &pricing)?.cap_usd };
    if spent_usd + reserved + cap > a.budget_usd + 1e-9 {
        a.status = "budget".into();
        a.why = Some(format!("This game's budget (${:.2}) is used up: ${:.2} spent, ${:.2} running. Raise it to keep going.", a.budget_usd, spent_usd, reserved));
        post(a, "autopilot", None, a.why.clone().unwrap_or_default().as_str());
        crate::notify::send_quiet(crate::notify::Kind::Budget, &format!("{}: budget reached", a.title), &[a.why.clone().unwrap_or_default()]);
        return Err("budget".into());
    }
    let target = format!("game:{}", a.slug);
    let c = changes::enqueue(root, Spec {
        target,
        text: job.replace("<game>", &a.slug),
        prompt: p,
        agent: agent.into(),
        kind: kind.into(),
        autopilot: Some(a.slug.clone()),
        milestone: Some(milestone.into()),
        key: Some(key.into()),
        attempt,
        source: source.into(),
        model,
    });
    match c {
        Ok(c) => Ok(c),
        Err(e) => {
            // Usually the monthly budget: stop starting work until it is raised.
            a.status = "budget".into();
            a.why = Some(e.clone());
            post(a, "autopilot", None, &format!("Could not start {agent}: {e}"));
            Err(e)
        }
    }
}

/// Latest attempt of every piece of work in a milestone, by key.
fn latest(all: &[Change], milestone: &str) -> Vec<Change> {
    let mut by: std::collections::BTreeMap<String, Change> = Default::default();
    for c in all.iter().filter(|c| c.milestone.as_deref() == Some(milestone) && c.status != "cancelled") {
        let k = c.key.clone().unwrap_or_else(|| c.id.clone());
        let newer = by.get(&k).map(|o| (c.attempt, &c.created_at) >= (o.attempt, &o.created_at)).unwrap_or(true);
        if newer {
            by.insert(k, c.clone());
        }
    }
    by.into_values().collect()
}

fn stuck(c: &Change) -> bool {
    c.status == "failed" && c.attempt + 1 >= MAX_ATTEMPTS
}

/// Move a game forward: start milestones and waves whose turn has come,
/// and mark milestones ready for review. Safe to call any time.
pub fn advance(root: &Path, slug: &str) {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Ok(mut a) = load(root, slug) else { return };
    if a.status != "running" {
        return;
    }
    let tpl = template();
    let all = mine(root, slug);
    let n = a.milestones.len();
    for i in 0..n {
        let (mid, title, review, waves) = (&tpl[i].0, tpl[i].1, tpl[i].2, &tpl[i].3);
        let st = a.milestones[i].status.clone();
        if st == "waiting" {
            // Start once the one before is ready (or approved); the plan must
            // be approved before anything after it starts.
            let prev_ok = i == 0 || {
                let p = &a.milestones[i - 1];
                p.status == "approved" || (p.status == "review" && a.milestones[i - 1].id != "plan")
            };
            if !prev_ok {
                break;
            }
            a.milestones[i].status = "working".into();
            a.milestones[i].wave = 0;
            post(&mut a, "autopilot", None, &format!("Starting: {title}."));
            for (k, (agent, job)) in waves[0].iter().enumerate() {
                let key = format!("{mid}/0/{k}");
                let job = format!("{job}{}", if *mid == "plan" { format!("\n\nThe idea: {}\nUse the slug \"{}\" in plan.json.", a.idea, a.slug) } else { String::new() });
                if queue(root, &mut a, mid, agent, &job, &key, 0, "autopilot", None, "task").is_err() {
                    let _ = save(root, &a);
                    return;
                }
            }
            continue;
        }
        if st == "working" {
            // Any planned job of the current wave that never got queued (the
            // budget ran out at that moment) is queued now.
            let w = a.milestones[i].wave.min(waves.len() - 1);
            for (k, (agent, job)) in waves[w].iter().enumerate() {
                let key = format!("{mid}/{w}/{k}");
                if !all.iter().any(|c| c.key.as_deref() == Some(&key) && c.status != "cancelled") {
                    let job = format!("{job}{}", if *mid == "plan" { format!("\n\nThe idea: {}\nUse the slug \"{}\" in plan.json.", a.idea, a.slug) } else { String::new() });
                    if queue(root, &mut a, mid, agent, &job, &key, 0, "autopilot", None, "task").is_err() {
                        let _ = save(root, &a);
                        return;
                    }
                }
            }
        }
        let all = mine(root, slug);
        if st == "working" || st == "revising" {
            let lat = latest(&all, mid);
            let open = lat.iter().any(|c| ["queued", "working", "paused"].contains(&c.status.as_str()));
            let failed_stuck = lat.iter().any(stuck);
            let wave = a.milestones[i].wave;
            let wave_done = (0..waves[wave.min(waves.len() - 1)].len()).all(|k| lat.iter().any(|c| c.key.as_deref() == Some(&format!("{mid}/{wave}/{k}")) && c.status == "done"));
            if st == "working" && wave_done && wave + 1 < waves.len() {
                let w = wave + 1;
                a.milestones[i].wave = w;
                for (k, (agent, job)) in waves[w].iter().enumerate() {
                    if queue(root, &mut a, mid, agent, job, &format!("{mid}/{w}/{k}"), 0, "autopilot", None, "task").is_err() {
                        let _ = save(root, &a);
                        return;
                    }
                }
                continue;
            }
            let all_waves_done = a.milestones[i].wave + 1 >= waves.len() && wave_done;
            if !open && !failed_stuck && (all_waves_done || st == "revising") && lat.iter().all(|c| c.status == "done") {
                if mid == &"plan" {
                    if let Some(p) = lat.iter().find(|c| c.agent == "planner") {
                        a.plan_run = Some(p.run_id.clone());
                    }
                }
                a.milestones[i].status = "review".into();
                a.milestones[i].ready_at = Some(crate::now_rfc3339());
                post(&mut a, "autopilot", Some("andrew"), &format!("Ready for your review: {title}. {review}"));
                crate::notify::send_quiet(crate::notify::Kind::NeedsApproval, &format!("{}: {title} is ready", a.title), &[review.to_string(), "Approve it or send notes in BrewixGames (Make the Game, Autopilot).".into()]);
            }
        }
    }
    if a.milestones.iter().all(|m| m.status == "approved") {
        a.status = "done".into();
        post(&mut a, "autopilot", Some("andrew"), "Every milestone is approved. The game is ready for you to publish.");
    }
    let _ = save(root, &a);
}

/// After a job ends: retry it with feedback if it failed, start or stop
/// agents it asked for, then move the game forward.
pub fn on_finish(root: &Path, c: &Change) {
    let Some(slug) = c.autopilot.clone() else { return };
    {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let Ok(mut a) = load(root, &slug) else { return };
        let mid = c.milestone.clone().unwrap_or_default();
        let key = c.key.clone().unwrap_or_else(|| c.id.clone());
        match c.status.as_str() {
            "done" => {
                post(&mut a, &c.agent, None, &format!("Done: {}", c.summary.clone().unwrap_or_else(|| c.text.chars().take(120).collect())));
                // Messages to other agents.
                let dir = root.join("runs").join(&c.run_id);
                let asks: Vec<Value> = fs::read_to_string(dir.join("requests.json")).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).map(|v| if v.is_array() { v.as_array().cloned().unwrap_or_default() } else { vec![v] }).unwrap_or_default();
                let made_by_agents = mine(root, &slug).iter().filter(|x| x.milestone.as_deref() == Some(&mid) && x.source.starts_with("agent:") && x.attempt == 0).count();
                let mut room = MAX_AGENT_REQUESTS.saturating_sub(made_by_agents);
                for ask in asks.iter().take(8) {
                    if let Some(stop) = ask["stop"].as_str() {
                        // Only extra work (asked by agents or notes); planned steps still have to happen.
                        let n = changes::cancel_queued(root, |x| x.autopilot.as_deref() == Some(&slug) && x.agent == stop && x.source != "autopilot");
                        post(&mut a, &c.agent, Some(stop), &format!("Stop ({} queued job{} cancelled): {}", n, if n == 1 { "" } else { "s" }, ask["why"].as_str().unwrap_or("")));
                        continue;
                    }
                    let (Some(to), Some(task)) = (ask["to"].as_str(), ask["task"].as_str()) else { continue };
                    let known = crate::cost::load_agents(root).map(|ag| ag.agents.contains_key(to)).unwrap_or(false);
                    if !known || to == "assistant" || to == "release" && mid != "polish" {
                        post(&mut a, "autopilot", Some(&c.agent), &format!("Ignored a request for {to}: not an agent that can take work now."));
                        continue;
                    }
                    if room == 0 {
                        post(&mut a, "autopilot", Some(&c.agent), &format!("Did not start {to}: this milestone already has {MAX_AGENT_REQUESTS} jobs started by agents."));
                        continue;
                    }
                    room -= 1;
                    post(&mut a, &c.agent, Some(to), &format!("{task} (why: {})", ask["why"].as_str().unwrap_or("not given")));
                    let k = format!("{mid}/ask/{}", c.id);
                    let _ = queue(root, &mut a, &mid, to, task, &format!("{k}/{to}"), 0, &format!("agent:{}", c.agent), None, "change");
                }
            }
            "failed" => {
                let why: Vec<String> = c.checks.iter().filter(|k| !k.passed).map(|k| format!("- {}: {}", k.name, k.detail.chars().take(300).collect::<String>())).collect();
                let why = if why.is_empty() { c.summary.clone().unwrap_or_else(|| "the run did not finish".into()) } else { why.join("\n") };
                if c.attempt + 1 < MAX_ATTEMPTS {
                    let next = c.attempt + 1;
                    post(&mut a, "autopilot", Some(&c.agent), &format!("{} did not pass ({}); trying again{} (attempt {} of {MAX_ATTEMPTS}).", c.agent, why.lines().next().unwrap_or("").trim_start_matches("- "), if next >= 2 { " with a stronger model" } else { "" }, next + 1));
                    let _ = queue(root, &mut a, &mid, &c.agent, &c.text, &key, next, &c.source, Some(&why), &c.kind);
                } else {
                    post(&mut a, "autopilot", Some("andrew"), &format!("Stuck: {} could not finish \"{}\" after {MAX_ATTEMPTS} tries. Everything else keeps going. Send a note to steer it, or press Try Again.\n{why}", c.agent, c.text.chars().take(100).collect::<String>()));
                    crate::notify::send_quiet(crate::notify::Kind::Failed, &format!("{}: {} is stuck", a.title, c.agent), &[c.text.chars().take(200).collect(), why.chars().take(500).collect()]);
                }
            }
            "paused" => post(&mut a, "autopilot", None, "Claude's usage limit is reached. Work continues by itself when it resets."),
            _ => {}
        }
        let _ = save(root, &a);
    }
    advance(root, &slug);
}

/// Andrew's note at any time: every later job reads it, and it goes to the
/// agent that fits it now.
pub fn note(root: &Path, slug: &str, text: &str, agent: Option<&str>, milestone: Option<&str>) -> Result<Autopilot, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("Write a note first.".into());
    }
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut a = load(root, slug)?;
    a.notes.push(text.into());
    let to = agent.filter(|x| !x.is_empty() && *x != "auto").map(String::from).unwrap_or_else(|| changes::agent_for_text(text).to_string());
    post(&mut a, "andrew", Some(&to), text);
    // Into the game's own notes file once the game exists, for every agent.
    let notes_file = root.join("projects").join(slug).join("docs/notes-from-andrew.md");
    if notes_file.parent().map(|p| p.parent().map(|g| g.is_dir()).unwrap_or(false)).unwrap_or(false) {
        let _ = fs::create_dir_all(notes_file.parent().unwrap());
        let mut old = fs::read_to_string(&notes_file).unwrap_or_else(|_| "# Notes from Andrew\n".into());
        old.push_str(&format!("\n- {} {}\n", &crate::now_rfc3339()[..10], text));
        let _ = fs::write(&notes_file, old);
    }
    // The milestone being worked on (or the last one), revising if in review.
    let i = milestone.and_then(|m| a.milestones.iter().position(|x| x.id == m)).or_else(|| a.milestones.iter().position(|m| m.status == "working" || m.status == "revising")).or_else(|| a.milestones.iter().rposition(|m| m.status == "review" || m.status == "approved")).unwrap_or(0);
    let mid = a.milestones[i].id.clone();
    if a.milestones[i].status == "review" || a.milestones[i].status == "approved" {
        a.milestones[i].status = "revising".into();
    }
    let was = a.status.clone();
    if a.status == "budget" {
        a.status = "running".into();
    }
    let key = format!("{mid}/note/{}", a.notes.len());
    let r = queue(root, &mut a, &mid, &to, &format!("Andrew's note: {text}. Make the change he asks for."), &key, 0, "andrew", None, "change");
    if r.is_err() && was != "budget" && a.status == "budget" {
        save(root, &a)?;
        return Err(a.why.clone().unwrap_or_default());
    }
    save(root, &a)?;
    Ok(a)
}

/// Andrew approves a finished milestone, or sends notes for a revision.
pub fn review(root: &Path, slug: &str, milestone: &str, approve: bool, notes: &str) -> Result<Autopilot, String> {
    {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = load(root, slug)?;
        let m = a.milestones.iter_mut().find(|m| m.id == milestone).ok_or("no such milestone")?;
        if m.status != "review" {
            return Err(format!("{} is {}, not waiting for review", m.title, m.status));
        }
        if approve {
            m.status = "approved".into();
            m.approved_at = Some(crate::now_rfc3339());
            let t = m.title.clone();
            post(&mut a, "andrew", None, &format!("Approved: {t}{}", if notes.trim().is_empty() { String::new() } else { format!(" ({})", notes.trim()) }));
            if !notes.trim().is_empty() {
                a.notes.push(notes.trim().into());
            }
            save(root, &a)?;
        }
    }
    if !approve {
        for line in notes.lines().map(str::trim).filter(|l| !l.is_empty()) {
            note(root, slug, line, None, Some(milestone))?;
        }
    }
    advance(root, slug);
    load(root, slug)
}

/// Pause or resume (Andrew's choice), raise the budget, or retry stuck work.
pub fn control(root: &Path, slug: &str, action: &str, budget_usd: Option<f64>) -> Result<Autopilot, String> {
    {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = load(root, slug)?;
        match action {
            "pause" => {
                a.status = "paused".into();
                post(&mut a, "andrew", None, "Paused the autopilot. Jobs already running finish; nothing new starts.");
            }
            "resume" => {
                a.status = "running".into();
                a.why = None;
                post(&mut a, "andrew", None, "Resumed the autopilot.");
            }
            "budget" => {
                let b = budget_usd.ok_or("no budget given")?;
                if b < a.budget_usd {
                    return Err("The new budget must be higher.".into());
                }
                a.budget_usd = b;
                if a.status == "budget" {
                    a.status = "running".into();
                    a.why = None;
                }
                post(&mut a, "andrew", None, &format!("Raised this game's budget to ${b:.2}."));
            }
            "retry" => {
                let all = mine(root, slug);
                let mut n = 0;
                for m in a.milestones.clone() {
                    for c in latest(&all, &m.id).into_iter().filter(stuck) {
                        let key = c.key.clone().unwrap_or(c.id.clone());
                        if queue(root, &mut a, &m.id, &c.agent, &c.text, &key, 0, &c.source, Some("Earlier attempts failed; read their change.md and checks in the earlier run folders and take a different approach."), &c.kind).is_ok() {
                            n += 1;
                        }
                    }
                }
                post(&mut a, "andrew", None, &format!("Try again: restarted {n} stuck job{}.", if n == 1 { "" } else { "s" }));
            }
            _ => return Err("unknown action".into()),
        }
        save(root, &a)?;
    }
    advance(root, slug);
    load(root, slug)
}

/// Everything the Autopilot page shows for one game.
pub fn view(root: &Path, slug: &str) -> Result<Value, String> {
    let a = load(root, slug)?;
    let all = mine(root, slug);
    let (spent_usd, reserved) = spent(root, slug);
    let agents = crate::cost::load_agents(root).ok();
    let tpl = template();
    let ms: Vec<Value> = a.milestones.iter().enumerate().map(|(i, m)| {
        let jobs: Vec<Value> = latest(&all, &m.id).into_iter().map(|c| {
            let out = if c.kind == "task" { agents.as_ref().and_then(|ag| ag.agents.get(&c.agent).and_then(|p| p.output.clone())) } else { None }.unwrap_or_else(|| "change.md".into());
            let d = root.join("runs").join(&c.run_id);
            let mocks: Vec<String> = fs::read_dir(d.join("mockups")).into_iter().flatten().flatten().map(|e| format!("runs/{}/mockups/{}", c.run_id, e.file_name().to_string_lossy())).filter(|p| p.ends_with(".svg")).collect();
            json!({ "id": c.id, "agent": c.agent, "text": c.text, "status": c.status, "stuck": stuck(&c), "attempt": c.attempt, "summary": c.summary, "cost_usd": c.cost_usd, "run_id": c.run_id, "source": c.source,
                    "output": d.join(&out).is_file().then(|| format!("runs/{}/{out}", c.run_id)), "mockups": mocks })
        }).collect();
        json!({ "id": m.id, "title": m.title, "status": m.status, "review": tpl.get(i).map(|t| t.2).unwrap_or(""), "jobs": jobs })
    }).collect();
    let working = all.iter().filter(|c| c.status == "working").count();
    Ok(json!({ "slug": a.slug, "title": a.title, "idea": a.idea, "status": a.status, "why": a.why, "budget_usd": a.budget_usd, "spent_usd": spent_usd, "reserved_usd": reserved,
               "plan_run": a.plan_run, "milestones": ms, "board": a.board.iter().rev().take(80).collect::<Vec<_>>(), "working": working,
               "project": root.join("projects").join(&a.slug).join("default.project.json").is_file().then(|| format!("projects/{}", a.slug)) }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn milestones_use_real_agents_and_slugs_are_safe() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let agents = crate::cost::load_agents(&root).unwrap();
        for (_, _, _, waves) in template() {
            for (a, _) in waves.into_iter().flatten() {
                assert!(agents.agents.contains_key(a), "{a}");
            }
        }
        assert_eq!(slugify("Sugar Rush: Factory!"), "sugar-rush-factory");
        assert!(crate::paths::valid_id(&slugify("../../etc")));
    }

    fn ch(key: &str, attempt: u32, status: &str, at: &str) -> Change {
        Change { id: format!("{key}-{attempt}"), target: "game:g".into(), item: None, text: "x".into(), agent: "frame".into(), status: status.into(), run_id: "r".into(), created_at: at.into(), finished_at: None, summary: None, checks: vec![], cost_usd: None, kind: "task".into(), autopilot: Some("g".into()), milestone: Some("ui".into()), key: Some(key.into()), attempt, source: "autopilot".into() }
    }

    fn studio() -> PathBuf {
        let d = std::env::temp_dir().join(format!("ap-t-{}-{:?}", std::process::id(), std::thread::current().id()).replace(['(', ')'], ""));
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
        d
    }
    fn finish_as(root: &Path, c: &Change, status: &str, fail: &str) -> Change {
        changes::set_result(root, &c.id, status, if fail.is_empty() { vec![] } else { vec![crate::queue::GateCheck { name: fail.into(), passed: false, detail: "missing".into() }] });
        let c = changes::list(root).into_iter().find(|x| x.id == c.id).unwrap();
        on_finish(root, &c);
        c
    }
    fn queued(root: &Path, slug: &str) -> Vec<Change> {
        mine(root, slug).into_iter().filter(|c| c.status == "queued").collect()
    }

    #[test]
    fn a_game_moves_through_milestones_retries_and_listens_to_agents() {
        let root = studio();
        let a = start(&root, "Test Game", "Collect stars, upgrade a rocket.", 500.0, None).unwrap();
        assert_eq!(a.milestones[0].status, "working");
        let q = queued(&root, &a.slug);
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].agent, "planner");
        // Plan done: waits for Andrew; nothing after the plan starts early.
        finish_as(&root, &q[0], "done", "");
        let a = load(&root, &a.slug).unwrap();
        assert_eq!(a.milestones[0].status, "review");
        assert_eq!(a.plan_run.as_deref(), Some(q[0].run_id.as_str()));
        assert!(queued(&root, &a.slug).is_empty(), "design waits for the plan approval");
        review(&root, &a.slug, "plan", true, "").unwrap();
        let q = queued(&root, &a.slug);
        assert_eq!(q.iter().map(|c| c.agent.as_str()).collect::<Vec<_>>(), vec!["architect"]);
        // A failure is retried with what failed.
        finish_as(&root, &q[0], "failed", "core loop section");
        let q2 = queued(&root, &a.slug);
        assert_eq!(q2.len(), 1);
        assert_eq!((q2[0].agent.as_str(), q2[0].attempt), ("architect", 1));
        let prompt = crate::runs::load(&root, &q2[0].run_id).unwrap().prompt;
        assert!(prompt.contains("YOUR LAST ATTEMPT DID NOT PASS") && prompt.contains("core loop section"), "{prompt}");
        // Third attempt uses the stronger model.
        finish_as(&root, &q2[0], "failed", "core loop section");
        let q3 = queued(&root, &a.slug);
        assert_eq!(crate::runs::load(&root, &q3[0].run_id).unwrap().estimate.model, STRONG_MODEL);
        // The agent asks Frame for something while finishing.
        let dir = root.join("runs").join(&q3[0].run_id);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("requests.json"), r#"[{"to": "frame", "task": "Sketch the rocket upgrade screen", "why": "the design needs it"}, {"to": "nobody", "task": "x"}]"#).unwrap();
        finish_as(&root, &q3[0], "done", "");
        let q4 = queued(&root, &a.slug);
        let names: Vec<&str> = q4.iter().map(|c| c.agent.as_str()).collect();
        assert!(names.contains(&"frame") && names.contains(&"director") && names.contains(&"simulator"), "{names:?}");
        assert_eq!(q4.iter().find(|c| c.agent == "frame").unwrap().source, "agent:architect");
        let board = load(&root, &a.slug).unwrap().board;
        assert!(board.iter().any(|m| m.text.contains("Ignored a request for nobody")));
        // Andrew's note goes to the fitting agent and every later job reads it.
        note(&root, &a.slug, "Make the rocket glow when it levels up", None, None).unwrap();
        let n = queued(&root, &a.slug).into_iter().find(|c| c.source == "andrew").unwrap();
        assert_eq!(n.agent, "motion");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn stops_at_the_games_budget() {
        let root = studio();
        let a = start(&root, "Tiny Budget", "An obby.", 0.01, None).unwrap();
        assert_eq!(a.status, "budget");
        assert!(a.why.unwrap().contains("budget"));
        assert!(queued(&root, &a.slug).is_empty());
        let a = control(&root, &a.slug, "budget", Some(500.0)).unwrap();
        assert_eq!(a.status, "running");
        assert_eq!(queued(&root, &a.slug).len(), 1);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn the_latest_attempt_counts_and_four_failures_mean_stuck() {
        let all = vec![ch("ui/0/0", 0, "failed", "1"), ch("ui/0/0", 1, "done", "2"), ch("ui/0/1", 3, "failed", "3")];
        let l = latest(&all, "ui");
        assert_eq!(l.len(), 2);
        assert_eq!(l.iter().find(|c| c.key.as_deref() == Some("ui/0/0")).unwrap().status, "done");
        assert!(stuck(l.iter().find(|c| c.key.as_deref() == Some("ui/0/1")).unwrap()));
        assert!(!stuck(&ch("k", 1, "failed", "1")));
    }
}
