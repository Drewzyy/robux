//! Money: Robux earned per game, converted to US dollars at the DevEx rate.
//! Entries come from Andrew (typed in, or a CSV exported from the Creator
//! Dashboard); BrewixGames never signs in to Roblox to fetch them.
//! Stored in <studio>/earnings/earnings.json, outside the vault, so app
//! updates never touch it.
//!
//! Rates (Roblox Creator Docs, developer-exchange.md and
//! 18-plus-devex-rate.md, read 2026-11-14): standard 0.0038 USD per Earned
//! Robux since 2025-09-05; 0.0054 only for certain purchases by age-verified
//! US adults in eligible games. 30,000 Earned Robux = $114 at the standard rate.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const STANDARD_RATE: f64 = 0.0038;
pub const DEVEX_MINIMUM: u64 = 30_000;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Entry {
    pub date: String, // YYYY-MM-DD
    pub game: String,
    pub robux: i64,
    #[serde(default)]
    pub note: String,
}

#[derive(Serialize, Debug, Default)]
pub struct Summary {
    pub rate: f64,
    pub total_robux: i64,
    pub total_usd: f64,
    pub this_month_robux: i64,
    pub this_month_usd: f64,
    pub last_30_robux: i64,
    pub per_game: Vec<(String, i64, f64)>,
    pub per_month: Vec<(String, i64, f64)>,
    pub devex_minimum: u64,
    pub entries: Vec<Entry>,
}

fn file(root: &Path) -> PathBuf {
    root.join("earnings").join("earnings.json")
}

