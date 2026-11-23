//! Connection lights (green, red, or off) for everything the agents use,
//! and opening Roblox Studio and Blender with BrewixGames when they are
//! switched on, so Andrew never has to open them himself.

use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Debug, Clone)]
pub struct Light {
    pub key: String,
    pub name: String,
    /// "green", "red" or "off" (not used by the current settings).
    pub state: String,
    pub detail: String,
}

fn light(key: &str, name: &str, state: &str, detail: impl Into<String>) -> Light {
    Light { key: key.into(), name: name.into(), state: state.into(), detail: detail.into() }
}

/// True when a program with this exact name is running.
pub fn is_running(name: &str) -> bool {
    if cfg!(windows) {
        crate::paths::command("tasklist")
            .args(["/FI", &format!("IMAGENAME eq {name}"), "/NH"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_lowercase().contains(&name.to_lowercase()))
            .unwrap_or(false)
    } else {
        crate::paths::command("pgrep").args(["-x", name]).output().map(|o| o.status.success()).unwrap_or(false)
    }
}

/// The newest installed Roblox Studio.
pub fn studio_exe() -> Option<PathBuf> {
    if cfg!(windows) {
        let base = PathBuf::from(std::env::var("LOCALAPPDATA").ok()?).join("Roblox").join("Versions");
        let mut found: Vec<(std::time::SystemTime, PathBuf)> = std::fs::read_dir(base)
            .ok()?
            .flatten()
            .map(|e| e.path().join("RobloxStudioBeta.exe"))
            .filter(|p| p.is_file())
            .map(|p| (std::fs::metadata(&p).and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH), p))
            .collect();
        found.sort();
        found.pop().map(|x| x.1)
    } else if cfg!(target_os = "macos") {
        let p = PathBuf::from("/Applications/RobloxStudio.app");
        p.exists().then_some(p)
    } else {
        None
    }
}

const STUDIO_PROCESS: &str = if cfg!(windows) { "RobloxStudioBeta.exe" } else { "RobloxStudio" };
const BLENDER_PROCESS: &str = if cfg!(windows) { "blender.exe" } else { "blender" };

