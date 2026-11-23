//! Claude connection, the no-API-key way: the app uses Andrew's Claude Code
//! login (his Claude subscription). It never asks for, stores, or passes an
//! API key, and agent runs strip ANTHROPIC_API_KEY from their environment.

use serde::Serialize;
use std::process::Stdio;

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
    /// Windows: Git for Windows found (Claude Code needs it).
    #[serde(default)]
    pub git_ok: bool,
}

/// Claude Code on Windows needs Git for Windows (it runs commands through Git
/// Bash). Where bash.exe is, if Git is installed. Always Some on macOS/Linux.
pub fn git_bash() -> Option<std::path::PathBuf> {
    if !cfg!(windows) {
        return Some("/bin/bash".into());
    }
    if let Some(p) = std::env::var_os("CLAUDE_CODE_GIT_BASH_PATH").map(std::path::PathBuf::from).filter(|p| p.is_file()) {
        return Some(p);
    }
    let mut c = vec![crate::paths::app_dir("git").join("bin/bash.exe")];
    for var in ["ProgramFiles", "ProgramW6432", "ProgramFiles(x86)", "LOCALAPPDATA"] {
        if let Some(d) = std::env::var_os(var) {
            let d = std::path::PathBuf::from(d);
            c.push(d.join("Git/bin/bash.exe"));
            c.push(d.join("Programs/Git/bin/bash.exe"));
        }
    }
    if let Some(git) = crate::paths::command("where").arg("git").output().ok().filter(|o| o.status.success()).and_then(|o| String::from_utf8_lossy(&o.stdout).lines().next().map(|l| std::path::PathBuf::from(l.trim()))) {
        // ...\Git\cmd\git.exe -> ...\Git\bin\bash.exe
        if let Some(root) = git.parent().and_then(|p| p.parent()) {
            c.push(root.join("bin/bash.exe"));
        }
    }
    c.into_iter().find(|p| p.is_file())
}

/// Tell every Claude Code the app starts where Git Bash is (it does not
/// always find it on its own, for example in a custom install folder).
pub fn export_git_bash() {
    if cfg!(windows) {
        if let Some(p) = git_bash() {
            std::env::set_var("CLAUDE_CODE_GIT_BASH_PATH", p);
        }
    }
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
                git_ok: git_bash().is_some(),
            }
        }
    };
    let version = crate::paths::output_timeout(crate::paths::command(&bin).arg("--version"), 20)
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    let out = crate::paths::output_timeout(crate::paths::command(&bin).args(["auth", "status", "--json"]).env_remove("ANTHROPIC_API_KEY"), 20);
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
    let git_ok = git_bash().is_some();
    let message = if git_ok { message } else { "Claude Code needs Git for Windows to run. Press Install Git (free), or install it from git-scm.com with the default options.".to_string() };
    Connection { installed: true, bin: bin.display().to_string(), version, logged_in, auth_method, uses_subscription, message, git_ok }
}

/// Install Claude Code with Anthropic's official installer
/// (`curl -fsSL https://claude.ai/install.sh | bash`, from the Claude Code
/// setup docs). Runs in the background; macOS ships curl and bash.
/// Install (or refresh) Claude Code, then keep BrewixGames' own copy in the
/// Downloads folder (<tools>/claude). The official installer has no option
/// to choose its folder, so it installs to its usual place and the program
/// is copied from there. The copy does not update itself; Install refreshes it.
pub fn install() -> Result<String, String> {
    let dest = crate::paths::app_dir("claude").join(if cfg!(windows) { "claude.exe" } else { "claude" });
    let found = crate::paths::claude_bin().ok().filter(|p| p != &dest);
    let source = match found {
        Some(p) => p,
        None => {
            let out = official_installer("curl -fsSL https://claude.ai/install.sh | bash", "irm https://claude.ai/install.ps1 | iex")?;
            if !out.status.success() {
                let err = String::from_utf8_lossy(&out.stderr);
                return Err(format!("installer failed: {}", err.lines().rev().take(3).collect::<Vec<_>>().join(" | ")));
            }
            let home = crate::paths::home().ok_or("no home folder")?;
            let p = home.join(".local/bin").join(if cfg!(windows) { "claude.exe" } else { "claude" });
            if !p.is_file() {
                return Err(format!("Claude Code installed, but not at {}. Paste its location in Settings.", p.display()));
            }
            p
        }
    };
    // The launcher may be a link to a versioned file; copy the real program.
    let real = std::fs::canonicalize(&source).unwrap_or(source);
    std::fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::copy(&real, &dest).map_err(|e| format!("could not copy Claude Code into the Downloads folder: {e}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755));
    }
    Ok(format!("Claude Code is in {}. Now press Sign In to Claude.", dest.display()))
}

/// Run an official one-line installer: bash on macOS and Linux, PowerShell
/// on Windows.
pub fn official_installer(unix: &str, windows: &str) -> Result<std::process::Output, String> {
    let mut c = if cfg!(windows) {
        let mut c = crate::paths::command("powershell");
        c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", windows]);
        c
    } else {
        let mut c = crate::paths::command("/bin/bash");
        c.args(["-c", unix]);
        c
    };
    crate::paths::output_timeout(&mut c, 900).map_err(|e| format!("the installer did not finish: {e}"))
}

/// Install uv (free, from astral.sh) with its official installer, so the
/// Blender connection can run.
pub fn install_uv() -> Result<String, String> {
    if let Some(p) = crate::blender_mcp::uvx() {
        return Ok(format!("uv is already installed ({}).", p.display()));
    }
    // uv supports a chosen folder (UV_INSTALL_DIR) and leaving PATH alone.
    let dir = crate::paths::tools_dir().join("uv");
    let d = dir.display().to_string().replace('\'', "''");
    let out = official_installer(
        &format!("UV_INSTALL_DIR='{d}' UV_NO_MODIFY_PATH=1 INSTALLER_NO_MODIFY_PATH=1 sh -c 'curl -LsSf https://astral.sh/uv/install.sh | sh'"),
        &format!("$env:UV_INSTALL_DIR='{d}'; $env:UV_NO_MODIFY_PATH='1'; $env:INSTALLER_NO_MODIFY_PATH='1'; irm https://astral.sh/uv/install.ps1 | iex"),
    )?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(format!("uv installer failed: {}", err.lines().rev().take(3).collect::<Vec<_>>().join(" | ")));
    }
    crate::blender_mcp::uvx().map(|p| format!("Installed uv at {}.", p.display())).ok_or("uv installed, but BrewixGames cannot see it yet. Restart BrewixGames.".into())
}

/// Sign in with Andrew's Claude account. Runs `claude auth login` in the
/// background and opens any login link it prints in the browser. If that
/// does not finish, falls back to a Terminal window (macOS).
pub fn sign_in() -> Result<String, String> {
    use std::io::{BufRead, BufReader};
    let bin = crate::paths::claude_bin()?;
    let mut child = crate::paths::command(&bin)
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
                        crate::paths::open_url(url);
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
        let _ = crate::paths::command("osascript").args(["-e", &script]).status();
        return Ok("Finish signing in in the window that opened, then press Check Again.".into());
    }
    Err(format!("Sign-in did not finish. Output: {}", seen.lines().rev().take(3).collect::<Vec<_>>().join(" | ")))
}
