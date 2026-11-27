//! Run requests. A run starts as `awaiting-approval` with a cost estimate.
//! It can only start after Andrew approves it, and approval must restate the
//! exact cap that was shown, so a run can never start on a number nobody saw.

use crate::cost::{self, Estimate};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunRequest {
    pub run_id: String,
    pub agent: String,
    pub prompt: String,
    pub estimate: Estimate,
    /// awaiting-approval | approved | running | done | failed | cancelled
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub approved_at: Option<String>,
    #[serde(default)]
    pub finished_at: Option<String>,
    #[serde(default)]
    pub actual_cost_usd: Option<f64>,
    #[serde(default)]
    pub result_text: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    /// Optional pipeline this run belongs to.
    #[serde(default)]
    pub pipeline_id: Option<String>,
    /// Chat this run belongs to (Ask Claude). Chat runs keep their session.
    #[serde(default)]
    pub chat: Option<String>,
    /// Claude Code session to continue (chat follow-ups).
    #[serde(default)]
    pub resume: Option<String>,
    /// "claude" or "ollama": which engine this run was prepared for.
    #[serde(default)]
    pub engine: Option<String>,
    /// Paused by a usage limit: when to continue (RFC 3339).
    #[serde(default)]
    pub resume_at: Option<String>,
    /// How many times this run was continued after a pause.
    #[serde(default)]
    pub continued: u32,
    /// Cost of the attempts before a pause (added to the final cost).
    #[serde(default)]
    pub prior_cost_usd: f64,
    /// Set when this run carries out a change request (changes.rs).
    #[serde(default)]
    pub change: Option<String>,
    /// Why the run ended early: "turn_limit", "budget_cap" or "usage_limit".
    #[serde(default)]
    pub stop_reason: Option<String>,
    /// What the run really used (from Claude Code's result): turns,
    /// duration_ms, input/cache_read/cache_write/output tokens.
    #[serde(default)]
    pub usage: Option<serde_json::Value>,
    /// The --max-turns this run was started with.
    #[serde(default)]
    pub turn_limit: Option<u32>,
}

/// True when Claude stopped because a usage limit was reached.
pub fn is_limit(text: &str) -> bool {
    let t = text.to_lowercase();
    // Claude's own wording only. Plain "rate limit" is common in Roblox
    // work (RemoteEvent rate limits) and must not pause a failed run.
    t.contains("usage limit reached") || t.contains("hit your limit") || t.contains("rate_limit_error") || t.contains("5-hour limit") || t.contains("weekly limit")
}

/// When the limit resets, from Claude's message ("resets 2pm (UTC)",
/// "resets 12:40am", or "|<unix time>"). Falls back to 30 minutes from now.
pub fn reset_time(text: &str, now: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Utc> {
    use chrono::{Duration, TimeZone, Timelike};
    if let Some(c) = regex::Regex::new(r"\|(\d{10})\b").unwrap().captures(text) {
        if let Some(t) = c[1].parse::<i64>().ok().and_then(|s| chrono::Utc.timestamp_opt(s, 0).single()) {
            if t > now {
                return t + Duration::minutes(1);
            }
        }
    }
    if let Some(c) = regex::Regex::new(r"(?i)resets?\s+(?:at\s+)?(\d{1,2})(?::(\d{2}))?\s*(am|pm)").unwrap().captures(text) {
        let mut h: u32 = c[1].parse().unwrap_or(0) % 12;
        if c[3].eq_ignore_ascii_case("pm") {
            h += 12;
        }
        let m: u32 = c.get(2).and_then(|x| x.as_str().parse().ok()).unwrap_or(0);
        // Claude states the reset in UTC; other zones fall back to retrying.
        if text.to_uppercase().contains("UTC") || !text.contains('(') {
            if let Some(t) = now.with_hour(h).and_then(|t| t.with_minute(m)).and_then(|t| t.with_second(0)) {
                let t = if t <= now { t + Duration::days(1) } else { t };
                return t + Duration::minutes(1);
            }
        }
    }
    now + Duration::minutes(30)
}

/// Mark a run paused by a usage limit, keeping what it cost so far.
pub fn pause(root: &Path, run_id: &str, message: &str) -> Result<RunRequest, String> {
    let mut r = load(root, run_id)?;
    let at = reset_time(message, chrono::Utc::now());
    r.status = "paused".into();
    r.resume_at = Some(at.to_rfc3339());
    r.prior_cost_usd = r.actual_cost_usd.unwrap_or(r.prior_cost_usd); // already includes earlier attempts
    r.actual_cost_usd = Some(r.prior_cost_usd);
    r.error = Some(format!("Paused: Claude's usage limit was reached. It continues where it stopped at about {} (or press Continue Now).", at.with_timezone(&chrono::Local).format("%-I:%M %p")));
    save(root, &r)?;
    Ok(r)
}

/// Make a paused (or cut-off) run ready to continue in the same folder.
/// Its earlier output stays; the agent is told to finish, not redo.
pub fn ready_to_continue(root: &Path, run_id: &str) -> Result<RunRequest, String> {
    let mut r = load(root, run_id)?;
    if r.status != "paused" {
        return Err(format!("run {run_id} is {}, not paused", r.status));
    }
    let dir = run_dir(root, run_id)?;
    if dir.join("stream.jsonl").is_file() {
        let _ = fs::rename(dir.join("stream.jsonl"), dir.join(format!("stream-{}.jsonl", r.continued + 1)));
    }
    r.continued += 1;
    r.status = "approved".into();
    r.resume_at = None;
    r.error = None;
    save(root, &r)?;
    Ok(r)
}

/// Paused runs whose limit has reset, oldest first.
pub fn due(root: &Path) -> Vec<RunRequest> {
    let now = chrono::Utc::now();
    let mut v: Vec<RunRequest> = list(root).unwrap_or_default().into_iter().filter(|r| r.status == "paused" && r.pipeline_id.is_none() && r.resume_at.as_deref().and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()).map(|t| t <= now).unwrap_or(true)).collect();
    v.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    v
}

