//! Self-update from the studio's own git repo. Free and private: no release
//! server, no signing keys, nothing public.
//!
//! check: `git fetch` the current branch and count new commits.
//! apply: refuse while an agent run is in progress or tracked files have
//! local edits; `git pull --ff-only`; rebuild the app the same way it is
//! currently running (debug, release, or a macOS .app bundle); the caller
//! then restarts the app.

use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub branch: String,
    pub local: String,
    pub remote: String,
    pub behind: u32,
    pub ahead: u32,
    pub dirty: bool,
    pub can_update: bool,
    pub message: String,
    pub build: String,
}

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let o = crate::paths::command("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("git not available: {e}"))?;
    if !o.status.success() {
        return Err(format!("git {}: {}", args.join(" "), String::from_utf8_lossy(&o.stderr).trim()));
    }
    Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
}

pub fn build_commit() -> String {
    option_env!("STUDIO_COMMIT").unwrap_or("unknown").to_string()
}

/// Where downloaded copies of the app get updates from (public; holds only
/// the built app, never the code).
pub const RELEASES_REPO: &str = "Drewzyy/robux";
#[cfg(not(windows))]
pub const RELEASE_ASSET: &str = "BrewixGames-macos.zip";
#[cfg(windows)]
pub const RELEASE_ASSET: &str = "BrewixGames-windows-setup.exe";

/// The release tag this app was built as (set by the release build).
pub fn release_tag() -> Option<&'static str> {
    option_env!("STUDIO_RELEASE")
}

fn is_git_checkout(root: &Path) -> bool {
    root.join(".git").exists()
}

fn latest_release() -> Result<(String, String), String> {
    let url = format!("https://api.github.com/repos/{RELEASES_REPO}/releases/latest");
    let v: serde_json::Value = ureq::get(&url)
        .set("User-Agent", "BrewixGames")
        .set("Accept", "application/vnd.github+json")
        .timeout(std::time::Duration::from_secs(20))
        .call()
        .map_err(|e| format!("could not reach GitHub releases: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;
    let tag = v.get("tag_name").and_then(|t| t.as_str()).ok_or("no release published yet")?.to_string();
    let asset = v
        .get("assets")
        .and_then(|a| a.as_array())
        .and_then(|a| a.iter().find(|x| x.get("name").and_then(|n| n.as_str()) == Some(RELEASE_ASSET)))
        .and_then(|x| x.get("browser_download_url").and_then(|u| u.as_str()))
        .ok_or_else(|| format!("release {tag} has no {RELEASE_ASSET}"))?
        .to_string();
    Ok((tag, asset))
}

pub fn check(root: &Path) -> Result<Status, String> {
    if !is_git_checkout(root) {
        let current = release_tag().unwrap_or("dev").to_string();
        let (latest, _) = latest_release()?;
        let newer = latest != current;
        return Ok(Status {
            branch: "release".into(),
            local: current.clone(),
            remote: latest.clone(),
            behind: newer as u32,
            ahead: 0,
            dirty: false,
            can_update: newer,
            message: if newer { format!("Version {latest} is ready.") } else { "You have the latest version.".into() },
            build: current,
        });
    }
    let branch = git(root, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    git(root, &["fetch", "--quiet", "origin", &branch])?;
    let local = git(root, &["rev-parse", "--short", "HEAD"])?;
    let remote = git(root, &["rev-parse", "--short", &format!("origin/{branch}")])?;
    let counts = git(root, &["rev-list", "--left-right", "--count", &format!("HEAD...origin/{branch}")])?;
    let mut it = counts.split_whitespace().map(|n| n.parse::<u32>().unwrap_or(0));
    let ahead = it.next().unwrap_or(0);
    let behind = it.next().unwrap_or(0);
    let dirty = !git(root, &["status", "--porcelain", "--untracked-files=no"])?.is_empty();
    let (can_update, message) = if behind == 0 {
        (false, "You have the latest version.".to_string())
    } else if dirty {
        (false, "Files in the studio folder were edited by hand. Commit or undo them, then update.".to_string())
    } else if ahead > 0 {
        (false, "This copy has commits that are not on GitHub. Push or reset them, then update.".to_string())
    } else {
        (true, format!("{behind} update{} ready.", if behind == 1 { "" } else { "s" }))
    };
    Ok(Status { branch, local, remote, behind, ahead, dirty, can_update, message, build: build_commit() })
}

/// How the running app was built, so the rebuild matches it.
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Debug,
    Release,
    /// Installed app: on macOS the .app bundle that is running, on Windows
    /// the installed brewixgames.exe.
    Bundle(PathBuf),
}

pub fn mode() -> Mode {
    let exe = std::env::current_exe().unwrap_or_default();
    let s = exe.display().to_string();
    if let Some(i) = s.find(".app/Contents/MacOS") {
        return Mode::Bundle(PathBuf::from(&s[..i + 4]));
    }
    if cfg!(windows) && !s.contains("\\target\\") && exe.with_file_name("studio-template.tar.gz").is_file() {
        return Mode::Bundle(exe);
    }
    if s.contains("/target/release/") {
        Mode::Release
    } else {
        Mode::Debug
    }
}

fn stream(mut cmd: Command, on_line: &mut dyn FnMut(&str)) -> Result<(), String> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("could not start {:?}: {e}", cmd.get_program()))?;
    let err = child.stderr.take().unwrap();
    let t = std::thread::spawn(move || BufReader::new(err).lines().map_while(Result::ok).collect::<Vec<_>>());
    for l in BufReader::new(child.stdout.take().unwrap()).lines().map_while(Result::ok) {
        on_line(&l);
    }
    let errs = t.join().unwrap_or_default();
    for l in &errs {
        on_line(l);
    }
    let st = child.wait().map_err(|e| e.to_string())?;
    if !st.success() {
        return Err(format!("{:?} failed: {}", cmd.get_program(), errs.iter().rev().take(5).cloned().collect::<Vec<_>>().join(" | ")));
    }
    Ok(())
}

