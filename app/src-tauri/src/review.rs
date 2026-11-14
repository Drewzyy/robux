//! Second opinion: a different AI (not Claude) reviews a game project or the
//! BrewixGames app itself. Free engines only, never an API key:
//! - local: a model in Ollama on this computer (nothing leaves the computer)
//! - codex: OpenAI's Codex CLI, if Andrew installed it and signed in
//! - chatgpt: a packed file Andrew drags into ChatGPT, then pastes the answer
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const CODE_EXT: [&str; 9] = ["rs", "js", "html", "css", "luau", "lua", "py", "toml", "json"];
const SKIP_DIRS: [&str; 6] = ["target", "node_modules", ".git", "dist", "gen", "icons"];
const MAX_FILE: u64 = 120_000;
const CHUNK: usize = 40_000; // characters per local-model request (about 10k tokens)

pub const PROMPT: &str = "You are a senior reviewer giving a second opinion. The files below are one codebase (a Roblox game in Luau, or a Tauri desktop app in Rust and JavaScript). Confirm what is sound and find what is not. Report only real problems you can point to, most serious first, each as: severity (blocker, major, minor) | file and line or function | what is wrong | the fix. Look for: bugs and crashes, security holes (client trust in Roblox, remote events not validated on the server, path traversal, command injection), data loss (DataStore misuse), and things that break on phones. Do not report style. If a part is fine, say so in one line. End with a verdict: SOUND, SOUND WITH FIXES, or NOT SOUND.";

fn collect(dir: &Path, base: &Path, out: &mut Vec<(String, String)>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut es: Vec<_> = rd.flatten().collect();
    es.sort_by_key(|e| e.path());
    for e in es {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if p.is_dir() {
            if !name.starts_with('.') && !SKIP_DIRS.contains(&name.as_str()) {
                collect(&p, base, out);
            }
            continue;
        }
        let ext = p.extension().and_then(|x| x.to_str()).unwrap_or("");
        if !CODE_EXT.contains(&ext) || name.ends_with(".lock") || name == "package-lock.json" {
            continue;
        }
        if fs::metadata(&p).map(|m| m.len() > MAX_FILE).unwrap_or(true) {
            continue;
        }
        if let Ok(t) = fs::read_to_string(&p) {
            let rel = p.strip_prefix(base).unwrap_or(&p).to_string_lossy().replace('\\', "/");
            out.push((rel, t));
        }
    }
}

/// All reviewable files of a folder, as (path, text).
pub fn files(dir: &Path) -> Vec<(String, String)> {
    let mut out = vec![];
    collect(dir, dir, &mut out);
    out
}

pub fn pack(title: &str, files: &[(String, String)]) -> String {
    let mut s = format!("# {title}\n\n{PROMPT}\n\n{} files\n", files.len());
    for (p, t) in files {
        s.push_str(&format!("\n===== {p} =====\n{t}\n"));
    }
    s
}

