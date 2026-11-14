//! Texture library: free CC0 textures from Poly Haven's public API, saved
//! into the vault (10-Art-Environment/textures/<group>/<id>/) with a
//! catalog note Obsidian shows as a gallery. Poly Haven's API terms ask for
//! a unique user agent and a visible credit; both are here ("Powered by
//! Poly Haven" in the app and the note). The assets themselves are CC0.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const API: &str = "https://api.polyhaven.com";

/// Friendly groups matched against each texture's name, categories, and tags.
pub const GROUPS: &[(&str, &[&str])] = &[
    ("brick", &["brick"]),
    ("wood", &["wood", "plank", "bark"]),
    ("stone", &["rock", "stone", "cobble", "pebble", "cliff"]),
    ("ground", &["ground", "dirt", "soil", "mud", "gravel", "forest floor"]),
    ("grass", &["grass", "moss", "leaves", "leaf", "foliage"]),
    ("metal", &["metal", "rust", "steel", "iron", "corrugated"]),
    ("fabric", &["fabric", "cloth", "carpet", "leather", "denim"]),
    ("concrete", &["concrete", "plaster", "stucco", "cement"]),
    ("tiles", &["tile", "floor", "marble", "parquet"]),
    ("roof", &["roof", "shingle"]),
    ("sand-snow", &["sand", "snow", "ice", "desert"]),
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Texture {
    pub id: String,
    pub name: String,
    pub group: String,
    pub tags: Vec<String>,
    pub maps: std::collections::BTreeMap<String, String>, // color/normal/roughness/metalness -> vault-relative path
}

fn dir(root: &Path) -> PathBuf {
    root.join("vault/10-Art-Environment/textures")
}

pub fn library(root: &Path) -> Vec<Texture> {
    fs::read_to_string(dir(root).join("library.json")).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn get_json(url: &str) -> Result<serde_json::Value, String> {
    ureq::get(url)
        .set("User-Agent", "BrewixGames")
        .timeout(std::time::Duration::from_secs(60))
        .call()
        .map_err(|e| format!("could not reach Poly Haven: {e}"))?
        .into_json()
        .map_err(|e| e.to_string())
}

/// Which group a texture belongs to, from its name, categories, and tags.
pub fn group_of(name: &str, cats: &[String], tags: &[String]) -> Option<&'static str> {
    let hay = format!("{} {} {}", name, cats.join(" "), tags.join(" ")).to_lowercase();
    GROUPS.iter().find(|(_, words)| words.iter().any(|w| hay.contains(w))).map(|(g, _)| *g)
}

/// Pick the map files to download from a /files/{id} response:
/// (kind, url, extension) for color, normal, roughness, metalness.
pub fn pick_maps(files: &serde_json::Value, res: &str) -> Vec<(&'static str, String, String)> {
    let Some(obj) = files.as_object() else { return vec![] };
    let kinds: [(&str, &[&str]); 4] = [("color", &["diff", "color", "albedo"]), ("normal", &["nor_gl", "nor"]), ("roughness", &["rough"]), ("metalness", &["metal"])];
    let mut out = vec![];
    for (kind, words) in kinds {
        // The first word is the preferred match (nor_gl over nor_dx).
        let key = words.iter().find_map(|w| obj.keys().find(|k| k.to_lowercase().contains(w) && !["blend", "gltf", "mtlx"].contains(&k.as_str())));
        let Some(key) = key else { continue };
        let by_res = &obj[key];
        let tier = by_res.get(res).or_else(|| by_res.as_object().and_then(|m| m.values().next()));
        let Some(tier) = tier else { continue };
        let fmt = ["jpg", "png"].iter().find(|f| tier.get(**f).is_some());
        if let Some(f) = fmt {
            if let Some(url) = tier[*f].get("url").and_then(|u| u.as_str()) {
                out.push((kind, url.to_string(), f.to_string()));
            }
        }
    }
    out
}

/// Download up to `per_group` of the most-downloaded textures in each chosen
/// group (skipping ones already in the library), then rewrite the note.
pub fn download(root: &Path, groups: &[String], per_group: usize, res: &str, on_line: &mut dyn FnMut(&str)) -> Result<String, String> {
    let all = get_json(&format!("{API}/assets?t=textures"))?;
    let Some(assets) = all.as_object() else { return Err("unexpected reply from Poly Haven".into()) };
    let mut lib = library(root);
    let mut added = 0;
    for g in groups {
        let mut cands: Vec<(&String, &serde_json::Value)> = assets
            .iter()
            .filter(|(_, a)| {
                let s = |k: &str| a.get(k).and_then(|x| x.as_array()).map(|v| v.iter().filter_map(|t| t.as_str().map(String::from)).collect::<Vec<_>>()).unwrap_or_default();
                group_of(a.get("name").and_then(|n| n.as_str()).unwrap_or(""), &s("categories"), &s("tags")) == Some(g.as_str())
            })
            .collect();
        cands.sort_by_key(|(_, a)| std::cmp::Reverse(a.get("download_count").and_then(|d| d.as_u64()).unwrap_or(0)));
        let mut n = 0;
        for (id, a) in cands {
            if n >= per_group {
                break;
            }
            if lib.iter().any(|t| &t.id == id) {
                n += 1;
                continue;
            }
            let name = a.get("name").and_then(|x| x.as_str()).unwrap_or(id).to_string();
            on_line(&format!("{g}: {name}"));
            let files = get_json(&format!("{API}/files/{id}"))?;
            let maps = pick_maps(&files, res);
            if !maps.iter().any(|(k, _, _)| *k == "color") {
                continue;
            }
            let folder = dir(root).join(g).join(id);
            fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
            let mut saved = std::collections::BTreeMap::new();
            for (kind, url, ext) in maps {
                let resp = ureq::get(&url).set("User-Agent", "BrewixGames").timeout(std::time::Duration::from_secs(300)).call().map_err(|e| format!("download failed: {e}"))?;
                let mut bytes = vec![];
                std::io::Read::read_to_end(&mut resp.into_reader(), &mut bytes).map_err(|e| e.to_string())?;
                let file = format!("{kind}.{ext}");
                fs::write(folder.join(&file), bytes).map_err(|e| e.to_string())?;
                saved.insert(kind.to_string(), format!("textures/{g}/{id}/{file}"));
            }
            let tags = a.get("tags").and_then(|x| x.as_array()).map(|v| v.iter().filter_map(|t| t.as_str().map(String::from)).take(6).collect()).unwrap_or_default();
            lib.push(Texture { id: id.clone(), name, group: g.clone(), tags, maps: saved });
            fs::write(dir(root).join("library.json"), serde_json::to_string_pretty(&lib).unwrap()).map_err(|e| e.to_string())?;
            added += 1;
            n += 1;
        }
    }
    write_note(root, &lib)?;
    Ok(format!("Added {added} textures ({} in the library). Powered by Poly Haven.", lib.len()))
}

const MATERIALS: &str = "Plastic, SmoothPlastic, Neon, Wood, WoodPlanks, Marble, Basalt, Slate, CrackedLava, Concrete, Limestone, Granite, Pavement, Brick, Pebble, Cobblestone, Rock, Sandstone, CorrodedMetal, DiamondPlate, Foil, Metal, Grass, LeafyGrass, Sand, Fabric, Snow, Mud, Ground, Asphalt, Salt, Ice, Glacier, Glass, ForceField, Cardboard, Carpet, CeramicTiles, ClayRoofTiles, RoofShingles, Leather, Plaster, Rubber";

pub fn write_note(root: &Path, lib: &[Texture]) -> Result<(), String> {
    let path = root.join("vault/10-Art-Environment/texture-library.md");
    let mut t = format!(
        "---\ndescription: Free textures for every game: Roblox's built-in materials plus CC0 colour and material maps from Poly Haven saved in this vault.\ntype: reference\nstatus: verified\nlast_checked: {}\nkeywords: textures, materials, pbr, color map, normal map, roughness, surfaceappearance, materialvariant, brick, wood, stone, metal, fabric, grass, poly haven\nsources: Roblox Material enum (creator-docs reference, read 2026-11-14); Poly Haven API (CC0 assets)\nverify: Built-in list from Roblox's Material enum; texture files downloaded by the app from Poly Haven (CC0). Written by the app (textures.rs).\ntags: [art-environment, reference]\n---\n# Texture library\n\nPowered by Poly Haven (polyhaven.com). All Poly Haven textures are CC0: free for any use, no credit needed in your game.\n\n## Do this\n- First try Roblox's built-in materials (free, fast, look good on phones): {MATERIALS}. Change their colour with the part's Color.\n- For a custom look on many parts, make a MaterialVariant in MaterialService from a texture below (ColorMap, NormalMap, RoughnessMap, MetalnessMap), then pick it on parts like a built-in material.\n- For one special mesh, use SurfaceAppearance on the MeshPart with the same maps. See [[10-Art-Environment/realism-when-needed]].\n- Andrew uploads the image files to Roblox (Creator Hub or Asset Manager); agents name the files and where they go.\n- 1K maps are enough for most surfaces on phones; reuse one texture with different tints instead of many textures.\n",
        &crate::now_rfc3339()[..10]
    );
    if lib.is_empty() {
        t.push_str("\nNo textures downloaded yet. In BrewixGames: Vault, Download Textures.\n");
    }
    for (g, _) in GROUPS {
        let items: Vec<&Texture> = lib.iter().filter(|x| x.group == *g).collect();
        if items.is_empty() {
            continue;
        }
        t.push_str(&format!("\n## {g}\n"));
        for x in items {
            let color = x.maps.get("color").cloned().unwrap_or_default();
            let maps: Vec<String> = x.maps.keys().cloned().collect();
            t.push_str(&format!("\n### {}\n![[{color}|200]]\nMaps: {} · Tags: {} · polyhaven.com/a/{}\n", x.name, maps.join(", "), x.tags.join(", "), x.id));
        }
    }
    fs::write(path, t).map_err(|e| e.to_string())?;
    let idx = root.join("vault/10-Art-Environment/index.md");
    if let Ok(cur) = fs::read_to_string(&idx) {
        let link = "[[10-Art-Environment/texture-library]]";
        if !cur.contains(link) {
            let _ = fs::write(&idx, format!("{}\n| {link} | Roblox built-in materials plus CC0 Poly Haven textures (colour, normal, roughness, metalness) saved in the vault. | verified |\n", cur.trim_end()));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_and_map_picking() {
        assert_eq!(group_of("Red Brick Wall", &[], &[]), Some("brick"));
        assert_eq!(group_of("Rusty Panel", &["metal".into()], &[]), Some("metal"));
        assert_eq!(group_of("Something", &[], &["moss".into()]), Some("grass"));
        assert_eq!(group_of("Abstract", &[], &[]), None);
        let files = serde_json::json!({
            "blend": {"1k": {"blend": {"url": "x"}}},
            "Diffuse": {"1k": {"jpg": {"url": "https://dl/diff_1k.jpg"}, "png": {"url": "p"}}, "2k": {"jpg": {"url": "d2"}}},
            "nor_dx": {"1k": {"jpg": {"url": "https://dl/nor_dx.jpg"}}},
            "nor_gl": {"1k": {"png": {"url": "https://dl/nor_gl.png"}}},
            "Rough": {"2k": {"jpg": {"url": "https://dl/rough_2k.jpg"}}}
        });
        let m = pick_maps(&files, "1k");
        assert_eq!(m[0], ("color", "https://dl/diff_1k.jpg".into(), "jpg".into()));
        assert_eq!(m[1], ("normal", "https://dl/nor_gl.png".into(), "png".into()), "OpenGL normal preferred");
        assert_eq!(m[2].0, "roughness", "falls back to another resolution");
        assert_eq!(m.len(), 3, "no metalness map in this asset");
    }

    #[test]
    fn note_lists_builtin_materials_and_credit() {
        let root = std::env::temp_dir().join(format!("brewix-tex-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("vault/10-Art-Environment")).unwrap();
        write_note(&root, &[]).unwrap();
        let t = fs::read_to_string(root.join("vault/10-Art-Environment/texture-library.md")).unwrap();
        assert!(t.contains("Powered by Poly Haven") && t.contains("CeramicTiles") && t.contains("No textures downloaded yet"));
        let _ = fs::remove_dir_all(&root);
    }
}
