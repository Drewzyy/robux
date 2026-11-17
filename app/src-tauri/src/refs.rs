//! Game References: pictures of popular games (UI, characters, worlds) for
//! Andrew and the agents to study. The list of games and what to learn from
//! each is studio/reference-games.json; the pictures are downloaded from
//! public sources on this computer into vault/attachments/game-refs/<id>/:
//! - Roblox: the game's icon and its thumbnails (public thumbnails API)
//! - Steam: header and screenshots (public store API)
//! - App Store: icon and screenshots (public iTunes lookup)
//! - Wikipedia: named images from the game's article
//! No account or key. The pictures are other companies' art: kept on this
//! computer for study, never committed, published or copied.

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

const UA: &str = "BrewixGames/0.1 (game reference downloader; https://github.com/Drewzyy/robux)";

pub fn catalog(root: &Path) -> Result<Value, String> {
    let t = fs::read_to_string(root.join("studio/reference-games.json")).map_err(|e| format!("reference-games.json: {e}"))?;
    serde_json::from_str(&t).map_err(|e| e.to_string())
}

pub fn dir(root: &Path) -> PathBuf {
    root.join("vault/attachments/game-refs")
}

fn get_json(url: &str) -> Result<Value, String> {
    let mut last = String::new();
    for i in 0..3 {
        match ureq::get(url).set("User-Agent", UA).timeout(Duration::from_secs(40)).call() {
            Ok(r) => return r.into_json().map_err(|e| e.to_string()),
            Err(ureq::Error::Status(429, _)) => {
                last = "the site asked us to slow down".into();
                std::thread::sleep(Duration::from_secs(5 * (i + 1)));
            }
            Err(e) => {
                last = e.to_string();
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
    Err(last)
}

fn get_bytes(url: &str) -> Result<Vec<u8>, String> {
    let r = ureq::get(url).set("User-Agent", UA).timeout(Duration::from_secs(90)).call().map_err(|e| e.to_string())?;
    let mut b = vec![];
    std::io::Read::read_to_end(&mut r.into_reader().take(15_000_000), &mut b).map_err(|e| e.to_string())?;
    Ok(b)
}
use std::io::Read as _;

/// Picture URLs for one game, best first, with a label for each.
pub fn picture_urls(source: &Value, max: usize) -> Result<Vec<(String, String)>, String> {
    picture_urls_with(source, max, &mut get_json)
}

/// Same, with the web requests supplied (tests use saved replies).
pub fn picture_urls_with(source: &Value, max: usize, get_json: &mut dyn FnMut(&str) -> Result<Value, String>) -> Result<Vec<(String, String)>, String> {
    let mut out: Vec<(String, String)> = vec![];
    if let Some(place) = source["roblox"].as_u64() {
        let u = get_json(&format!("https://apis.roblox.com/universes/v1/places/{place}/universe"))?["universeId"].as_u64().ok_or("Roblox did not find this game")?;
        let icon = get_json(&format!("https://thumbnails.roblox.com/v1/games/icons?universeIds={u}&size=512x512&format=Png&isCircular=false"))?;
        if let Some(url) = icon["data"][0]["imageUrl"].as_str() {
            out.push(("icon".into(), url.into()));
        }
        let th = get_json(&format!("https://thumbnails.roblox.com/v1/games/multiget/thumbnails?universeIds={u}&countPerUniverse={max}&defaults=false&size=768x432&format=Png&isCircular=false"))?;
        for t in th["data"][0]["thumbnails"].as_array().into_iter().flatten() {
            if t["state"] == "Completed" {
                if let Some(url) = t["imageUrl"].as_str() {
                    out.push(("thumbnail".into(), url.into()));
                }
            }
        }
    } else if let Some(app) = source["steam"].as_u64() {
        let v = get_json(&format!("https://store.steampowered.com/api/appdetails?appids={app}&filters=basic,screenshots"))?;
        let d = &v[app.to_string()]["data"];
        if let Some(h) = d["header_image"].as_str() {
            out.push(("header".into(), h.into()));
        }
        for s in d["screenshots"].as_array().into_iter().flatten() {
            if let Some(u) = s["path_full"].as_str() {
                out.push(("screenshot".into(), u.into()));
            }
        }
    } else if let Some(id) = source["ios"].as_u64() {
        let v = get_json(&format!("https://itunes.apple.com/lookup?id={id}&country=us"))?;
        let r = &v["results"][0];
        if let Some(a) = r["artworkUrl512"].as_str() {
            out.push(("icon".into(), a.into()));
        }
        for key in ["screenshotUrls", "ipadScreenshotUrls"] {
            for s in r[key].as_array().into_iter().flatten() {
                if let Some(u) = s.as_str() {
                    out.push(("screenshot".into(), u.into()));
                }
            }
        }
    } else if let Some(files) = source["wiki"].as_array() {
        let titles: Vec<&str> = files.iter().filter_map(|f| f.as_str()).collect();
        let q = titles.iter().map(|t| urlencode(t)).collect::<Vec<_>>().join("%7C");
        let v = get_json(&format!("https://en.wikipedia.org/w/api.php?action=query&format=json&prop=imageinfo&iiprop=url&iiurlwidth=1280&titles={q}"))?;
        // Keep the order the catalog gives.
        let pages: Vec<Value> = v["query"]["pages"].as_object().map(|m| m.values().cloned().collect()).unwrap_or_default();
        for t in titles {
            if let Some(p) = pages.iter().find(|p| p["title"].as_str() == Some(t)) {
                if let Some(u) = p["imageinfo"][0]["thumburl"].as_str().or(p["imageinfo"][0]["url"].as_str()) {
                    out.push(("image".into(), u.into()));
                }
            }
        }
    } else {
        return Err("no picture source".into());
    }
    out.truncate(max);
    Ok(out)
}

fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn ext_for(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG") {
        Some("png")
    } else if bytes.starts_with(&[0xFF, 0xD8]) {
        Some("jpg")
    } else if bytes.len() > 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

/// Pictures already downloaded for a game (vault-relative paths).
pub fn pictures(root: &Path, id: &str) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(dir(root).join(id))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            ["png", "jpg", "webp"].iter().any(|x| n.ends_with(x)).then(|| format!("attachments/game-refs/{id}/{n}"))
        })
        .collect();
    v.sort();
    v
}

