//! Game Score: one number out of 100 for how ready a game is, from checks
//! the studio can run itself plus the latest Player Test and Second Opinion.
//! A game under the standard (75) gets a "keep working" list of what to add.
use regex::Regex;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const STANDARD: f64 = 75.0;

fn luau(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            luau(&p, out);
        } else if matches!(p.extension().and_then(|x| x.to_str()), Some("luau") | Some("lua")) {
            out.push(p);
        }
    }
}

struct Part {
    name: &'static str,
    max: f64,
    got: f64,
    missing: Vec<String>,
}

impl Part {
    fn new(name: &'static str, max: f64) -> Self {
        Part { name, max, got: 0.0, missing: vec![] }
    }
    /// Add `pts` when `ok`, otherwise note what to add.
    fn check(&mut self, ok: bool, pts: f64, todo: &str) {
        if ok {
            self.got += pts;
        } else {
            self.missing.push(todo.to_string());
        }
    }
}

/// Feature signals found in the game's code (and project file).
pub fn signals(code: &str) -> Vec<(&'static str, bool)> {
    let has = |re: &str| Regex::new(re).map(|r| r.is_match(code)).unwrap_or(false);
    vec![
        ("lighting", has(r"Lighting\.\w+\s*=|Atmosphere|ColorCorrectionEffect|BloomEffect|\x22Lighting\x22\s*:\s*\{[^}]*\x22\$properties\x22")),
        ("materials", has(r"Enum\.Material\.|\x22Material\x22\s*:|MaterialVariant|SurfaceAppearance")),
        ("tweens", has(r"TweenService")),
        ("particles", has(r"ParticleEmitter|\bBeam\b|\bTrail\b")),
        ("sound", has(r"SoundService|:Play\(\)|\bSound\b")),
        ("ui", has(r"ScreenGui|BillboardGui|SurfaceGui")),
        ("favorite", has(r"(?i)favorite")),
        ("daily", has(r"(?i)daily|streak")),
        ("timed", has(r"(?i)playtime|timedreward|session ?reward|giftsclaim|freegift")),
        ("goals", has(r"(?i)quest|mission|objective|goal")),
        ("leaderboard", has(r"leaderstats|OrderedDataStore|(?i)leaderboard")),
        ("badges", has(r"BadgeService")),
        ("social", has(r"SocialService|PromptGameInvite|(?i)invite")),
        ("saves", has(r"DataStoreService|ProfileStore|ProfileService")),
        ("receipt", has(r"ProcessReceipt")),
        ("gamepass", has(r"UserOwnsGamePassAsync")),
        ("prompts", has(r"PromptProductPurchase|PromptGamePassPurchase|PromptPurchase")),
        ("reward_fx", has(r"(?i)reward|celebrat|confetti|reveal")),
    ]
}

/// Purchase prompts that sit in a file with no button handler are likely
/// shown on a timer or on join, which players hate (and we never do).
fn prompts_on_tap(files: &[(String, String)]) -> bool {
    files.iter().filter(|(_, t)| t.contains("PromptProductPurchase") || t.contains("PromptGamePassPurchase") || t.contains("PromptPurchase(")).all(|(_, t)| t.contains("Activated") || t.contains("MouseButton1Click") || t.contains("Triggered") || t.contains("OnServerEvent"))
}

fn latest_sim(root: &Path, project: &str) -> Option<(f64, String)> {
    let sims = crate::studio::simulations(root, &format!("projects/{project}")).ok()?;
    let v = sims.into_iter().next()?;
    Some((v.pointer("/rating/score")?.as_f64()?, v["at"].as_str().unwrap_or("").to_string()))
}

fn latest_review(root: &Path, project: &str) -> Option<String> {
    crate::review::list(root).into_iter().find(|r| r["target"] == project && r["report"].is_string()).map(|r| {
        let t = r["report"].as_str().unwrap_or("");
        if t.contains("NOT SOUND") {
            "NOT SOUND".to_string()
        } else if t.contains("SOUND WITH FIXES") {
            "SOUND WITH FIXES".to_string()
        } else if t.contains("SOUND") {
            "SOUND".to_string()
        } else {
            "no verdict".to_string()
        }
    })
}

