//! The review queue. One JSON file per item in `queue/items/`. Every decision
//! is also appended to `queue/decisions.jsonl` as an audit trail.
//!
//! Approving an item only records Andrew's approval. It never publishes,
//! uploads, or posts anything. Publishing is a separate, explicit action that
//! is not part of this module.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GateCheck {
    pub name: String,
    pub passed: bool,
    #[serde(default)]
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gate {
    pub passed: bool,
    #[serde(default)]
    pub checks: Vec<GateCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    pub verdict: String, // "approved" | "rejected"
    pub by: String,
    pub at: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueueItem {
    pub id: String,
    /// build | design | pitch | asset | thumbnail | playtest-report | update-plan | other
    pub kind: String,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    pub created_at: String,
    pub created_by: String,
    #[serde(default)]
    pub run_id: Option<String>,
    /// Paths relative to the studio root.
    #[serde(default)]
    pub artifacts: Vec<String>,
    pub gate: Gate,
    /// pending | approved | rejected
    pub status: String,
    #[serde(default)]
    pub decision: Option<Decision>,
}

pub fn items_dir(root: &Path) -> PathBuf {
    root.join("queue").join("items")
}

fn item_path(root: &Path, id: &str) -> Result<PathBuf, String> {
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(format!("invalid queue item id: {id:?}"));
    }
    Ok(items_dir(root).join(format!("{id}.json")))
}

pub fn list(root: &Path) -> Result<Vec<QueueItem>, String> {
    let dir = items_dir(root);
    if !dir.is_dir() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    let entries = fs::read_dir(&dir).map_err(|e| format!("read {}: {e}", dir.display()))?;
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let text = fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
        let item: QueueItem =
            serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", p.display()))?;
        out.push(item);
    }
    // Pending first, then newest first.
    out.sort_by(|a, b| {
        let ap = a.status != "pending";
        let bp = b.status != "pending";
        ap.cmp(&bp).then(b.created_at.cmp(&a.created_at))
    });
    Ok(out)
}

pub fn get(root: &Path, id: &str) -> Result<QueueItem, String> {
    let p = item_path(root, id)?;
    let text = fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", p.display()))
}

pub fn add(root: &Path, item: &QueueItem) -> Result<(), String> {
    let p = item_path(root, &item.id)?;
    if p.exists() {
        return Err(format!("queue item {} already exists", item.id));
    }
    fs::create_dir_all(items_dir(root)).map_err(|e| e.to_string())?;
    write_item(&p, item)
}

fn write_item(p: &Path, item: &QueueItem) -> Result<(), String> {
    let text = serde_json::to_string_pretty(item).map_err(|e| e.to_string())?;
    let tmp = p.with_extension("json.tmp");
    fs::write(&tmp, text).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    fs::rename(&tmp, p).map_err(|e| format!("rename {}: {e}", p.display()))
}

/// Record Andrew's decision. Approval is refused when the item's verification
/// gate did not pass, so a failed gate can never be approved by accident.
pub fn decide(root: &Path, id: &str, verdict: &str, note: &str) -> Result<QueueItem, String> {
    if verdict != "approved" && verdict != "rejected" {
        return Err(format!("verdict must be approved or rejected, got {verdict:?}"));
    }
    let p = item_path(root, id)?;
    let mut item = get(root, id)?;
    if item.status != "pending" {
        return Err(format!("item {id} is already {}", item.status));
    }
    if verdict == "approved" && !item.gate.passed {
        return Err(format!(
            "item {id} failed its verification gate and cannot be approved; reject it and rerun"
        ));
    }
    let decision = Decision {
        verdict: verdict.to_string(),
        by: "Andrew".to_string(),
        at: crate::now_rfc3339(),
        note: note.to_string(),
    };
    item.status = verdict.to_string();
    item.decision = Some(decision.clone());
    write_item(&p, &item)?;
    let log = root.join("queue").join("decisions.jsonl");
    let line = serde_json::json!({ "id": id, "title": item.title, "decision": decision });
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log)
        .map_err(|e| format!("open {}: {e}", log.display()))?;
    writeln!(f, "{line}").map_err(|e| e.to_string())?;
    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_root(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("studio-q-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn item(id: &str, passed: bool) -> QueueItem {
        QueueItem {
            id: id.into(),
            kind: "build".into(),
            title: "t".into(),
            summary: String::new(),
            created_at: crate::now_rfc3339(),
            created_by: "builder".into(),
            run_id: None,
            artifacts: vec![],
            gate: Gate { passed, checks: vec![] },
            status: "pending".into(),
            decision: None,
        }
    }

    #[test]
    fn empty_queue_lists_nothing() {
        let r = tmp_root("empty");
        assert!(list(&r).unwrap().is_empty());
    }

    #[test]
    fn approve_and_reject_flow() {
        let r = tmp_root("flow");
        add(&r, &item("a1", true)).unwrap();
        add(&r, &item("a2", false)).unwrap();
        assert_eq!(list(&r).unwrap().len(), 2);
        assert!(decide(&r, "a2", "approved", "").is_err(), "failed gate must not approve");
        let a = decide(&r, "a1", "approved", "looks good").unwrap();
        assert_eq!(a.status, "approved");
        assert!(decide(&r, "a1", "rejected", "").is_err(), "no double decisions");
        let b = decide(&r, "a2", "rejected", "gate failed").unwrap();
        assert_eq!(b.status, "rejected");
        let log = fs::read_to_string(r.join("queue/decisions.jsonl")).unwrap();
        assert_eq!(log.lines().count(), 2);
    }

    #[test]
    fn rejects_path_traversal_ids() {
        let r = tmp_root("trav");
        assert!(get(&r, "../etc/passwd").is_err());
    }
}
