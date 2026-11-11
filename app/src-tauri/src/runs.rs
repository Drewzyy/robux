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
    if st.engine == "ollama" && std::env::var("ROBUX_SKIP_OLLAMA_CHECK").is_err() {
        let o = crate::ollama::status();
        let m = &st.ollama_model;
        if !o.running || !o.version_ok {
            return Err(o.message);
        }
        if !o.installed.iter().any(|i| i == m || *i == format!("{m}:latest")) {
            return Err(format!("The local model {m} is not downloaded yet. Download it in Settings, or switch the engine back to Claude."));
        }
    }
    if st.engine == "ollama" {
        // Local models cost nothing to run; the cap stays as a formality.
        estimate.model = st.ollama_model.clone();
        estimate.low_usd = 0.0;
        estimate.expected_usd = 0.0;
        estimate.high_usd = 0.0;
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
        engine: Some(st.engine.clone()),
    };
    save(root, &req)?;
    Ok(req)
}

pub fn save(root: &Path, req: &RunRequest) -> Result<(), String> {
    let d = run_dir(root, &req.run_id)?;
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    let p = d.join("request.json");
    let tmp = d.join("request.json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(req).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    fs::rename(&tmp, &p).map_err(|e| e.to_string())
}

pub fn load(root: &Path, run_id: &str) -> Result<RunRequest, String> {
    let p = run_dir(root, run_id)?.join("request.json");
    let t = fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
    serde_json::from_str(&t).map_err(|e| format!("parse {}: {e}", p.display()))
}

pub fn list(root: &Path) -> Result<Vec<RunRequest>, String> {
    let d = runs_dir(root);
    let mut out = vec![];
    if let Ok(entries) = fs::read_dir(&d) {
        for e in entries.flatten() {
            let p = e.path().join("request.json");
            if let Ok(t) = fs::read_to_string(&p) {
                if let Ok(r) = serde_json::from_str::<RunRequest>(&t) {
                    out.push(r);
                }
            }
        }
    }
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(out)
}

/// Approve a prepared run. `confirmed_cap_usd` must equal the cap that was
/// shown to Andrew.
pub fn approve(root: &Path, run_id: &str, confirmed_cap_usd: f64) -> Result<RunRequest, String> {
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

pub fn cancel(root: &Path, run_id: &str) -> Result<RunRequest, String> {
    let mut req = load(root, run_id)?;
    if req.status != "awaiting-approval" {
        return Err(format!("run {run_id} is {}, cannot cancel", req.status));
    }
    req.status = "cancelled".into();
    save(root, &req)?;
    Ok(req)
}
