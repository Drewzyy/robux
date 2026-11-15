//! App settings. Stored outside the repo (they hold the Discord webhook URL,
//! which must never be committed):
//! - macOS: ~/Library/Application Support/com.andrewdieu.brewixgames/settings.json
//! - other: ~/.config/brewixgames/settings.json
//! - override: ROBUX_CONFIG_DIR (used by tests)

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotifyPrefs {
    /// Something landed in the review queue and needs Andrew.
    #[serde(default = "yes")]
    pub needs_approval: bool,
    /// A pipeline or run finished (a game finished building).
    #[serde(default = "yes")]
    pub finished: bool,
    /// A run failed or stopped.
    #[serde(default = "yes")]
    pub failed: bool,
    /// The monthly budget stopped a run.
    #[serde(default = "yes")]
    pub budget: bool,
    /// An app update is available or was installed.
    #[serde(default)]
    pub updates: bool,
}
fn yes() -> bool {
    true
}
impl Default for NotifyPrefs {
    fn default() -> Self {
        NotifyPrefs { needs_approval: true, finished: true, failed: true, budget: true, updates: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    /// Studio folder (the repo that holds vault/). Empty = auto-detect.
    #[serde(default)]
    pub studio_root: String,
    /// Agent runs stop and ask once this month's spend would pass this
    /// amount (API-equivalent USD). 0 means no budget.
    #[serde(default = "default_budget")]
    pub monthly_budget_usd: f64,
    /// Discord webhook URL. Empty = notifications off.
    #[serde(default)]
    pub discord_webhook_url: String,
    #[serde(default)]
    pub notify: NotifyPrefs,
    /// "claude" (your Claude subscription) or "ollama" (free local models).
    #[serde(default = "default_engine")]
    pub engine: String,
    /// Local model name for the Ollama engine, e.g. "qwen3-coder".
    #[serde(default)]
    pub ollama_model: String,
    /// Roblox Studio connection: "off" (default), "studio" (Roblox's Studio
    /// MCP server on this computer), or "mock" (built-in stand-in).
    #[serde(default = "default_studio_mcp")]
    pub studio_mcp: String,
    /// Optional custom path to Roblox's Studio MCP server.
    #[serde(default)]
    pub studio_mcp_path: String,
    /// Blender connection: "off" (default), "blender" (MCP for Blender via
    /// uvx), or "mock" (built-in stand-in).
    #[serde(default = "default_studio_mcp")]
    pub blender_mcp: String,
    /// Days between automatic tune-ups (0 turns them off).
    #[serde(default = "default_tuneup_days")]
    pub tuneup_days: u32,
    /// Model for Ask Claude (empty: the default) and for agent runs (empty: each agent's own).
    #[serde(default)]
    pub chat_model: String,
    #[serde(default)]
    pub run_model: String,
    /// Start Ollama in the background when the app opens (if installed).
    #[serde(default = "default_true")]
    pub ollama_autostart: bool,
    /// Figma login connection for the UI agents (off by default).
    #[serde(default)]
    pub figma_mcp: bool,
    /// Optional path to the claude program when it is not found on its own.
    #[serde(default)]
    pub claude_path: String,
    /// Where BrewixGames puts everything it downloads (Claude Code copy, uv,
    /// Python packages, the Blender add-on). Empty means <studio>/tools.
    #[serde(default)]
    pub tools_dir: String,
    /// Chosen install folders per app ("claude", "blender", "obsidian").
    /// Empty or missing means a folder inside the Downloads folder.
    #[serde(default)]
    pub app_dirs: std::collections::BTreeMap<String, String>,
    /// US dollars per Earned Robux for the Money page (DevEx standard rate).
    #[serde(default = "default_rate")]
    pub devex_rate: f64,
}
fn default_rate() -> f64 {
    crate::earnings::STANDARD_RATE
}
fn default_true() -> bool {
    true
}
fn default_tuneup_days() -> u32 {
    7
}
fn default_studio_mcp() -> String {
    "off".into()
}
fn default_engine() -> String {
    "claude".into()
}
fn default_budget() -> f64 {
    25.0
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            studio_root: String::new(),
            monthly_budget_usd: default_budget(),
            discord_webhook_url: String::new(),
            notify: NotifyPrefs::default(),
            engine: default_engine(),
            ollama_model: String::new(),
            studio_mcp: default_studio_mcp(),
            studio_mcp_path: String::new(),
            blender_mcp: default_studio_mcp(),
            figma_mcp: false,
            ollama_autostart: true,
            chat_model: String::new(),
            run_model: String::new(),
            tuneup_days: default_tuneup_days(),
            claude_path: String::new(),
            tools_dir: String::new(),
            app_dirs: Default::default(),
            devex_rate: default_rate(),
        }
    }
}

pub fn config_dir() -> PathBuf {
    if let Ok(d) = std::env::var("ROBUX_CONFIG_DIR") {
        return PathBuf::from(d);
    }
    let home = crate::paths::home().unwrap_or_else(|| PathBuf::from("."));
    if cfg!(target_os = "windows") {
        return std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or(home).join("BrewixGames");
    }
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support/com.andrewdieu.brewixgames")
    } else {
        home.join(".config/brewixgames")
    }
}

pub fn path() -> PathBuf {
    config_dir().join("settings.json")
}