/// Pull and rebuild. Returns the path of the rebuilt app to restart into.
pub fn apply(root: &Path, on_line: &mut dyn FnMut(&str)) -> Result<PathBuf, String> {
    let st = check(root)?;
    if !st.can_update {
        return Err(st.message);
    }
    let busy: Vec<String> = crate::runs::list(root)?
        .into_iter()
        .filter(|r| r.status == "running" || r.status == "approved")
        .map(|r| r.run_id)
        .collect();
    if !busy.is_empty() {
        return Err(format!("An agent run is in progress ({}). Update after it finishes.", busy.join(", ")));
    }
    if !is_git_checkout(root) {
        return apply_release(root, on_line);
    }
    on_line(&format!("Downloading {} update(s) from origin/{}...", st.behind, st.branch));
    let out = git(root, &["pull", "--ff-only", "origin", &st.branch])?;
    on_line(&out);
    let app_dir = root.join("app");
    let tauri_dir = app_dir.join("src-tauri");
    match mode() {
        Mode::Debug | Mode::Release => {
            let release = mode() == Mode::Release;
            on_line(&format!("Rebuilding ({})...", if release { "release" } else { "debug" }));
            let mut c = crate::paths::command("cargo");
            c.current_dir(&tauri_dir).arg("build").arg("--bins");
            if release {
                c.arg("--release");
            }
            stream(c, on_line)?;
            Ok(std::env::current_exe().map_err(|e| e.to_string())?)
        }
        Mode::Bundle(running) => {
            on_line("Rebuilding the app bundle (this takes a few minutes)...");
            let mut c = crate::paths::command("npm");
            c.current_dir(&app_dir).args(["run", "build", "--", "--bundles", "app"]);
            stream(c, on_line)?;
            let built = tauri_dir.join("target/release/bundle/macos/BrewixGames.app");
            if built != running {
                on_line(&format!("Replacing {}", running.display()));
                let mut d = crate::paths::command("ditto");
                d.arg(&built).arg(&running);
                stream(d, on_line)?;
            }
            Ok(running.join("Contents/MacOS/brewixgames"))
        }
    }
}

