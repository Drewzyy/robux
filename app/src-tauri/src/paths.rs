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
    let custom = crate::settings::load().claude_path;
    if !custom.trim().is_empty() {
        let p = expand_home(custom.trim());
        if p.is_file() {
            return Ok(p);
        }
        return Err(format!("The Claude Code location in Settings ({}) is not a file.", p.display()));
    }
    let mut candidates: Vec<PathBuf> = vec![app_dir("claude").join(if cfg!(windows) { "claude.exe" } else { "claude" })];
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
    if let Some(a) = std::env::var_os("APPDATA") {
        candidates.push(PathBuf::from(&a).join("npm").join("claude.cmd"));
    }
    if let Some(found) = candidates.into_iter().find(|c| c.is_file()) {
        return Ok(found);
    }
    // The Claude desktop app keeps its own copy of Claude Code in its app
    // data (one folder per version). Use the newest one found.
    let mut roots: Vec<PathBuf> = vec![];
    if let Some(h) = home() {
        roots.push(h.join("Library/Application Support/Claude"));
    }
    for v in ["APPDATA", "LOCALAPPDATA"] {
        if let Some(a) = std::env::var_os(v) {
            roots.push(PathBuf::from(&a).join("Claude"));
            roots.push(PathBuf::from(&a).join("AnthropicClaude"));
        }
    }
    let mut found: Vec<(std::time::SystemTime, PathBuf)> = vec![];
    for r in roots {
        find_named(&r, 4, &mut found);
    }
    found.sort();
    found.pop().map(|(_, p)| p).ok_or_else(|| {
        "Could not find Claude Code. The Claude desktop app or website is not enough on its own: press Install Claude Code in Settings, or paste where claude is in Settings.".to_string()
    })
}

/// Files named claude / claude.exe up to `depth` folders down, with their
/// modified time.
fn find_named(dir: &std::path::Path, depth: u32, out: &mut Vec<(std::time::SystemTime, PathBuf)>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if depth > 0 && !name.starts_with('.') && name != "Cache" && name != "node_modules" {
                find_named(&p, depth - 1, out);
            }
        } else if name == "claude" || name == "claude.exe" {
            if let Ok(t) = e.metadata().and_then(|m| m.modified()) {
                out.push((t, p));
            }
        }
    }
}

/// `~/x` and `~\x` to the home folder.
pub fn expand_home(p: &str) -> PathBuf {
    match (p.strip_prefix("~/").or_else(|| p.strip_prefix("~\\")), home()) {
        (Some(rest), Some(h)) => h.join(rest),
        _ => PathBuf::from(p),
    }
}

/// The user's home folder (HOME on macOS/Linux, USERPROFILE on Windows).
pub fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).map(PathBuf::from)
}

/// Start a program without flashing a console window on Windows (the
/// CREATE_NO_WINDOW flag); identical to Command::new elsewhere. Every
/// background program BrewixGames runs goes through here.
pub fn command<S: AsRef<std::ffi::OsStr>>(program: S) -> std::process::Command {
    #[allow(unused_mut)]
    let mut c = std::process::Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        c.creation_flags(CREATE_NO_WINDOW);
    }
    c
}

/// Open a web link in the default browser.
pub fn open_url(url: &str) {
    let _ = if cfg!(windows) {
        command("rundll32").args(["url.dll,FileProtocolHandler", url]).status()
    } else if cfg!(target_os = "macos") {
        command("open").arg(url).status()
    } else {
        command("xdg-open").arg(url).status()
    };
}

/// The Downloads folder: Settings > Downloads folder, or <studio>/tools.
pub fn tools_dir() -> PathBuf {
    let custom = crate::settings::load().tools_dir;
    if !custom.trim().is_empty() {
        return expand_home(custom.trim());
    }
    studio_root().map(|r| r.join("tools")).unwrap_or_else(|_| home().unwrap_or_default().join("BrewixGames").join("tools"))
}

/// Install folder for one app: the folder chosen for it in Settings, or
/// <Downloads folder>/<default name>.
pub fn app_dir(app: &str) -> PathBuf {
    let chosen = crate::settings::load().app_dirs.get(app).cloned().unwrap_or_default();
    if !chosen.trim().is_empty() {
        return expand_home(chosen.trim());
    }
    let default = match app {
        "blender" => "blender-app",
        "obsidian" => "obsidian-app",
        other => other,
    };
    tools_dir().join(default)
}