/// Score one game. `build` is the build gate result (pass count, total).
pub fn score_with(root: &Path, project: &str, build: (usize, usize)) -> Result<Value, String> {
    if !crate::paths::valid_id(project) {
        return Err("bad project name".into());
    }
    let proj = root.join("projects").join(project);
    if !proj.join("default.project.json").is_file() {
        return Err(format!("{project} has no game files yet"));
    }
    let mut paths = vec![];
    luau(&proj.join("src"), &mut paths);
    let files: Vec<(String, String)> = paths.iter().filter_map(|p| Some((p.to_string_lossy().to_string(), fs::read_to_string(p).ok()?))).collect();
    let mut code: String = files.iter().map(|f| f.1.as_str()).collect::<Vec<_>>().join("\n");
    code.push_str(&fs::read_to_string(proj.join("default.project.json")).unwrap_or_default());
    let sig: std::collections::HashMap<&str, bool> = signals(&code).into_iter().collect();
    let s = |k: &str| *sig.get(k).unwrap_or(&false);

    let mut runs = Part::new("Builds and runs", 15.0);
    let (ok, total) = build;
    runs.got = if total > 0 { 15.0 * ok as f64 / total as f64 } else { 0.0 };
    if ok < total {
        runs.missing.push(format!("Fix the build checks: {} of {total} pass", ok));
    }

    let mut health = Part::new("Code health", 10.0);
    let (findings, _) = crate::tuneup::scan_code(&proj);
    let pen: f64 = findings.iter().map(|f| match f["severity"].as_str() { Some("blocker") => 4.0, Some("major") => 1.5, _ => 0.3 }).sum();
    health.got = (10.0 - pen).max(0.0);
    if pen > 0.0 {
        health.missing.push(format!("Clear the {} Tune-Up finding(s)", findings.len()));
    }

    let mut gfx = Part::new("Graphics and feel", 20.0);
    gfx.check(s("lighting"), 5.0, "Set a lighting mood (Lighting, Atmosphere, ColorCorrection) from 10-Art-Environment/lighting-presets");
    gfx.check(s("materials"), 3.0, "Use real materials or textures, not plain plastic everywhere");
    gfx.check(s("tweens"), 3.0, "Animate the UI and objects with tweens (buttons, popups, pickups)");
    gfx.check(s("particles"), 3.0, "Add particles or trails on rewards, hits and pickups");
    gfx.check(s("sound"), 3.0, "Add sounds for every action and the six UI sounds");
    gfx.check(s("ui"), 3.0, "Build the HUD and menus");

    let mut hooks = Part::new("Keeps players playing", 25.0);
    hooks.check(s("saves"), 4.0, "Save progress (ProfileStore or DataStore) so players keep what they earn");
    hooks.check(s("daily"), 4.0, "Daily reward with a streak");
    hooks.check(s("timed"), 3.0, "Playtime gifts (a free reward every few minutes of play)");
    hooks.check(s("goals"), 3.0, "Quests or goals so the next target is always visible");
    hooks.check(s("leaderboard"), 3.0, "Leaderboard or leaderstats");
    hooks.check(s("social"), 3.0, "Invite friends prompt and a reason to play together");
    hooks.check(s("favorite"), 3.0, "Favorite reminder after about 6 minutes of play (see 06-Retention notes)");
    hooks.check(s("badges"), 2.0, "Badges for milestones");

    let mut money = Part::new("Fair monetization", 10.0);
    money.check(s("receipt"), 4.0, "Handle purchases safely with ProcessReceipt (07-Monetization/processreceipt-template)");
    money.check(s("gamepass"), 2.0, "Check gamepass ownership with UserOwnsGamePassAsync");
    money.check(s("prompts") && prompts_on_tap(&files), 4.0, if s("prompts") { "Show purchase prompts only when the player taps Buy" } else { "Add a shop: products that give an instant, visible reward" });

    let mut test = Part::new("Player Test", 12.0);
    let sim = latest_sim(root, project);
    match &sim {
        Some((r, _)) => {
            test.got = 12.0 * (r / 100.0).clamp(0.0, 1.0);
            if *r < 75.0 {
                test.missing.push(format!("Player Test rated it {r:.0}/100: fix its top improvements"));
            }
        }
        None => test.missing.push("Run a Player Test (Make the Game, Player Test)".into()),
    }

    let mut second = Part::new("Second Opinion", 8.0);
    let verdict = latest_review(root, project);
    match verdict.as_deref() {
        Some("SOUND") => second.got = 8.0,
        Some("SOUND WITH FIXES") => {
            second.got = 5.0;
            second.missing.push("Apply the Second Opinion fixes".into());
        }
        Some(v) => second.missing.push(format!("Second Opinion verdict: {v}. Fix and check again")),
        None => second.missing.push("Get a Second Opinion on the code".into()),
    }

    let parts = [runs, health, gfx, hooks, money, test, second];
    let total: f64 = parts.iter().map(|p| p.got).sum::<f64>().round();
    let grade = if total >= 90.0 { "A" } else if total >= STANDARD { "B" } else if total >= 60.0 { "C" } else { "D" };
    let mut todo: Vec<(f64, String)> = parts.iter().flat_map(|p| p.missing.iter().map(move |m| (p.max - p.got, m.clone()))).collect();
    todo.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    Ok(json!({
        "project": project,
        "score": total,
        "grade": grade,
        "standard": STANDARD,
        "ready": total >= STANDARD,
        "parts": parts.iter().map(|p| json!({ "name": p.name, "got": (p.got * 10.0).round() / 10.0, "max": p.max, "missing": p.missing })).collect::<Vec<_>>(),
        "todo": todo.into_iter().map(|t| t.1).collect::<Vec<_>>(),
        "player_test": sim.map(|s| s.0),
        "second_opinion": verdict,
        "at": crate::now_rfc3339(),
    }))
}