/// The catalog with each game's downloaded pictures.
pub fn library(root: &Path) -> Result<Value, String> {
    let mut c = catalog(root)?;
    let mut total = 0;
    if let Some(gs) = c["games"].as_array_mut() {
        for g in gs.iter_mut() {
            let id = g["id"].as_str().unwrap_or("").to_string();
            let p = pictures(root, &id);
            total += p.len();
            g["pictures"] = json!(p);
        }
    }
    c["total_pictures"] = json!(total);
    Ok(c)
}

/// Download pictures for the chosen games (all when empty). Games that
/// already have pictures are skipped unless `refresh`. Never stops on one
/// failed game; reports them at the end.
pub fn download(root: &Path, ids: &[String], per_game: usize, refresh: bool, on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let c = catalog(root)?;
    let games: Vec<&Value> = c["games"].as_array().ok_or("no games in the catalog")?.iter().filter(|g| ids.is_empty() || ids.iter().any(|i| g["id"] == i.as_str())).collect();
    let (mut got, mut games_done, mut failed) = (0, 0, vec![]);
    for (n, g) in games.iter().enumerate() {
        let id = g["id"].as_str().unwrap_or("");
        let name = g["name"].as_str().unwrap_or(id);
        if !crate::paths::valid_id(id) {
            continue;
        }
        let gd = dir(root).join(id);
        if !refresh && !pictures(root, id).is_empty() {
            continue;
        }
        on_line(&format!("{} of {}: {name}", n + 1, games.len()));
        let urls = match picture_urls(&g["source"], per_game) {
            Ok(u) if !u.is_empty() => u,
            Ok(_) => {
                failed.push(format!("{name} (no pictures listed)"));
                continue;
            }
            Err(e) => {
                failed.push(format!("{name} ({e})"));
                continue;
            }
        };
        let _ = fs::create_dir_all(&gd);
        let mut saved = vec![];
        for (i, (label, url)) in urls.iter().enumerate() {
            let Ok(bytes) = get_bytes(url) else { continue };
            let Some(ext) = ext_for(&bytes) else { continue };
            let file = format!("{:02}-{label}.{ext}", i + 1);
            if fs::write(gd.join(&file), &bytes).is_ok() {
                saved.push(json!({ "file": file, "from": url }));
            }
            std::thread::sleep(Duration::from_millis(150));
        }
        if saved.is_empty() {
            failed.push(format!("{name} (downloads failed)"));
            continue;
        }
        got += saved.len();
        games_done += 1;
        let _ = fs::write(gd.join("sources.json"), serde_json::to_string_pretty(&json!({ "game": name, "source": g["source"], "pictures": saved, "at": crate::now_rfc3339(), "note": "Other companies' art, for study on this computer only." })).unwrap_or_default());
        std::thread::sleep(Duration::from_millis(400));
    }
    write_gallery(root)?;
    let mut msg = format!("Downloaded {got} pictures for {games_done} games.");
    if !failed.is_empty() {
        msg.push_str(&format!(" Could not get {}: {}.", failed.len(), failed.join("; ")));
    }
    Ok(msg)
}