/// Blender: ours, or a normal install.
pub fn blender_exe() -> Option<PathBuf> {
    crate::apps::blender_exe().or_else(|| {
        if cfg!(windows) {
            let base = PathBuf::from(std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".into())).join("Blender Foundation");
            let mut v: Vec<PathBuf> = std::fs::read_dir(base).ok()?.flatten().map(|e| e.path().join("blender.exe")).filter(|p| p.is_file()).collect();
            v.sort();
            v.pop()
        } else if cfg!(target_os = "macos") {
            let p = PathBuf::from("/Applications/Blender.app/Contents/MacOS/Blender");
            p.is_file().then_some(p)
        } else {
            None
        }
    })
}

/// The Blender MCP add-on listens here once its Connect button is pressed.
fn blender_addon_listening() -> bool {
    std::net::TcpStream::connect_timeout(&"127.0.0.1:9876".parse().unwrap(), std::time::Duration::from_millis(400)).is_ok()
}

/// Open a program without waiting for it.
fn launch(exe: &PathBuf) -> bool {
    if cfg!(target_os = "macos") && exe.extension().map(|e| e == "app").unwrap_or(false) {
        return crate::paths::command("open").arg(exe).spawn().is_ok();
    }
    crate::paths::command(exe).stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().is_ok()
}

/// Open Roblox Studio and Blender if their connections are on and they
/// are not open yet. Returns what was opened.
pub fn open_needed() -> Vec<String> {
    let st = crate::settings::load();
    let mut opened = vec![];
    if st.studio_mcp == "studio" && !is_running(STUDIO_PROCESS) {
        if let Some(exe) = studio_exe() {
            if launch(&exe) {
                opened.push("Roblox Studio".to_string());
            }
        }
    }
    if st.blender_mcp == "blender" && !is_running(BLENDER_PROCESS) {
        if let Some(exe) = blender_exe() {
            if launch(&exe) {
                opened.push("Blender".to_string());
            }
        }
    }
    opened
}

/// Before an agent that uses Studio or Blender starts: open what it needs
/// and give it up to a minute to come up.
pub fn ready_for(agent: &str) {
    let st = crate::settings::load();
    let wants_studio = st.studio_mcp == "studio" && crate::studio_mcp::AGENTS.contains(&agent);
    let wants_blender = st.blender_mcp == "blender" && crate::blender_mcp::AGENTS.contains(&agent);
    if !wants_studio && !wants_blender {
        return;
    }
    if open_needed().is_empty() {
        return;
    }
    for _ in 0..60 {
        let s_ok = !wants_studio || is_running(STUDIO_PROCESS);
        let b_ok = !wants_blender || blender_addon_listening();
        if s_ok && b_ok {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/// Claude's sign-in, checked at most every 3 minutes (it starts two small
/// programs). A red light is checked again on the next refresh.
fn claude_cached() -> crate::claude::Connection {
    static CACHE: std::sync::Mutex<Option<(std::time::Instant, crate::claude::Connection)>> = std::sync::Mutex::new(None);
    let mut c = CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((at, v)) = c.as_ref() {
        if at.elapsed() < std::time::Duration::from_secs(180) && v.installed && v.logged_in {
            return v.clone();
        }
    }
    let v = crate::claude::status();
    *c = Some((std::time::Instant::now(), v.clone()));
    v
}

/// One light per connection.
pub fn lights() -> Vec<Light> {
    let st = crate::settings::load();
    let mut v = vec![];
    let c = claude_cached();
    v.push(if c.installed && c.logged_in {
        light("claude", "Claude", if st.engine == "ollama" { "off" } else { "green" }, match st.engine.as_str() { "claude" => "Signed in", "hybrid" => "Signed in; used only for testing in Studio and models in Blender", _ => "Signed in; the local model is in use" })
    } else if !c.installed {
        light("claude", "Claude", "red", "Claude Code is not installed: Settings, Claude Code")
    } else {
        light("claude", "Claude", "red", "Signed out: Settings, Claude Code, Sign In")
    });
    if st.ollama_model.trim().is_empty() {
        v.push(light("local", "Local model", "off", "Not set up (Settings, AI engine)"));
    } else {
        let o = crate::ollama::status();
        let has = o.installed.iter().any(|m| m == &st.ollama_model || m.split(':').next() == Some(st.ollama_model.as_str()));
        v.push(if o.running && o.version_ok && has {
            light("local", "Local model", "green", format!("{} ready", st.ollama_model))
        } else if st.engine == "claude" && !o.running {
            light("local", "Local model", "off", "Starts when needed")
        } else {
            light("local", "Local model", "red", if !o.running || !o.version_ok { o.message } else { format!("{} is not downloaded (Settings, AI engine)", st.ollama_model) })
        });
    }
    match st.studio_mcp.as_str() {
        "studio" => {
            let running = is_running(STUDIO_PROCESS);
            let server = crate::studio_mcp::default_path().is_some() || !st.studio_mcp_path.is_empty();
            v.push(if running && server {
                light("studio", "Roblox Studio", "green", "Open and connected")
            } else if !server {
                light("studio", "Roblox Studio", "red", "Studio's MCP server was not found: update Roblox Studio")
            } else if studio_exe().is_none() {
                light("studio", "Roblox Studio", "red", "Roblox Studio is not installed")
            } else {
                light("studio", "Roblox Studio", "red", "Not open: BrewixGames opens it when an agent needs it")
            });
        }
        "mock" => v.push(light("studio", "Roblox Studio", "green", "Practice stand-in")),
        _ => v.push(light("studio", "Roblox Studio", "off", "Off (Settings, Roblox Studio)")),
    }
    match st.blender_mcp.as_str() {
        "blender" => {
            v.push(if crate::blender_mcp::uvx().is_none() {
                light("blender", "Blender", "red", "uv is missing: Settings, Blender, Set Up")
            } else if blender_addon_listening() {
                light("blender", "Blender", "green", "Open and connected")
            } else if is_running(BLENDER_PROCESS) {
                light("blender", "Blender", "red", "Open, but its MCP add-on is not connected: in Blender press N, BlenderMCP tab, Connect")
            } else if blender_exe().is_none() {
                light("blender", "Blender", "red", "Blender is not installed: Settings, Blender, Install")
            } else {
                light("blender", "Blender", "red", "Not open: BrewixGames opens it when an agent needs it")
            });
        }
        "mock" => v.push(light("blender", "Blender", "green", "Practice stand-in")),
        _ => v.push(light("blender", "Blender", "off", "Off (Settings, Blender)")),
    }
    v
}
