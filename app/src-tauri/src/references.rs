//! Reference board: images and video frames Andrew adds for a game. Files go
//! to vault/19-Project-Memory/references/<game>/ (Obsidian shows them) and a
//! copy to projects/<game>/references/. The note
//! 19-Project-Memory/<game>-references.md embeds each one with what to take
//! from it; agents read the note and open the images.

use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const CATEGORIES: &[&str] = &["Character", "Model", "Environment", "UI", "Animation", "Effects", "Sound and music", "Other"];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reference {
    pub file: String,
    pub category: String,
    pub note: String,
    pub source: String,
    pub added: String,
}

fn game_of(project: &str) -> Result<String, String> {
    let g = project.trim_start_matches("projects/");
    if g.is_empty() || !g.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err("choose a game first".into());
    }
    Ok(g.to_string())
}

fn dir(root: &Path, game: &str) -> PathBuf {
    root.join("vault/19-Project-Memory/references").join(game)
}

pub fn list(root: &Path, project: &str) -> Result<Vec<Reference>, String> {
    let game = game_of(project)?;
    Ok(fs::read_to_string(dir(root, &game).join("index.json")).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default())
}

/// Add one image (PNG, JPEG, or WebP bytes as base64).
pub fn add(root: &Path, project: &str, name: &str, data_b64: &str, category: &str, note: &str, source: &str) -> Result<Reference, String> {
    let game = game_of(project)?;
    if !root.join("projects").join(&game).is_dir() {
        return Err(format!("no game folder projects/{game}"));
    }
    let b64 = data_b64.split(',').next_back().unwrap_or("");
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64.trim()).map_err(|_| "that is not an image".to_string())?;
    if bytes.len() > 15 << 20 {
        return Err("image is over 15 MB".into());
    }
    let ext = if bytes.starts_with(b"\x89PNG") {
        "png"
    } else if bytes.starts_with(&[0xFF, 0xD8]) {
        "jpg"
    } else if bytes.len() > 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        "webp"
    } else {
        return Err("only PNG, JPG, or WebP images (videos are added as frames)".into());
    };
    let category = if CATEGORIES.contains(&category) { category } else { "Other" };
    let stem: String = Path::new(name).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
        .chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).take(40).collect();
    let stamp = chrono::Utc::now().format("%Y%m%d-%H%M%S%3f");
    let file = format!("{stamp}-{}.{ext}", if stem.is_empty() { "reference".into() } else { stem });
    let d = dir(root, &game);
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    fs::write(d.join(&file), &bytes).map_err(|e| e.to_string())?;
    let pd = root.join("projects").join(&game).join("references");
    fs::create_dir_all(&pd).map_err(|e| e.to_string())?;
    fs::write(pd.join(&file), &bytes).map_err(|e| e.to_string())?;
    let r = Reference { file, category: category.into(), note: note.trim().to_string(), source: source.trim().to_string(), added: crate::now_rfc3339() };
    let mut all = list(root, project)?;
    all.push(r.clone());
    fs::write(d.join("index.json"), serde_json::to_string_pretty(&all).unwrap()).map_err(|e| e.to_string())?;
    write_note(root, &game, &all)?;
    Ok(r)
}

pub fn remove(root: &Path, project: &str, file: &str) -> Result<(), String> {
    let game = game_of(project)?;
    let mut all = list(root, project)?;
    all.retain(|r| r.file != file);
    let d = dir(root, &game);
    if file.contains('/') || file.contains('\\') || file.contains("..") {
        return Err("bad file name".into());
    }
    let _ = fs::remove_file(d.join(file));
    let _ = fs::remove_file(root.join("projects").join(&game).join("references").join(file));
    fs::write(d.join("index.json"), serde_json::to_string_pretty(&all).unwrap()).map_err(|e| e.to_string())?;
    write_note(root, &game, &all)
}