/// A gallery note Obsidian shows with every downloaded picture.
pub fn write_gallery(root: &Path) -> Result<(), String> {
    let lib = library(root)?;
    let mut s = String::from("---\ndescription: Every downloaded game reference picture, grouped like the visual-refs notes. Made by the app; do not edit.\ntype: generated\nstatus: draft\nlast_checked: ");
    s.push_str(&crate::now_rfc3339()[..10]);
    s.push_str("\nverify: Rewritten by BrewixGames after each download.\n---\n# Game reference pictures\n\nOther companies' art, for study only. What to learn from each game is in the visual-refs notes in 16-Market-Research.\n");
    for grp in lib["groups"].as_array().into_iter().flatten() {
        let gid = grp["id"].as_str().unwrap_or("");
        let games: Vec<&Value> = lib["games"].as_array().into_iter().flatten().filter(|g| g["group"] == gid && g["pictures"].as_array().map(|p| !p.is_empty()).unwrap_or(false)).collect();
        if games.is_empty() {
            continue;
        }
        s.push_str(&format!("\n## {}\n", grp["title"].as_str().unwrap_or(gid)));
        for g in games {
            s.push_str(&format!("\n### {}\n\n{}\n\n", g["name"].as_str().unwrap_or(""), g["learn"]["steal"].as_str().unwrap_or("")));
            for p in g["pictures"].as_array().into_iter().flatten().filter_map(|p| p.as_str()) {
                // Relative markdown images (not wikilinks) so the note stays valid without pictures.
                s.push_str(&format!("![]({})\n", p.trim_start_matches("attachments/game-refs/")));
            }
        }
    }
    fs::create_dir_all(dir(root)).map_err(|e| e.to_string())?;
    fs::write(dir(root).join("gallery.md"), s).map_err(|e| e.to_string())
}