pub fn score(root: &Path, project: &str) -> Result<Value, String> {
    let rel = format!("projects/{project}");
    if !crate::paths::valid_id(project) {
        return Err("bad project name".into());
    }
    let g = crate::gates::build_gate(root, &rel);
    let v = score_with(root, project, (g.iter().filter(|c| c.passed).count(), g.len()))?;
    let dir = root.join("projects").join(project).join("docs");
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(dir.join("score.json"), serde_json::to_string_pretty(&v).unwrap_or_default());
    Ok(v)
}

/// The last saved score, without re-running anything.
pub fn saved(root: &Path, project: &str) -> Option<Value> {
    if !crate::paths::valid_id(project) {
        return None;
    }
    fs::read_to_string(root.join("projects").join(project).join("docs/score.json")).ok().and_then(|t| serde_json::from_str(&t).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scores_features_and_lists_what_is_missing() {
        let d = std::env::temp_dir().join(format!("score-t-{}", std::process::id()));
        let p = d.join("projects/g");
        fs::create_dir_all(p.join("src/server")).unwrap();
        fs::create_dir_all(d.join("runs")).unwrap();
        fs::write(p.join("default.project.json"), "{}").unwrap();
        fs::write(p.join("src/server/a.server.luau"), "local DataStoreService = game:GetService(\"DataStoreService\")\nlocal TweenService = game:GetService(\"TweenService\")\n").unwrap();
        let v = score_with(&d, "g", (4, 4)).unwrap();
        assert_eq!(v["parts"][0]["got"], 15.0);
        assert!(v["score"].as_f64().unwrap() < STANDARD);
        assert_eq!(v["ready"], false);
        let todo: Vec<&str> = v["todo"].as_array().unwrap().iter().map(|x| x.as_str().unwrap()).collect();
        assert!(todo.iter().any(|t| t.starts_with("Daily reward")));
        assert!(!todo.iter().any(|t| t.starts_with("Save progress")));
        let tap = vec![("a".to_string(), "btn.Activated:Connect(function() MarketplaceService:PromptProductPurchase(p, 1) end)".to_string())];
        let timer = vec![("b".to_string(), "task.delay(30, function() MarketplaceService:PromptProductPurchase(p, 1) end)".to_string())];
        assert!(prompts_on_tap(&tap) && !prompts_on_tap(&timer));
        assert!(score_with(&d, "../x", (0, 0)).is_err());
        let _ = fs::remove_dir_all(&d);
    }
}
