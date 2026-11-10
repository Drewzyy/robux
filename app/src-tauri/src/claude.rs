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

/// Install Claude Code with Anthropic's official installer
/// (`curl -fsSL https://claude.ai/install.sh | bash`, from the Claude Code
/// setup docs). Runs in the background; macOS ships curl and bash.
pub fn install() -> Result<String, String> {
    if crate::paths::claude_bin().is_ok() {
        return Ok("Claude Code is already installed.".into());
    }
    let out = Command::new("/bin/bash")
        .args(["-c", "curl -fsSL https://claude.ai/install.sh | bash"])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("could not start the installer: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("installer failed: {}", err.lines().rev().take(3).collect::<Vec<_>>().join(" | ")));
    }
    crate::paths::claude_bin().map(|p| format!("Installed Claude Code at {}. Now press Sign In to Claude.", p.display()))
}

/// Sign in with Andrew's Claude account. Runs `claude auth login` in the
/// background and opens any login link it prints in the browser. If that
/// does not finish, falls back to a Terminal window (macOS).
pub fn sign_in() -> Result<String, String> {
    use std::io::{BufRead, BufReader};
    let bin = crate::paths::claude_bin()?;
    let mut child = Command::new(&bin)
        .args(["auth", "login"])
        .env_remove("ANTHROPIC_API_KEY")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut opened = false;
    let mut seen = String::new();
    let streams: Vec<Box<dyn std::io::Read + Send>> = vec![Box::new(child.stdout.take().unwrap()), Box::new(child.stderr.take().unwrap())];
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    for s in streams {
        let tx = tx.clone();
        std::thread::spawn(move || {
            for l in BufReader::new(s).lines().map_while(Result::ok) {
                let _ = tx.send(l);
            }
        });
    }
    drop(tx);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
    while std::time::Instant::now() < deadline {
        match rx.recv_timeout(std::time::Duration::from_millis(500)) {
            Ok(line) => {
                seen.push_str(&line);
                seen.push('\n');
                if !opened {
                    if let Some(url) = line.split_whitespace().find(|w| w.starts_with("https://")) {
                        let _ = Command::new(if cfg!(target_os = "macos") { "open" } else { "xdg-open" }).arg(url).status();
                        opened = true;
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if let Ok(Some(_)) = child.try_wait() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    let _ = child.kill();
    if status().logged_in {
        return Ok("Signed in. Agent runs use your Claude subscription.".into());
    }
    if cfg!(target_os = "macos") {
        let script = format!(
            "tell application \"Terminal\" to do script \"'{}' auth login\"\ntell application \"Terminal\" to activate",
            bin.display().to_string().replace('\'', "")
        );
        let _ = Command::new("osascript").args(["-e", &script]).status();
        return Ok("Finish signing in in the window that opened, then press Check Again.".into());
    }
    Err(format!("Sign-in did not finish. Output: {}", seen.lines().rev().take(3).collect::<Vec<_>>().join(" | ")))
}