/// Downloaded copies: fetch the newest app from the public releases page,
/// replace this app, refresh the vault (with a backup), and hand back the
/// new binary to restart into.
fn apply_release(root: &Path, on_line: &mut dyn FnMut(&str)) -> Result<PathBuf, String> {
    let Mode::Bundle(running) = mode() else {
        return Err("This copy is not an installed app; update it from GitHub instead.".into());
    };
    let (tag, url) = latest_release()?;
    if release_tag() == Some(tag.as_str()) {
        return Err(format!("You already have the newest version ({tag})."));
    }
    on_line(&format!("Downloading BrewixGames {tag}..."));
    let tmp = std::env::temp_dir().join(format!("brewix-update-{tag}"));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
    let zip = tmp.join(RELEASE_ASSET);
    let resp = ureq::get(&url)
        .set("User-Agent", "BrewixGames")
        .timeout(std::time::Duration::from_secs(600))
        .call()
        .map_err(|e| format!("download failed: {e}"))?;
    let mut f = std::fs::File::create(&zip).map_err(|e| e.to_string())?;
    std::io::copy(&mut resp.into_reader(), &mut f).map_err(|e| e.to_string())?;
    drop(f);
    if cfg!(windows) {
        // The installer replaces the running app, so hand over to it: run it
        // in passive mode (progress bar, no questions), then reopen the app.
        // The vault refresh happens on that next launch (sync_release).
        on_line("Installing. BrewixGames will close and reopen by itself.");
        crate::paths::command("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-EncodedCommand", &handoff_script(&zip, &running)])
            .spawn()
            .map_err(|e| format!("could not start the installer: {e}"))?;
        std::process::exit(0);
    }
    on_line("Unpacking...");
    let mut x = crate::paths::command("ditto");
    x.args(["-x", "-k"]).arg(&zip).arg(&tmp);
    stream(x, on_line)?;
    let new_app = tmp.join("BrewixGames.app");
    if !new_app.join("Contents/MacOS/brewixgames").is_file() {
        return Err("the download did not contain BrewixGames.app".into());
    }
    on_line(&format!("Replacing {}", running.display()));
    let mut d = crate::paths::command("ditto");
    d.arg(&new_app).arg(&running);
    stream(d, on_line)?;
    let _ = root; // the vault refresh runs on the next launch (studio::sync_release)
    let _ = std::fs::remove_dir_all(&tmp);
    Ok(running.join("Contents/MacOS/brewixgames"))
}

/// PowerShell that waits for the installer (passive: progress bar only),
/// then reopens the app. Sent as -EncodedCommand (base64 of UTF-16LE) so no
/// quoting layer can mangle the paths; a cmd.exe /C line did exactly that.
fn handoff_script(setup: &Path, app: &Path) -> String {
    use base64::Engine;
    let q = |p: &Path| p.display().to_string().replace('\'', "''");
    let script = format!(
        "Start-Sleep -Seconds 1; Start-Process -FilePath '{}' -ArgumentList '/P' -Wait; Start-Process -FilePath '{}'",
        q(setup),
        q(app)
    );
    let utf16: Vec<u8> = script.encode_utf16().flat_map(|u| u.to_le_bytes()).collect();
    base64::engine::general_purpose::STANDARD.encode(utf16)
}

#[cfg(test)]
mod handoff_tests {
    use super::*;

    #[test]
    fn handoff_survives_spaces_and_quotes() {
        use base64::Engine;
        let enc = handoff_script(Path::new("C:\\Users\\Al O'Neil\\Temp\\setup.exe"), Path::new("C:\\Users\\Al O'Neil\\AppData\\Local\\BrewixGames\\brewixgames.exe"));
        let bytes = base64::engine::general_purpose::STANDARD.decode(enc).unwrap();
        let units: Vec<u16> = bytes.chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let text = String::from_utf16(&units).unwrap();
        assert!(text.contains("-FilePath 'C:\\Users\\Al O''Neil\\Temp\\setup.exe' -ArgumentList '/P' -Wait"), "{text}");
        assert!(text.ends_with("BrewixGames\\brewixgames.exe'"), "{text}");
    }
}
