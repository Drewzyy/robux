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
    /// Files Andrew attached (images, video frames, documents).
    #[serde(default)]
    pub attachments: Vec<String>,
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

/// Where attached files are kept (agent runs can read this folder).
pub fn files_dir(root: &Path) -> PathBuf {
    root.join("runs").join("chat-files")
}

/// Save an attached file and return its full path. Images, PDFs and text
/// files only (video arrives as frames, links as text).
pub fn attach(root: &Path, name: &str, data_b64: &str) -> Result<String, String> {
    use base64::Engine;
    let b64 = data_b64.split(',').next_back().unwrap_or("");
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64.trim()).map_err(|_| "could not read that file".to_string())?;
    if bytes.len() > 20 << 20 {
        return Err("That file is over 20 MB.".into());
    }
    let ext = Path::new(name).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let ok = ["png", "jpg", "jpeg", "webp", "gif", "pdf", "txt", "md", "json", "lua", "luau", "csv", "log", "rbxmx", "toml", "yaml", "yml"];
    if !ok.contains(&ext.as_str()) {
        return Err(format!("Ask Claude can read images, PDFs and text files, not .{ext} files."));
    }
    let stem: String = Path::new(name).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default().chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).take(50).collect();
    let file = files_dir(root).join(format!("{}-{}.{ext}", chrono::Utc::now().format("%Y%m%d-%H%M%S%3f"), if stem.is_empty() { "file".into() } else { stem }));
    fs::create_dir_all(files_dir(root)).map_err(|e| e.to_string())?;
    fs::write(&file, bytes).map_err(|e| e.to_string())?;
    Ok(file.display().to_string())
}

/// What the assistant is told beyond Andrew's words: the game he is on and
/// the files he attached.
pub fn with_context(root: &Path, text: &str, project: Option<&str>, attachments: &[String]) -> String {
    let mut s = text.to_string();
    if let Some(p) = project.filter(|p| crate::paths::valid_id(p) && root.join("projects").join(p).is_dir()) {
        s.push_str(&format!("\n\n[Andrew is working on the game projects/{p}. Its own direction is in projects/{p}/docs (brief.md, notes-from-andrew.md, design.md, score.json when present) and its references in vault/19-Project-Memory/{p}-references.md. Read only what this question needs.]"));
    }
    let real: Vec<&String> = attachments.iter().filter(|a| Path::new(a).starts_with(files_dir(root))).collect();
    if !real.is_empty() {
        s.push_str("\n\n[Andrew attached these files. Open every one with Read before answering. For pictures, say what you see and what to take from it for the game; video frames are in time order.]\n");
        for a in real {
            s.push_str(&format!("- {a}\n"));
        }
    }
    s
}

pub fn begin(root: &Path, text: &str) -> Result<String, String> {
    begin_with(root, text, text, &[])
}

/// `shown` is what the chat displays; `text` is what the assistant gets.
pub fn begin_with(root: &Path, shown: &str, text: &str, attachments: &[String]) -> Result<String, String> {
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
    c.messages.push(Message { role: "you".into(), text: shown.trim().into(), at: crate::now_rfc3339(), cost_usd: None, run_id: Some(req.run_id.clone()), failed: false , attachments: attachments.to_vec() });
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
    let paused = runs::load(root, run_id).ok().filter(|r| r.status == "paused");
    let msg = match res {
        // Usage limit: keep the conversation and say when it continues.
        Ok(o) if paused.is_some() => {
            if session.is_some() {
                c.session = session;
            }
            let at = paused.as_ref().and_then(|r| r.resume_at.as_deref()).and_then(|t| chrono::DateTime::parse_from_rfc3339(t).ok()).map(|t| t.with_timezone(&chrono::Local).format("%-I:%M %p").to_string()).unwrap_or_else(|| "soon".into());
            Message { role: "assistant".into(), text: format!("Claude's usage limit is reached. I'll pick this up by myself at about {at}, right where we left off. Press Continue Now to try sooner."), at: crate::now_rfc3339(), cost_usd: o.actual_cost_usd, run_id: Some(run_id.into()), failed: true, attachments: vec![] }
        }
        Ok(o) if o.ok => {
            if session.is_some() {
                c.session = session;
            }
            Message { role: "assistant".into(), text: o.result_text, at: crate::now_rfc3339(), cost_usd: o.actual_cost_usd, run_id: Some(run_id.into()), failed: false, attachments: vec![] }
        }
        Ok(o) => Message {
            role: "assistant".into(),
            text: if o.result_text.is_empty() { format!("That didn't finish. {}", o.stderr_tail.lines().last().unwrap_or("Open the run to see why.")) } else { o.result_text },
            at: crate::now_rfc3339(),
            cost_usd: o.actual_cost_usd,
            run_id: Some(run_id.into()),
            failed: true,
            attachments: vec![],
        },
        Err(e) => Message { role: "assistant".into(), text: format!("Could not run: {e}"), at: crate::now_rfc3339(), cost_usd: None, run_id: Some(run_id.into()), failed: true, attachments: vec![] },
    };
    // A continued message replaces its "paused" notice.
    match c.messages.iter_mut().find(|m| m.role == "assistant" && m.run_id.as_deref() == Some(run_id)) {
        Some(m) => *m = msg.clone(),
        None => c.messages.push(msg.clone()),
    }
    let _ = save(root, &c);
    msg
}

/// Ready a chat message paused by a usage limit to be sent again in the
/// same conversation.
pub fn ready_to_continue(root: &Path, run_id: &str) -> Result<(), String> {
    let session = load(root).session;
    runs::ready_to_continue(root, run_id)?;
    let mut r = runs::load(root, run_id)?;
    r.resume = session;
    runs::save(root, &r)
}

#[cfg(test)]
mod tests {
    #[test]
    fn spots_a_lost_session() {
        assert!(super::is_lost_session("Error: No conversation found with session ID: abc"));
        assert!(!super::is_lost_session("rate limited"));
    }
}

