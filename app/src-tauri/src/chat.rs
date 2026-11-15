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

/// Write to a temp file then swap, so a crash never leaves a half-written
/// chat (which would read as empty and lose the history and session).
fn save(root: &Path, c: &Chat) -> Result<(), String> {
    fs::create_dir_all(root.join("runs")).map_err(|e| e.to_string())?;
    let tmp = file(root).with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(c).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::rename(&tmp, file(root)).map_err(|e| e.to_string())
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
/// begin and finish each load, change and save chat.json: one at a time.
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn begin(root: &Path, text: &str) -> Result<String, String> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
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

pub fn is_lost_session(text: &str) -> bool {
    let t = text.to_lowercase();
    t.contains("no conversation found") || (t.contains("session") && t.contains("not found"))
}

/// Clear the lost session and prepare the same message as a new conversation.
fn restart(root: &Path, failed_run: &str) -> Result<String, String> {
    let mut c = load(root);
    c.session = None;
    let text = c.messages.iter().rev().find(|m| m.role == "you").map(|m| m.text.clone()).unwrap_or_default();
    let recent: Vec<String> = c.messages.iter().rev().skip(1).take(8).collect::<Vec<_>>().into_iter().rev().filter(|m| !m.failed).map(|m| format!("{}: {}", if m.role == "you" { "Andrew" } else { "You" }, m.text.chars().take(600).collect::<String>())).collect();
    let prompt = if recent.is_empty() { text } else { format!("Our earlier conversation, for context:\n{}\n\nAndrew's new message:\n{text}", recent.join("\n")) };
    let mut req = runs::prepare(root, AGENT, &prompt)?;
    req.chat = Some("main".into());
    req.resume = None;
    runs::save(root, &req)?;
    runs::approve(root, &req.run_id, req.estimate.cap_usd)?;
    for m in c.messages.iter_mut().rev() {
        if m.run_id.as_deref() == Some(failed_run) {
            m.run_id = Some(req.run_id.clone());
            break;
        }
    }
    save(root, &c)?;
    Ok(req.run_id)
}

/// Run the prepared message to completion and store the reply.
pub fn finish<F: FnMut(&str)>(root: &Path, run_id: &str, claude_bin: &Path, mut on_line: F) -> Message {
    let mut run_id = run_id.to_string();
    let mut res = bridge::run_approved(root, &run_id, claude_bin, &mut on_line);
    // The saved conversation can be gone (studio folder moved, engine
    // switched, Claude Code updated). Forget it and send the message again
    // as a new conversation, with the recent messages as context.
    if let Ok(o) = &res {
        if !o.ok && is_lost_session(&format!("{} {}", o.result_text, o.stderr_tail)) {
            if let Ok(id) = restart(root, &run_id) {
                run_id = id;
                res = bridge::run_approved(root, &run_id, claude_bin, &mut on_line);
            }
        }
    }
    let run_id = run_id.as_str();
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
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

#[cfg(test)]
mod tests {
    #[test]
    fn spots_a_lost_session() {
        assert!(super::is_lost_session("Error: No conversation found with session ID: abc"));
        assert!(!super::is_lost_session("rate limited"));
    }
}

