//! Blender and Obsidian installed inside the Downloads folder (Settings), so
//! BrewixGames uses only copies it can see and nothing else lands on C:.
//!
//! - Blender: the official portable zip (5.2 LTS series) from
//!   download.blender.org, plus an empty `portable` folder next to the exe,
//!   which Blender's manual says keeps preferences and add-ons beside it. The
//!   MCP for Blender add-on is copied in and enabled headlessly.
//! - Obsidian: the official installer from obsidianmd/obsidian-releases, run
//!   silently with NSIS's /D= folder switch; the result is checked, because
//!   Obsidian does not document /D=.
//! Windows only for installs; on macOS people install these the usual way.

use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const BLENDER_SERIES: &str = "5.2";

#[derive(Serialize, Debug, Default)]
pub struct Status {
    pub dirs: std::collections::BTreeMap<String, String>,
    pub blender: Option<String>,
    pub obsidian: Option<String>,
    /// Git Bash (Windows): Claude Code needs it.
    pub git: Option<String>,
    /// Graphify (code map for agents), if installed.
    pub graphify: Option<String>,
    pub can_install: bool,
    pub folder: String,
}

fn blender_dir() -> PathBuf {
    crate::paths::app_dir("blender")
}
fn obsidian_dir() -> PathBuf {
    crate::paths::app_dir("obsidian")
}

/// blender.exe inside the Downloads folder, if installed.
pub fn blender_exe() -> Option<PathBuf> {
    let rd = std::fs::read_dir(blender_dir()).ok()?;
    let mut found: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path().join(if cfg!(windows) { "blender.exe" } else { "blender" }))
        .filter(|p| p.is_file())
        .collect();
    found.sort();
    found.pop()
}

pub fn obsidian_exe() -> Option<PathBuf> {
    let p = obsidian_dir().join("Obsidian.exe");
    p.is_file().then_some(p)
}

pub fn status() -> Status {
    Status {
        dirs: ["claude", "blender", "obsidian", "git", "graphify"].iter().map(|a| (a.to_string(), crate::paths::app_dir(a).display().to_string())).collect(),
        blender: blender_exe().map(|p| p.display().to_string()),
        obsidian: obsidian_exe().map(|p| p.display().to_string()),
        graphify: graphify_bin().map(|p| p.display().to_string()),
        git: crate::claude::git_bash().filter(|_| cfg!(windows)).map(|p| p.display().to_string()),
        can_install: cfg!(windows),
        folder: crate::paths::tools_dir().display().to_string(),
    }
}

/// Newest point release of a Blender series from the official listing page,
/// e.g. ("5.2.3", "https://download.blender.org/release/Blender5.2/blender-5.2.3-windows-x64.zip").
fn latest_blender() -> Result<(String, String), String> {
    let base = format!("https://download.blender.org/release/Blender{BLENDER_SERIES}/");
    let page = ureq::get(&base)
        .set("User-Agent", "BrewixGames")
        .timeout(std::time::Duration::from_secs(30))
        .call()
        .map_err(|e| format!("could not reach download.blender.org: {e}"))?
        .into_string()
        .map_err(|e| e.to_string())?;
    pick_blender(&page).map(|v| (v.clone(), format!("{base}blender-{v}-windows-x64.zip"))).ok_or_else(|| format!("no Windows zip found for Blender {BLENDER_SERIES}"))
}

fn pick_blender(listing: &str) -> Option<String> {
    let re = regex::Regex::new(&format!(r"blender-({}\.(\d+))-windows-x64\.zip", regex::escape(BLENDER_SERIES))).unwrap();
    re.captures_iter(listing).max_by_key(|c| c[2].parse::<u32>().unwrap_or(0)).map(|c| c[1].to_string())
}

