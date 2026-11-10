//! Locating the studio root (the repo folder that holds `vault/`, `queue/`,
//! `runs/`, `studio/`) and the `claude` binary.

use std::path::{Path, PathBuf};

const MARKER: &str = "studio/agents.json";

/// Find the studio root. Order: `ROBUX_STUDIO_ROOT`, then walk up from the
/// current directory, then walk up from the executable's directory.
pub fn studio_root() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("ROBUX_STUDIO_ROOT") {
        let p = PathBuf::from(p);
        if p.join(MARKER).is_file() {
            return Ok(p);
        }
        return Err(format!(
            "ROBUX_STUDIO_ROOT={} does not contain {}",
            p.display(),
            MARKER
        ));
    }
    let s = crate::settings::load();
    if !s.studio_root.is_empty() {
        let p = PathBuf::from(&s.studio_root);
        if p.join(MARKER).is_file() {
            return Ok(p);
        }
    }
    if let Some(home) = home() {
        let d = home.join("BrewixGames");
        if d.join(MARKER).is_file() {
            return Ok(d);
        }
    }
    let mut starts = vec![];
    if let Ok(c) = std::env::current_dir() {
        starts.push(c);
    }
    if let Ok(e) = std::env::current_exe() {
        if let Some(d) = e.parent() {
            starts.push(d.to_path_buf());
        }
    }
    for s in starts {
        if let Some(r) = walk_up(&s) {
            return Ok(r);
        }
    }
    Err(format!(
        "Could not find the studio root (a folder containing {}). Set ROBUX_STUDIO_ROOT.",
        MARKER
    ))
}

fn walk_up(start: &Path) -> Option<PathBuf> {
    let mut cur = Some(start);
    while let Some(d) = cur {
        if d.join(MARKER).is_file() {
            return Some(d.to_path_buf());
        }
        cur = d.parent();
    }
    None
}

/// Find the `claude` CLI. macOS GUI apps do not inherit the shell PATH, so
/// common install locations are checked as well.
pub fn claude_bin() -> Result<PathBuf, String> {
    if let Ok(p) = std::env::var("CLAUDE_BIN") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Ok(p);
        }
        return Err(format!("CLAUDE_BIN={} is not a file", p.display()));
    }
    let mut candidates: Vec<PathBuf> = vec![];
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            candidates.push(dir.join("claude"));
            candidates.push(dir.join("claude.exe"));
        }
    }
    if let Some(h) = home() {
        candidates.push(h.join(".local/bin/claude"));
        candidates.push(h.join(".local/bin/claude.exe"));
        candidates.push(h.join(".claude/local/claude"));
        candidates.push(h.join(".npm-global/bin/claude"));
    }
    candidates.push(PathBuf::from("/opt/homebrew/bin/claude"));
    candidates.push(PathBuf::from("/usr/local/bin/claude"));
    candidates
        .into_iter()
        .find(|c| c.is_file())
        .ok_or_else(|| {
            "Could not find the `claude` CLI. Install Claude Code or set CLAUDE_BIN.".to_string()
        })
}

/// The user's home folder (HOME on macOS/Linux, USERPROFILE on Windows).
pub fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from)
}
