//! Full-text search over the vault: a SQLite FTS5 index in
//! .cache/vault-index.sqlite, rebuilt automatically whenever a note changes.
//! Used by the Vault search box and by agents through `brewix search`.

use rusqlite::{params, Connection};
use serde::Serialize;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

#[derive(Serialize, Debug, Clone)]
pub struct Hit {
    pub path: String,
    pub section: String,
    pub title: String,
    pub snippet: String,
    pub score: f64,
}

fn notes(root: &Path) -> Vec<PathBuf> {
    fn walk(d: &Path, out: &mut Vec<PathBuf>) {
        if let Ok(rd) = fs::read_dir(d) {
            for e in rd.flatten() {
                let p = e.path();
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with('.') {
                    continue; // .claude (agents and skills), .obsidian
                }
                if p.is_dir() {
                    walk(&p, out);
                } else if name.ends_with(".md") {
                    out.push(p);
                }
            }
        }
    }
    let mut out = vec![];
    walk(&root.join("vault"), &mut out);
    out.sort();
    out
}

fn fingerprint(files: &[PathBuf]) -> String {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for f in files {
        f.hash(&mut h);
        if let Ok(m) = fs::metadata(f) {
            m.len().hash(&mut h);
            m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_nanos()).hash(&mut h);
        }
    }
    format!("{:x}", h.finish())
}

/// Title from front matter `title:` or the first `# ` heading; body without
/// the front matter.
fn split_note(text: &str) -> (String, String) {
    let mut title = String::new();
    let mut body = text;
    if let Some(rest) = text.strip_prefix("---") {
        if let Some(end) = rest.find("\n---") {
            for l in rest[..end].lines() {
                if let Some(t) = l.strip_prefix("title:") {
                    title = t.trim().trim_matches('"').to_string();
                }
            }
            body = &rest[end + 4..];
        }
    }
    if title.is_empty() {
        title = body.lines().find_map(|l| l.strip_prefix("# ")).unwrap_or("").trim().to_string();
    }
    (title, body.to_string())
}

pub fn index_path(root: &Path) -> PathBuf {
    root.join(".cache").join("vault-index.sqlite")
}

/// Open the index, rebuilding it if any note was added, removed, or edited.
pub fn open(root: &Path) -> Result<Connection, String> {
    let p = index_path(root);
    fs::create_dir_all(p.parent().unwrap()).map_err(|e| e.to_string())?;
    let mut db = Connection::open(&p).map_err(|e| format!("open search index: {e}"))?;
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS meta(k TEXT PRIMARY KEY, v TEXT);
         CREATE VIRTUAL TABLE IF NOT EXISTS notes USING fts5(path UNINDEXED, section UNINDEXED, title, body, tokenize='porter unicode61');",
    )
    .map_err(|e| e.to_string())?;
    let files = notes(root);
    let fp = fingerprint(&files);
    let have: Option<String> = db.query_row("SELECT v FROM meta WHERE k='fp'", [], |r| r.get(0)).ok();
    if have.as_deref() != Some(fp.as_str()) {
        let tx = db.transaction().map_err(|e| e.to_string())?;
        tx.execute("DELETE FROM notes", []).map_err(|e| e.to_string())?;
        for f in &files {
            let rel = f.strip_prefix(root).unwrap_or(f).to_string_lossy().replace('\\', "/");
            let section = rel.split('/').nth(1).unwrap_or("").to_string();
            let (title, body) = split_note(&fs::read_to_string(f).unwrap_or_default());
            tx.execute("INSERT INTO notes(path, section, title, body) VALUES (?1, ?2, ?3, ?4)", params![rel, section, title, body])
                .map_err(|e| e.to_string())?;
        }
        tx.execute("INSERT OR REPLACE INTO meta(k, v) VALUES ('fp', ?1)", params![fp]).map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
    }
    Ok(db)
}

/// Turn what someone typed into a safe FTS5 query: words only, each quoted,
/// the last one as a prefix. `any` joins with OR instead of AND.
fn fts_query(q: &str, any: bool) -> Option<String> {
    let words: Vec<String> = q
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();
    if words.is_empty() {
        return None;
    }
    let n = words.len();
    let parts: Vec<String> = words.iter().enumerate().map(|(i, w)| if i + 1 == n { format!("\"{w}\"*") } else { format!("\"{w}\"") }).collect();
    Some(parts.join(if any { " OR " } else { " " }))
}

/// Best matches first (BM25, titles weighted 5x). `sections` limits results
/// to those vault sections when not empty. Falls back to matching any word
/// when no note has all of them.
pub fn search(root: &Path, q: &str, sections: &[String], limit: usize) -> Result<Vec<Hit>, String> {
    let db = open(root)?;
    for any in [false, true] {
        let Some(fq) = fts_query(q, any) else { return Ok(vec![]) };
        let mut st = db
            .prepare(
                "SELECT path, section, title, snippet(notes, 3, char(1), char(2), ' ... ', 14), bm25(notes, 0, 0, 5.0, 1.0) AS s
                 FROM notes WHERE notes MATCH ?1 ORDER BY s LIMIT 200",
            )
            .map_err(|e| e.to_string())?;
        let hits: Vec<Hit> = st
            .query_map(params![fq], |r| {
                Ok(Hit { path: r.get(0)?, section: r.get(1)?, title: r.get(2)?, snippet: r.get::<_, String>(3)?.replace('\n', " ").replace("[[", "").replace("]]", ""), score: -r.get::<_, f64>(4)? })
            })
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|h| sections.is_empty() || sections.iter().any(|s| s == &h.section))
            .take(limit)
            .collect();
        if !hits.is_empty() || any {
            return Ok(hits);
        }
    }
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_and_ranks_and_refreshes() {
        let root = std::env::temp_dir().join(format!("brewix-search-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("vault/20-luau-rojo")).unwrap();
        fs::create_dir_all(root.join("vault/40-monetization")).unwrap();
        fs::create_dir_all(root.join("vault/.claude/skills")).unwrap();
        fs::write(root.join("vault/20-luau-rojo/datastore.md"), "---\ntitle: Saving player data\n---\nUse DataStoreService with UpdateAsync and retries.").unwrap();
        fs::write(root.join("vault/40-monetization/passes.md"), "# Gamepasses\nPrice gamepasses after the first session; mention data only once.").unwrap();
        fs::write(root.join("vault/.claude/skills/secret.md"), "datastore datastore datastore").unwrap();
        let h = search(&root, "datastore retries", &[], 5).unwrap();
        assert_eq!(h[0].path, "vault/20-luau-rojo/datastore.md");
        assert_eq!(h[0].title, "Saving player data");
        assert!(h.iter().all(|x| !x.path.contains(".claude")));
        // prefix on the last word, and the section filter
        assert_eq!(search(&root, "gamepa", &[], 5).unwrap()[0].section, "40-monetization");
        assert!(search(&root, "gamepasses", &["20-luau-rojo".into()], 5).unwrap().is_empty());
        // OR fallback when no note has every word
        assert!(!search(&root, "UpdateAsync gamepasses", &[], 5).unwrap().is_empty());
        // a new note is picked up without a manual rebuild
        fs::write(root.join("vault/40-monetization/robux.md"), "# Robux pricing\nDevEx rates.").unwrap();
        assert_eq!(search(&root, "devex", &[], 5).unwrap()[0].path, "vault/40-monetization/robux.md");
        // junk input never reaches FTS5 as syntax
        assert!(search(&root, "\" OR ( NEAR *", &[], 5).is_ok());
        let _ = fs::remove_dir_all(&root);
    }
}
