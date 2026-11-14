//! Game plans from the Planner agent: a checklist Andrew approves item by item
//! before the build team spends anything.
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub const REQUIRED: [&str; 13] = ["core-loop", "world", "ui", "characters", "cosmetics", "animations", "sounds", "style-kit", "monetization", "retention", "accessibility", "tech", "risks"];
pub const MOCKS: [&str; 4] = ["map.svg", "ui-hud.svg", "ui-shop.svg", "character-effects.svg"];

fn c(name: &str, passed: bool, detail: &str) -> crate::queue::GateCheck {
    crate::queue::GateCheck { name: name.into(), passed, detail: detail.into() }
}

/// Gate checks on plan.json and mockups next to the Planner's plan.md.
pub fn checks(out_path: &Path) -> Vec<crate::queue::GateCheck> {
    let dir = out_path.parent().unwrap_or(Path::new("."));
    let v: Option<Value> = fs::read_to_string(dir.join("plan.json")).ok().and_then(|t| serde_json::from_str(&t).ok());
    let Some(v) = v else { return vec![c("plan.json parses", false, "missing or not JSON")] };
    let secs = v["sections"].as_array().cloned().unwrap_or_default();
    let thin: Vec<&str> = REQUIRED.iter().copied().filter(|id| !secs.iter().any(|s| s["id"] == *id && s["items"].as_array().map(|a| a.len() >= 2).unwrap_or(false))).collect();
    let mut bad = vec![];
    for s in &secs {
        for it in s["items"].as_array().into_iter().flatten() {
            if let Some(m) = it["mock"].as_str() {
                if m.contains("..") || !dir.join(m).is_file() {
                    bad.push(m.to_string());
                }
            }
        }
    }
    let missing: Vec<&str> = MOCKS.iter().copied().filter(|m| !dir.join("mockups").join(m).is_file()).collect();
    let slug_ok = v["slug"].as_str().map(|s| !s.is_empty() && s.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')).unwrap_or(false);
    vec![
        c("plan.json parses", true, &format!("{} sections", secs.len())),
        c("every required section with 2+ items", thin.is_empty(), &thin.join(", ")),
        c("slug is lowercase-with-dashes", slug_ok, v["slug"].as_str().unwrap_or("")),
        c("every mock path exists", bad.is_empty(), &bad.join(", ")),
        c("map, HUD, shop and character-effects mockups present", missing.is_empty(), &missing.join(", ")),
    ]
}

fn approvals_path(root: &Path, run_id: &str) -> std::path::PathBuf {
    root.join("runs").join(run_id).join("approvals.json")
}

fn safe_id(run_id: &str) -> Result<(), String> {
    if run_id.is_empty() || !run_id.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_') {
        return Err("bad run id".into());
    }
    Ok(())
}

/// Plans from finished Planner runs, newest first, with Andrew's checkmarks.
pub fn list(root: &Path) -> Vec<Value> {
    let mut out = vec![];
    for r in crate::runs::list(root).unwrap_or_default() {
        if r.agent != "planner" || r.status != "done" {
            continue;
        }
        let dir = root.join("runs").join(&r.run_id);
        let Some(mut v) = fs::read_to_string(dir.join("plan.json")).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) else { continue };
        let ap: Value = fs::read_to_string(approvals_path(root, &r.run_id)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({}));
        let (mut total, mut done) = (0, 0);
        for s in v["sections"].as_array().into_iter().flatten() {
            for it in s["items"].as_array().into_iter().flatten() {
                total += 1;
                if ap["checked"][it["id"].as_str().unwrap_or("")] == true {
                    done += 1;
                }
            }
        }
        v["run_id"] = json!(r.run_id);
        v["at"] = json!(r.finished_at);
        v["approvals"] = ap;
        v["total"] = json!(total);
        v["done"] = json!(done);
        out.push(v);
    }
    out
}

/// Save one checkmark or note.
pub fn set(root: &Path, run_id: &str, item: &str, checked: Option<bool>, note: Option<String>) -> Result<(), String> {
    safe_id(run_id)?;
    let p = approvals_path(root, run_id);
    if !p.parent().map(|d| d.join("plan.json").is_file()).unwrap_or(false) {
        return Err("no plan in that run".into());
    }
    let mut ap: Value = fs::read_to_string(&p).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({"checked": {}, "notes": {}}));
    if let Some(b) = checked {
        ap["checked"][item] = json!(b);
    }
    if let Some(n) = note {
        ap["notes"][item] = json!(n);
    }
    fs::write(&p, serde_json::to_string_pretty(&ap).unwrap()).map_err(|e| e.to_string())
}

/// Read one mockup SVG from a plan's run folder.
pub fn mock(root: &Path, run_id: &str, path: &str) -> Result<String, String> {
    safe_id(run_id)?;
    if !path.starts_with("mockups/") || path.contains("..") || !path.ends_with(".svg") {
        return Err("bad mock path".into());
    }
    fs::read_to_string(root.join("runs").join(run_id).join(path)).map_err(|e| e.to_string())
}

/// The build instruction, only when every item is checked.
pub fn build_input(root: &Path, run_id: &str) -> Result<String, String> {
    let p = list(root).into_iter().find(|p| p["run_id"] == run_id).ok_or("plan not found")?;
    if p["total"].as_u64().unwrap_or(0) == 0 || p["done"] != p["total"] {
        return Err(format!("{} of {} items checked; check every item first", p["done"], p["total"]));
    }
    let notes: Vec<String> = p["approvals"]["notes"].as_object().into_iter().flatten().filter_map(|(k, v)| v.as_str().filter(|s| !s.trim().is_empty()).map(|s| format!("- {k}: {s}"))).collect();
    Ok(format!(
        "Andrew approved every item of the game plan at runs/{run_id}/plan.json (readable version runs/{run_id}/plan.md, mockups in runs/{run_id}/mockups/). Build exactly that plan as projects/{}.{}",
        p["slug"].as_str().unwrap_or("game"),
        if notes.is_empty() { String::new() } else { format!("\nAndrew's notes per item:\n{}", notes.join("\n")) }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plan_checks_need_sections_and_mocks() {
        let d = std::env::temp_dir().join(format!("plan-t-{}", std::process::id()));
        fs::create_dir_all(d.join("mockups")).unwrap();
        let out = d.join("plan.md");
        assert!(!checks(&out)[0].passed);
        let secs: Vec<Value> = REQUIRED.iter().map(|id| json!({"id": id, "title": id, "items": [{"id": format!("{id}-1"), "text": "x", "mock": null}, {"id": format!("{id}-2"), "text": "y", "mock": if *id == "world" { json!("mockups/map.svg") } else { Value::Null }}]})).collect();
        fs::write(d.join("plan.json"), json!({"title": "T", "slug": "t-game", "one_liner": "o", "sections": secs}).to_string()).unwrap();
        let r = checks(&out);
        assert!(r[1].passed && r[2].passed);
        assert!(!r[3].passed && !r[4].passed);
        for m in MOCKS {
            fs::write(d.join("mockups").join(m), "<svg/>").unwrap();
        }
        assert!(checks(&out).iter().all(|x| x.passed));
        let _ = fs::remove_dir_all(&d);
    }
}
