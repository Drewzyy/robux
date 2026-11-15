//! Find My Files: look for BrewixGames studio folders, loose vaults and the
//! apps (Blender, Obsidian, Claude Code copy) anywhere they usually end up,
//! then point the app at them or move them into the chosen folders.
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const MARKER: &str = "studio/agents.json";
const SKIP: [&str; 14] = ["node_modules", ".git", "target", "Windows", "Program Files", "Program Files (x86)", "ProgramData", "$Recycle.Bin", "System Volume Information", "AppData", "Library", ".cache", "proc", "sys"];

/// Folders worth searching, most likely first.
pub fn search_roots() -> Vec<PathBuf> {
    let mut v = vec![];
    if let Some(h) = crate::paths::home() {
        for sub in ["", "Documents", "Desktop", "Downloads", "OneDrive", "OneDrive/Documents", "OneDrive/Desktop", "Games"] {
            v.push(if sub.is_empty() { h.clone() } else { h.join(sub) });
        }
    }
    let st = crate::settings::load();
    for p in [st.tools_dir.as_str(), st.studio_root.as_str()] {
        if !p.is_empty() {
            let p = crate::paths::expand_home(p);
            v.push(p.parent().map(|x| x.to_path_buf()).unwrap_or(p));
        }
    }
    if cfg!(windows) {
        for d in b'C'..=b'H' {
            v.push(PathBuf::from(format!("{}:\\", d as char)));
        }
    } else {
        v.push("/Volumes".into());
    }
    // The studio in use is always shown, wherever it is.
    if let Ok(cur) = crate::paths::studio_root() {
        v.insert(0, cur.parent().map(|p| p.to_path_buf()).unwrap_or(cur));
    }
    v.retain(|p| p.is_dir());
    // Drop places already inside an earlier one (home already covers Documents).
    let mut out: Vec<PathBuf> = vec![];
    for p in v {
        if !out.iter().any(|o| p.starts_with(o) && p != *o && o.components().count() > 1 && o.parent().is_some() && o.to_string_lossy().len() > 3) && !out.contains(&p) {
            out.push(p);
        }
    }
    let v = out;
    v
}

fn walk(dir: &Path, depth: usize, found: &mut Vec<(String, PathBuf)>, seen: &mut usize) {
    *seen += 1;
    if *seen > 40_000 {
        return;
    }
    if dir.join(MARKER).is_file() {
        found.push(("studio".into(), dir.to_path_buf()));
        return; // do not look inside a studio for more
    }
    if dir.join("00-Start-Here/index.md").is_file() && dir.join("02-Game-Design").is_dir() {
        found.push(("vault".into(), dir.to_path_buf()));
        return;
    }
    let name = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    if name == "blender-app" || name == "obsidian-app" || name == "claude" && dir.parent().map(|p| p.join("blender-app").is_dir() || p.join("obsidian-app").is_dir()).unwrap_or(false) {
        if fs::read_dir(dir).map(|mut r| r.next().is_some()).unwrap_or(false) {
            found.push((name.replace("-app", ""), dir.to_path_buf()));
        }
        return;
    }
    if depth == 0 {
        return;
    }
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        if n.starts_with('.') || SKIP.contains(&n.as_str()) {
            continue;
        }
        if e.file_type().map(|t| t.is_dir() && !t.is_symlink()).unwrap_or(false) {
            walk(&e.path(), depth - 1, found, seen);
        }
    }
}

fn count_md(d: &Path) -> usize {
    fs::read_dir(d).into_iter().flatten().flatten().map(|e| {
        let p = e.path();
        if p.is_dir() { count_md(&p) } else { (p.extension().map(|x| x == "md").unwrap_or(false)) as usize }
    }).sum()
}

fn modified(p: &Path) -> String {
    fs::metadata(p).and_then(|m| m.modified()).ok().map(|t| chrono::DateTime::<chrono::Local>::from(t).format("%Y-%m-%d %H:%M").to_string()).unwrap_or_default()
}

/// Everything found, with enough detail to pick the right one.
pub fn scan_in(roots: &[PathBuf]) -> Vec<Value> {
    let mut found = vec![];
    for r in roots {
        // Each place gets its own search budget, so a big home folder cannot
        // use it all up before another drive is searched.
        let mut seen = 0;
        // home-like folders a bit deeper, whole drives shallower
        let depth = if r.parent().is_none() || r.to_string_lossy().len() <= 3 { 3 } else { 4 };
        walk(r, depth, &mut found, &mut seen);
    }
    found.sort_by(|a, b| a.1.cmp(&b.1));
    found.dedup_by(|a, b| a.1 == b.1);
    let current = crate::paths::studio_root().ok();
    found
        .into_iter()
        .map(|(kind, p)| {
            let vault = if kind == "studio" { p.join("vault") } else { p.clone() };
            let projects = fs::read_dir(p.join("projects")).map(|r| r.flatten().filter(|e| e.path().join("default.project.json").is_file()).count()).unwrap_or(0);
            json!({
                "kind": kind,
                "path": p.display().to_string(),
                "notes": if kind == "studio" || kind == "vault" { count_md(&vault) } else { 0 },
                "projects": projects,
                "modified": modified(&p),
                "current": current.as_ref().map(|c| c == &p).unwrap_or(false),
            })
        })
        .collect()
}

