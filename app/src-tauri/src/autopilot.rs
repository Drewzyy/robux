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
/// After all tries fail, Lead rescues the job right away this many times;
/// after that it keeps rescuing every 30 minutes (the budget is the limit).
pub const MAX_RESCUES: usize = 2;
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
        ("quality", "Aiming for the top games' standard", "How the game scores against the best Roblox games in its niche, area by area, after the team closed the gaps. A score, not a played-in-Studio check.", vec![vec![]]),
        ("polish", "Polish and release prep", "Fixed problems, phone performance, simulated players, thumbnails and the release checklist. You publish it yourself.", vec![
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
    /// Quality milestone: the current benchmark round and each round's
    /// overall score (0 to 10) against the top games.
    #[serde(default)]
    pub round: u32,
    #[serde(default)]
    pub history: Vec<f64>,
    /// Reminders sent while this milestone waits for Andrew's review.
    #[serde(default)]
    pub reminded: u32,
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
    let mut a: Autopilot = crate::paths::read_json(&file(root, slug)).ok_or_else(|| format!("no autopilot for {slug}"))?;
    // Games started on an older version get milestones added since, in order.
    let tpl = template();
    // Budgets were removed: a game that stopped at its budget carries on.
    if a.status == "budget" {
        a.status = "running".into();
        a.why = None;
    }
    if a.milestones.len() != tpl.len() || a.milestones.iter().zip(tpl.iter()).any(|(m, t)| m.id != t.0) {
        let old = std::mem::take(&mut a.milestones);
        a.milestones = tpl.iter().map(|(id, t, _, _)| old.iter().find(|m| m.id == *id).cloned().unwrap_or(Milestone { id: id.to_string(), title: t.to_string(), status: "waiting".into(), wave: 0, ready_at: None, approved_at: None, round: 0, history: vec![], reminded: 0 })).collect();
    }
    Ok(a)
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
    let st = crate::settings::load();
    let mut per = vec![];
    let mut total = 0.0;
    for (id, title, _, waves) in template() {
        let mut m = 0.0;
        for (agent, job) in waves.into_iter().flatten() {
            let prof = agents.agents.get(agent).ok_or_else(|| format!("agent {agent} is missing"))?;
            if st.engine_for(agent) != "ollama" {
                m += crate::cost::estimate(agent, prof, &format!("{job}\n{idea}"), &pricing)?.cap_usd;
            }
        }
        if id == "quality" {
            // No fixed jobs: assume three rounds of a benchmark and four fixes.
            let cap = |agent: &str, job: &str| -> f64 { if st.engine_for(agent) == "ollama" { return 0.0; } agents.agents.get(agent).and_then(|p| crate::cost::estimate(agent, p, job, &pricing).ok()).map(|e| e.cap_usd).unwrap_or(0.0) };
            m += 3.0 * (cap("director", BENCH_JOB) + 4.0 * cap("builder", "Close one gap to the top games in projects/<game>."));
        }
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
        milestones: template().into_iter().map(|(id, t, _, _)| Milestone { id: id.into(), title: t.into(), status: "waiting".into(), wave: 0, ready_at: None, approved_at: None, round: 0, history: vec![], reminded: 0 }).collect(),
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
    // Finished and failed work (a rescuer reads the failed attempts); the
    // most recent 40, so the list stays reasonable on a long game.
    let all: Vec<Change> = mine(root, slug).into_iter().filter(|c| c.status == "done" || c.status == "failed").collect();
    for c in all.iter().skip(all.len().saturating_sub(40)) {
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
    let agents = crate::cost::load_agents(root).ok();
    for c in mine(root, &a.slug).into_iter().filter(|c| c.status == "done") {
        let out = agents.as_ref().and_then(|ag| ag.agents.get(&c.agent).and_then(|p| p.output.clone())).unwrap_or_else(|| "change.md".into());
        drafts.push(format!("- {} ({}): runs/{}/{} {}", c.agent, c.milestone.as_deref().unwrap_or(""), c.run_id, if c.kind == "task" { out } else { "change.md".into() }, if c.kind == "task" { "" } else { "(a change)" }));
    }
    let drafts = if drafts.is_empty() { String::new() } else { format!("\nEarlier work by the team (read what your job needs):\n{}\n", drafts.into_iter().rev().take(25).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n")) };
    let plan = a.plan_run.as_ref().map(|p| format!("The approved plan: runs/{p}/plan.json (readable: plan.md, mockups in mockups/).\n")).unwrap_or_default();
    let notes = if a.notes.is_empty() { String::new() } else { format!("\nAndrew's notes so far (follow them):\n{}\n", a.notes.iter().rev().take(12).map(|n| format!("- {n}")).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n")) };
    let fb = feedback.map(|f| format!("\nYOUR LAST ATTEMPT DID NOT PASS. Fix exactly this, keep what was fine:\n{f}\n")).unwrap_or_default();
    format!(
        "Autopilot job for the game \"{}\" (slug {}, folder {project}), milestone \"{milestone}\".\nThe idea Andrew approved: {}\n{plan}{drafts}{notes}{fb}\nYour job: {job}\n\nWork on your own; nobody can answer questions mid-run. If another agent must do something (or redo something) for the game to come out right, write requests.json in your run folder: [{{\"to\": \"<agent name>\", \"task\": \"<what to do>\", \"why\": \"<why>\"}}]. To stop another agent's queued work that would be wasted, add {{\"stop\": \"<agent name>\", \"why\": \"...\"}}. Agents: architect, builder, frame, scenic, motion, modeler, librarian, merchant, tuner, crowd, splash, playtest, simulator, director. Never publish or upload anything. Never write that Andrew decided, asked or approved something unless it is in his notes above. Never ask Andrew anything: he wants the team to decide. For every open choice, pick what gets a high-quality game out fastest, do it, and list it under a heading \"Decisions made\" (one line each). Also write change.md: what you did in 3 to 8 lines (first line a one-sentence summary).\n\n{ON_TRACK}",
        a.title, a.slug, idea_for_prompt(root, a)
    )
}

/// The idea, short enough for every job. A long spec (Andrew's full game
/// document) is saved once to projects/<game>/docs/idea.md and only its
/// opening and the path go into each job: the whole document in every
/// prompt wasted usage and overflowed local models (the task never
/// reached them).
fn idea_for_prompt(root: &Path, a: &Autopilot) -> String {
    if a.idea.len() <= 1500 {
        return a.idea.clone();
    }
    let rel = format!("projects/{}/docs/idea.md", a.slug);
    let path = root.join(&rel);
    if fs::read_to_string(&path).map(|t| t != a.idea).unwrap_or(true) {
        let _ = fs::create_dir_all(path.parent().unwrap());
        let _ = fs::write(&path, &a.idea);
    }
    let mut head: String = a.idea.chars().take(900).collect();
    if let Some(i) = head.rfind(char::is_whitespace) {
        head.truncate(i);
    }
    format!("{head} ... [The full game document is {rel} ({} words): read only the sections your job needs.]", a.idea.split_whitespace().count())
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

/// Put the BrewixGames UI kit (studio/kits/ui) into the game's shared code
/// once the game has code, so Frame and Builder build screens from finished
/// parts instead of raw frames. Missing files only: the game's own edits to
/// the kit are kept.
pub fn ensure_ui_kit(root: &Path, slug: &str) {
    let proj = root.join("projects").join(slug);
    if !proj.join("default.project.json").is_file() {
        return;
    }
    let kit = root.join("studio/kits/ui/UI");
    let dest = proj.join("src/shared/UI");
    let Ok(files) = fs::read_dir(&kit) else { return };
    for f in files.flatten() {
        let to = dest.join(f.file_name());
        if !to.exists() {
            let _ = fs::create_dir_all(&dest);
            let _ = fs::copy(f.path(), &to);
        } else if let Ok(old) = fs::read_to_string(&to) {
            // Older kit copies failed the lint on three lines; fix just those.
            let new = old.replace("Size = UDim2.new(0, 0, 1, 0),", "Size = UDim2.fromScale(0, 1),").replace("Size = UDim2.new(1, 0, 0.5, 0),", "Size = UDim2.fromScale(1, 0.5),");
            if new != old {
                let _ = fs::write(&to, new);
            }
        }
    }
}

/// A line from the Warden on a game's board.
pub fn warden_note(root: &Path, slug: &str, text: &str) {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Ok(mut a) = load(root, slug) {
        post(&mut a, "warden", None, text);
        let _ = save(root, &a);
    }
}

/// Warden: while a step waits for Andrew, research the next steps (once
/// per waiting step). Returns true when it queued something.
pub fn warden_research(root: &Path, slug: &str) -> bool {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Ok(mut a) = load(root, slug) else { return false };
    let Some(m) = a.milestones.iter().find(|m| m.status == "review").map(|m| m.id.clone()) else { return false };
    let key = format!("{m}/warden-research");
    if mine(root, slug).iter().any(|c| c.key.as_deref() == Some(&key)) {
        return false;
    }
    let job = format!("{RESEARCH_JOB} Focus on what the next steps of this game need (after: {m}).");
    let ok = queue(root, &mut a, &m, "pulse", &job, &key, 0, "autopilot", None, "change").is_ok();
    if ok {
        post(&mut a, "warden", None, "The team was resting while a step waits for Andrew: Pulse researches the next steps meanwhile.");
    }
    let _ = save(root, &a);
    ok
}

/// Warden: a running game with nothing queued and no step to start: Lead
/// decides the next work (at most once an hour).
pub fn warden_lead(root: &Path, slug: &str) -> bool {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Ok(mut a) = load(root, slug) else { return false };
    let recent = mine(root, slug).iter().any(|c| c.key.as_deref().map(|k| k.contains("/warden-lead")).unwrap_or(false) && c.created_at.as_str() > (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339().as_str());
    if recent {
        return false;
    }
    // Lead's last two "decide the next work" runs failed: wait 6 hours
    // instead of trying again every round.
    let mut last: Vec<Change> = mine(root, slug).into_iter().filter(|c| c.key.as_deref().map(|k| k.contains("/warden-lead/")).unwrap_or(false)).collect();
    last.sort_by(|x, y| x.created_at.cmp(&y.created_at));
    let tail: Vec<&Change> = last.iter().rev().take(2).collect();
    if tail.len() == 2 && tail.iter().all(|c| c.status == "failed" || c.summary.as_deref().map(|s| s.starts_with("Skipped")).unwrap_or(false)) && tail[0].created_at.as_str() > (chrono::Utc::now() - chrono::Duration::hours(6)).to_rfc3339().as_str() {
        return false;
    }
    let m = a.milestones.iter().find(|m| m.status == "working" || m.status == "revising").or_else(|| a.milestones.iter().rev().find(|m| m.status != "waiting")).map(|m| m.id.clone()).unwrap_or_else(|| "plan".into());
    let job = "Warden check: the team is idle but the game is not finished. Read the game's state (milestones, the last jobs and their results, Andrew's notes) and decide the most useful next work for this step. Start the agents that should do it with requests.json, and write change.md: what you decided and why.";
    let ok = queue(root, &mut a, &m, "lead", job, &format!("{m}/warden-lead/{}", chrono::Utc::now().timestamp()), 0, "autopilot", None, "change").is_ok();
    if ok {
        post(&mut a, "warden", None, "The team was idle with nothing queued: Lead is deciding the next work.");
    }
    let _ = save(root, &a);
    ok
}

/// When the app opens: every game carries on by itself. Games an older
/// version paused because of a problem (they carry a reason; a pause Andrew
/// pressed has none) resume, and jobs waiting on a retry timer start now
/// instead of waiting out the timer. A game Andrew paused stays paused.
pub fn wake_all(root: &Path) {
    tidy(root);
    for a in list(root) {
        if a.status == "paused" && a.why.is_some() {
            let _ = control(root, &a.slug, "resume", None);
        }
    }
    changes::clear_retry_timers(root);
    for a in list(root).into_iter().filter(|a| a.status == "running") {
        advance(root, &a.slug);
    }
}

/// A milestone waiting for Andrew: remind him every 6 hours, 4 times at most.
fn remind_reviews(root: &Path, a: &mut Autopilot) {
    let now = chrono::Utc::now();
    let n = a.milestones.len();
    for i in 0..n {
        let m = &a.milestones[i];
        if m.status != "review" || m.reminded >= 4 {
            continue;
        }
        let Some(at) = m.ready_at.as_deref().and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()) else { continue };
        if now.signed_duration_since(at) < chrono::Duration::hours(6 * (m.reminded as i64 + 1)) {
            continue;
        }
        a.milestones[i].reminded += 1;
        let _ = save(root, a); // counted before sending, so it never repeats
        let (title, last) = (a.milestones[i].title.clone(), i + 1 == n);
        crate::notify::send_quiet(crate::notify::Kind::NeedsApproval, &format!("Reminder: {}: {} {title}", a.title, if last { "final approval for" } else { "waiting for your approval:" }), &["Open BrewixGames: it is the Next step on the Dashboard.".into()]);
    }
}

/// Jobs finished between two code audits.
pub const AUDIT_EVERY: usize = 12;

/// Self-improvement: once the game has code, every AUDIT_EVERY finished
/// jobs Builder audits it for waste and bugs (slow loops, leaks, network
/// spam, unsafe saves, duplicated code) and fixes them without changing
/// how the game plays. Runs on its own; nothing for Andrew to do.
fn code_audit(root: &Path, a: &mut Autopilot, all: &[Change]) {
    if !root.join("projects").join(&a.slug).join("default.project.json").is_file() {
        return;
    }
    let audits: Vec<&Change> = all.iter().filter(|c| c.key.as_deref().map(|k| k.starts_with("audit/")).unwrap_or(false)).collect();
    if audits.iter().any(|c| ["queued", "working", "paused"].contains(&c.status.as_str())) {
        return;
    }
    let n = audits.iter().filter_map(|c| c.key.as_deref()?.strip_prefix("audit/")?.split('/').next()?.parse::<usize>().ok()).max().unwrap_or(0);
    let done = all.iter().filter(|c| c.status == "done" && !c.key.as_deref().unwrap_or("").starts_with("audit/")).count();
    if done < AUDIT_EVERY * (n + 1) {
        return;
    }
    // Only when the game's code changed since the last audit, and at most
    // every 12 hours: back-to-back audits mostly found nothing to fix.
    if let Some(last) = audits.iter().filter_map(|c| c.finished_at.clone()).max() {
        let since = chrono::DateTime::parse_from_rfc3339(&last).map(|t| t.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now());
        if chrono::Utc::now() - since < chrono::Duration::hours(12) {
            return;
        }
        let changed = crate::paths::newest_mtime(&root.join("projects").join(&a.slug).join("src")).map(|m| chrono::DateTime::<chrono::Utc>::from(m) > since).unwrap_or(true);
        if !changed {
            return;
        }
    }
    let mid = a.milestones.iter().find(|m| m.status == "working" || m.status == "revising").or_else(|| a.milestones.iter().rev().find(|m| m.status != "waiting")).map(|m| m.id.clone()).unwrap_or_else(|| "build".into());
    let job = "Code audit (self-improvement): read projects/<game>'s code and make it better without changing how the game plays. Find and fix: work done every frame that need not be, tables or connections that grow and are never cleaned up, remote events fired too often, DataStore calls without pcall and retries, duplicated code that should be one module, slow searches through the whole workspace, and anything the build checks warn about. Smallest safe changes only; the game must still build. In change.md list each change, why, and what you measured or checked.";
    post(a, "autopilot", Some("builder"), "Code audit: Builder checks the game's code for waste and bugs and fixes them.");
    let _ = queue(root, a, &mid, "builder", job, &format!("audit/{}", n + 1), 0, "autopilot", None, "change");
}

/// The one rule that stops every kind of loop seen so far (retries,
/// rescues of rescues, re-issued plans, repeated notes, audits): the same
/// job, or the same work by the same agent, is not started again once it
/// has been tried too often in the last 24 hours.
pub const LOOP_SAME_KEY: usize = 8;
pub const LOOP_SAME_TEXT: usize = 6;
pub const LOOP_PER_HOUR: usize = 40;

pub fn loop_guard(all: &[Change], agent: &str, job: &str, key: &str) -> Option<String> {
    let day = (chrono::Utc::now() - chrono::Duration::hours(24)).to_rfc3339();
    let hour = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();
    let recent: Vec<&Change> = all.iter().filter(|c| c.created_at > day).collect();
    let base = |k: &str| k.rsplit_once("/improve/").map(|x| x.0.to_string()).unwrap_or_else(|| k.to_string());
    if recent.iter().filter(|c| c.key.as_deref().map(|k| base(k) == base(key)).unwrap_or(false)).count() >= LOOP_SAME_KEY {
        return Some(format!("tried the same job {LOOP_SAME_KEY} times in a day"));
    }
    let norm = |t: &str| t.to_lowercase().split_whitespace().take(40).collect::<Vec<_>>().join(" ");
    let me = norm(job);
    if recent.iter().filter(|c| c.agent == agent && c.key.as_deref() != Some(key) && norm(&c.text) == me).count() >= LOOP_SAME_TEXT {
        return Some(format!("was given the same work {LOOP_SAME_TEXT} times in a day"));
    }
    // A step's own planned jobs (key "design/0/1") are the real work: the
    // hourly ceiling never holds them back, or a loop elsewhere would stall
    // the whole game for an hour.
    let planned = planned_key(key);
    if !planned && recent.iter().filter(|c| c.created_at > hour).count() >= LOOP_PER_HOUR {
        return Some(format!("would be the {}th job this hour for this game", LOOP_PER_HOUR + 1));
    }
    None
}

/// The next model up for a last try.
pub fn step_up(model: &str) -> String {
    if model.contains("haiku") { "claude-sonnet-5-5".into() } else { STRONG_MODEL.into() }
}

/// Queue one job, unless it would go over this game's budget.
fn queue(root: &Path, a: &mut Autopilot, milestone: &str, agent: &str, job: &str, key: &str, attempt: u32, source: &str, feedback: Option<&str>, kind: &str) -> Result<Change, String> {
    if let Some(why) = loop_guard(&mine(root, &a.slug), agent, job, key) {
        // Post once per hour per key, not on every refusal.
        let line = format!("Stopped a loop: {agent} {why}. Skipped it; the team moves on with other work.");
        if !a.board.iter().rev().take(40).any(|m| m.text == line) {
            post(a, "warden", None, &line);
        }
        // Close the job it would have redone, so the step is not left
        // waiting on it.
        if let Some(c) = mine(root, &a.slug).into_iter().rev().find(|c| c.key.as_deref() == Some(key) && c.status == "failed") {
            changes::mark_skipped(root, &c.id, &format!("Skipped: {why}."));
        }
        return Err(format!("loop guard: {why}"));
    }
    // Only the last try moves up one model (Haiku to Sonnet, Sonnet to
    // Opus): bigger models use up Claude's usage much faster.
    let own = crate::cost::load_agents(root).ok().and_then(|x| x.agents.get(agent).map(|p| p.model.clone())).unwrap_or_default();
    let model = if attempt + 1 >= MAX_ATTEMPTS { Some(step_up(&own)) } else { None };
    let agents = crate::cost::load_agents(root)?;
    let mut prof = agents.agents.get(agent).cloned().ok_or_else(|| format!("unknown agent {agent}"))?;
    let mut p = prompt(root, a, milestone, job, feedback);
    // Inspiration from what the studio researched, for the agents that design and build.
    if prof.allowed_sections.iter().any(|s| s == "18-Source-Library") && !milestone.is_empty() {
        p.push_str("\n\nInspiration (read only the \"Do this\" block of each, then use what fits this job): vault 18-Source-Library/open-source-game-lessons.md and 18-Source-Library/uncopylocked-games-studied.md (patterns from real Roblox games; rebuild the ideas, never copy code or models)");
        if prof.allowed_sections.iter().any(|s| s == "08-UI-UX") {
            p.push_str(", and 08-UI-UX/ui-kit.md (build every screen from the UI kit in src/shared/UI)");
        }
        p.push('.');
    }
    if let Some(m) = &model {
        prof.model = m.clone();
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
        parent_run: None,
        group: None,
        depth: 0,
        after: None,
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
    // `all` is in the order jobs were queued: the last one for a key wins
    // (a fresh start after a rescue has attempt 0 but is still the newest).
    for c in all.iter().filter(|c| c.milestone.as_deref() == Some(milestone) && c.status != "cancelled") {
        let k = c.key.clone().unwrap_or_else(|| c.id.clone());
        by.insert(k, c.clone());
    }
    by.into_values().collect()
}

/// What the game page lists for a step: one line per piece of work. Old
/// rescues of the same job collapse into the newest, and jobs the team gave
/// up on or the Warden cleared are not shown.
fn shown_jobs(jobs: Vec<Change>) -> Vec<Change> {
    let rescue = |c: &Change| -> Option<(u32, String)> {
        let k = c.key.as_deref()?.strip_prefix("rescue|")?;
        let (n, rest) = k.split_once('|')?;
        Some((n.parse().ok()?, rest.to_string()))
    };
    // Only the newest code audit is listed (they ran often and said little).
    let last_audit = jobs.iter().filter(|c| c.key.as_deref().unwrap_or("").starts_with("audit/")).map(|c| c.created_at.clone()).max();
    jobs.iter()
        .filter(|c| !c.key.as_deref().unwrap_or("").starts_with("audit/") || Some(&c.created_at) == last_audit.as_ref())
        .filter(|c| !c.summary.as_deref().map(|s| s.starts_with("Skipped") || s.starts_with("Cancelled by the Warden")).unwrap_or(false))
        .filter(|c| match rescue(c) {
            Some((n, k)) => !jobs.iter().any(|o| rescue(o).map(|(m, ok)| ok == k && m > n).unwrap_or(false)),
            None => true,
        })
        .cloned()
        .collect()
}

/// Code audits, the Warden's "decide the next work" for Lead, and rescues
/// of either: useful, but never a reason to hold a step open or retry forever.
/// A step's own planned job: key "<step>/<wave>/<n>".
fn planned_key(key: &str) -> bool {
    let p: Vec<&str> = key.split('/').collect();
    p.len() == 3 && p[1].parse::<u32>().is_ok() && p[2].parse::<u32>().is_ok()
}

fn side_work(key: &str) -> bool {
    let k = key.strip_prefix("rescue|").and_then(|k| k.split_once('|')).map(|x| x.1).unwrap_or(key);
    k.starts_with("audit/") || k.contains("/warden-lead/") || key.starts_with("rescue|") && (k.starts_with("audit/") || k.contains("/warden-lead/"))
}

fn stuck(c: &Change) -> bool {
    c.status == "failed" && c.attempt + 1 >= MAX_ATTEMPTS && !changes::launch_failed(c)
}

/// Move a game forward: start milestones and waves whose turn has come,
/// and mark milestones ready for review. Safe to call any time.
pub fn advance(root: &Path, slug: &str) {
    advance_inner(root, slug);
    revive(root, slug);
}

thread_local! {
    static REVIVING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Failed jobs nobody picked up (the app closed between finishing and
/// retrying, or the budget ran out at that moment) get their retry, rescue
/// or skip now, so no stage waits on a dead job forever.
fn revive(root: &Path, slug: &str) {
    if REVIVING.with(|r| r.get()) {
        return;
    }
    let Ok(a) = load(root, slug) else { return };
    if a.status != "running" {
        return;
    }
    let all = mine(root, slug);
    let open = |k: &str| all.iter().any(|x| ["queued", "working", "paused"].contains(&x.status.as_str()) && x.key.as_deref().map(|xk| xk == k || xk.ends_with(&format!("|{k}"))).unwrap_or(false));
    let old = |c: &Change| c.finished_at.as_deref().and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()).map(|t| chrono::Utc::now().signed_duration_since(t) > chrono::Duration::minutes(2)).unwrap_or(true);
    let mut seen = std::collections::HashSet::new();
    let mut dead = vec![];
    for c in all.iter().rev() {
        let Some(k) = c.key.clone() else { continue };
        if !seen.insert(k.clone()) {
            continue; // only the newest job of each key
        }
        if c.status == "failed" && !open(&k) && old(c) {
            dead.push(c.clone());
        }
    }
    if dead.is_empty() {
        return;
    }
    REVIVING.with(|r| r.set(true));
    for c in dead.iter().take(3) {
        on_finish(root, c);
    }
    REVIVING.with(|r| r.set(false));
}

fn advance_inner(root: &Path, slug: &str) {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let Ok(mut a) = load(root, slug) else { return };
    if a.status != "running" {
        return;
    }
    let tpl = template();
    let all = mine(root, slug);
    code_audit(root, &mut a, &all);
    remind_reviews(root, &mut a);
    ensure_ui_kit(root, &a.slug);
    // A plan already waiting for Andrew goes ahead (he asked the team to decide).
    if let Some(m) = a.milestones.iter_mut().find(|m| m.id == "plan" && m.status == "review") {
        m.status = "approved".into();
        m.approved_at = Some(crate::now_rfc3339());
    }
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
            if *mid == "quality" {
                a.milestones[i].round = 1;
                quality_step(root, &mut a, i, &all);
                continue;
            }
            for (k, (agent, job)) in waves[0].iter().enumerate() {
                let key = format!("{mid}/0/{k}");
                let job = format!("{job}{}", if *mid == "plan" { format!("\n\nThe idea is above (the full document, when long, is in the file it names). Use the slug \"{}\" in plan.json.", a.slug) } else { String::new() });
                if queue(root, &mut a, mid, agent, &job, &key, 0, "autopilot", None, "task").is_err() {
                    let _ = save(root, &a);
                    return;
                }
            }
            continue;
        }
        if *mid == "quality" && st == "working" {
            quality_step(root, &mut a, i, &all);
            continue;
        }
        if st == "working" {
            // Any planned job of the current wave that never got queued (the
            // budget ran out at that moment) is queued now.
            let w = a.milestones[i].wave.min(waves.len() - 1);
            for (k, (agent, job)) in waves[w].iter().enumerate() {
                let key = format!("{mid}/{w}/{k}");
                if !all.iter().any(|c| c.key.as_deref() == Some(&key) && c.status != "cancelled") {
                    let job = format!("{job}{}", if *mid == "plan" { format!("\n\nThe idea is above (the full document, when long, is in the file it names). Use the slug \"{}\" in plan.json.", a.slug) } else { String::new() });
                    if queue(root, &mut a, mid, agent, &job, &key, 0, "autopilot", None, "task").is_err() {
                        let _ = save(root, &a);
                        return;
                    }
                }
            }
        }
        let all = mine(root, slug);
        if st == "working" || st == "revising" {
            // Code audits are side work: they never hold a step open.
            let lat: Vec<Change> = latest(&all, mid).into_iter().filter(|c| !side_work(c.key.as_deref().unwrap_or(""))).collect();
            // Planned jobs still waiting hold the step; extra work holds it
            // only while it is actually running (waiting or paused extras
            // still run later, but never block the next step).
            let six_h = (chrono::Utc::now() - chrono::Duration::hours(6)).to_rfc3339();
            let now_s = crate::now_rfc3339();
            let holds = |c: &Change| {
                let k = c.key.as_deref().unwrap_or("");
                if planned_key(k) {
                    return ["queued", "working", "paused"].contains(&c.status.as_str());
                }
                if c.status == "working" {
                    return true;
                }
                // Agents' requests and Andrew's notes hold while waiting, but
                // not improve rounds, rescues, paused, delayed or stale work.
                c.status == "queued" && !k.contains("/improve/") && !k.starts_with("rescue|") && c.not_before.as_deref().map(|t| t <= now_s.as_str()).unwrap_or(true) && c.created_at > six_h
            };
            let open = lat.iter().any(holds);
            // Only a step's own planned jobs must succeed for it to finish.
            // Extra work that failed (an improve round, a rescue, an agent's
            // request) never holds the step, or the game stalls with every
            // listed job "Done".
            let failed_stuck = lat.iter().filter(|c| planned_key(c.key.as_deref().unwrap_or(""))).any(stuck);
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
            if !open && !failed_stuck && (all_waves_done || st == "revising") && lat.iter().filter(|c| planned_key(c.key.as_deref().unwrap_or(""))).all(|c| c.status == "done") {
                if mid == &"plan" {
                    // Other agents revised parts of the plan in their own
                    // folders: Planner folds those into one plan before
                    // Andrew sees it (at most twice, so it cannot loop).
                    // Queue order (timestamps can tie within a second).
                    let pos = |c: &Change| all.iter().position(|x| x.id == c.id).unwrap_or(0);
                    let last_planner = lat.iter().filter(|c| c.agent == "planner").max_by_key(|c| pos(c)).cloned();
                    let newer = lat.iter().any(|c| c.agent != "planner" && last_planner.as_ref().map(|p| pos(c) > pos(p)).unwrap_or(false));
                    let folds = lat.iter().filter(|c| c.key.as_deref().map(|k| k.starts_with("plan/fold/")).unwrap_or(false)).count();
                    if newer && folds < 2 {
                        let job = "Fold the team's agreed revisions into one plan. Read every earlier job of this milestone (folders listed above: prices, sounds, sections, mockups other agents changed) and write the complete updated plan in your run folder: plan.json, plan.md and mockups/ (copy mockups that did not change). Where two revisions disagree, take the later one and say so under Open questions. Do not start other agents.";
                        if queue(root, &mut a, mid, "planner", job, &format!("plan/fold/{}", folds + 1), 0, "autopilot", None, "task").is_ok() {
                            post(&mut a, "autopilot", Some("planner"), "Other agents revised the plan; Planner is folding their changes in before it goes to Andrew.");
                        }
                        continue;
                    }
                    if let Some(p) = last_planner {
                        a.plan_run = Some(p.run_id.clone());
                    }
                }
                // Andrew asked the team to decide and move fast: the plan
                // (which has no code to score) goes ahead without waiting.
                if ["plan", "design", "ui", "world"].contains(mid) {
                    a.milestones[i].status = "approved".into();
                    a.milestones[i].approved_at = Some(crate::now_rfc3339());
                    post(&mut a, "autopilot", None, &format!("{title} is done; the team moved on to the next step (no code to score yet, and Andrew asked the team to decide and keep going)."));
                    continue;
                }
                // Andrew only sees work worth his time: under the bar
                // (Game Score 80), the team keeps improving on its own.
                {
                    if let Ok(sc) = crate::score::score(root, &a.slug) {
                        let total = sc["score"].as_f64().unwrap_or(0.0);
                        if total < REVIEW_BAR {
                            let n = lat.iter().filter(|c| c.key.as_deref().map(|k| k.starts_with(&format!("{mid}/improve/"))).unwrap_or(false)).count();
                            let hour_ago = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();
                            if all.iter().any(|c| c.key.as_deref().map(|k| k.starts_with(&format!("{mid}/improve/"))).unwrap_or(false) && c.created_at > hour_ago) {
                                continue;
                            }
                            let gaps: Vec<String> = sc["parts"].as_array().into_iter().flatten().flat_map(|p| p["missing"].as_array().cloned().unwrap_or_default()).filter_map(|m| m.as_str().map(String::from)).take(6).collect();
                            let job = format!("{NOT_GOOD_ENOUGH}\n\nThe Game Score is {total:.0}/100 (grade {}); Andrew reviews only at {REVIEW_BAR:.0}+. Biggest gaps: {}.", sc["grade"].as_str().unwrap_or("?"), if gaps.is_empty() { "see the score".into() } else { gaps.join("; ") });
                            if queue(root, &mut a, mid, "lead", &job, &format!("{mid}/improve/{}", n + 1), 0, "autopilot", None, "task").is_ok() {
                                post(&mut a, "autopilot", None, &format!("{title} scored {total:.0}/100, under the {REVIEW_BAR:.0} needed for Andrew's review: improving it (round {}).", n + 1));
                            }
                            continue;
                        }
                    }
                }
                a.milestones[i].status = "review".into();
                a.milestones[i].ready_at = Some(crate::now_rfc3339());
                post(&mut a, "autopilot", Some("andrew"), &format!("Ready for your review: {title}. {review}"));
                a.milestones[i].reminded = 0;
                let last = i + 1 == a.milestones.len();
                crate::notify::send_quiet(
                    crate::notify::Kind::NeedsApproval,
                    &if last { format!("{}: FINAL APPROVAL needed ({title})", a.title) } else { format!("{}: {title} is ready for your approval", a.title) },
                    &[review.to_string(), "Open BrewixGames: it is the Next step on the Dashboard. Approve it, or press Not Good Enough to keep improving.".into()],
                );
            }
        }
    }
    if a.milestones.iter().all(|m| m.status == "approved") {
        a.status = "done".into();
        post(&mut a, "autopilot", Some("andrew"), "Every milestone is approved. The game is ready for you to publish.");
        crate::notify::send_quiet(crate::notify::Kind::Finished, &format!("{}: done, ready for you to publish", a.title), &["Every part is approved. Nothing was published: open the game in Roblox Studio (Build, Open in Studio), play it once, then File, Publish to Roblox, and make it Public in its settings.".into()]);
    }
    let _ = save(root, &a);
}

/// The bar a game must reach against the best games in its niche.
pub const QUALITY_OVERALL: f64 = 8.5;
pub const QUALITY_MIN_AREA: f64 = 8.0;
/// Game Score needed, as a share of the points reachable without a Second
/// Opinion engine (which Andrew may not have set up).
pub const QUALITY_SCORE: f64 = 0.88;
const QUALITY_AGENTS: [&str; 10] = ["builder", "frame", "motion", "scenic", "modeler", "librarian", "merchant", "tuner", "crowd", "splash"];

const BENCH_JOB: &str = "Benchmark projects/<game> against the 3 best Roblox games in its niche. Find them in 16-Market-Research (visual-refs-*, teardown-*, reviews-* notes; their pictures are in attachments/game-refs/<id>/). Read the game's code and UI, and the latest playtest and simulation in the earlier work. Score each area 0 to 10 against those games (10 = as good as the best of them): first-60-seconds, core-loop, progression, ui-polish, visuals, environment, audio, game-feel, retention, monetization-fairness, phone-performance, content. Be strict: a rough game is 3 to 5; 8.5 or more means it would hold its own next to them. Write benchmark.md, and benchmark.json exactly: {\"compared_to\": [\"<game>\", ...], \"areas\": {\"first-60-seconds\": n, ...}, \"overall\": n, \"gaps\": [{\"area\": \"...\", \"fix\": \"<one specific change>\", \"agent\": \"<builder|frame|motion|scenic|modeler|librarian|merchant|tuner|crowd|splash>\"}]} with the biggest gaps first (up to 6). Read the last benchmark and the last test (change.md of the playtest in the earlier work): in benchmark.md say which of last round's fixes moved their area's score and which did not, and do not suggest again a fix that did not work; pick a different approach. Do not start other agents; the app assigns the fixes.";

/// The researcher's job (every third round of the quality loop).
const RESEARCH_JOB: &str = "Research what the 3 best Roblox games in this game's niche do best right now, area by area: first-60-seconds, core-loop, progression, ui-polish, visuals, environment, audio, game-feel, retention, monetization-fairness, phone-performance, content. Be concrete and observable (what the player sees, hears and does, with numbers where you can: timings, rewards, screen layouts). Use the 16-Market-Research notes and the game's references, and the web if you can reach it; mark anything you could not verify as UNVERIFIED. Also look for open-source or uncopylocked Roblox games in this niche (GitHub Rojo projects, DevForum uncopylocked posts) and study one or two with `brewix study <link>` (it saves their scripts and a backdoor scan; nothing runs): add the techniques worth borrowing per area, and never suggest copying flagged scripts, admin systems or free models. Read the newest notes in 19-Project-Memory/research/ too. Write top-games.md in your run folder (one section per area) and a short change.md. Do not change the game.";

/// Read a benchmark: overall, the lowest area, and the gaps.
pub fn read_benchmark(dir: &Path) -> Option<(f64, f64, Vec<(String, String, String)>, Vec<String>)> {
    let v: Value = serde_json::from_str(&fs::read_to_string(dir.join("benchmark.json")).ok()?).ok()?;
    let areas: Vec<f64> = v["areas"].as_object()?.values().filter_map(|x| x.as_f64()).collect();
    if areas.len() < 5 {
        return None;
    }
    let overall = v["overall"].as_f64().unwrap_or(areas.iter().sum::<f64>() / areas.len() as f64);
    let lowest = areas.iter().cloned().fold(10.0, f64::min);
    let gaps = v["gaps"].as_array().into_iter().flatten().filter_map(|g| Some((g["area"].as_str().unwrap_or("").to_string(), g["fix"].as_str()?.to_string(), g["agent"].as_str().unwrap_or("builder").to_string()))).take(6).collect();
    let vs = v["compared_to"].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)).collect();
    Some((overall, lowest, gaps, vs))
}

/// Whether a benchmark round reaches the top-games bar.
pub fn quality_pass(overall: f64, lowest: f64, score_share: f64) -> bool {
    overall >= QUALITY_OVERALL && lowest >= QUALITY_MIN_AREA && score_share >= QUALITY_SCORE
}

/// The game's own score as a share of what it can reach (0 to 1).
fn score_share(root: &Path, slug: &str) -> Option<f64> {
    let v = crate::score::score(root, slug).ok()?;
    let mut total = v["score"].as_f64()?;
    let mut max = 100.0;
    for p in v["parts"].as_array().into_iter().flatten() {
        if p["name"] == "Second Opinion" && p["got"].as_f64() == Some(0.0) {
            max -= p["max"].as_f64().unwrap_or(0.0);
            total -= 0.0;
        }
    }
    Some(total / max)
}

/// One step of the quality loop: benchmark against the top games, close the
/// biggest gaps, benchmark again, until the game reaches the bar. It never
/// stops on its own; only the game's budget (or Andrew's pause) does.
fn quality_step(root: &Path, a: &mut Autopilot, i: usize, all: &[Change]) {
    let r = a.milestones[i].round.max(1);
    a.milestones[i].round = r;
    let lat = latest(all, "quality");
    let get = |k: &str| lat.iter().find(|c| c.key.as_deref() == Some(k)).cloned();
    let open = |c: &Change| ["queued", "working", "paused"].contains(&c.status.as_str());
    // Notes from Andrew in this milestone come first.
    if lat.iter().any(|c| c.key.as_deref().map(|k| k.contains("/note/")).unwrap_or(false) && open(c)) {
        return;
    }
    // Rethink (after scores stopped improving) must finish before benchmarking.
    if let Some(t) = get(&format!("quality/{r}/rethink")) {
        if t.status != "done" {
            return;
        }
    }
    // Researcher: every third round (1, 4, 7...) find out again what the
    // best games in the niche do best, so the bar follows them.
    let research_round = r - (r - 1) % 3;
    let research_key = format!("quality/{research_round}/research");
    match get(&research_key) {
        None if research_round == r => {
            let _ = queue(root, a, "quality", "pulse", RESEARCH_JOB, &research_key, 0, "autopilot", None, "change");
            return;
        }
        Some(t) if open(&t) => return,
        _ => {}
    }
    let research = get(&research_key).filter(|t| t.status == "done").map(|t| format!(" The latest research on what the top games do best is runs/{}/top-games.md: score against it.", t.run_id)).unwrap_or_default();
    let bench_key = format!("quality/{r}/bench");
    let Some(bench) = get(&bench_key) else {
        let _ = queue(root, a, "quality", "director", &format!("{BENCH_JOB}{research}"), &bench_key, 0, "autopilot", None, "change");
        return;
    };
    if bench.status != "done" {
        return; // running, or being retried by on_finish
    }
    let dir = root.join("runs").join(&bench.run_id);
    let Some((overall, lowest, gaps, vs)) = read_benchmark(&dir) else {
        // Done but no readable benchmark.json: do it again.
        let _ = queue(root, a, "quality", "director", BENCH_JOB, &bench_key, bench.attempt + 1, "autopilot", Some("benchmark.json was missing or not in the required shape (at least 5 areas scored 0 to 10, overall, gaps). Write it exactly as described."), "change");
        return;
    };
    let fix_keys: Vec<String> = lat.iter().filter_map(|c| c.key.clone()).filter(|k| k.starts_with(&format!("quality/{r}/fix/"))).collect();
    if fix_keys.is_empty() {
        if a.milestones[i].history.len() < r as usize {
            a.milestones[i].history.push(overall);
        }
        let share = score_share(root, &a.slug).unwrap_or(0.0);
        let pass = quality_pass(overall, lowest, share);
        let line = format!("Round {r} against {}: overall {overall:.1}/10, lowest area {lowest:.1}, Game Score {:.0}% of reachable.", if vs.is_empty() { "the top games".into() } else { vs.join(", ") }, share * 100.0);
        if pass {
            post(a, "director", Some("andrew"), &format!("{line} The game reaches the bar ({QUALITY_OVERALL}+ overall, every area {QUALITY_MIN_AREA}+)."));
            a.milestones[i].status = "review".into();
            a.milestones[i].ready_at = Some(crate::now_rfc3339());
            post(a, "autopilot", Some("andrew"), "Ready for your review: the benchmark score reached the target (build checks passed; not yet played in Studio).");
            crate::notify::send_quiet(crate::notify::Kind::NeedsApproval, &format!("{}: benchmark score reached the target", a.title), &[line]);
            return;
        }
        post(a, "director", None, &format!("{line} Not there yet; fixing the biggest gaps."));
        let mut queued = 0;
        let mut list = gaps.clone();
        if list.is_empty() {
            list.push(("weakest areas".into(), format!("Raise the weakest areas in benchmark.md (runs/{}) toward the top games.", bench.run_id), "builder".into()));
        }
        if share < QUALITY_SCORE {
            list.push(("game score".into(), "Work through the Game Score's keep-working list in projects/<game>/docs/score.json (the parts other than Second Opinion).".into(), "builder".into()));
        }
        for (k, (area, fix, agent)) in list.into_iter().enumerate() {
            let agent = if QUALITY_AGENTS.contains(&agent.as_str()) { agent } else { "builder".into() };
            let job = format!("Close this gap to the top games ({area}): {fix} The full benchmark is runs/{}/benchmark.md. Make the change in projects/<game>; the game must still build.", bench.run_id);
            if queue(root, a, "quality", &agent, &job, &format!("quality/{r}/fix/{k}"), 0, "autopilot", None, "change").is_ok() {
                queued += 1;
            } else {
                return; // budget: the game waits for Andrew
            }
        }
        let _ = queued;
        return;
    }
    // Fixes of this round: wait until every one is done (failures are
    // retried and rescued by on_finish).
    if !fix_keys.iter().all(|k| get(k).map(|c| c.status == "done").unwrap_or(false)) {
        return;
    }
    // Tester: check this round's fixes work together before scoring again.
    let test_key = format!("quality/{r}/test");
    match get(&test_key) {
        None => {
            let job = format!("Test this round's fixes (quality round {r}; what they were meant to fix: runs/{}/benchmark.md). Run the build checks and walk through the first 60 seconds and the core loop (in Roblox Studio if it is connected, otherwise in the code). In change.md list what works, what broke, and every error, and say which fixes actually improved the game. If a fix broke something, ask builder to repair it with requests.json.", bench.run_id);
            let _ = queue(root, a, "quality", "playtest", &job, &test_key, 0, "autopilot", None, "change");
            return;
        }
        Some(t) if t.status != "done" => return,
        _ => {}
    }
    // Next round. If the score stopped climbing for two rounds, Lead
    // rethinks the approach first.
    let h = a.milestones[i].history.clone();
    let stalled = h.len() >= 3 && h[h.len() - 1] <= h[h.len() - 3] + 0.2;
    a.milestones[i].round = r + 1;
    if stalled {
        let job = format!("Rethink: the game's benchmark against the top games stopped improving ({}). Read the last benchmarks (runs/{} and earlier) and the fixes made, find why the gaps are not closing, and decide bigger changes: what to rebuild, cut or add. Write the plan in change.md and start the agents that should do it with requests.json.", h.iter().map(|x| format!("{x:.1}")).collect::<Vec<_>>().join(" → "), bench.run_id);
        post(a, "autopilot", Some("lead"), "The score stopped climbing; Lead is rethinking the approach before the next round.");
        let _ = queue(root, a, "quality", "lead", &job, &format!("quality/{}/rethink", r + 1), 0, "autopilot", None, "change");
    } else {
        let _ = queue(root, a, "quality", "director", BENCH_JOB, &format!("quality/{}/bench", r + 1), 0, "autopilot", None, "change");
    }
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
            "done" if key.starts_with("rescue|") => {
                rescue_done(root, &mut a, &mid, &key, c, true);
            }
            "done" => {
                post(&mut a, &c.agent, None, &format!("Done: {}", c.summary.clone().unwrap_or_else(|| c.text.chars().take(120).collect())));
                // Messages to other agents.
                let dir = root.join("runs").join(&c.run_id);
                let asks: Vec<Value> = fs::read_to_string(dir.join("requests.json")).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()).map(|v| if v.is_array() { v.as_array().cloned().unwrap_or_default() } else { vec![v] }).unwrap_or_default();
                let made_by_agents = mine(root, &slug).iter().filter(|x| x.milestone.as_deref() == Some(&mid) && x.source.starts_with("agent:") && x.attempt == 0 && ["queued", "working", "paused"].contains(&x.status.as_str())).count();
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
                    if mid == "plan" && to == "builder" {
                        post(&mut a, "autopilot", Some(&c.agent), "Builder starts after Andrew approves the plan; request skipped.");
                        continue;
                    }
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
            "failed" if changes::launch_failed(c) && {
                let kind = changes::env_problem(c).unwrap_or("launch");
                mine(root, &slug).iter().filter(|x| x.key.as_deref() == Some(&key) && changes::env_problem(x) == Some(kind)).count() <= changes::env_retry_limit(kind)
            } => {
                // Claude never started: the app or the computer, not the
                // agent. Try again by itself, waiting longer each time, and
                // without using up the agent's tries.
                let tries = mine(root, &slug).iter().filter(|x| x.key.as_deref() == Some(&key) && changes::launch_failed(x)).count();
                let wait = changes::backoff_minutes(tries);
                let detail = c.checks.iter().find(|k| !k.passed).map(|k| k.detail.clone()).unwrap_or_default();
                let kind = changes::env_problem(c).unwrap_or("launch");
                if kind == "localmodel" {
                    // Ollama's window was too small. The next local run restarts
                    // Ollama with a big window, so try once more at once; if it
                    // still fails, stop and say what to do instead of looping.
                    if tries >= 2 {
                        post(&mut a, "autopilot", Some(&c.agent), &format!("The local model keeps losing the request for {}. Trying again in 30 min (details in runs/{}/diagnostics.json).", c.agent, c.run_id));
                    } else {
                        post(&mut a, "autopilot", Some(&c.agent), &format!("The local model lost the request for {} (memory window too small). Restarting Ollama with a bigger window and trying again.", c.agent));
                    }
                    if let Ok(n) = queue(root, &mut a, &mid, &c.agent, &c.text, &key, c.attempt, &c.source, None, &c.kind) {
                        if tries >= 2 {
                            changes::delay(root, &n.id, 30);
                        }
                    }
                    let _ = save(root, &a);
                    drop(_g);
                    advance(root, &slug);
                    return;
                }
                let what = match kind { "signin" => "Claude is signed out", "network" => "No connection to Claude (the network or Claude's servers)", _ => "Could not start Claude" };
                post(&mut a, "autopilot", Some(&c.agent), &format!("{what} for {}. Trying again by itself in {wait} min. ({})", c.agent, detail.chars().take(140).collect::<String>()));
                let signouts = mine(root, &slug).iter().filter(|x| x.key.as_deref() == Some(&key) && changes::env_problem(x) == Some("signin")).count();
                if kind == "signin" && signouts == 1 {
                    // Only Andrew can sign in; tell him at once. Work picks up by itself after.
                    crate::notify::send_quiet(crate::notify::Kind::Failed, &format!("{}: Claude is signed out", a.title), &["Open BrewixGames, Settings, Claude Code, and sign in. The agents carry on by themselves once you have.".into()]);
                    post(&mut a, "autopilot", Some("andrew"), "Claude is signed out. Sign in from Settings (Claude Code); the agents carry on by themselves once you have.");
                }
                if tries == 3 && kind != "signin" {
                    crate::notify::send_quiet(crate::notify::Kind::Failed, &format!("{}: Claude will not start", a.title), &[detail.chars().take(300).collect(), "BrewixGames keeps retrying every 30 minutes. Check Settings, Claude Code, if this keeps happening.".into()]);
                }
                if let Ok(n) = queue(root, &mut a, &mid, &c.agent, &c.text, &key, c.attempt, &c.source, None, &c.kind) {
                    changes::delay(root, &n.id, wait);
                }
            }
            "failed" if key.starts_with("rescue|") => {
                // The rescuer itself failed: count it as a used rescue and go
                // back to the job with what we have.
                rescue_done(root, &mut a, &mid, &key, c, false);
            }
            "failed" => {
                let why: Vec<String> = c.checks.iter().filter(|k| !k.passed).map(|k| format!("- {}: {}", k.name, k.detail.chars().take(300).collect::<String>())).collect();
                let why = if why.is_empty() { c.summary.clone().unwrap_or_else(|| "the run did not finish".into()) } else { why.join("\n") };
                let rescues = mine(root, &slug).iter().filter(|x| x.key.as_deref().map(|k| k.starts_with("rescue|") && k.ends_with(&format!("|{key}"))).unwrap_or(false)).count();
                // Notes from Andrew and side work get one try: if it failed,
                // retrying the same thing only burns compute.
                if c.attempt + 1 >= MAX_ATTEMPTS || key.contains("/note/") || side_work(&key) {
                    // Out of tries: Lead works out why and how, then the job
                    // starts over with that advice.
                    let fails: Vec<String> = mine(root, &slug).iter().filter(|x| x.key.as_deref() == Some(&key) && x.status == "failed").map(|x| format!("runs/{}", x.run_id)).collect();
                    let job = format!(
                        "Rescue: {agent} failed this job {MAX_ATTEMPTS} times: \"{text}\". What failed last time:\n{why}\nIts attempts are in {folders}. Read them and find out why it keeps failing. Then write change.md: the cause, and exactly what {agent} must do differently (which inputs to use, what to cut or simplify, how to pass each failing check). If an input from another agent is missing or wrong, start that agent with requests.json. Do not do {agent}'s job yourself.",
                        agent = c.agent, text = c.text.chars().take(300).collect::<String>(), folders = fails.join(", ")
                    );
                    // It never gives up: after the first rescues it keeps trying a new
                    // approach every 30 minutes (only the game's budget stops it).
                    if rescues >= MAX_RESCUES || key.contains("/note/") || key.starts_with("plan/fold") || side_work(&key) {
                        // Lead's fixes did not help (or it was a note, which may
                        // just be a question): skip this job and keep the game
                        // moving instead of burning usage or waiting for Andrew.
                        skip(root, &mut a, c, &why);
                        let _ = save(root, &a);
                        drop(_g);
                        advance(root, &slug);
                        return;
                    }
                    let wait = 0;
                    post(&mut a, "autopilot", Some("lead"), &format!("{} failed {MAX_ATTEMPTS} times; Lead is working out why and a new way to do it{}.", c.agent, if wait > 0 { format!(" (rescue {}, in {wait} min)", rescues + 1) } else { String::new() }));
                    // One rescue at a time per game: more Leads at once only pile up.
                    let busy_rescue = mine(root, &slug).iter().any(|x| x.key.as_deref().map(|k| k.starts_with("rescue|")).unwrap_or(false) && ["queued", "working"].contains(&x.status.as_str()));
                    if let Ok(n) = queue(root, &mut a, &mid, "lead", &job, &format!("rescue|{}|{key}", rescues + 1), 0, "autopilot", None, "change") {
                        if wait > 0 || busy_rescue {
                            changes::delay(root, &n.id, if busy_rescue { wait.max(20) } else { wait });
                        }
                    }
                } else {
                    let next = c.attempt + 1;
                    post(&mut a, "autopilot", Some(&c.agent), &format!("{} did not pass ({}); trying again{} (attempt {} of {MAX_ATTEMPTS}).", c.agent, why.lines().next().unwrap_or("").trim_start_matches("- "), if next + 1 >= MAX_ATTEMPTS { " with a stronger model" } else { "" }, next + 1));
                    let _ = queue(root, &mut a, &mid, &c.agent, &c.text, &key, next, &c.source, Some(&why), &c.kind);
                }
            }
            "paused" if crate::runs::load(root, &c.run_id).ok().and_then(|r| r.resume_at).as_deref() == Some("9999-12-31T00:00:00Z") => post(&mut a, "andrew", Some(&c.agent), &format!("Paused {}. Its files are kept; Resume continues where it stopped.", c.agent)),
            "cancelled" => post(&mut a, "andrew", Some(&c.agent), &format!("Stopped {}.", c.agent)),
            "paused" => post(&mut a, "autopilot", None, "Claude's usage limit is reached. Work continues by itself when it resets."),
            _ => {}
        }
        let _ = save(root, &a);
    }
    advance(root, &slug);
}

/// Give up on one job and move on: it counts as finished (marked skipped,
/// with why), so the milestone carries on with the work that exists.
fn skip(root: &Path, a: &mut Autopilot, c: &Change, why: &str) {
    let first = why.lines().next().unwrap_or("").trim_start_matches("- ").chars().take(200).collect::<String>();
    changes::mark_skipped(root, &c.id, &format!("Skipped after {} tries: {first}", c.attempt + 1));
    let line = format!("Skipped {}'s job after it kept failing ({first}); the team carries on with what exists. Send a note if you want it tried another way.", c.agent);
    post(a, "autopilot", None, &line);
}

/// A rescue ended: start the rescued job over (fresh tries) with Lead's
/// advice as feedback.
fn rescue_done(root: &Path, a: &mut Autopilot, mid: &str, key: &str, rescue: &Change, ok: bool) {
    let orig_key = key.splitn(3, '|').nth(2).unwrap_or("").to_string();
    let Some(orig) = latest(&mine(root, &a.slug), mid).into_iter().find(|x| x.key.as_deref() == Some(orig_key.as_str())) else { return };
    // Once per rescue: the job was already queued again after this rescue.
    if orig.created_at > rescue.created_at || side_work(&orig_key) {
        return;
    }
    let advice = fs::read_to_string(root.join("runs").join(&rescue.run_id).join("change.md")).unwrap_or_default();
    let fb = if ok && !advice.trim().is_empty() {
        format!("Earlier attempts all failed. Lead looked into why; follow this advice:\n{}", advice.chars().take(3000).collect::<String>())
    } else {
        "Earlier attempts all failed. Read their change.md and checks in the earlier run folders and take a clearly different, simpler approach.".to_string()
    };
    post(a, "lead", Some(&orig.agent), &format!("Rescue {}: {}", if ok { "advice" } else { "did not finish" }, rescue.summary.clone().unwrap_or_default()));
    let _ = queue(root, a, mid, &orig.agent, &orig.text, &orig_key, 0, &orig.source, Some(&fb), &orig.kind);
}

/// Andrew's note at any time: every later job reads it, and it goes to the
/// agent that fits it now.
pub fn note(root: &Path, slug: &str, text: &str, agent: Option<&str>, milestone: Option<&str>) -> Result<Autopilot, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("Write a note first.".into());
    }
    // "Is it going?", "get to work", "retry now": a nudge, not a job. Wake
    // the team (resume, start waiting jobs now, next step) and say so.
    if is_ping(text) {
        {
            let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let mut a = load(root, slug)?;
            post(&mut a, "andrew", None, text);
            if a.status != "running" && a.status != "done" {
                a.status = "running".into();
                a.why = None;
            }
            save(root, &a)?;
        }
        changes::clear_retry_timers(root);
        advance(root, slug);
        let open = mine(root, slug).iter().filter(|c| ["queued", "working"].contains(&c.status.as_str())).count();
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut a = load(root, slug)?;
        post(&mut a, "warden", Some("andrew"), &if open > 0 { format!("On it: {open} job(s) are working or starting now. Nothing was waiting on you.") } else { "Nothing is queued; the Warden will give the team its next work.".into() });
        save(root, &a)?;
        return Ok(a);
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
    if a.milestones[i].id == "quality" && ["review", "approved"].contains(&a.milestones[i].status.as_str()) {
        // Notes on the quality milestone: make the change, then measure again.
        a.milestones[i].status = "working".into();
        a.milestones[i].round += 1;
    } else if a.milestones[i].status == "review" || a.milestones[i].status == "approved" {
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

/// Clear out wasted work: jobs made from status messages ("is it going?"),
/// rescues of notes or of the optional plan merge, rescues whose job is
/// already finished, and extra rescues (one per game). Returns how many.
pub fn tidy(root: &Path) -> usize {
    let all = changes::list(root);
    let note_text = |c: &Change| c.text.strip_prefix("Andrew's note: ").map(|t| t.trim_end_matches(". Make the change he asks for.").to_string());
    let ping_job = |c: &Change| c.source == "andrew" && note_text(c).map(|t| is_ping(&t)).unwrap_or(false);
    // The newest job of each key: is it finished?
    let finished = |key: &str| all.iter().filter(|x| x.key.as_deref() == Some(key)).next_back().map(|x| x.status == "done" || x.status == "cancelled").unwrap_or(true);
    let mut n = changes::cancel_queued(root, |c| ping_job(c));
    n += changes::cancel_queued(root, |c| {
        let Some(k) = c.key.as_deref().and_then(|k| k.strip_prefix("rescue|")) else { return false };
        let target = k.splitn(2, '|').nth(1).unwrap_or("");
        target.contains("/note/") || target.starts_with("plan/fold") || side_work(target) || finished(target)
    });
    // Failed status-message jobs and failed rescues of notes are not
    // waiting for anything: close them so the step can finish.
    for c in changes::list(root).into_iter().filter(|c| c.status == "failed") {
        let k = c.key.clone().unwrap_or_default();
        let note_rescue = k.starts_with("rescue|") && (k.contains("/note/") || k.contains("plan/fold") || side_work(&k));
        if ping_job(&c) || note_rescue {
            changes::mark_skipped(root, &c.id, "Skipped: a status message or a rescue that is no longer needed.");
            n += 1;
        }
    }
    // One queued rescue per game: keep the oldest.
    let mut kept: Vec<String> = vec![];
    let ids: Vec<String> = changes::list(root).into_iter().filter(|c| c.status == "queued" && c.key.as_deref().map(|k| k.starts_with("rescue|")).unwrap_or(false))
        .filter(|c| { let g = c.autopilot.clone().unwrap_or_default(); if kept.contains(&g) { true } else { kept.push(g); false } })
        .map(|c| c.id).collect();
    n += changes::cancel_queued(root, |c| ids.contains(&c.id));
    // Notes that were only status messages leave the notes list too.
    for mut a in list(root) {
        let before = a.notes.len();
        a.notes.retain(|t| !is_ping(t));
        if a.notes.len() != before {
            let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let _ = save(root, &a);
        }
    }
    n
}

/// A short status question or "go" command, not a change to make.
pub fn is_ping(text: &str) -> bool {
    let t = text.trim().to_lowercase();
    if t.len() > 90 {
        return false;
    }
    const PINGS: [&str; 25] = ["next step", "move on", "start the next", "is it going", "is it goin", "is it working", "is anything", "are they", "stuck", "why wait", "retry", "get to work", "keep going", "keep goin", "keep it going", "continue", "go on", "hurry", "status", "what is happening", "whats happening", "what's happening", "why is everyone", "start working", "resume"];
    let change_words = ["add ", "make ", "change", "fix ", "remove", "build ", "use ", "replace", "improve"];
    PINGS.iter().any(|p| t.contains(p)) && !change_words.iter().any(|w| t.contains(w))
        || matches!(t.trim_end_matches(['.', '!', '?']), "go" | "go go" | "start" | "work" | "hello" | "hi" | "?")
}

/// The rules every job gets, so the team spends its time on the game.
pub const ON_TRACK: &str = "STAY ON TRACK (always):\n1. Every run must move the game forward: new or better gameplay, UI, world, models, sound, or a real bug fixed. Checking, re-verifying or reporting work that is already done is not progress.\n2. If your job is already done, or there is nothing worth changing, write change.md with the first line \"Nothing to do:\" and the reason, and stop. Do not invent work.\n3. Do not start other agents unless the game needs that work now. Never ask an agent to re-check, re-verify or re-run something that passed.\n4. A message from Andrew that is only a question or a nudge (\"is it going?\", \"keep going\") needs no work: write \"Nothing to do:\" and stop.\n5. Never work on the app, its tools or its checks: only the game.\n6. Smallest change that finishes the job. Read only the files the job needs.";

/// Below this Game Score a step never waits for Andrew: the team keeps improving it.
pub const REVIEW_BAR: f64 = 80.0;

/// What "Not Good Enough" sends when Andrew writes nothing.
pub const NOT_GOOD_ENOUGH: &str = "Not good enough yet. Compare this part with the best Roblox games in this niche and raise every aspect that falls short (visuals, environment, UI, sound, animation and game feel, the core loop, code quality), weakest first. Decide the changes and start the agents that should make them with requests.json";

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
        if notes.trim().is_empty() {
            // "Not good enough" with no details: Lead decides what to raise.
            note(root, slug, NOT_GOOD_ENOUGH, Some("lead"), Some(milestone))?;
        }
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
                // Agents working now pause too (their files are kept).
                let mut n = 0;
                for c in mine(root, slug).iter().filter(|c| c.status == "working") {
                    crate::bridge::request_end(&c.run_id, true);
                    n += 1;
                }
                post(&mut a, "andrew", None, &format!("Paused the game{}. Nothing new starts until you resume.", if n > 0 { format!(" and {n} working agent(s); they continue where they stopped on Resume") } else { String::new() }));
            }
            "resume" => {
                a.status = "running".into();
                a.why = None;
                // Agents Andrew paused continue (the app's minute check starts them).
                for c in mine(root, slug).iter().filter(|c| c.status == "paused") {
                    if let Ok(mut r) = crate::runs::load(root, &c.run_id) {
                        if r.status == "paused" && r.resume_at.as_deref() == Some("9999-12-31T00:00:00Z") {
                            r.resume_at = None;
                            let _ = crate::runs::save(root, &r);
                        }
                    }
                }
                post(&mut a, "andrew", None, "Resumed the game.");
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

/// Lines under a "Questions for Andrew" or "Open questions" heading in an
/// agent's output: the choices left for Andrew, shown on the review card.
pub fn questions_in(path: &Path) -> Vec<String> {
    let Ok(t) = fs::read_to_string(path) else { return vec![] };
    let mut out = vec![];
    let mut on = false;
    for l in t.lines() {
        let h = l.trim_start();
        if h.starts_with('#') {
            let low = h.to_lowercase();
            on = low.contains("questions for andrew") || low.contains("open question") || low.contains("decisions for andrew");
            continue;
        }
        if on {
            let item = h.trim_start_matches(['-', '*', ' ']).trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ')').trim();
            let low = item.to_lowercase();
            if item.len() > 8 && !["none", "none.", "no open questions", "nothing"].iter().any(|n| low == *n || low.starts_with("none ")) {
                out.push(item.chars().take(300).collect());
            }
        }
        if out.len() >= 5 {
            break;
        }
    }
    out
}

/// Everything the Autopilot page shows for one game.
pub fn view(root: &Path, slug: &str) -> Result<Value, String> {
    let a = load(root, slug)?;
    let all = mine(root, slug);
    let (spent_usd, reserved) = spent(root, slug);
    let agents = crate::cost::load_agents(root).ok();
    let tpl = template();
    let ms: Vec<Value> = a.milestones.iter().enumerate().map(|(i, m)| {
        let jobs: Vec<Value> = shown_jobs(latest(&all, &m.id)).into_iter().map(|c| {
            let out = if c.kind == "task" { agents.as_ref().and_then(|ag| ag.agents.get(&c.agent).and_then(|p| p.output.clone())) } else { None }.unwrap_or_else(|| "change.md".into());
            let d = root.join("runs").join(&c.run_id);
            let mocks: Vec<String> = fs::read_dir(d.join("mockups")).into_iter().flatten().flatten().map(|e| format!("runs/{}/mockups/{}", c.run_id, e.file_name().to_string_lossy())).filter(|p| p.ends_with(".svg")).collect();
            let questions = questions_in(&d.join(&out)).into_iter().chain(questions_in(&d.join("change.md"))).collect::<Vec<_>>();
            let key = c.key.clone().unwrap_or_default();
            let rescues = all.iter().filter(|x| x.key.as_deref().map(|k| k.starts_with("rescue|") && k.ends_with(&format!("|{key}"))).unwrap_or(false)).count();
            let out_of_tries = stuck(&c);
            json!({ "id": c.id, "agent": c.agent, "text": c.text, "status": c.status, "stuck": false, "rescuing": out_of_tries, "rescues": rescues, "launch_failed": changes::launch_failed(&c), "retry_at": c.not_before, "questions": questions, "attempt": c.attempt, "summary": c.summary, "cost_usd": c.cost_usd, "run_id": c.run_id, "source": c.source,
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
        Change { id: format!("{key}-{attempt}"), target: "game:g".into(), item: None, text: "x".into(), agent: "frame".into(), status: status.into(), run_id: "r".into(), created_at: at.into(), finished_at: None, summary: None, checks: vec![], cost_usd: None, kind: "task".into(), autopilot: Some("g".into()), milestone: Some("ui".into()), key: Some(key.into()), attempt, source: "autopilot".into(), parent_run: None, group: None, depth: 0, after: None, not_before: None }
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
        // Its run ended too (nothing stays reserved against the budget).
        if let Ok(mut r) = crate::runs::load(root, &c.run_id) {
            r.status = if status == "done" { "done" } else { "failed" }.into();
            r.actual_cost_usd = Some(0.01);
            r.finished_at = Some(crate::now_rfc3339());
            let _ = crate::runs::save(root, &r);
        }
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
        // (First: the planner asked Merchant for a price check, which revises the plan.)
        let pdir = root.join("runs").join(&q[0].run_id);
        fs::create_dir_all(&pdir).unwrap();
        fs::write(pdir.join("requests.json"), r#"[{"to": "merchant", "task": "Check the prices", "why": "fair to kids"}]"#).unwrap();
        finish_as(&root, &q[0], "done", "");
        let m = queued(&root, &a.slug);
        assert_eq!(m.iter().map(|c| c.agent.as_str()).collect::<Vec<_>>(), vec!["merchant"]);
        finish_as(&root, &m[0], "done", "");
        let fold = queued(&root, &a.slug);
        assert_eq!(fold.len(), 1, "Planner folds Merchant's revision into the plan first");
        assert!(fold[0].key.as_deref().unwrap().starts_with("plan/fold/") && fold[0].agent == "planner");
        assert_eq!(load(&root, &a.slug).unwrap().milestones[0].status, "working");
        finish_as(&root, &fold[0], "done", "");
        let q = vec![fold[0].clone()];
        let a = load(&root, &a.slug).unwrap();
        assert_eq!(a.milestones[0].status, "approved", "the plan goes ahead without waiting for Andrew");
        assert_eq!(a.plan_run.as_deref(), Some(q[0].run_id.as_str()));
        let q = queued(&root, &a.slug);
        assert_eq!(q.iter().map(|c| c.agent.as_str()).collect::<Vec<_>>(), vec!["architect"]);
        // A failure is retried with what failed.
        finish_as(&root, &q[0], "failed", "core loop section");
        let q2 = queued(&root, &a.slug);
        assert_eq!(q2.len(), 1);
        assert_eq!((q2[0].agent.as_str(), q2[0].attempt), ("architect", 1));
        let prompt = crate::runs::load(&root, &q2[0].run_id).unwrap().prompt;
        assert!(prompt.contains("YOUR LAST ATTEMPT DID NOT PASS") && prompt.contains("core loop section"), "{prompt}");
        // Third attempt keeps the agent's own model; only the last moves up.
        finish_as(&root, &q2[0], "failed", "core loop section");
        let q3 = queued(&root, &a.slug);
        assert_ne!(crate::runs::load(&root, &q3[0].run_id).unwrap().estimate.model, STRONG_MODEL);
        assert_eq!(step_up("claude-haiku-5-5"), "claude-sonnet-5-5");
        assert_eq!(step_up("claude-sonnet-5-5"), STRONG_MODEL);
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
    fn a_job_that_keeps_failing_is_skipped_and_the_game_keeps_going() {
        let root = studio();
        let a = start(&root, "Stuck Game", "Catch falling stars.", 1_000_000.0, None).unwrap();
        let mut cur = queued(&root, &a.slug).remove(0);
        let first_key = cur.key.clone();
        // Every try and every rescue fails: after MAX_RESCUES rescues it is skipped.
        for _ in 0..(MAX_ATTEMPTS as usize * (MAX_RESCUES + 1) + MAX_RESCUES + 2) {
            if cur.key != first_key && cur.agent != "lead" { break; }
            if cur.agent == "lead" {
                finish_as(&root, &cur, "done", "");
            } else {
                finish_as(&root, &cur, "failed", "has a Loaded section");
            }
            match queued(&root, &a.slug).last() { Some(n) => cur = n.clone(), None => break }
        }
        let g = load(&root, &a.slug).unwrap();
        assert_eq!(g.status, "running", "never pauses for Andrew");
        let rescues = changes::list(&root).iter().filter(|x| x.key.as_deref().map(|k| k.starts_with("rescue|")).unwrap_or(false)).count();
        assert_eq!(rescues, MAX_RESCUES, "no endless rescues");
        assert!(changes::list(&root).iter().any(|c| c.key == first_key && c.status == "done" && c.summary.as_deref().unwrap_or("").starts_with("Skipped")), "skipped, counted as finished");
        assert!(g.board.iter().any(|b| b.text.contains("Skipped")));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn builder_audits_the_code_every_dozen_finished_jobs() {
        let root = studio();
        let a = start(&root, "Audit Game", "Catch falling stars.", 1_000_000.0, None).unwrap();
        let mut g = load(&root, &a.slug).unwrap();
        let few: Vec<Change> = (0..AUDIT_EVERY - 1).map(|i| ch(&format!("x/{i}"), 0, "done", "2026-01-01")).collect();
        code_audit(&root, &mut g, &few);
        assert!(!queued(&root, &a.slug).iter().any(|c| c.key.as_deref().unwrap_or("").starts_with("audit/")), "not before the game has code");
        fs::create_dir_all(root.join("projects").join(&a.slug)).unwrap();
        fs::write(root.join("projects").join(&a.slug).join("default.project.json"), "{}").unwrap();
        code_audit(&root, &mut g, &few);
        assert!(!queued(&root, &a.slug).iter().any(|c| c.key.as_deref().unwrap_or("").starts_with("audit/")), "not before a dozen jobs");
        // The UI kit is copied in once the game has code, without overwriting edits.
        fs::create_dir_all(root.join("studio/kits/ui/UI")).unwrap();
        fs::write(root.join("studio/kits/ui/UI/init.luau"), "return {}").unwrap();
        fs::create_dir_all(root.join("projects").join(&a.slug).join("src/shared/UI")).unwrap();
        fs::write(root.join("projects").join(&a.slug).join("src/shared/UI/Theme.luau"), "-- edited").unwrap();
        ensure_ui_kit(&root, &a.slug);
        assert!(root.join("projects").join(&a.slug).join("src/shared/UI/init.luau").is_file());
        assert_eq!(fs::read_to_string(root.join("projects").join(&a.slug).join("src/shared/UI/Theme.luau")).unwrap(), "-- edited");
        let enough: Vec<Change> = (0..AUDIT_EVERY).map(|i| ch(&format!("x/{i}"), 0, "done", "2026-01-01")).collect();
        code_audit(&root, &mut g, &enough);
        let q = queued(&root, &a.slug);
        let audit = q.iter().find(|c| c.key.as_deref() == Some("audit/1")).expect("audit queued");
        assert_eq!(audit.agent, "builder");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn a_paused_game_starts_nothing_and_dead_failures_are_picked_up() {
        let root = studio();
        let a = start(&root, "Held Game", "Catch falling stars.", 1_000_000.0, None).unwrap();
        // Paused: its waiting jobs do not start.
        control(&root, &a.slug, "pause", None).unwrap();
        assert!(changes::take_next(&root).iter().all(|c| c.autopilot.as_deref() != Some(a.slug.as_str())), "a paused game starts nothing");
        control(&root, &a.slug, "resume", None).unwrap();
        // A job failed, but its retry was never queued (the app closed in
        // between): the next advance picks it up.
        let job = queued(&root, &a.slug).remove(0);
        changes::set_result(&root, &job.id, "failed", vec![crate::queue::GateCheck { name: "has a Loaded section".into(), passed: false, detail: "missing".into() }]);
        changes::update_for_test(&root, &job.id, |x| x.finished_at = Some("2020-01-01T00:00:00Z".into()));
        assert!(!queued(&root, &a.slug).iter().any(|c| c.key == job.key), "nothing queued yet");
        advance(&root, &a.slug);
        let again = queued(&root, &a.slug);
        assert!(again.iter().any(|c| c.key == job.key && c.attempt == 1), "retried: {:?}", again.iter().map(|c| (c.key.clone(), c.attempt)).collect::<Vec<_>>());
        // Build-check text that says "timed out" is the agent's problem, not the network.
        let mut c = job.clone();
        c.status = "failed".into();
        c.checks = vec![crate::queue::GateCheck { name: "rojo build".into(), passed: false, detail: "No such file or directory (os error 2); request timed out".into() }];
        assert_eq!(changes::env_problem(&c), None);
        c.checks.push(crate::queue::GateCheck { name: "the agent finished".into(), passed: false, detail: "No response for 10 minutes: the agent was stuck".into() });
        assert_eq!(changes::env_problem(&c), Some("stall"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn a_finished_step_starts_the_next_even_with_failed_leftovers() {
        let root = studio();
        let a = start(&root, "Next Step Game", "Catch falling stars.", 0.0, None).unwrap();
        let mut g = load(&root, &a.slug).unwrap();
        g.milestones[0].status = "approved".into();
        save(&root, &g).unwrap();
        advance(&root, &a.slug);
        // Design: every planned wave done, plus a failed improve job and a failed rescue.
        for _ in 0..4 {
            let q: Vec<Change> = queued(&root, &a.slug).into_iter().filter(|c| c.milestone.as_deref() == Some("design")).collect();
            if q.is_empty() { break; }
            for c in &q { finish_as(&root, c, "done", ""); }
        }
        let mut g = load(&root, &a.slug).unwrap();
        for (k, st) in [("design/improve/1", "failed"), ("rescue|1|design/improve/1", "failed"), ("design/improve/2", "queued"), ("design/note/x", "paused")] {
            let c = queue(&root, &mut g, "design", "lead", &format!("extra {k}"), k, 3, "autopilot", None, "change").unwrap();
            changes::update_for_test(&root, &c.id, |x| { x.status = st.into(); x.attempt = 3; });
        }
        save(&root, &g).unwrap();
        // The game has code, so it has a (low) Game Score: Design must
        // still go ahead, not wait to reach 80.
        let proj = root.join("projects").join(&a.slug);
        fs::create_dir_all(proj.join("src/server")).unwrap();
        fs::write(proj.join("default.project.json"), r#"{"name":"g","tree":{"$className":"DataModel"}}"#).unwrap();
        fs::write(proj.join("src/server/Main.server.luau"), "print(1)").unwrap();
        assert!(crate::score::score(&root, &a.slug).map(|v| v["score"].as_f64().unwrap_or(0.0) < REVIEW_BAR).unwrap_or(false), "the test game scores under the bar");
        // A waiting extra job does not hold the step either.
        advance(&root, &a.slug);
        let g = load(&root, &a.slug).unwrap();
        assert_eq!(g.milestones[1].status, "approved", "design closes: {:?}", g.board.iter().rev().take(5).map(|b| b.text.clone()).collect::<Vec<_>>());
        assert_eq!(g.milestones[2].status, "working", "UI starts");
        assert!(queued(&root, &a.slug).iter().any(|c| c.agent == "frame" && c.milestone.as_deref() == Some("ui")), "Frame drafts the UI");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn no_job_can_loop_forever() {
        let mk = |i: usize, agent: &str, key: &str, text: &str| {
            let mut c = crate::changes::Change { id: format!("c{i}"), target: "game:g".into(), item: None, text: text.into(), agent: agent.into(), status: "failed".into(), run_id: format!("r{i}"), created_at: crate::now_rfc3339(), finished_at: None, summary: None, checks: vec![], cost_usd: None, kind: "change".into(), autopilot: Some("g".into()), milestone: Some("design".into()), key: Some(key.into()), attempt: 0, source: "autopilot".into(), parent_run: None, group: None, depth: 0, after: None, not_before: None };
            c.attempt = (i % 4) as u32;
            c
        };
        let same_key: Vec<_> = (0..8).map(|i| mk(i, "lead", "design/improve/1", &format!("try {i}"))).collect();
        assert!(loop_guard(&same_key, "lead", "new", "design/improve/2").is_some(), "improve rounds count as one job");
        let same_text: Vec<_> = (0..6).map(|i| mk(i, "builder", &format!("k{i}"), "Make the shop button bigger")).collect();
        assert!(loop_guard(&same_text, "builder", "Make the shop button bigger", "k9").is_some());
        assert!(loop_guard(&same_text, "frame", "Make the shop button bigger", "k9").is_none(), "another agent is different work");
        let busy: Vec<_> = (0..40).map(|i| mk(i, "builder", &format!("x{i}"), &format!("job {i}"))).collect();
        assert!(loop_guard(&busy, "frame", "something new", "y").is_some(), "40 jobs an hour is the ceiling");
        assert!(loop_guard(&busy[..5], "frame", "something new", "y").is_none());
    }

    #[test]
    fn a_waiting_approval_is_reminded_every_six_hours_at_most_four_times() {
        let root = studio();
        let a = start(&root, "Remind Game", "Catch falling stars.", 0.0, None).unwrap();
        let mut g = load(&root, &a.slug).unwrap();
        g.milestones[0].status = "approved".into();
        g.milestones[1].status = "review".into();
        g.milestones[1].ready_at = Some((chrono::Utc::now() - chrono::Duration::hours(7)).to_rfc3339());
        save(&root, &g).unwrap();
        advance(&root, &a.slug);
        assert_eq!(load(&root, &a.slug).unwrap().milestones[1].reminded, 1);
        advance(&root, &a.slug);
        assert_eq!(load(&root, &a.slug).unwrap().milestones[1].reminded, 1, "not again before 12 hours");
        let mut g = load(&root, &a.slug).unwrap();
        g.milestones[1].ready_at = Some((chrono::Utc::now() - chrono::Duration::hours(100)).to_rfc3339());
        save(&root, &g).unwrap();
        for _ in 0..6 {
            advance(&root, &a.slug);
        }
        assert_eq!(load(&root, &a.slug).unwrap().milestones[1].reminded, 4, "four at most");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn ask_claude_starts_work_on_the_game_itself() {
        let root = studio();
        let a = start(&root, "Chat Game", "Catch falling stars.", 0.0, None).unwrap();
        let before = queued(&root, &a.slug).len();
        let run = root.join("runs/run-assistant-t1");
        fs::create_dir_all(&run).unwrap();
        fs::write(run.join("requests.json"), r#"[{"to": "frame", "task": "Build the shop UI"}, {"action": "resume"}]"#).unwrap();
        let done = crate::chat::act_on_requests(&root, "run-assistant-t1");
        assert!(done.iter().any(|d| d.starts_with("Started frame on chat-game")), "{done:?}");
        assert!(queued(&root, &a.slug).iter().any(|c| c.agent == "frame" && c.text.contains("Build the shop UI")));
        assert!(queued(&root, &a.slug).len() > before);
        assert!(crate::chat::act_on_requests(&root, "run-assistant-t1").is_empty(), "never twice");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn opening_the_app_picks_everything_back_up() {
        let root = studio();
        let a = start(&root, "Wake Game", "Catch falling stars.", 0.0, None).unwrap();
        let b = start(&root, "Held Game", "Collect shells.", 0.0, None).unwrap();
        // An older version paused one game for a problem; Andrew paused the other.
        let mut g = load(&root, &a.slug).unwrap();
        g.status = "paused".into();
        g.why = Some("planner is stuck".into());
        save(&root, &g).unwrap();
        control(&root, &b.slug, "pause", None).unwrap();
        let j = queued(&root, &a.slug).remove(0);
        changes::delay(&root, &j.id, 30);
        wake_all(&root);
        assert_eq!(load(&root, &a.slug).unwrap().status, "running", "a problem pause is lifted");
        assert_eq!(load(&root, &b.slug).unwrap().status, "paused", "Andrew's own pause stays");
        assert!(changes::list(&root).iter().find(|c| c.id == j.id).unwrap().not_before.is_none(), "no waiting out an old retry timer");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn the_warden_keeps_the_team_working() {
        let root = studio();
        let a = start(&root, "Warden Game", "Catch falling stars.", 0.0, None).unwrap();
        // A job that says working, but its agent is gone (run says running, no process).
        let j = queued(&root, &a.slug).remove(0);
        changes::update_for_test(&root, &j.id, |x| x.status = "working".into());
        let mut r = crate::runs::load(&root, &j.run_id).unwrap();
        r.status = "running".into();
        crate::runs::save(&root, &r).unwrap();
        let did = crate::warden::rounds(&root);
        assert!(did.iter().any(|d| d.contains("stuck")), "{did:?}");
        assert!(queued(&root, &a.slug).iter().any(|c| c.key == j.key && c.id != j.id), "retried");
        // An idle team: nothing open at all. The Warden gets someone working.
        changes::cancel_queued(&root, |_| true);
        let did = crate::warden::rounds(&root);
        assert!(!did.is_empty(), "the Warden acted: {did:?}");
        assert!(!queued(&root, &a.slug).is_empty(), "someone is working again");
        // A step waiting for Andrew: research meanwhile, once.
        changes::cancel_queued(&root, |_| true);
        let mut g = load(&root, &a.slug).unwrap();
        g.milestones[0].status = "review".into();
        save(&root, &g).unwrap();
        let did = crate::warden::rounds(&root);
        assert!(did.iter().any(|d| d.contains("research")), "{did:?}");
        changes::cancel_queued(&root, |_| true);
        assert!(crate::warden::rounds(&root).is_empty(), "only once per waiting step");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn pings_long_ideas_and_lost_tasks_waste_nothing() {
        // Status messages are nudges, change requests are not.
        for t in ["is it goign they are both stuck on starting...?", "retry now why wait to 4:52", "get to work", "keep going", "why is everyone paused get to work"] {
            assert!(is_ping(t), "{t}");
        }
        for t in ["make the UI nicer", "add footsteps to the game too", "fix the shop button"] {
            assert!(!is_ping(t), "{t}");
        }
        assert!(crate::bridge::lost_task("I'm ready to help with file operations. Please provide the file path"));
        assert!(!crate::bridge::lost_task("Built the shop UI from the kit."));
        let root = studio();
        // A long spec goes to a file once; jobs get a short opening plus its path.
        let spec = "BUILD A COMPLETE ROBLOX GAME. ".repeat(400);
        let a = start(&root, "Big Game", &spec, 0.0, None).unwrap();
        let job = queued(&root, &a.slug).remove(0);
        let prompt = crate::runs::load(&root, &job.run_id).unwrap().prompt;
        assert!(prompt.len() < spec.len() / 2, "prompt {} vs spec {}", prompt.len(), spec.len());
        assert!(prompt.contains("projects/big-game/docs/idea.md"));
        assert_eq!(fs::read_to_string(root.join("projects/big-game/docs/idea.md")).unwrap(), spec.trim());
        // A ping note starts no job.
        let before = changes::list(&root).len();
        note(&root, &a.slug, "is it going?", None, None).unwrap();
        assert_eq!(changes::list(&root).len(), before, "no job from a status message");
        assert!(load(&root, &a.slug).unwrap().board.iter().any(|b| b.from == "warden"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn not_good_enough_sends_it_back_to_keep_improving() {
        let root = studio();
        let a = start(&root, "Meh Game", "Catch falling stars.", 500.0, None).unwrap();
        let mut g = load(&root, &a.slug).unwrap();
        g.milestones[0].status = "review".into();
        let mid = g.milestones[0].id.clone();
        save(&root, &g).unwrap();
        review(&root, &a.slug, &mid, false, "").unwrap();
        let g = load(&root, &a.slug).unwrap();
        assert_eq!(g.milestones[0].status, "revising");
        assert!(queued(&root, &a.slug).iter().any(|c| c.agent == "lead" && c.text.contains("Not good enough yet")), "Lead picks up the work");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn a_cut_off_local_request_is_retried_then_waits() {
        let root = studio();
        let a = start(&root, "Local Game", "Catch falling stars.", 500.0, None).unwrap();
        let mut cur = queued(&root, &a.slug).remove(0);
        for _ in 0..2 {
            changes::set_result(&root, &cur.id, "failed", vec![crate::queue::GateCheck { name: "the agent finished".into(), passed: false, detail: "API Error: 500 no user query found in messages. (127.0.0.1:11434)".into() }]);
            let c = changes::list(&root).into_iter().find(|x| x.id == cur.id).unwrap();
            assert_eq!(changes::env_problem(&c), Some("localmodel"), "not a network problem");
            on_finish(&root, &c);
            cur = queued(&root, &a.slug).last().unwrap().clone();
        }
        assert!(cur.not_before.is_some(), "after two, it waits 30 min instead of looping fast");
        assert_eq!(load(&root, &a.slug).unwrap().status, "running", "no pause");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn launch_failures_retry_by_themselves_and_lead_rescues_real_failures() {
        let root = studio();
        let a = start(&root, "Rescue Game", "Catch falling stars.", 500.0, None).unwrap();
        let planner = queued(&root, &a.slug).remove(0);
        // Claude could not start (the Windows command-line limit): no try used.
        changes::set_result(&root, &planner.id, "failed", vec![crate::queue::GateCheck { name: "the agent finished".into(), passed: false, detail: "could not start D:\\claude.exe: The filename or extension is too long. (os error 206)".into() }]);
        let c = changes::list(&root).into_iter().find(|x| x.id == planner.id).unwrap();
        on_finish(&root, &c);
        let again = queued(&root, &a.slug);
        assert_eq!(again.len(), 1);
        assert_eq!((again[0].agent.as_str(), again[0].attempt), ("planner", 0), "no try used up");
        assert!(again[0].not_before.is_some(), "waits a minute first");
        assert!(changes::take_next(&root).is_empty(), "not started before its time");
        // Four real failures: Lead is called in instead of giving up.
        let mut cur = again[0].clone();
        for _ in 0..MAX_ATTEMPTS {
            finish_as(&root, &cur, "failed", "has a Loaded section");
            let q = queued(&root, &a.slug);
            cur = q.last().unwrap().clone();
        }
        assert_eq!(cur.agent, "lead", "Lead rescues after {MAX_ATTEMPTS} failed tries");
        assert!(cur.key.as_deref().unwrap().starts_with("rescue|1|"));
        let rp = crate::runs::load(&root, &cur.run_id).unwrap().prompt;
        assert!(rp.contains("failed this job") && rp.contains("has a Loaded section"), "{rp}");
        let rd = root.join("runs").join(&cur.run_id);
        fs::create_dir_all(&rd).unwrap();
        fs::write(rd.join("change.md"), "Cause: the plan had no Loaded section. Add one listing the notes read.").unwrap();
        finish_as(&root, &cur, "done", "");
        let fresh = queued(&root, &a.slug);
        assert_eq!((fresh[0].agent.as_str(), fresh[0].attempt), ("planner", 0), "fresh tries with the advice");
        let fp = crate::runs::load(&root, &fresh[0].run_id).unwrap().prompt;
        assert!(fp.contains("Lead looked into why") && fp.contains("Add one listing the notes read"), "{fp}");
        let v = view(&root, &a.slug).unwrap();
        assert_eq!(v["milestones"][0]["jobs"].as_array().unwrap().iter().filter(|j| j["agent"] == "planner").next().unwrap()["status"], "queued", "the screen shows the fresh start, not the old failure");
        let _ = fs::remove_dir_all(&root);
    }

    fn bench(root: &Path, c: &Change, overall: f64, areas: f64) {
        let d = root.join("runs").join(&c.run_id);
        fs::create_dir_all(&d).unwrap();
        let a: serde_json::Map<String, Value> = ["first-60-seconds", "core-loop", "progression", "ui-polish", "visuals", "game-feel"].iter().map(|k| (k.to_string(), json!(areas))).collect();
        fs::write(d.join("benchmark.json"), json!({"compared_to": ["Pet Simulator 99", "Adopt Me!"], "areas": a, "overall": overall,
            "gaps": [{"area": "game-feel", "fix": "Add a squash and sparkle when a pet is caught", "agent": "motion"}, {"area": "ui-polish", "fix": "Bigger, outlined shop buttons", "agent": "frame"}, {"area": "x", "fix": "y", "agent": "wizard"}]}).to_string()).unwrap();
    }

    #[test]
    fn quality_loop_benchmarks_fixes_and_never_stops_until_the_bar() {
        assert!(quality_pass(8.6, 8.2, 0.9));
        assert!(!quality_pass(9.5, 7.5, 1.0), "every area must reach the bar");
        assert!(!quality_pass(8.6, 8.2, 0.5), "the Game Score must agree");
        let root = studio();
        let a = start(&root, "Quality Game", "Catch pets.", 500.0, None).unwrap();
        // Jump to the quality milestone.
        {
            let mut g = load(&root, &a.slug).unwrap();
            changes::cancel_queued(&root, |_| true);
            for m in g.milestones.iter_mut() {
                m.status = if m.id == "quality" { "waiting".into() } else if m.id == "polish" { "waiting".into() } else { "approved".into() };
            }
            save(&root, &g).unwrap();
        }
        advance(&root, &a.slug);
        // Researcher first: what the top games do best.
        let rs = queued(&root, &a.slug);
        assert_eq!((rs.len(), rs[0].agent.as_str(), rs[0].key.as_deref()), (1, "pulse", Some("quality/1/research")));
        finish_as(&root, &rs[0], "done", "");
        let b1 = queued(&root, &a.slug);
        assert_eq!((b1.len(), b1[0].agent.as_str(), b1[0].key.as_deref()), (1, "director", Some("quality/1/bench")));
        assert!(crate::runs::load(&root, &b1[0].run_id).unwrap().prompt.contains("top-games.md"), "the reviewer scores against the research");
        assert!(crate::runs::load(&root, &b1[0].run_id).unwrap().prompt.contains("3 best Roblox games in its niche"));
        // Round 1: far from the bar: the gaps become fixes for the right agents.
        bench(&root, &b1[0], 5.0, 5.0);
        finish_as(&root, &b1[0], "done", "");
        let fixes = queued(&root, &a.slug);
        let agents: Vec<&str> = fixes.iter().map(|c| c.agent.as_str()).collect();
        assert!(agents.contains(&"motion") && agents.contains(&"frame") && agents.contains(&"builder"), "{agents:?}");
        assert!(fixes.iter().all(|c| c.key.as_deref().unwrap().starts_with("quality/1/fix/")));
        assert!(load(&root, &a.slug).unwrap().board.iter().any(|m| m.text.contains("Round 1") && m.text.contains("5.0/10")));
        // Fixes done: round 2 benchmark. Three flat rounds: Lead rethinks first.
        let mut round_bench = None;
        for r in 1..=3 {
            for f in queued(&root, &a.slug).into_iter().filter(|c| c.key.as_deref().map(|k| k.contains("/fix/")).unwrap_or(false)) {
                finish_as(&root, &f, "done", "");
            }
            // Tester checks the fixes before the next score.
            let t = queued(&root, &a.slug).into_iter().find(|c| c.key.as_deref() == Some(&format!("quality/{r}/test"))).unwrap_or_else(|| panic!("tester after the fixes: {:?} {:?} {:?}", queued(&root, &a.slug).iter().map(|c| c.key.clone()).collect::<Vec<_>>(), load(&root, &a.slug).unwrap().status, load(&root, &a.slug).unwrap().board.last().map(|b| b.text.clone())));
            assert_eq!(t.agent, "playtest");
            finish_as(&root, &t, "done", "");
            let q = queued(&root, &a.slug);
            if r < 3 {
                let b = q.iter().find(|c| c.key.as_deref() == Some(&format!("quality/{}/bench", r + 1))).cloned().expect("next round's benchmark");
                bench(&root, &b, 5.0, 5.0);
                finish_as(&root, &b, "done", "");
                round_bench = Some(b);
            } else {
                assert!(q.iter().any(|c| c.agent == "lead" && c.key.as_deref() == Some("quality/4/rethink")), "{:?}", q.iter().map(|c| c.key.clone()).collect::<Vec<_>>());
                assert!(!q.iter().any(|c| c.key.as_deref() == Some("quality/4/bench")), "rethink comes first");
            }
        }
        assert!(round_bench.is_some());
        let m = load(&root, &a.slug).unwrap().milestones.into_iter().find(|m| m.id == "quality").unwrap();
        assert_eq!(m.status, "working", "below the bar it never goes to review");
        assert_eq!(m.history, vec![5.0, 5.0, 5.0]);
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn older_games_get_the_new_milestone_in_order() {
        let root = studio();
        let a = start(&root, "Old Game", "x", 500.0, None).unwrap();
        let mut g = load(&root, &a.slug).unwrap();
        g.milestones.retain(|m| m.id != "quality");
        g.milestones[0].status = "approved".into();
        save(&root, &g).unwrap();
        let back = load(&root, &a.slug).unwrap();
        assert_eq!(back.milestones.iter().map(|m| m.id.as_str()).collect::<Vec<_>>(), vec!["plan", "design", "ui", "world", "build", "quality", "polish"]);
        assert_eq!(back.milestones[0].status, "approved", "statuses kept");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn never_stops_for_a_budget() {
        let root = studio();
        let a = start(&root, "Tiny Budget", "An obby.", 0.0, None).unwrap();
        assert_eq!(a.status, "running", "budgets were removed");
        assert_eq!(queued(&root, &a.slug).len(), 1, "work starts");
        // A game saved as stopped at its budget (older version) carries on.
        let mut g = load(&root, &a.slug).unwrap();
        g.status = "budget".into();
        save(&root, &g).unwrap();
        assert_eq!(load(&root, &a.slug).unwrap().status, "running");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn finds_the_questions_left_for_andrew() {
        let p = std::env::temp_dir().join(format!("q-{}.md", std::process::id()));
        fs::write(&p, "# Design\nstuff\n## Questions for Andrew\n- Keep the Cloud Decor bundle or cut it from v1?\n2. Rainbow Wand at 99 or 149 R$?\n## Loaded\n- 00-Start-Here\n").unwrap();
        assert_eq!(questions_in(&p), vec!["Keep the Cloud Decor bundle or cut it from v1?", "Rainbow Wand at 99 or 149 R$?"]);
        fs::write(&p, "## Open questions\n- None.\n").unwrap();
        assert!(questions_in(&p).is_empty());
        let _ = fs::remove_file(&p);
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
