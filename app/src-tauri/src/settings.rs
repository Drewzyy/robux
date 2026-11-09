//! App settings. Stored outside the repo (they hold the Discord webhook URL,
//! which must never be committed):
//! - macOS: ~/Library/Application Support/com.andrewdieu.robuxstudio/settings.json
//! - other: ~/.config/robux-studio/settings.json
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
        }
    }
}

pub fn config_dir() -> PathBuf {
    if let Ok(d) = std::env::var("ROBUX_CONFIG_DIR") {
        return PathBuf::from(d);
    }
    let home = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()));
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support/com.andrewdieu.robuxstudio")
    } else {
        home.join(".config/robux-studio")
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
