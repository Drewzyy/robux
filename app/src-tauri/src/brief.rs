//! Game Brief: a checklist Andrew fills in for an idea (his thoughts and
//! reference images per item) so Planner starts from as much as possible.
//! Stored in projects/<slug>/docs/brief.json; images go through the
//! references board, so every agent already knows where to find them.
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

/// (id, group, title, what to write).
pub const ITEMS: &[(&str, &str, &str, &str)] = &[
    ("idea", "The idea", "The idea in one sentence", "What is the game? Who is it for?"),
    ("like", "The idea", "Games it is like", "Name 1 to 3 games and what you want from each. What is your twist?"),
    ("goal", "Game loop", "The player's goal", "What is the player trying to do, short term and long term?"),
    ("loop", "Game loop", "The core loop", "What does the player do over and over? (do, earn, spend, repeat)"),
    ("first-minute", "Game loop", "The first 60 seconds", "What happens the moment they join? What is the first win?"),
    ("return", "Game loop", "Why they come back tomorrow", "Daily rewards, friends, events, things left to collect?"),
    ("world", "Graphics", "The world and map", "Where is it set? Zones, landmarks, size. Add map sketches or photos."),
    ("look", "Graphics", "Art style and colours", "Cartoony, realistic, blocky? Mood, colours, lighting. Add images you like."),
    ("characters", "Graphics", "Characters and NPCs", "Player look, enemies, pets, shopkeepers. Add character references."),
    ("ui", "Graphics", "Screens and menus", "HUD, shop, inventory, menus. Add screenshots of UI you like."),
    ("animations", "Animation and sound", "Animations", "How things move: running, attacks, emotes, opening rewards."),
    ("effects", "Animation and sound", "Effects", "Glows, trails, auras, particles, screen shake, number popups."),
    ("sounds", "Animation and sound", "Sound and music", "The feel of the music and the key sounds (collect, buy, win)."),
    ("progression", "Systems", "Progression", "Levels, upgrades, rebirths, unlocks. How long to max out?"),
    ("multiplayer", "Systems", "Playing with others", "Solo, co-op, PvP, trading, teams? Server size?"),
    ("shop", "Systems", "What is sold", "Gamepasses, products, cosmetics, and rough prices. What is never sold?"),
    ("cosmetics", "Systems", "Cosmetics", "Capes, hats, trails, pets, skins: earned or sold?"),
    ("avoid", "Rules", "Must have and must avoid", "Anything it must include, and anything you do not want."),
    ("thumbnail", "Rules", "Icon and thumbnail idea", "What should the icon show? Add thumbnails you like."),
];

fn slugify(title: &str) -> String {
    let s: String = title.to_lowercase().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    s.split('-').filter(|p| !p.is_empty()).collect::<Vec<_>>().join("-").chars().take(40).collect()
}

fn file(root: &Path, slug: &str) -> Result<PathBuf, String> {
    if !crate::paths::valid_id(slug) {
        return Err("bad game name".into());
    }
    Ok(root.join("projects").join(slug).join("docs").join("brief.json"))
}

fn load(root: &Path, slug: &str) -> Result<Value, String> {
    let t = fs::read_to_string(file(root, slug)?).map_err(|_| "no brief for that game".to_string())?;
    serde_json::from_str(&t).map_err(|e| e.to_string())
}

/// Start a brief for a new idea (creates the game folder), or for an existing game.
pub fn create(root: &Path, title: &str) -> Result<String, String> {
    let slug = slugify(title);
    if slug.is_empty() {
        return Err("Give the game a name first.".into());
    }
    let f = file(root, &slug)?;
    if !f.is_file() {
        fs::create_dir_all(f.parent().unwrap()).map_err(|e| e.to_string())?;
        fs::write(&f, json!({ "title": title.trim(), "slug": slug, "answers": {} }).to_string()).map_err(|e| e.to_string())?;
    }
    Ok(slug)
}

/// Briefs, as (slug, title, answered count).
pub fn list(root: &Path) -> Vec<Value> {
    let mut out = vec![];
    for e in fs::read_dir(root.join("projects")).into_iter().flatten().flatten() {
        let slug = e.file_name().to_string_lossy().to_string();
        if let Ok(b) = get(root, &slug) {
            out.push(json!({ "slug": slug, "title": b["title"], "done": b["done"], "total": ITEMS.len() }));
        }
    }
    out
}