/// One picture as a data URL for the app.
pub fn picture(root: &Path, rel: &str) -> Result<String, String> {
    if !rel.starts_with("attachments/game-refs/") || !crate::paths::safe_rel(rel) {
        return Err("not a reference picture".into());
    }
    let bytes = fs::read(root.join("vault").join(rel)).map_err(|e| e.to_string())?;
    let mime = match ext_for(&bytes) {
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        _ => "image/jpeg",
    };
    use base64::Engine;
    Ok(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_is_valid_and_ids_are_safe() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let c = catalog(&root).unwrap();
        let games = c["games"].as_array().unwrap();
        assert!(games.len() >= 150, "{}", games.len());
        let groups: Vec<&str> = c["groups"].as_array().unwrap().iter().map(|g| g["id"].as_str().unwrap()).collect();
        let mut seen = std::collections::HashSet::new();
        for g in games {
            let id = g["id"].as_str().unwrap();
            assert!(crate::paths::valid_id(id) && seen.insert(id), "{id}");
            assert!(groups.contains(&g["group"].as_str().unwrap()), "{id}");
            let s = &g["source"];
            assert!(s["roblox"].is_u64() || s["steam"].is_u64() || s["ios"].is_u64() || s["wiki"].is_array(), "{id}");
            for k in ["ui", "characters", "world", "steal"] {
                assert!(g["learn"][k].as_str().map(|t| t.len() > 5).unwrap_or(false), "{id} {k}");
            }
            // Each group has a vault note.
            assert!(root.join(format!("vault/16-Market-Research/visual-refs-{}.md", g["group"].as_str().unwrap())).is_file());
        }
    }
    #[test]
    fn reads_real_replies_from_each_source() {
        let d = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/testdata/refs");
        let mut fake = |url: &str| -> Result<Value, String> {
            let f = if url.contains("/universe") { "roblox-universe" } else if url.contains("games/icons") { "roblox-icon" } else if url.contains("multiget") { "roblox-thumbs" } else if url.contains("steampowered") { "steam" } else if url.contains("itunes") { "ios" } else { "wiki" };
            Ok(serde_json::from_str(&fs::read_to_string(d.join(format!("{f}.json"))).unwrap()).unwrap())
        };
        let r = picture_urls_with(&json!({"roblox": 920587237u64}), 6, &mut fake).unwrap();
        assert_eq!(r[0].0, "icon");
        assert!(r.len() >= 3 && r.iter().all(|(_, u)| u.starts_with("https://tr.rbxcdn.com/")), "{r:?}");
        let s = picture_urls_with(&json!({"steam": 1145360u64}), 3, &mut fake).unwrap();
        assert_eq!(s.len(), 3);
        assert!(s[0].1.contains("header") && s[1].0 == "screenshot", "{s:?}");
        let i = picture_urls_with(&json!({"ios": 1229016807u64}), 5, &mut fake).unwrap();
        assert!(i[0].0 == "icon" && i.len() == 5 && i.iter().all(|(_, u)| u.contains("mzstatic.com")), "{i:?}");
        let w = picture_urls_with(&json!({"wiki": ["File:Splatoon 3 Turf War.jpg", "File:Splatoon 3 Salmon Run.jpg", "File:Nope.jpg"]}), 5, &mut fake).unwrap();
        assert_eq!(w.iter().map(|x| x.1.as_str()).collect::<Vec<_>>(), vec!["https://upload.wikimedia.org/wikipedia/en/b/bb/Splatoon_3_Turf_War.jpg", "https://upload.wikimedia.org/wikipedia/en/thumb/a/aa/Splatoon_3_Salmon_Run.jpg/1280px-Splatoon_3_Salmon_Run.jpg"], "catalog order, thumb preferred, missing skipped");
    }
    #[test]
    fn sniffs_images_and_refuses_other_paths() {
        assert_eq!(ext_for(b"\x89PNG\r\n"), Some("png"));
        assert_eq!(ext_for(&[0xFF, 0xD8, 0xFF]), Some("jpg"));
        assert_eq!(ext_for(b"<html>"), None);
        let root = std::env::temp_dir();
        assert!(picture(&root, "attachments/game-refs/../../secret.png").is_err());
        assert!(picture(&root, "projects/x.png").is_err());
        assert_eq!(urlencode("File:A b.jpg"), "File%3AA%20b.jpg");
    }
}