/// Latest Obsidian Windows x64 installer from the official releases repo.
fn latest_obsidian() -> Result<(String, String), String> {
    let v: serde_json::Value = ureq::get("https://api.github.com/repos/obsidianmd/obsidian-releases/releases/latest")
        .set("User-Agent", "BrewixGames")
        .timeout(std::time::Duration::from_secs(30))
        .call()
        .map_err(|e| format!("could not reach GitHub: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;
    let tag = v.get("tag_name").and_then(|t| t.as_str()).unwrap_or("").to_string();
    let re = regex::Regex::new(r"^Obsidian-\d+\.\d+\.\d+\.exe$").unwrap();
    let url = v
        .get("assets")
        .and_then(|a| a.as_array())
        .and_then(|a| a.iter().find(|x| x.get("name").and_then(|n| n.as_str()).map(|n| re.is_match(n)).unwrap_or(false)))
        .and_then(|x| x.get("browser_download_url").and_then(|u| u.as_str()))
        .ok_or("the latest Obsidian release has no Windows installer")?
        .to_string();
    Ok((tag, url))
}

/// Download a file, resuming where it stopped after a dropped connection
/// (up to 5 tries; 2 minutes without data counts as dropped), and check
/// the size before using it.
pub(crate) fn download(url: &str, dest: &Path, on_line: &mut dyn FnMut(&str)) -> Result<(), String> {
    std::fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    let tmp = dest.with_extension("part");
    let _ = std::fs::remove_file(&tmp);
    let agent = ureq::AgentBuilder::new().timeout_connect(std::time::Duration::from_secs(20)).timeout_read(std::time::Duration::from_secs(120)).build();
    let mut total: u64 = 0;
    let mut last_err = String::new();
    for attempt in 0..5 {
        if attempt > 0 {
            on_line(&format!("Connection dropped; resuming (try {} of 5)", attempt + 1));
            std::thread::sleep(std::time::Duration::from_secs(3 * attempt as u64));
        }
        let have = std::fs::metadata(&tmp).map(|m| m.len()).unwrap_or(0);
        let mut req = agent.get(url).set("User-Agent", "BrewixGames");
        if have > 0 {
            req = req.set("Range", &format!("bytes={have}-"));
        }
        let resp = match req.call() {
            Ok(r) => r,
            Err(e) => {
                last_err = e.to_string();
                continue;
            }
        };
        let resumed = have > 0 && resp.status() == 206;
        let len: u64 = resp.header("Content-Length").and_then(|h| h.parse().ok()).unwrap_or(0);
        if total == 0 || !resumed {
            total = if resumed { have + len } else { len };
        }
        let mut f = match std::fs::OpenOptions::new().create(true).write(true).append(resumed).truncate(!resumed).open(&tmp) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        let mut got = if resumed { have } else { 0 };
        let (mut buf, mut last) = (vec![0u8; 1 << 16], got);
        let mut r = resp.into_reader();
        let finished = loop {
            match r.read(&mut buf) {
                Ok(0) => break true,
                Ok(n) => {
                    if let Err(e) = std::io::Write::write_all(&mut f, &buf[..n]) {
                        return Err(format!("could not save the download (disk full?): {e}"));
                    }
                    got += n as u64;
                    if got - last > 8 << 20 {
                        last = got;
                        on_line(&if total > 0 { format!("Downloading {}%", got * 100 / total) } else { format!("Downloading {} MB", got >> 20) });
                    }
                }
                Err(e) => {
                    last_err = e.to_string();
                    break false;
                }
            }
        };
        drop(f);
        if finished && (total == 0 || got >= total) {
            return std::fs::rename(&tmp, dest).map_err(|e| e.to_string());
        }
        if finished {
            last_err = format!("the download ended early ({} of {} MB)", got >> 20, total >> 20);
        }
    }
    Err(format!("download failed after 5 tries: {last_err}. Check the internet connection and try again; it resumes where it stopped."))
}

pub fn install(app: &str, on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    if app == "graphify" {
        return install_graphify(on_line); // uv works on every platform
    }
    if !cfg!(windows) {
        return Err("Installing into the Downloads folder is for Windows. On a Mac, install it the usual way.".into());
    }
    match app {
        "blender" => install_blender(on_line),
        "obsidian" => install_obsidian(on_line),
        "git" => install_git(on_line),
        "graphify" => install_graphify(on_line),
        _ => Err(format!("unknown app {app}")),
    }
}

fn install_blender(on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let (ver, url) = latest_blender()?;
    on_line(&format!("Blender {ver} (portable, about 400 MB)"));
    let zip = crate::paths::tools_dir().join("downloads").join(format!("blender-{ver}-windows-x64.zip"));
    if !zip.is_file() {
        download(&url, &zip, on_line)?;
    }
    on_line("Unpacking...");
    let dir = blender_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let st = crate::paths::command("tar").arg("-xf").arg(&zip).arg("-C").arg(&dir).status().map_err(|e| format!("tar: {e}"))?;
    if !st.success() {
        return Err("could not unpack the Blender zip".into());
    }
    let exe = blender_exe().ok_or("unpacked, but blender.exe was not found")?;
    let home = exe.parent().unwrap();
    // Portable mode: settings and add-ons live beside blender.exe.
    let addons = home.join("portable").join("scripts").join("addons");
    std::fs::create_dir_all(&addons).map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&zip);
    let mut msg = format!("Blender {ver} is in {} (portable: its settings stay in that folder).", home.display());
    match crate::blender_mcp::fetch_addon(Path::new(".")) {
        Ok(addon) => {
            std::fs::copy(&addon, addons.join("mcp_for_blender_addon.py")).map_err(|e| e.to_string())?;
            on_line("Turning on the MCP for Blender add-on...");
            let enabled = crate::paths::command(&exe)
                .args(["--background", "--factory-startup", "--python-expr", "import bpy; bpy.ops.preferences.addon_enable(module='mcp_for_blender_addon'); bpy.ops.wm.save_userpref()"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            msg.push_str(if enabled { " The MCP for Blender add-on is installed and on." } else { " The add-on was copied in; turn it on in Edit, Preferences, Add-ons." });
        }
        Err(e) => msg.push_str(&format!(" The add-on was not added ({e}); press Get the Blender Add-on after installing uv.")),
    }
    Ok(msg)
}

fn install_obsidian(on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let (tag, url) = latest_obsidian()?;
    on_line(&format!("Obsidian {tag}"));
    let setup = crate::paths::tools_dir().join("downloads").join("obsidian-setup.exe");
    download(&url, &setup, on_line)?;
    on_line("Installing...");
    let dir = obsidian_dir();
    let _ = &dir;
    let mut cmd = crate::paths::command(&setup);
    cmd.arg("/S");
    // NSIS wants /D= last and unquoted, even with spaces.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.raw_arg(format!("/D={}", dir.display()));
    }
    let st = cmd.status().map_err(|e| format!("could not run the Obsidian installer: {e}"))?;
    let _ = std::fs::remove_file(&setup);
    if !st.success() {
        return Err("the Obsidian installer reported a problem".into());
    }
    match obsidian_exe() {
        Some(p) => Ok(format!("Obsidian {tag} is in {}.", p.parent().unwrap().display())),
        None => Err("Obsidian installed, but not in the Downloads folder: its installer ignored the chosen folder and used its usual place.".into()),
    }
}

/// Git for Windows from its official GitHub releases, installed silently into
/// the chosen folder (Inno Setup's /DIR), then handed to Claude Code.
fn install_git(on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let v: serde_json::Value = ureq::get("https://api.github.com/repos/git-for-windows/git/releases/latest")
        .set("User-Agent", "BrewixGames")
        .timeout(std::time::Duration::from_secs(30))
        .call()
        .map_err(|e| format!("could not reach GitHub: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())?;
    let arch = if cfg!(target_arch = "aarch64") { "arm64" } else { "64-bit" };
    let re = regex::Regex::new(&format!(r"^Git-[\d.]+-{arch}\.exe$")).unwrap();
    let url = v["assets"].as_array().and_then(|a| a.iter().find(|x| x["name"].as_str().map(|n| re.is_match(n)).unwrap_or(false))).and_then(|x| x["browser_download_url"].as_str()).ok_or("the latest Git for Windows release has no installer for this PC")?.to_string();
    on_line(&format!("Git for Windows {}", v["tag_name"].as_str().unwrap_or("")));
    let setup = crate::paths::tools_dir().join("downloads").join("git-setup.exe");
    download(&url, &setup, on_line)?;
    on_line("Installing...");
    let dir = crate::paths::app_dir("git");
    let _ = &dir;
    let mut cmd = crate::paths::command(&setup);
    cmd.args(["/VERYSILENT", "/NORESTART", "/NOCANCEL", "/SP-", "/SUPPRESSMSGBOXES", "/CURRENTUSER"]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.raw_arg(format!("/DIR=\"{}\"", dir.display()));
    }
    let st = cmd.status().map_err(|e| format!("could not run the Git installer: {e}"))?;
    let _ = std::fs::remove_file(&setup);
    if !st.success() {
        return Err("the Git installer reported a problem".into());
    }
    crate::claude::export_git_bash();
    match crate::claude::git_bash() {
        Some(p) => Ok(format!("Git is installed ({}). Claude Code is connected to it.", p.display())),
        None => Err("Git installed, but Git Bash was not found in the chosen folder. Press Check Again, or choose the folder Git went to.".into()),
    }
}

/// First time the vault opens in Obsidian, open it on the Graph view (the
/// "second brain" picture: notes as dots, links as lines), with the file
/// list on the left. Obsidian keeps its own layout after that.
pub fn graph_first(vault: &Path) {
    let ws = vault.join(".obsidian/workspace.json");
    if ws.is_file() {
        return;
    }
    let layout = serde_json::json!({
        "main": {"id": "bg-main", "type": "split", "direction": "vertical", "children": [
            {"id": "bg-tabs", "type": "tabs", "children": [
                {"id": "bg-graph", "type": "leaf", "state": {"type": "graph", "state": {}}}
            ]}
        ]},
        "left": {"id": "bg-left", "type": "split", "direction": "horizontal", "width": 280, "children": [
            {"id": "bg-left-tabs", "type": "tabs", "children": [
                {"id": "bg-files", "type": "leaf", "state": {"type": "file-explorer", "state": {"sortOrder": "alphabetical"}}}
            ]}
        ]},
        "right": {"id": "bg-right", "type": "split", "direction": "horizontal", "width": 300, "collapsed": true, "children": []},
        "active": "bg-graph",
        "lastOpenFiles": []
    });
    let _ = std::fs::create_dir_all(ws.parent().unwrap_or(vault));
    let _ = std::fs::write(ws, serde_json::to_string_pretty(&layout).unwrap_or_default());
}

/// Graphify's own folder for its program (Apache-2.0, PyPI "graphifyy").
pub fn graphify_bin_dir() -> PathBuf {
    crate::paths::app_dir("graphify").join("bin")
}

pub fn graphify_bin() -> Option<PathBuf> {
    let p = graphify_bin_dir().join(if cfg!(windows) { "graphify.exe" } else { "graphify" });
    p.is_file().then_some(p)
}

/// Install Graphify with uv into its own folder. It never runs
/// `graphify install` (that would change Claude Code's own settings).
fn install_graphify(on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let uvx = crate::blender_mcp::uvx().ok_or("Graphify installs with uv, which is not installed yet. Press Get uv in Settings (Blender section) first.")?;
    let uv = uvx.with_file_name(if cfg!(windows) { "uv.exe" } else { "uv" });
    let dir = crate::paths::app_dir("graphify");
    on_line("Installing Graphify (code map for the agents)...");
    let out = crate::paths::command(&uv)
        .args(["tool", "install", "--force", "graphifyy"])
        .env("UV_TOOL_DIR", dir.join("tools"))
        .env("UV_TOOL_BIN_DIR", graphify_bin_dir())
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|e| format!("could not run uv: {e}"))?;
    match graphify_bin() {
        Some(p) => Ok(format!("Graphify is installed ({}). Builder, Tuner and Playtest can map your game's code with it.", p.display())),
        None => Err(format!("Graphify did not install: {}", String::from_utf8_lossy(&out.stderr).lines().last().unwrap_or(""))),
    }
}

/// Open Blender, or the studio's vault in Obsidian, from the Downloads folder.
pub fn open(app: &str, vault: &Path) -> Result<(), String> {
    match app {
        "blender" => {
            let exe = blender_exe().ok_or("Blender is not in the Downloads folder yet. Press Install Here.")?;
            crate::paths::command(&exe).spawn().map(|_| ()).map_err(|e| e.to_string())
        }
        "obsidian" => {
            graph_first(vault);
            let uri = format!("obsidian://open?path={}", urlencode(&vault.display().to_string()));
            match obsidian_exe() {
                Some(exe) => crate::paths::command(&exe).arg(uri).spawn().map(|_| ()).map_err(|e| e.to_string()),
                None => {
                    crate::paths::open_url(&uri);
                    Ok(())
                }
            }
        }
        _ => Err(format!("unknown app {app}")),
    }
}

fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_newest_point_release() {
        let page = r#"<a href="blender-5.2.0-windows-x64.zip">x</a> <a href="blender-5.2.10-windows-x64.zip">x</a>
            <a href="blender-5.2.3-windows-x64.zip">x</a> <a href="blender-5.2.11-windows-arm64.zip">x</a> <a href="blender-5.2.12-macos-arm64.dmg">x</a>"#;
        assert_eq!(pick_blender(page).as_deref(), Some("5.2.10"));
        assert_eq!(pick_blender("nothing here"), None);
    }

    #[test]
    fn vault_uri_is_encoded() {
        assert_eq!(urlencode("D:\\Brewix Games\\vault"), "D%3A%5CBrewix%20Games%5Cvault");
    }
}