pub fn runs_dir(root: &Path) -> PathBuf {
    root.join("runs")
}

pub fn run_dir(root: &Path, run_id: &str) -> Result<PathBuf, String> {
    if run_id.is_empty()
        || !run_id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(format!("invalid run id {run_id:?}"));
    }
    Ok(runs_dir(root).join(run_id))
}

pub fn prepare(root: &Path, agent: &str, prompt: &str) -> Result<RunRequest, String> {
    prepare_with_id(root, agent, prompt, &crate::new_id(&format!("run-{agent}")), None)
}

pub fn prepare_with_id(
    root: &Path,
    agent: &str,
    prompt: &str,
    run_id: &str,
    pipeline_id: Option<&str>,
) -> Result<RunRequest, String> {
    if prompt.trim().is_empty() {
        return Err("prompt is empty".into());
    }
    let agents = cost::load_agents(root)?;
    let profile = agents
        .agents
        .get(agent)
        .ok_or_else(|| format!("unknown agent {agent:?} (not in studio/agents.json)"))?;
    let agent_file = root.join("vault/.claude/agents").join(format!("{agent}.md"));
    if !agent_file.is_file() {
        return Err(format!("agent file missing: {}", agent_file.display()));
    }
    let pricing = cost::load_pricing(root)?;
    let mut estimate = cost::estimate(agent, profile, prompt, &pricing)?;
    let st = crate::settings::load();
    let mut engine = st.engine_for(agent).to_string();
    // "Mostly local": an agent the local model keeps failing gets Claude for
    // a while, instead of waiting or retrying the same weak attempt.
    if engine == "ollama" && st.engine == "hybrid" && claude_for(root, agent) && !claude_limited(root) {
        engine = "claude".into();
    }
    if engine == "ollama" && std::env::var("ROBUX_SKIP_OLLAMA_CHECK").is_err() {
        let o = crate::ollama::status();
        let m = &st.ollama_model;
        if !o.running || !o.version_ok {
            return Err(o.message);
        }
        if !o.installed.iter().any(|i| i == m || *i == format!("{m}:latest")) {
            return Err(format!("The local model {m} is not downloaded yet. Download it in Settings, or switch the engine back to Claude."));
        }
    }
    if engine == "ollama" {
        // Local models cost nothing to run; the cap stays as a formality.
        estimate.model = st.ollama_model.clone();
        estimate.low_usd = 0.0;
        estimate.expected_usd = 0.0;
        estimate.high_usd = 0.0;
        estimate.cap_usd = 0.0; // free: reserves no budget
        estimate.assumptions = vec![format!("Runs on your computer with the local model {} through Ollama. No Claude usage, no cost.", st.ollama_model), "Local models are weaker than Claude: expect more failed checks and slower runs.".into()];
        estimate.subscription_note = "Free: local model.".into();
    }
    let req = RunRequest {
        run_id: run_id.to_string(),
        agent: agent.to_string(),
        prompt: prompt.to_string(),
        estimate,
        status: "awaiting-approval".into(),
        created_at: crate::now_rfc3339(),
        approved_at: None,
        finished_at: None,
        actual_cost_usd: None,
        result_text: None,
        error: None,
        pipeline_id: pipeline_id.map(|p| p.to_string()),
        chat: None,
        resume: None,
        engine: Some(engine.clone()),
        resume_at: None,
        continued: 0,
        prior_cost_usd: 0.0,
        change: None,
        stop_reason: None,
        usage: None,
        turn_limit: None,
    };
    save(root, &req)?;
    Ok(req)
}