/// Split files into chunks that fit a local model, never splitting a file
/// unless it alone is bigger than a chunk.
pub fn chunks(files: &[(String, String)]) -> Vec<String> {
    let mut out = vec![];
    let mut cur = String::new();
    for (p, t) in files {
        let block = format!("\n===== {p} =====\n{t}\n");
        if !cur.is_empty() && cur.len() + block.len() > CHUNK {
            out.push(std::mem::take(&mut cur));
        }
        if block.len() > CHUNK {
            for part in block.as_bytes().chunks(CHUNK) {
                out.push(String::from_utf8_lossy(part).to_string());
            }
        } else {
            cur.push_str(&block);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn review_dir(root: &Path, id: &str) -> PathBuf {
    root.join("reviews").join(id)
}

/// The folder to review: a project under projects/, or the app's source for
/// this exact version, downloaded from GitHub (public, no login).
pub fn source(root: &Path, target: &str, version: &str, on_line: &mut dyn FnMut(&str)) -> Result<(String, PathBuf), String> {
    if target == "app" {
        let dir = crate::paths::tools_dir().join("app-source").join(format!("v{version}"));
        if !dir.is_dir() {
            let zip = dir.with_extension("zip");
            crate::apps::download(&format!("https://codeload.github.com/Drewzyy/robux/zip/refs/tags/v{version}"), &zip, on_line)?;
            fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let st = crate::paths::command("tar").arg("-xf").arg(&zip).arg("-C").arg(&dir).status().map_err(|e| format!("tar: {e}"))?;
            let _ = fs::remove_file(&zip);
            if !st.success() {
                return Err("could not unpack the app source".into());
            }
        }
        let top = fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten().map(|e| e.path()).find(|p| p.is_dir()).ok_or("app source folder is empty")?;
        return Ok((format!("BrewixGames app v{version}"), top.join("app")));
    }
    let name = target.strip_prefix("projects/").unwrap_or(target);
    if name.is_empty() || name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err("bad project name".into());
    }
    let p = root.join("projects").join(name);
    if !p.is_dir() {
        return Err(format!("no project named {name}"));
    }
    Ok((format!("Roblox game {name}"), p))
}

/// Best installed Ollama model for code review.
pub fn local_model() -> Option<String> {
    let st = crate::ollama::status();
    let pref = ["qwen2.5-coder", "qwen3-coder", "gpt-oss", "devstral", "qwen3"];
    pref.iter().find_map(|p| st.installed.iter().find(|m| m.starts_with(p)).cloned())
}

fn ask_local(model: &str, text: &str) -> Result<String, String> {
    let v: Value = ureq::post(&format!("{}/api/chat", crate::ollama::BASE))
        .timeout(std::time::Duration::from_secs(1800))
        .send_json(json!({ "model": model, "stream": false, "options": { "num_ctx": 16384 },
            "messages": [{ "role": "system", "content": PROMPT }, { "role": "user", "content": text }] }))
        .map_err(|e| format!("could not reach Ollama: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;
    Ok(v["message"]["content"].as_str().unwrap_or("").to_string())
}

pub fn codex_installed() -> bool {
    crate::paths::command(if cfg!(windows) { "codex.cmd" } else { "codex" }).arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
}

/// Run a review. Returns the review id; report.md lands in reviews/<id>/.
pub fn run(root: &Path, target: &str, engine: &str, version: &str, on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let (title, dir) = source(root, target, version, on_line)?;
    let fs_ = files(&dir);
    if fs_.is_empty() {
        return Err("no code files found to review".into());
    }
    let id = crate::new_id("review");
    let rd = review_dir(root, &id);
    fs::create_dir_all(&rd).map_err(|e| e.to_string())?;
    let packed = pack(&title, &fs_);
    fs::write(rd.join("pack.md"), &packed).map_err(|e| e.to_string())?;
    let meta = json!({ "id": id, "title": title, "target": target, "engine": engine, "files": fs_.len(), "chars": packed.len(), "at": crate::now_rfc3339() });
    fs::write(rd.join("review.json"), meta.to_string()).map_err(|e| e.to_string())?;
    let report = match engine {
        "local" => {
            let model = local_model().ok_or("No local model for reviews. In Settings, Free Local Engine, download one (qwen3:8b or bigger), then try again.")?;
            let parts = chunks(&fs_);
            let mut out = format!("# Second opinion: {title}\n\nReviewer: {model} on this computer (Ollama). {} files in {} parts.\n", fs_.len(), parts.len());
            for (i, p) in parts.iter().enumerate() {
                on_line(&format!("part {} of {}", i + 1, parts.len()));
                out.push_str(&format!("\n## Part {}\n\n{}\n", i + 1, ask_local(&model, p)?));
            }
            out
        }
        "codex" => {
            if !codex_installed() {
                return Err("Codex CLI is not installed. Install it from OpenAI (free with a ChatGPT login), sign in once in a terminal, then try again.".into());
            }
            on_line("Codex is reading the code");
            let o = crate::paths::command(if cfg!(windows) { "codex.cmd" } else { "codex" })
                .current_dir(&rd)
                .args(["exec", "--skip-git-repo-check", "--sandbox", "read-only"])
                .arg(format!("{PROMPT}\n\nThe code is in pack.md in this folder. Read it all, then write the review."))
                .output()
                .map_err(|e| format!("codex: {e}"))?;
            if !o.status.success() {
                return Err(format!("Codex stopped: {}", String::from_utf8_lossy(&o.stderr).chars().take(400).collect::<String>()));
            }
            format!("# Second opinion: {title}\n\nReviewer: OpenAI Codex CLI.\n\n{}", String::from_utf8_lossy(&o.stdout))
        }
        "chatgpt" => {
            crate::paths::open_url("https://chatgpt.com/");
            return Ok(id);
        }
        _ => return Err("unknown engine".into()),
    };
    fs::write(rd.join("report.md"), report).map_err(|e| e.to_string())?;
    Ok(id)
}

/// Save an answer Andrew pasted from ChatGPT.
pub fn save_answer(root: &Path, id: &str, text: &str) -> Result<(), String> {
    if !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("bad review id".into());
    }
    let rd = review_dir(root, id);
    if !rd.is_dir() {
        return Err("no such review".into());
    }
    fs::write(rd.join("report.md"), format!("# Second opinion\n\nReviewer: ChatGPT (pasted by Andrew).\n\n{text}")).map_err(|e| e.to_string())
}

/// Past reviews, newest first, with their report when there is one.
pub fn list(root: &Path) -> Vec<Value> {
    let mut out: Vec<Value> = fs::read_dir(root.join("reviews"))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let mut v: Value = serde_json::from_str(&fs::read_to_string(e.path().join("review.json")).ok()?).ok()?;
            v["report"] = json!(fs::read_to_string(e.path().join("report.md")).ok());
            v["pack"] = json!(e.path().join("pack.md").to_string_lossy());
            Some(v)
        })
        .collect();
    out.sort_by(|a, b| b["at"].as_str().cmp(&a["at"].as_str()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packs_code_and_chunks_by_file() {
        let d = std::env::temp_dir().join(format!("rev-t-{}", std::process::id()));
        fs::create_dir_all(d.join("src/target")).unwrap();
        fs::write(d.join("src/a.luau"), "print(1)").unwrap();
        fs::write(d.join("src/target/skip.rs"), "x").unwrap();
        fs::write(d.join("notes.txt"), "x").unwrap();
        let f = files(&d);
        assert_eq!(f.iter().map(|x| x.0.as_str()).collect::<Vec<_>>(), vec!["src/a.luau"]);
        assert!(pack("T", &f).contains("===== src/a.luau ====="));
        let big = vec![("a".to_string(), "x".repeat(30_000)), ("b".to_string(), "y".repeat(30_000)), ("c".to_string(), "z".repeat(90_000))];
        assert_eq!(chunks(&big).len(), 5);
        let _ = fs::remove_dir_all(&d);
    }
}
