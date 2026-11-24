//! The Warden: every few minutes, makes sure the team is actually working
//! while the PC is on. Fixes "working" jobs with no agent behind them,
//! wakes a running game whose team is idle (next step, or Lead decides),
//! and puts the team on research while a step waits for Andrew. Writes
//! runs/warden.json (last round and what it did) for the village.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub fn rounds(root: &Path) -> Vec<String> {
    let mut did = vec![];
    let cleared = crate::autopilot::tidy(root);
    if cleared > 0 {
        did.push(format!("cleared {cleared} wasted job(s) (status messages, duplicate or pointless rescues)"));
    }
    // 0. Two workers on the same thing waste compute: cancel the copy.
    let dups = crate::changes::cancel_duplicates(root);
    if !dups.is_empty() {
        let mut names: Vec<String> = dups.iter().map(|d| d.0.clone()).collect();
        names.sort();
        names.dedup();
        did.push(format!("cancelled {} duplicate job(s) that repeated work already running or waiting ({})", dups.len(), names.join(", ")));
    }
    // 1. Jobs that say "working" but have no agent running: let them retry.
    let live = crate::bridge::running();
    for c in crate::changes::list(root).into_iter().filter(|c| c.status == "working" && !live.contains(&c.run_id)) {
        // Only when its agent really stopped: the run ended, or it says
        // running but no process is behind it. A job just starting (run
        // still "approved") is left alone.
        let Ok(run) = crate::runs::load(root, &c.run_id) else { continue };
        let dead = match run.status.as_str() {
            "approved" => false,
            "running" => !live.contains(&run.run_id),
            "paused" => false,
            _ => true,
        };
        if dead {
            crate::changes::set_result(root, &c.id, "failed", vec![crate::queue::GateCheck { name: "the agent finished".into(), passed: false, detail: "No response for a while: the Warden found no agent running this job".into() }]);
            if let Some(done) = crate::changes::list(root).into_iter().find(|x| x.id == c.id) {
                if done.autopilot.is_some() {
                    crate::autopilot::on_finish(root, &done);
                }
            }
            did.push(format!("found {} stuck with nobody working on it and sent it back to be retried", c.agent));
        }
    }
    // 2. Efficiency: an agent that failed 6+ times in the last hour is
    // benched for an hour (its waiting jobs wait), with the reason posted,
    // instead of burning tries and usage on the same problem.
    let hour_ago = (chrono::Utc::now() - chrono::Duration::hours(1)).to_rfc3339();
    let all = crate::changes::list(root);
    let mut agents: Vec<String> = all.iter().map(|c| c.agent.clone()).collect();
    agents.sort();
    agents.dedup();
    for ag in agents {
        let fails: Vec<&crate::changes::Change> = all.iter().filter(|c| c.agent == ag && c.status == "failed" && c.finished_at.as_deref().map(|t| t > hour_ago.as_str()).unwrap_or(false)).collect();
        if fails.len() < 6 {
            continue;
        }
        let waiting: Vec<String> = all.iter().filter(|c| c.agent == ag && c.status == "queued" && c.not_before.as_deref().map(|t| t < (chrono::Utc::now() + chrono::Duration::minutes(50)).to_rfc3339().as_str()).unwrap_or(true)).map(|c| c.id.clone()).collect();
        if waiting.is_empty() {
            continue;
        }
        for id in &waiting {
            crate::changes::delay(root, id, 60);
        }
        let why = fails.last().and_then(|c| c.checks.iter().find(|k| !k.passed).map(|k| format!("{}: {}", k.name, k.detail.chars().take(120).collect::<String>()))).unwrap_or_default();
        for g in crate::autopilot::list(root).into_iter().filter(|g| all.iter().any(|c| c.agent == ag && c.autopilot.as_deref() == Some(g.slug.as_str()) && waiting.contains(&c.id))) {
            crate::autopilot::warden_note(root, &g.slug, &format!("{ag} failed {} times in the last hour ({why}). Its {} waiting job(s) wait an hour so the team stops repeating the same failure; the rest keep working.", fails.len(), waiting.len()));
        }
        did.push(format!("{ag} kept failing ({} times in an hour); held its {} waiting job(s) for an hour", fails.len(), waiting.len()));
    }
    // 3. Running games with an idle team.
    for g in crate::autopilot::list(root).into_iter().filter(|g| g.status == "running") {
        let open = |root: &Path| crate::changes::list(root).into_iter().any(|c| c.autopilot.as_deref() == Some(g.slug.as_str()) && ["queued", "working", "paused"].contains(&c.status.as_str()));
        if open(root) {
            continue;
        }
        let waiting_review = g.milestones.iter().any(|m| m.status == "review");
        if waiting_review {
            if crate::autopilot::warden_research(root, &g.slug) {
                did.push(format!("{}: while a step waits for Andrew, sent Pulse to research the top games for the next steps", g.title));
            }
            continue;
        }
        crate::autopilot::advance(root, &g.slug);
        if open(root) {
            did.push(format!("{}: the team was idle; started the next step", g.title));
            continue;
        }
        if crate::autopilot::warden_lead(root, &g.slug) {
            did.push(format!("{}: the team was idle with nothing queued; asked Lead to decide the next work", g.title));
        }
    }
    let _ = fs::write(root.join("runs/warden.json"), json!({ "at": crate::now_rfc3339(), "did": did }).to_string());
    did
}

pub fn last(root: &Path) -> Value {
    crate::paths::read_json(&root.join("runs/warden.json")).unwrap_or(json!({}))
}
