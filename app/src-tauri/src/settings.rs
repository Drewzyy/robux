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

pub fn load() -> Settings {
    fs::read_to_string(path())
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
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
    if !["off", "studio", "mock"].contains(&s.studio_mcp.as_str()) {
        return Err("Studio connection must be off, studio, or mock".into());
    }
    if s.studio_mcp == "studio" && crate::studio_mcp::server_command(s).is_none() {
        return Err("Roblox's Studio MCP server is not installed. Install it from Roblox (see Settings), or give its path.".into());
    }
    if !s.discord_webhook_url.is_empty() {
        crate::notify::validate_url(&s.discord_webhook_url)?;
    }
    if !s.studio_root.is_empty() && !PathBuf::from(&s.studio_root).join("studio/agents.json").is_file() {
        return Err(format!("{} is not a studio folder (no studio/agents.json)", s.studio_root));
    }
    Ok(())
}

pub fn save(s: &Settings) -> Result<(), String> {
    validate(s)?;
    let d = config_dir();
    fs::create_dir_all(&d).map_err(|e| format!("create {}: {e}", d.display()))?;
    let tmp = d.join("settings.json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(s).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path()).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path(), fs::Permissions::from_mode(0o600));
    }
    Ok(())
}