pub fn save(root: &Path, req: &RunRequest) -> Result<(), String> {
    let d = run_dir(root, &req.run_id)?;
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    crate::paths::write_atomic(&d.join("request.json"), serde_json::to_string_pretty(req).map_err(|e| e.to_string())?.as_bytes())
}

pub fn load(root: &Path, run_id: &str) -> Result<RunRequest, String> {
    let p = run_dir(root, run_id)?.join("request.json");
    // Falls back to the last good copy if the file is damaged.
    crate::paths::read_json(&p).ok_or_else(|| format!("could not read {}", p.display()))
}

/// Parsed request.json files, kept while the file is unchanged (same size
/// and time). The app lists every run every few seconds; parsing hundreds
/// of big prompts each time made it lag.
static CACHE: std::sync::Mutex<Option<std::collections::HashMap<PathBuf, (std::time::SystemTime, u64, RunRequest)>>> = std::sync::Mutex::new(None);

pub fn list(root: &Path) -> Result<Vec<RunRequest>, String> {
    let d = runs_dir(root);
    let mut out = vec![];
    let mut guard = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    let cache = guard.get_or_insert_with(Default::default);
    let mut seen = std::collections::HashSet::new();
    if let Ok(entries) = fs::read_dir(&d) {
        for e in entries.flatten() {
            let p = e.path().join("request.json");
            let Ok(m) = fs::metadata(&p) else { continue };
            let (t, len) = (m.modified().unwrap_or(std::time::UNIX_EPOCH), m.len());
            seen.insert(p.clone());
            if let Some((ct, cl, r)) = cache.get(&p) {
                if *ct == t && *cl == len {
                    out.push(r.clone());
                    continue;
                }
            }
            if let Ok(text) = fs::read_to_string(&p) {
                if let Ok(r) = serde_json::from_str::<RunRequest>(&text) {
                    cache.insert(p, (t, len, r.clone()));
                    out.push(r);
                }
            }
        }
    }
    cache.retain(|k, _| seen.contains(k) || !k.starts_with(&d));
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(out)
}

/// Approve a prepared run. `confirmed_cap_usd` must equal the cap that was
/// shown to Andrew.
pub fn approve(root: &Path, run_id: &str, confirmed_cap_usd: f64) -> Result<RunRequest, String> {
    let _g = crate::budget::APPROVE.lock().unwrap_or_else(|e| e.into_inner());
    let req = load(root, run_id)?;
    if req.status == "awaiting-approval" {
        crate::budget::check(root, req.estimate.cap_usd)?;
    }
    approve_unchecked(root, run_id, confirmed_cap_usd)
}

/// Approve without the budget check. Only for pipeline stages, whose total
/// cap was already checked against the budget in `pipeline::approve`.
pub(crate) fn approve_unchecked(root: &Path, run_id: &str, confirmed_cap_usd: f64) -> Result<RunRequest, String> {
    let mut req = load(root, run_id)?;
    if req.status != "awaiting-approval" {
        return Err(format!("run {run_id} is {}, not awaiting approval", req.status));
    }
    if (req.estimate.cap_usd - confirmed_cap_usd).abs() > 1e-9 {
        return Err(format!(
            "confirmed cap ${confirmed_cap_usd} does not match the estimate cap ${}",
            req.estimate.cap_usd
        ));
    }
    req.status = "approved".into();
    req.approved_at = Some(crate::now_rfc3339());
    save(root, &req)?;
    Ok(req)
}

/// A run that never finished (app closed, crash, early error) is marked
/// failed, so it stops counting as reserved budget and showing "working".
pub fn fail_if_unfinished(root: &Path, run_id: &str, why: &str) {
    if let Ok(mut r) = load(root, run_id) {
        if r.status == "running" || r.status == "approved" {
            r.status = "failed".into();
            r.error = Some(why.to_string());
            r.finished_at = Some(crate::now_rfc3339());
            let _ = save(root, &r);
        }
    }
}