pub fn scan() -> Vec<Value> {
    scan_in(&search_roots())
}

/// Use a found folder where it is: a studio becomes the studio folder, an app
/// folder becomes that app's folder.
pub fn use_found(kind: &str, path: &str) -> Result<String, String> {
    let p = PathBuf::from(path);
    match kind {
        "studio" => {
            if !p.join(MARKER).is_file() {
                return Err("that folder is not a BrewixGames studio".into());
            }
            crate::settings::patch(&json!({ "studio_root": path }))?;
            Ok(format!("Using the studio at {path}."))
        }
        "blender" | "obsidian" | "claude" => {
            crate::settings::patch(&json!({ "app_dirs": { kind: path } }))?;
            Ok(format!("{} is now {path}.", match kind { "blender" => "Blender's folder", "obsidian" => "Obsidian's folder", _ => "Claude Code's folder" }))
        }
        _ => Err("only a studio or an app folder can be used directly".into()),
    }
}

/// When the app cannot find its studio: use the one found that has the most
/// in it (most projects, then most notes). Returns the folder if it fixed it.
pub fn auto_heal() -> Option<PathBuf> {
    if crate::paths::studio_root().is_ok() {
        return None;
    }
    let mut studios: Vec<Value> = scan().into_iter().filter(|v| v["kind"] == "studio").collect();
    studios.sort_by_key(|v| (v["projects"].as_u64().unwrap_or(0), v["notes"].as_u64().unwrap_or(0)));
    let best = studios.pop()?;
    let path = best["path"].as_str()?.to_string();
    use_found("studio", &path).ok()?;
    Some(PathBuf::from(path))
}

/// Bring a loose vault's notes into the studio's vault without overwriting
/// anything: notes that already exist are kept as they are.
pub fn merge_vault(from: &Path, studio: &Path) -> Result<String, String> {
    if !from.join("00-Start-Here/index.md").is_file() {
        return Err("that folder is not a BrewixGames vault".into());
    }
    let dest = studio.join("vault");
    let (mut added, mut kept) = (0, 0);
    fn go(src: &Path, dst: &Path, added: &mut usize, kept: &mut usize) -> Result<(), String> {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
        for e in fs::read_dir(src).map_err(|e| e.to_string())?.flatten() {
            let (s, d) = (e.path(), dst.join(e.file_name()));
            if s.is_dir() {
                go(&s, &d, added, kept)?;
            } else if d.exists() {
                *kept += 1;
            } else {
                fs::copy(&s, &d).map_err(|e| e.to_string())?;
                *added += 1;
            }
        }
        Ok(())
    }
    go(from, &dest, &mut added, &mut kept)?;
    Ok(format!("Added {added} file(s) from {} into your vault; {kept} already there were kept as they are. The old folder was not deleted.", from.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_studios_vaults_and_apps() {
        let d = std::env::temp_dir().join(format!("locate-t-{}", std::process::id()));
        let s = d.join("Old Stuff/BrewixGames");
        fs::create_dir_all(s.join("studio")).unwrap();
        fs::write(s.join(MARKER), "{}").unwrap();
        fs::create_dir_all(s.join("vault/00-Start-Here")).unwrap();
        fs::write(s.join("vault/00-Start-Here/index.md"), "x").unwrap();
        let v = d.join("Backup/vault");
        fs::create_dir_all(v.join("00-Start-Here")).unwrap();
        fs::create_dir_all(v.join("02-Game-Design")).unwrap();
        fs::write(v.join("00-Start-Here/index.md"), "x").unwrap();
        fs::write(v.join("02-Game-Design/new-note.md"), "x").unwrap();
        fs::create_dir_all(d.join("D/tools/blender-app/blender-5.2")).unwrap();
        let r = scan_in(&[d.clone()]);
        let kinds: Vec<&str> = r.iter().map(|x| x["kind"].as_str().unwrap()).collect();
        assert!(kinds.contains(&"studio") && kinds.contains(&"vault") && kinds.contains(&"blender"), "{kinds:?}");
        let msg = merge_vault(&v, &s).unwrap();
        assert!(msg.starts_with("Added 1 file(s)"), "{msg}");
        assert!(s.join("vault/02-Game-Design/new-note.md").is_file());
        let _ = fs::remove_dir_all(&d);
    }
}