/// Read settings without ever losing them: if the file does not parse in
/// full, keep every field that still reads, fall back to the last good
/// backup, and only then to defaults.
pub fn load() -> Settings {
    let main = fs::read_to_string(path()).ok();
    if let Some(s) = main.as_deref().and_then(|t| serde_json::from_str::<Settings>(t).ok()) {
        return s;
    }
    let bak = fs::read_to_string(path().with_extension("json.bak")).ok();
    if let Some(s) = bak.as_deref().and_then(|t| serde_json::from_str::<Settings>(t).ok()) {
        return s;
    }
    main.or(bak).map(|t| salvage(&t)).unwrap_or_default()
}

/// Keep each field of a damaged settings file that still has a valid value.
pub fn salvage(text: &str) -> Settings {
    let Ok(serde_json::Value::Object(saved)) = serde_json::from_str::<serde_json::Value>(text) else { return Settings::default() };
    let mut cur = serde_json::to_value(Settings::default()).unwrap_or_default();
    for (k, v) in saved {
        let mut trial = cur.clone();
        trial[&k] = v;
        if serde_json::from_value::<Settings>(trial.clone()).is_ok() {
            cur = trial;
        }
    }
    serde_json::from_value(cur).unwrap_or_default()
}

pub fn validate(s: &Settings) -> Result<(), String> {
    if !(s.monthly_budget_usd.is_finite() && s.monthly_budget_usd >= 0.0) {
        return Err("budget must be a number, 0 or more".into());
    }
    if s.engine != "claude" && s.engine != "ollama" {
        return Err("engine must be claude or ollama".into());
    }
    if s.engine == "ollama" && s.ollama_model.trim().is_empty() {
        return Err("Type the local model's name (for example qwen3-coder) before switching to Ollama.".into());
    }
    if !(s.devex_rate.is_finite() && s.devex_rate > 0.0 && s.devex_rate < 1.0) {
        return Err("the Robux rate must be a small positive number, like 0.0038".into());
    }
    if !["off", "studio", "mock"].contains(&s.studio_mcp.as_str()) {
        return Err("Studio connection must be off, studio, or mock".into());
    }
    if s.studio_mcp == "studio" && crate::studio_mcp::server_command(s).is_none() {
        return Err("Roblox's Studio MCP server is not installed. Install it from Roblox (see Settings), or give its path.".into());
    }
    if !["off", "blender", "mock"].contains(&s.blender_mcp.as_str()) {
        return Err("Blender connection must be off, blender, or mock".into());
    }
    if s.blender_mcp == "blender" && crate::blender_mcp::uvx().is_none() {
        return Err("uv is not installed. It is free: install it from astral.sh/uv (Windows setup can do it for you), then try again.".into());
    }
    if !s.discord_webhook_url.is_empty() {
        crate::notify::validate_url(&s.discord_webhook_url)?;
    }
    if !s.studio_root.is_empty() && !PathBuf::from(&s.studio_root).join("studio/agents.json").is_file() {
        return Err(format!("{} is not a studio folder (no studio/agents.json)", s.studio_root));
    }
    Ok(())
}

static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn merge(a: &mut serde_json::Value, b: &serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(x), serde_json::Value::Object(y)) => {
            for (k, v) in y {
                merge(x.entry(k.clone()).or_insert(serde_json::Value::Null), v);
            }
        }
        (a, b) => *a = b.clone(),
    }
}

/// Change only the given fields, on top of what is on disk right now, so a
/// page holding an old copy can never undo someone else's save.
pub fn patch(p: &serde_json::Value) -> Result<Settings, String> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut cur = serde_json::to_value(load()).map_err(|e| e.to_string())?;
    merge(&mut cur, p);
    let next: Settings = serde_json::from_value(cur).map_err(|e| format!("bad setting: {e}"))?;
    // If the saved settings were already invalid for the same reason (an
    // unplugged drive, a removed tool), still save this unrelated change.
    let old = load();
    match (validate(&next), validate(&old)) {
        (Err(a), Err(b)) if a == b => write_unchecked(&next)?,
        _ => save_unlocked(&next)?,
    }
    Ok(next)
}

pub fn save(s: &Settings) -> Result<(), String> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    save_unlocked(s)
}

fn save_unlocked(s: &Settings) -> Result<(), String> {
    validate(s)?;
    write_unchecked(s)
}

fn write_unchecked(s: &Settings) -> Result<(), String> {
    let d = config_dir();
    fs::create_dir_all(&d).map_err(|e| format!("create {}: {e}", d.display()))?;
    let tmp = d.join("settings.json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(s).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    // Keep the last good copy, then swap in the new file. Windows can briefly
    // lock a file (antivirus, indexing), so retry the swap a few times.
    if path().is_file() {
        let _ = fs::copy(path(), path().with_extension("json.bak"));
    }
    let mut last = Ok(());
    for _ in 0..5 {
        last = fs::rename(&tmp, path());
        if last.is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(80));
    }
    last.map_err(|e| format!("could not save settings to {}: {e}", path().display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path(), fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn salvage_keeps_good_fields() {
        let s = salvage(r#"{"studio_root": "D:/Games/Studio", "tools_dir": "D:/Tools", "monthly_budget_usd": "oops", "tuneup_days": 3}"#);
        assert_eq!(s.studio_root, "D:/Games/Studio");
        assert_eq!(s.tools_dir, "D:/Tools");
        assert_eq!(s.tuneup_days, 3);
    }
}