/// The brief with every checklist item, its answer and its images.
pub fn get(root: &Path, slug: &str) -> Result<Value, String> {
    let b = load(root, slug)?;
    let refs = crate::references::list(root, slug).unwrap_or_default();
    let mut done = 0;
    let items: Vec<Value> = ITEMS
        .iter()
        .map(|(id, group, title, hint)| {
            let text = b["answers"][*id].as_str().unwrap_or("").to_string();
            let images: Vec<&str> = refs.iter().filter(|r| r.source == format!("brief:{id}")).map(|r| r.file.as_str()).collect();
            if !text.trim().is_empty() || !images.is_empty() {
                done += 1;
            }
            json!({ "id": id, "group": group, "title": title, "hint": hint, "text": text, "images": images })
        })
        .collect();
    Ok(json!({ "title": b["title"], "slug": slug, "items": items, "done": done, "total": ITEMS.len() }))
}

/// Save one answer, and keep a readable brief.md next to it.
pub fn set(root: &Path, slug: &str, id: &str, text: &str) -> Result<(), String> {
    if !ITEMS.iter().any(|i| i.0 == id) {
        return Err("unknown checklist item".into());
    }
    let mut b = load(root, slug)?;
    b["answers"][id] = json!(text);
    let f = file(root, slug)?;
    let tmp = f.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(&b).unwrap()).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &f).map_err(|e| e.to_string())?;
    fs::write(f.with_file_name("brief.md"), markdown(root, slug)?).map_err(|e| e.to_string())
}

/// Attach an image to one item (saved on the references board).
pub fn add_image(root: &Path, slug: &str, id: &str, name: &str, data_b64: &str) -> Result<(), String> {
    let item = ITEMS.iter().find(|i| i.0 == id).ok_or("unknown checklist item")?;
    load(root, slug)?;
    let cat = match item.1 {
        "Graphics" if id == "characters" => "Character",
        "Graphics" if id == "ui" => "UI",
        "Graphics" => "Environment",
        "Animation and sound" if id == "sounds" => "Sound and music",
        "Animation and sound" => "Animation",
        _ => "Other",
    };
    crate::references::add(root, slug, name, data_b64, cat, &format!("From the game brief: {}", item.2), &format!("brief:{id}")).map(|_| ())
}

fn markdown(root: &Path, slug: &str) -> Result<String, String> {
    let b = get(root, slug)?;
    let mut s = format!("# Game brief: {}\n\nAndrew's own answers. Follow them. Where an item is empty, decide it and list the choice under risks for him to confirm.\n", b["title"].as_str().unwrap_or(slug));
    let mut group = "";
    for (i, it) in b["items"].as_array().into_iter().flatten().enumerate() {
        if ITEMS[i].1 != group {
            group = ITEMS[i].1;
            s.push_str(&format!("\n## {group}\n"));
        }
        let text = it["text"].as_str().unwrap_or("").trim();
        let n = it["images"].as_array().map(|a| a.len()).unwrap_or(0);
        s.push_str(&format!("- **{}:** {}{}\n", ITEMS[i].2, if text.is_empty() { "(not answered)" } else { text }, if n > 0 { format!(" [{n} reference image(s) on the references board, source brief:{}]", ITEMS[i].0) } else { String::new() }));
    }
    Ok(s)
}

/// The instruction that sends this brief to Planner.
pub fn planner_prompt(root: &Path, slug: &str) -> Result<String, String> {
    Ok(format!(
        "Plan this game from Andrew's own brief below. Use the slug \"{slug}\" in plan.json. His reference images are in 19-Project-Memory/{slug}-references.md: read that note and open the images before you design.\n\n{}",
        markdown(root, slug)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brief_roundtrip() {
        let d = std::env::temp_dir().join(format!("brief-t-{}", std::process::id()));
        fs::create_dir_all(d.join("projects")).unwrap();
        let slug = create(&d, "Candy Rush!! 2").unwrap();
        assert_eq!(slug, "candy-rush-2");
        set(&d, &slug, "idea", "Race for sugar").unwrap();
        assert!(set(&d, &slug, "nope", "x").is_err());
        let b = get(&d, &slug).unwrap();
        assert_eq!(b["done"], 1);
        let p = planner_prompt(&d, &slug).unwrap();
        assert!(p.contains("Race for sugar") && p.contains("(not answered)") && p.contains("candy-rush-2"));
        assert_eq!(list(&d).len(), 1);
        let _ = fs::remove_dir_all(&d);
    }
}
