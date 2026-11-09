//! Claude connection, the no-API-key way: the app uses Andrew's Claude Code
//! login (his Claude subscription). It never asks for, stores, or passes an
//! API key, and agent runs strip ANTHROPIC_API_KEY from their environment.

use serde::Serialize;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize)]
pub struct Connection {
    pub installed: bool,
    pub bin: String,
    pub version: String,
    pub logged_in: bool,
    /// e.g. "claude.ai" (subscription login) or "api_key".
    pub auth_method: String,
    pub uses_subscription: bool,
    pub message: String,
}

pub fn status() -> Connection {
    let bin = match crate::paths::claude_bin() {
        Ok(b) => b,
        Err(e) => {
            return Connection {
                installed: false,
                bin: String::new(),
                version: String::new(),
                logged_in: false,
                auth_method: String::new(),
                uses_subscription: false,
                message: e,
            }
        }
    };
    let version = Command::new(&bin)
        .arg("--version")
        .stdin(Stdio::null())
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    let out = Command::new(&bin)
        .args(["auth", "status", "--json"])
        .env_remove("ANTHROPIC_API_KEY")
        .stdin(Stdio::null())
        .output();
    let v: serde_json::Value = out
        .ok()
        .and_then(|o| serde_json::from_slice(&o.stdout).ok())
        .unwrap_or(serde_json::Value::Null);
    let logged_in = v.get("loggedIn").and_then(|x| x.as_bool()).unwrap_or(false);
    let auth_method = v.get("authMethod").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let uses_subscription = logged_in && !auth_method.to_lowercase().contains("api");
    let message = if !logged_in {
        "Not signed in. Click Sign in to Claude, finish in the browser, then check again.".into()
    } else if uses_subscription {
        "Signed in with your Claude account. Agent runs use your subscription; no API key.".into()
    } else {
        format!("Signed in with {auth_method}. Agent runs ignore API keys; sign in with your Claude account instead.")
    };
    Connection { installed: true, bin: bin.display().to_string(), version, logged_in, auth_method, uses_subscription, message }
}

/// Open a Terminal window running `claude auth login` (macOS). The login
/// itself happens in Andrew's browser; nothing is stored by this app.
pub fn sign_in() -> Result<String, String> {
    let bin = crate::paths::claude_bin()?;
    if cfg!(target_os = "macos") {
        let script = format!(
            "tell application \"Terminal\" to do script \"'{}' auth login\"\ntell application \"Terminal\" to activate",
            bin.display().to_string().replace('\'', "")
        );
        Command::new("osascript")
            .args(["-e", &script])
            .status()
            .map_err(|e| e.to_string())?;
        Ok("Terminal opened. Finish signing in there, then press Check again.".into())
    } else {
        Ok(format!("Run this in a terminal, then press Check again: {} auth login", bin.display()))
    }
}