/// At startup nothing can still be running (runs live in this app's
/// threads), so anything left running or approved was cut off.
pub fn recover_stale(root: &Path) -> usize {
    let mut n = 0;
    for r in list(root).unwrap_or_default() {
        // A paused or cut-off pipeline sorts out its own stages.
        let behind_pause = r.pipeline_id.as_deref().and_then(|p| crate::pipeline::load(root, p).ok()).map(|p| p.status == "paused" || p.status == "running").unwrap_or(false);
        if behind_pause {
            continue;
        }
        if r.status == "running" {
            // Cut off mid-run: pause it so it continues instead of failing.
            let mut x = r.clone();
            x.status = "paused".into();
            x.resume_at = Some(chrono::Utc::now().to_rfc3339());
            x.prior_cost_usd = x.actual_cost_usd.unwrap_or(x.prior_cost_usd);
            x.error = Some("Paused: BrewixGames closed while this ran. It continues where it stopped.".into());
            let _ = save(root, &x);
            n += 1;
        } else if r.status == "approved" && r.change.is_some() {
            // A queued change request: it starts again from the queue.
        } else if r.status == "approved" {
            fail_if_unfinished(root, &r.run_id, "BrewixGames closed before this run finished.");
            n += 1;
        } else if r.status == "awaiting-approval" {
            let _ = cancel(root, &r.run_id); // an estimate nobody approved
        }
    }
    n + crate::pipeline::recover_stale(root)
}

pub fn cancel(root: &Path, run_id: &str) -> Result<RunRequest, String> {
    let mut req = load(root, run_id)?;
    if req.status != "awaiting-approval" {
        return Err(format!("run {run_id} is {}, cannot cancel", req.status));
    }
    req.status = "cancelled".into();
    save(root, &req)?;
    Ok(req)
}

