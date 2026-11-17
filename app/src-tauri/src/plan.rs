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
    if !crate::paths::valid_id(run_id) {
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
static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn set(root: &Path, run_id: &str, item: &str, checked: Option<bool>, note: Option<String>) -> Result<(), String> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    safe_id(run_id)?;
    let p = approvals_path(root, run_id);
    if !p.parent().map(|d| d.join("plan.json").is_file()).unwrap_or(false) {
        return Err("no plan in that run".into());
    }
    let mut ap: Value = crate::paths::read_json(&p).unwrap_or(json!({"checked": {}, "notes": {}}));
    if let Some(b) = checked {
        ap["checked"][item] = json!(b);
    }
    if let Some(n) = note {
        ap["notes"][item] = json!(n);
    }
    crate::paths::write_atomic(&p, serde_json::to_string_pretty(&ap).unwrap_or_default().as_bytes())
}

/// Merge one agent's change.json (from its run folder) into the plan: the
/// item gets the new text (and mockup), its checkmark is cleared so Andrew
/// looks again, and the old text is kept in approvals.json under "history".
pub fn apply_change(root: &Path, run_id: &str, item: &str, agent_dir: &Path, change_id: &str) -> Result<String, String> {
    let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    safe_id(run_id)?;
    let ch: Value = serde_json::from_str(&fs::read_to_string(agent_dir.join("change.json")).map_err(|_| "change.json missing".to_string())?).map_err(|e| format!("change.json is not JSON: {e}"))?;
    if ch["item"].as_str() != Some(item) {
        return Err(format!("change.json names item {:?}, expected {item}", ch["item"]));
    }
    let text = ch["text"].as_str().map(str::trim).filter(|t| !t.is_empty()).ok_or("change.json has no new text")?.to_string();
    let dir = root.join("runs").join(run_id);
    let pj = dir.join("plan.json");
    let mut plan: Value = serde_json::from_str(&fs::read_to_string(&pj).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let mut new_mock: Option<String> = None;
    if let Some(m) = ch["mock"].as_str() {
        if !m.starts_with("mockups/") || !m.ends_with(".svg") || !crate::paths::safe_rel(m) {
            return Err(format!("bad mock path {m}"));
        }
        let svg = fs::read_to_string(agent_dir.join(m)).map_err(|_| format!("{m} missing in the agent's folder"))?;
        if !svg.contains("<svg") {
            return Err(format!("{m} is not an SVG"));
        }
        let name = format!("mockups/{item}-{}.svg", change_id.trim_start_matches("chg-"));
        if !crate::paths::safe_rel(&name) {
            return Err("bad item id".into());
        }
        crate::paths::write_atomic(&dir.join(&name), svg.as_bytes())?;
        new_mock = Some(name);
    }
    let mut old = Value::Null;
    for s in plan["sections"].as_array_mut().into_iter().flatten() {
        for it in s["items"].as_array_mut().into_iter().flatten() {
            if it["id"] == item {
                old = it.clone();
                it["text"] = json!(text);
                if let Some(m) = &new_mock {
                    it["mock"] = json!(m);
                }
            }
        }
    }
    if old.is_null() {
        return Err(format!("no item {item} in the plan"));
    }
    crate::paths::write_atomic(&pj, serde_json::to_string_pretty(&plan).unwrap_or_default().as_bytes())?;
    let ap_path = approvals_path(root, run_id);
    let mut ap: Value = crate::paths::read_json(&ap_path).unwrap_or(json!({"checked": {}, "notes": {}}));
    ap["checked"][item] = json!(false);
    ap["notes"][item] = json!("");
    ap["changed"][item] = json!({ "by": change_id, "at": crate::now_rfc3339() });
    if !ap["history"][item].is_array() {
        ap["history"][item] = json!([]);
    }
    if let Some(h) = ap["history"][item].as_array_mut() {
        h.push(json!({ "text": old["text"], "mock": old["mock"], "replaced_by": change_id }));
    }
    crate::paths::write_atomic(&ap_path, serde_json::to_string_pretty(&ap).unwrap_or_default().as_bytes())?;
    Ok(text)
}

/// Read one mockup SVG from a plan's run folder.
pub fn mock(root: &Path, run_id: &str, path: &str) -> Result<String, String> {
    safe_id(run_id)?;
    if !path.starts_with("mockups/") || !crate::paths::safe_rel(path) || !path.ends_with(".svg") {
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

#[cfg(test)]
mod change_tests {
    use super::*;
    #[test]
    fn an_agents_change_is_merged_and_the_item_unchecked() {
        let root = std::env::temp_dir().join(format!("plan-chg-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let plan_dir = root.join("runs/run-plan-1");
        fs::create_dir_all(plan_dir.join("mockups")).unwrap();
        fs::write(plan_dir.join("plan.json"), json!({"title": "T", "slug": "t", "sections": [{"id": "ui", "items": [{"id": "ui-1", "text": "Small shop button", "mock": "mockups/ui-hud.svg"}]}]}).to_string()).unwrap();
        fs::write(approvals_path(&root, "run-plan-1"), json!({"checked": {"ui-1": true}, "notes": {"ui-1": "make it bigger"}}).to_string()).unwrap();
        let agent = root.join("runs/run-frame-1");
        fs::create_dir_all(agent.join("mockups")).unwrap();
        // wrong item is refused
        fs::write(agent.join("change.json"), json!({"item": "ui-9", "text": "x", "mock": null}).to_string()).unwrap();
        assert!(apply_change(&root, "run-plan-1", "ui-1", &agent, "chg-1").is_err());
        // a path outside mockups/ is refused
        fs::write(agent.join("change.json"), json!({"item": "ui-1", "text": "Big", "mock": "../x.svg"}).to_string()).unwrap();
        assert!(apply_change(&root, "run-plan-1", "ui-1", &agent, "chg-1").is_err());
        fs::write(agent.join("mockups/ui-1.svg"), "<svg xmlns='http://www.w3.org/2000/svg'/>").unwrap();
        fs::write(agent.join("change.json"), json!({"item": "ui-1", "text": "Big round shop button, bottom right", "mock": "mockups/ui-1.svg"}).to_string()).unwrap();
        apply_change(&root, "run-plan-1", "ui-1", &agent, "chg-1").unwrap();
        let plan: Value = serde_json::from_str(&fs::read_to_string(plan_dir.join("plan.json")).unwrap()).unwrap();
        let it = &plan["sections"][0]["items"][0];
        assert_eq!(it["text"], "Big round shop button, bottom right");
        assert_eq!(it["mock"], "mockups/ui-1-1.svg");
        assert!(plan_dir.join("mockups/ui-1-1.svg").is_file());
        let ap: Value = serde_json::from_str(&fs::read_to_string(approvals_path(&root, "run-plan-1")).unwrap()).unwrap();
        assert_eq!(ap["checked"]["ui-1"], false, "Andrew checks it again");
        assert_eq!(ap["notes"]["ui-1"], "");
        assert_eq!(ap["history"]["ui-1"][0]["text"], "Small shop button");
        assert_eq!(ap["changed"]["ui-1"]["by"], "chg-1");
        let _ = fs::remove_dir_all(&root);
    }
}
