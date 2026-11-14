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
    pub blender: Option<String>,
    pub obsidian: Option<String>,
    pub can_install: bool,
    pub folder: String,
}

fn blender_dir() -> PathBuf {
    crate::paths::tools_dir().join("blender-app")
}
fn obsidian_dir() -> PathBuf {
    crate::paths::tools_dir().join("obsidian-app")
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
        blender: blender_exe().map(|p| p.display().to_string()),
        obsidian: obsidian_exe().map(|p| p.display().to_string()),
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

fn download(url: &str, dest: &Path, on_line: &mut dyn FnMut(&str)) -> Result<(), String> {
    std::fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    let resp = ureq::get(url)
        .set("User-Agent", "BrewixGames")
        .timeout(std::time::Duration::from_secs(3600))
        .call()
        .map_err(|e| format!("download failed: {e}"))?;
    let total: u64 = resp.header("Content-Length").and_then(|h| h.parse().ok()).unwrap_or(0);
    let mut r = resp.into_reader();
    let tmp = dest.with_extension("part");
    let mut f = std::fs::File::create(&tmp).map_err(|e| e.to_string())?;
    let (mut got, mut buf, mut last) = (0u64, vec![0u8; 1 << 16], 0u64);
    loop {
        let n = r.read(&mut buf).map_err(|e| format!("download interrupted: {e}"))?;
        if n == 0 {
            break;
        }
        std::io::Write::write_all(&mut f, &buf[..n]).map_err(|e| e.to_string())?;
        got += n as u64;
        if got - last > 8 << 20 {
            last = got;
            on_line(&if total > 0 { format!("Downloading {}%", got * 100 / total) } else { format!("Downloading {} MB", got >> 20) });
        }
    }
    drop(f);
    std::fs::rename(&tmp, dest).map_err(|e| e.to_string())
}

pub fn install(app: &str, on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    if !cfg!(windows) {
        return Err("Installing into the Downloads folder is for Windows. On a Mac, install it the usual way.".into());
    }
    match app {
        "blender" => install_blender(on_line),
        "obsidian" => install_obsidian(on_line),
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

/// Open Blender, or the studio's vault in Obsidian, from the Downloads folder.
pub fn open(app: &str, vault: &Path) -> Result<(), String> {
    match app {
        "blender" => {
            let exe = blender_exe().ok_or("Blender is not in the Downloads folder yet. Press Install Here.")?;
            crate::paths::command(&exe).spawn().map(|_| ()).map_err(|e| e.to_string())
        }
        "obsidian" => {
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