#[cfg(test)]
mod stale_tests {
    use super::*;
    #[test]
    fn cut_off_runs_pause_and_continue_with_their_cost() {
        let d = std::env::temp_dir().join(format!("stale-t-{}", std::process::id()));
        fs::create_dir_all(d.join("studio")).unwrap();
        for f in ["agents.json", "pricing.json", "pipelines.json"] {
            fs::copy(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../studio").join(f), d.join("studio").join(f)).unwrap();
        }
        fs::create_dir_all(d.join("vault/.claude/agents")).unwrap();
        fs::write(d.join("vault/.claude/agents/architect.md"), "---\nname: architect\n---\n").unwrap();
        let r = prepare(&d, "architect", "test").unwrap();
        let mut req = load(&d, &r.run_id).unwrap();
        req.status = "running".into();
        req.actual_cost_usd = Some(0.25);
        save(&d, &req).unwrap();
        assert!(crate::budget::spend(&d, 100.0).reserved_usd > 0.0);
        recover_stale(&d);
        let after = load(&d, &r.run_id).unwrap();
        assert_eq!(after.status, "paused");
        assert_eq!(after.prior_cost_usd, 0.25);
        assert_eq!(crate::budget::spend(&d, 100.0).reserved_usd, 0.0);
        assert_eq!(due(&d).len(), 1, "app-closed pauses continue right away");
        let c = ready_to_continue(&d, &r.run_id).unwrap();
        assert_eq!((c.status.as_str(), c.continued), ("approved", 1));
        assert!(ready_to_continue(&d, &r.run_id).is_err(), "only paused runs continue");
        // a usage-limit pause waits for the reset time
        let mut x = load(&d, &r.run_id).unwrap();
        x.status = "failed".into();
        save(&d, &x).unwrap();
        let p = pause(&d, &r.run_id, "Claude AI usage limit reached|4102444800").unwrap();
        assert_eq!(p.status, "paused");
        assert!(due(&d).is_empty());
        let _ = fs::remove_dir_all(&d);
    }

    #[test]
    fn spots_limits_and_reset_times() {
        use chrono::TimeZone;
        assert!(is_limit("Claude AI usage limit reached|1760000000"));
        assert!(is_limit("You've hit your limit · resets 2pm (UTC)"));
        assert!(!is_limit("Error: file not found"));
        assert!(!is_limit("Added a rate limit to the RemoteEvent; the per-player limit reached 10/s"), "Roblox work is not Claude's limit");
        let now = chrono::Utc.with_ymd_and_hms(2026, 11, 17, 10, 0, 0).unwrap();
        assert_eq!(reset_time("You've hit your limit · resets 2pm (UTC)", now), chrono::Utc.with_ymd_and_hms(2026, 11, 17, 14, 1, 0).unwrap());
        assert_eq!(reset_time("resets 9:30am (UTC)", now), chrono::Utc.with_ymd_and_hms(2026, 11, 18, 9, 31, 0).unwrap());
        assert_eq!(reset_time("limit reached|1795432800", now), chrono::Utc.timestamp_opt(1795432800 + 60, 0).unwrap());
        assert_eq!(reset_time("usage limit reached", now), now + chrono::Duration::minutes(30));
        assert_eq!(reset_time("resets 2pm (America/New_York)", now), now + chrono::Duration::minutes(30));
    }
}

#[cfg(test)]
mod list_speed_tests {
    #[test]
    fn listing_many_big_runs_is_fast_the_second_time() {
        let root = std::env::temp_dir().join(format!("runs-speed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let base: serde_json::Value = serde_json::json!({"run_id":"x","agent":"builder","prompt":"p".repeat(100_000),"estimate":{"agent":"builder","model":"m","low_usd":0.0,"expected_usd":0.0,"high_usd":0.0,"cap_usd":0.0,"cache_write_tokens":0,"cache_read_tokens":0,"output_tokens":0,"assumptions":[],"pricing_checked":"","subscription_note":""},"status":"done","created_at":"2026-11-01T00:00:00Z"});
        for i in 0..400 {
            let d = root.join("runs").join(format!("run-builder-{i}"));
            std::fs::create_dir_all(&d).unwrap();
            let mut v = base.clone();
            v["run_id"] = format!("run-builder-{i}").into();
            std::fs::write(d.join("request.json"), v.to_string()).unwrap();
        }
        let t = std::time::Instant::now();
        let a = super::list(&root).unwrap();
        let first = t.elapsed();
        let t = std::time::Instant::now();
        let b = super::list(&root).unwrap();
        let second = t.elapsed();
        eprintln!("first {first:?}, second {second:?}");
        assert_eq!(a.len(), 400);
        assert_eq!(b.len(), 400);
        assert!(second < first / 2, "first {first:?}, second {second:?}");
        // A changed run is read again.
        let p = root.join("runs/run-builder-7/request.json");
        let mut v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&p).unwrap()).unwrap();
        v["status"] = "failed".into();
        std::fs::write(&p, v.to_string()).unwrap();
        assert_eq!(super::list(&root).unwrap().iter().find(|r| r.run_id == "run-builder-7").unwrap().status, "failed");
        let _ = std::fs::remove_dir_all(&root);
    }
}


/// Claude's usage looks used up: a run paused for the limit that has not
/// reset yet, a run that failed on the limit, or Claude runs that never
/// started (2+ in the last 20 minutes).
pub fn claude_limited(root: &Path) -> bool {
    let now = chrono::Utc::now();
    let recent = (now - chrono::Duration::minutes(30)).to_rfc3339();
    let twenty = (now - chrono::Duration::minutes(20)).to_rfc3339();
    let runs = list(root).unwrap_or_default();
    let limit_text = |e: &str| { let e = e.to_lowercase(); e.contains("usage limit") || e.contains("limit reached") || e.contains("rate limit") || e.contains("out of usage") };
    runs.iter().any(|x| {
        (x.status == "paused" && x.resume_at.as_deref().filter(|t| !t.starts_with("9999")).and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()).map(|t| t > now).unwrap_or(false))
            || (x.status == "failed" && x.engine.as_deref() != Some("ollama") && x.finished_at.as_deref().map(|t| t > recent.as_str()).unwrap_or(false) && x.error.as_deref().map(limit_text).unwrap_or(false))
    }) || runs.iter().filter(|x| x.status == "failed" && x.engine.as_deref() != Some("ollama") && x.finished_at.as_deref().map(|t| t > twenty.as_str()).unwrap_or(false)
        && x.error.as_deref().map(|e| e.contains("No response for a while") || e.contains("no result line")).unwrap_or(false)).count() >= 2
}

/// Agents that use Claude for a while in "Mostly local" because the local
/// model kept failing their jobs (runs/claude_for.json: agent -> until).
pub fn claude_for(root: &Path, agent: &str) -> bool {
    let v: serde_json::Value = crate::paths::read_json(&root.join("runs/claude_for.json")).unwrap_or(serde_json::json!({}));
    v.get(agent).and_then(|t| t.as_str()).map(|t| t > crate::now_rfc3339().as_str()).unwrap_or(false)
}

pub fn use_claude_for(root: &Path, agent: &str, hours: i64) {
    let path = root.join("runs/claude_for.json");
    let mut v: serde_json::Value = crate::paths::read_json(&path).unwrap_or(serde_json::json!({}));
    v[agent] = serde_json::json!((chrono::Utc::now() + chrono::Duration::hours(hours)).to_rfc3339());
    let _ = std::fs::write(path, v.to_string());
}