pub fn load(root: &Path) -> Vec<Entry> {
    fs::read_to_string(file(root)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default()
}

fn save(root: &Path, all: &[Entry]) -> Result<(), String> {
    let f = file(root);
    fs::create_dir_all(f.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(f, serde_json::to_string_pretty(all).unwrap()).map_err(|e| e.to_string())
}

fn valid_date(d: &str) -> bool {
    chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").is_ok()
}

pub fn add(root: &Path, e: Entry) -> Result<(), String> {
    if !valid_date(&e.date) {
        return Err("date must look like 2026-11-14".into());
    }
    if e.game.trim().is_empty() {
        return Err("name the game".into());
    }
    let mut all = load(root);
    all.push(Entry { game: e.game.trim().to_string(), note: e.note.trim().to_string(), ..e });
    save(root, &all)
}

pub fn remove(root: &Path, index: usize) -> Result<(), String> {
    let mut all = load(root);
    if index >= all.len() {
        return Err("no such entry".into());
    }
    all.remove(index);
    save(root, &all)
}

/// Import a CSV: needs a date column and a Robux/amount/revenue column; an
/// experience/game/universe column is used when present, else `default_game`.
/// Duplicate rows (same date, game, Robux) are skipped. Returns rows added.
pub fn import_csv(root: &Path, text: &str, default_game: &str) -> Result<usize, String> {
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let header = lines.next().ok_or("the file is empty")?;
    let cols: Vec<String> = split_csv(header).into_iter().map(|c| c.to_lowercase()).collect();
    let find = |words: &[&str]| cols.iter().position(|c| words.iter().any(|w| c.contains(w)));
    let di = find(&["date", "day"]).ok_or("no date column found")?;
    let ri = find(&["robux", "amount", "revenue", "earnings"]).ok_or("no Robux or amount column found")?;
    let gi = find(&["experience", "game", "universe", "place"]);
    let mut all = load(root);
    let mut added = 0;
    for l in lines {
        let f = split_csv(l);
        let date = f.get(di).map(|d| d.trim().chars().take(10).collect::<String>()).unwrap_or_default();
        let robux: i64 = match f.get(ri).map(|r| r.replace([',', ' '], "")).and_then(|r| r.parse::<f64>().ok()) {
            Some(v) => v.round() as i64,
            None => continue,
        };
        let game = gi.and_then(|i| f.get(i)).map(|g| g.trim().to_string()).filter(|g| !g.is_empty()).unwrap_or_else(|| default_game.to_string());
        if !valid_date(&date) || game.is_empty() {
            continue;
        }
        let e = Entry { date, game, robux, note: "imported".into() };
        if all.iter().any(|x| x.date == e.date && x.game == e.game && x.robux == e.robux) {
            continue;
        }
        all.push(e);
        added += 1;
    }
    save(root, &all)?;
    Ok(added)
}

fn split_csv(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut q) = (vec![], String::new(), false);
    let mut it = line.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '"' if q && it.peek() == Some(&'"') => {
                cur.push('"');
                it.next();
            }
            '"' => q = !q,
            ',' if !q => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

pub fn summary(root: &Path, rate: f64, today: chrono::NaiveDate) -> Summary {
    let all = load(root);
    let usd = |r: i64| (r as f64 * rate * 100.0).round() / 100.0;
    let month = today.format("%Y-%m").to_string();
    let since = today - chrono::Duration::days(30);
    let (mut games, mut months): (BTreeMap<String, i64>, BTreeMap<String, i64>) = Default::default();
    let (mut total, mut this_month, mut last30) = (0, 0, 0);
    for e in &all {
        total += e.robux;
        *games.entry(e.game.clone()).or_default() += e.robux;
        *months.entry(e.date[..7].to_string()).or_default() += e.robux;
        if e.date.starts_with(&month) {
            this_month += e.robux;
        }
        if chrono::NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").map(|d| d > since && d <= today).unwrap_or(false) {
            last30 += e.robux;
        }
    }
    let mut per_game: Vec<(String, i64, f64)> = games.into_iter().map(|(g, r)| (g, r, usd(r))).collect();
    per_game.sort_by(|a, b| b.1.cmp(&a.1));
    Summary {
        rate,
        total_robux: total,
        total_usd: usd(total),
        this_month_robux: this_month,
        this_month_usd: usd(this_month),
        last_30_robux: last30,
        per_game,
        per_month: months.into_iter().map(|(m, r)| (m, r, usd(r))).collect(),
        devex_minimum: DEVEX_MINIMUM,
        entries: all,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_conversion_and_csv_import() {
        let root = std::env::temp_dir().join(format!("brewix-money-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        add(&root, Entry { date: "2026-11-02".into(), game: "Scrap Garden".into(), robux: 30_000, note: String::new() }).unwrap();
        assert!(add(&root, Entry { date: "11/02/2026".into(), game: "x".into(), robux: 1, note: String::new() }).is_err());
        let csv = "Date,Experience,Robux Earned\n2026-10-15,\"Rule Bender, Run\",\"1,200\"\n2026-11-10,Scrap Garden,800\n2026-11-10,Scrap Garden,800\nbad,row,x\n";
        assert_eq!(import_csv(&root, csv, "fallback").unwrap(), 2, "duplicate and bad rows skipped");
        let s = summary(&root, STANDARD_RATE, chrono::NaiveDate::from_ymd_opt(2026, 11, 14).unwrap());
        assert_eq!(s.total_robux, 32_000);
        assert_eq!(s.total_usd, 121.6);
        assert_eq!(s.this_month_robux, 30_800);
        assert_eq!(s.last_30_robux, 30_800);
        assert_eq!(s.per_game[0], ("Scrap Garden".into(), 30_800, 117.04));
        assert_eq!(s.per_game[1].0, "Rule Bender, Run");
        assert_eq!(summary(&root, STANDARD_RATE, chrono::NaiveDate::from_ymd_opt(2026, 11, 14).unwrap()).per_month.len(), 2);
        assert_eq!((30_000.0 * STANDARD_RATE * 100.0_f64).round() / 100.0, 114.0, "matches Roblox's $114 for 30,000");
        remove(&root, 0).unwrap();
        assert_eq!(load(&root).len(), 2);
        let _ = fs::remove_dir_all(&root);
    }
}