/// Data URL of one stored reference, for the app's grid.
pub fn image(root: &Path, project: &str, file: &str) -> Result<String, String> {
    let game = game_of(project)?;
    if file.contains('/') || file.contains('\\') || file.contains("..") {
        return Err("bad file name".into());
    }
    let bytes = fs::read(dir(root, &game).join(file)).map_err(|e| e.to_string())?;
    let mime = if file.ends_with(".png") { "image/png" } else if file.ends_with(".webp") { "image/webp" } else { "image/jpeg" };
    Ok(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

fn write_note(root: &Path, game: &str, all: &[Reference]) -> Result<(), String> {
    let mem = root.join("vault/19-Project-Memory");
    let path = mem.join(format!("{game}-references.md"));
    let mut t = format!(
        "---\ndescription: Reference images and video frames Andrew chose for {game}, each with what to take from it.\ntype: project-references\nstatus: verified\nlast_checked: {}\nverify: written by the app from the References tab (references.rs); not edited by hand or by agents.\ntags: [project-memory, references]\n---\n# {game}: references\n\nUse these for look, feel, and ideas. Match the feeling; never copy a copyrighted character, logo, or exact design.\n",
        &crate::now_rfc3339()[..10]
    );
    for cat in CATEGORIES {
        let items: Vec<&Reference> = all.iter().filter(|r| r.category == *cat).collect();
        if items.is_empty() {
            continue;
        }
        t.push_str(&format!("\n## {cat}\n"));
        for r in items {
            t.push_str(&format!("\n![[references/{game}/{}]]\n", r.file));
            if !r.note.is_empty() {
                t.push_str(&format!("Use: {}\n", r.note.replace('\n', " ")));
            }
            if !r.source.is_empty() {
                t.push_str(&format!("Source: {}\n", r.source.replace('\n', " ")));
            }
        }
    }
    fs::write(&path, t).map_err(|e| e.to_string())?;
    let idx = mem.join("index.md");
    let link = format!("[[19-Project-Memory/{game}-references]]");
    if let Ok(cur) = fs::read_to_string(&idx) {
        if !cur.contains(&link) {
            let _ = fs::write(&idx, format!("{}\n| {link} | Images and video frames for {game}, with what to take from each. | verified |\n", cur.trim_end()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_lists_notes_and_removes() {
        let root = std::env::temp_dir().join(format!("brewix-refs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("projects/g1")).unwrap();
        fs::create_dir_all(root.join("vault/19-Project-Memory")).unwrap();
        fs::write(root.join("vault/19-Project-Memory/index.md"), "# idx\n").unwrap();
        let png = base64::engine::general_purpose::STANDARD.encode(b"\x89PNG\r\n\x1a\nfake");
        let r = add(&root, "projects/g1", "Hero Pose.png", &format!("data:image/png;base64,{png}"), "Character", "big friendly eyes", "pasted").unwrap();
        assert!(r.file.ends_with("-Hero-Pose.png"));
        assert!(root.join("projects/g1/references").join(&r.file).is_file());
        let note = fs::read_to_string(root.join("vault/19-Project-Memory/g1-references.md")).unwrap();
        assert!(note.contains("## Character") && note.contains(&format!("![[references/g1/{}]]", r.file)) && note.contains("Use: big friendly eyes"));
        assert!(add(&root, "projects/g1", "x.txt", &base64::engine::general_purpose::STANDARD.encode(b"hello"), "UI", "", "").is_err());
        assert!(add(&root, "projects/../x", "a.png", &png, "UI", "", "").is_err());
        assert_eq!(list(&root, "projects/g1").unwrap().len(), 1);
        remove(&root, "projects/g1", &r.file).unwrap();
        assert!(list(&root, "projects/g1").unwrap().is_empty());
        assert_eq!(fs::read_to_string(root.join("vault/19-Project-Memory/index.md")).unwrap().matches("g1-references").count(), 1);
        let _ = fs::remove_dir_all(&root);
    }
}
