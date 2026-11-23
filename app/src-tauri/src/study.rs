//! `brewix study <link or file>`: learn from someone else's Roblox game
//! without running anything in it. A place file (.rbxl, a link to one or a
//! local file) is opened with Lune: every script is saved, with a summary
//! and a backdoor scan. A GitHub repository is cloned (shallow). Output goes
//! to runs/studies/<id>/ for agents to read.

use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::Path;

const MAX_BYTES: u64 = 80 * 1024 * 1024;

fn id_for(src: &str) -> String {
    let base: String = src.rsplit(['/', '\\']).find(|s| !s.is_empty()).unwrap_or("study").chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    format!("{}-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"), base.trim_matches('-').chars().take(40).collect::<String>())
}

pub fn study(root: &Path, src: &str) -> Result<Value, String> {
    let id = id_for(src);
    let dir = root.join("runs/studies").join(&id);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let r = study_into(&dir, &id, root, src);
    if r.is_err() {
        let _ = fs::remove_dir_all(&dir); // nothing half-made left behind
    }
    r
}

fn study_into(dir: &Path, id: &str, root: &Path, src: &str) -> Result<Value, String> {
    let dir = dir.to_path_buf();
    let is_repo = src.starts_with("https://github.com/") && !src.ends_with(".rbxl") && !src.ends_with(".rbxlx");
    if is_repo {
        let url = src.trim_end_matches('/').trim_end_matches(".git").to_string() + ".git";
        let o = crate::paths::output_timeout(crate::paths::command("git").args(["clone", "--depth", "1", &url]).arg(dir.join("repo")), 180).map_err(|e| format!("git clone: {e}"))?;
        if !o.status.success() {
            return Err(format!("Could not clone {src}: {}", String::from_utf8_lossy(&o.stderr).lines().last().unwrap_or("")));
        }
        let mut n = 0;
        count_code(&dir.join("repo"), &mut n);
        return Ok(json!({ "id": id, "kind": "repo", "dir": format!("runs/studies/{id}/repo"), "code_files": n,
            "next": "Read its README and project layout first (default.project.json, src/), then the main modules. Check the LICENSE before borrowing anything." }));
    }
    // A place file: download it (or copy a local one), never run it.
    let place = dir.join("place.rbxl");
    if src.starts_with("http") {
        let resp = ureq::get(src).timeout(std::time::Duration::from_secs(120)).call().map_err(|e| format!("download failed: {e}"))?;
        let mut bytes = vec![];
        resp.into_reader().take(MAX_BYTES).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        if bytes.len() < 32 || !(bytes.starts_with(b"<roblox!") || bytes.starts_with(b"<roblox")) {
            return Err("That link is not a Roblox place file (.rbxl). Game pages cannot be downloaded: open the game in Studio and use File, Save to File.".into());
        }
        fs::write(&place, bytes).map_err(|e| e.to_string())?;
    } else {
        fs::copy(src, &place).map_err(|e| format!("could not read {src}: {e}"))?;
    }
    let script = root.join("studio/tools/rbxl_study.luau");
    if !script.is_file() {
        return Err("studio/tools/rbxl_study.luau is missing; update BrewixGames".into());
    }
    let o = crate::paths::output_timeout(crate::paths::command(crate::gates::tool("lune")).arg("run").arg(&script).arg(&place).arg(&dir), 300).map_err(|e| format!("could not open the place: {e}"))?;
    if !o.status.success() {
        return Err(format!("Could not open the place file: {}", String::from_utf8_lossy(&o.stderr).lines().last().unwrap_or("")));
    }
    let _ = fs::remove_file(&place); // the scripts and summary are what agents need
    Ok(json!({ "id": id, "kind": "place", "dir": format!("runs/studies/{id}"), "summary": format!("runs/studies/{id}/summary.md"),
        "line": String::from_utf8_lossy(&o.stdout).trim().to_string(),
        "next": "Read summary.md first (it lists possible backdoors: never copy those scripts), then the game's own biggest scripts under scripts/." }))
}

fn count_code(d: &Path, n: &mut usize) {
    for e in fs::read_dir(d).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().map(|f| f != ".git").unwrap_or(true) {
                count_code(&p, n);
            }
        } else if p.extension().map(|x| x == "lua" || x == "luau").unwrap_or(false) {
            *n += 1;
        }
    }
}
