//! Ask Claude: a chat with the studio's assistant agent. Each message is a
//! normal capped, budget-checked agent run that continues one Claude Code
//! session, so the assistant remembers the conversation. Pressing Send is
//! the approval; the cap per message is shown next to the button.

use crate::{bridge, cost, runs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const AGENT: &str = "assistant";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String, // "you" | "assistant"
    pub text: String,
    pub at: String,
    #[serde(default)]
    pub cost_usd: Option<f64>,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub failed: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chat {
    #[serde(default)]
    pub session: Option<String>,
    #[serde(default)]
    pub messages: Vec<Message>,
}

fn file(root: &Path) -> PathBuf {
    root.join("runs").join("chat.json")
}

pub fn load(root: &Path) -> Chat {
    fs::read_to_string(file(root)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn save(root: &Path, c: &Chat) -> Result<(), String> {
    fs::create_dir_all(root.join("runs")).map_err(|e| e.to_string())?;
    fs::write(file(root), serde_json::to_string_pretty(c).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

/// Start a fresh conversation (the old one stays in run history).
pub fn reset(root: &Path) -> Result<(), String> {
    save(root, &Chat::default())
}

/// The most one message can cost, shown on the Send button.
pub fn cap(root: &Path) -> Result<f64, String> {
    let agents = cost::load_agents(root)?;
    let prof = agents.agents.get(AGENT).ok_or("assistant agent missing from studio/agents.json")?;
    let st = crate::settings::load();
    if st.engine == "ollama" {
        return Ok(0.0);
    }
    Ok(cost::estimate(AGENT, prof, "", &cost::load_pricing(root)?)?.cap_usd)
}

/// Record the user's message and prepare + approve the run. Returns run id.
pub fn begin(root: &Path, text: &str) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() {
        return Err("Type a message first.".into());
    }
    let mut c = load(root);
    let mut req = runs::prepare(root, AGENT, text)?;
    req.chat = Some("main".into());
    req.resume = c.session.clone();
    runs::save(root, &req)?;
    if let Err(e) = runs::approve(root, &req.run_id, req.estimate.cap_usd) {
        let _ = runs::cancel(root, &req.run_id);
        return Err(e);
    }
    c.messages.push(Message { role: "you".into(), text: text.into(), at: crate::now_rfc3339(), cost_usd: None, run_id: Some(req.run_id.clone()), failed: false });
    save(root, &c)?;
    Ok(req.run_id)
}

/// Run the prepared message to completion and store the reply.
pub fn finish<F: FnMut(&str)>(root: &Path, run_id: &str, claude_bin: &Path, on_line: F) -> Message {
    let res = bridge::run_approved(root, run_id, claude_bin, on_line);
    let mut c = load(root);
    let session = fs::read_to_string(root.join("runs").join(run_id).join("result.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .and_then(|v| v.get("session_id").and_then(|s| s.as_str()).map(|s| s.to_string()));
    let msg = match res {
        Ok(o) if o.ok => {
            if session.is_some() {
                c.session = session;
            }
            Message { role: "assistant".into(), text: o.result_text, at: crate::now_rfc3339(), cost_usd: o.actual_cost_usd, run_id: Some(run_id.into()), failed: false }
        }
        Ok(o) => Message {
            role: "assistant".into(),
            text: if o.result_text.is_empty() { format!("That didn't finish. {}", o.stderr_tail.lines().last().unwrap_or("Open the run to see why.")) } else { o.result_text },
            at: crate::now_rfc3339(),
            cost_usd: o.actual_cost_usd,
            run_id: Some(run_id.into()),
            failed: true,
        },
        Err(e) => Message { role: "assistant".into(), text: format!("Could not run: {e}"), at: crate::now_rfc3339(), cost_usd: None, run_id: Some(run_id.into()), failed: true },
    };
    c.messages.push(msg.clone());
    let _ = save(root, &c);
    msg
}
