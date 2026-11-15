//! Claude Code usage by day, for the Dashboard heatmap (the idea comes from
//! seunggabi/ai-heatmap, MIT, which reads the same logs through ccusage).
//! Sources: Claude Code's own session logs (~/.claude/projects/**/*.jsonl,
//! every assistant message with its token usage) plus BrewixGames' agent
//! runs (they run without session logs; their usage is in result.json).
//! Each log file is parsed once and cached by size and modified time.
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Day {
    pub input: u64,
    pub output: u64,
    pub cache_write: u64,
    pub cache_read: u64,
    pub cost: f64,
    pub messages: u64,
}

impl Day {
    fn add(&mut self, o: &Day) {
        self.input += o.input;
        self.output += o.output;
        self.cache_write += o.cache_write;
        self.cache_read += o.cache_read;
        self.cost += o.cost;
        self.messages += o.messages;
    }
    pub fn total(&self) -> u64 {
        self.input + self.output + self.cache_write + self.cache_read
    }
}

fn claude_dirs() -> Vec<PathBuf> {
    let mut v = vec![];
    if let Ok(d) = std::env::var("CLAUDE_CONFIG_DIR") {
        for p in d.split([',', ';']) {
            v.push(PathBuf::from(p.trim()).join("projects"));
        }
    }
    if let Some(h) = crate::paths::home() {
        v.push(h.join(".claude/projects"));
        v.push(h.join(".config/claude/projects"));
    }
    v.retain(|p| p.is_dir());
    v.dedup();
    v
}

fn jsonl_files(d: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(d).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            jsonl_files(&p, out);
        } else if p.extension().map(|x| x == "jsonl").unwrap_or(false) {
            out.push(p);
        }
    }
}

fn local_date(ts: &str) -> Option<String> {
    let t = chrono::DateTime::parse_from_rfc3339(ts).ok()?;
    Some(t.with_timezone(&chrono::Local).format("%Y-%m-%d").to_string())
}

fn price(prices: &BTreeMap<String, crate::cost::ModelPrice>, model: &str) -> Option<crate::cost::ModelPrice> {
    prices.get(model).cloned().or_else(|| {
        let fam = ["haiku", "sonnet", "opus", "fable"].into_iter().find(|f| model.contains(f))?;
        prices.iter().find(|(k, _)| k.contains(fam)).map(|(_, p)| p.clone())
    })
}

fn cost_of(d: &Day, p: Option<crate::cost::ModelPrice>) -> f64 {
    let Some(p) = p else { return 0.0 };
    (d.input as f64 * p.input + d.output as f64 * p.output + d.cache_read as f64 * p.cache_read + d.cache_write as f64 * p.input * p.cache_write_multiplier) / 1_000_000.0
}

/// Per-day usage in one session log. Messages are counted once even when a
/// log repeats them (same message id and request id).
pub fn parse_log(path: &Path, prices: &BTreeMap<String, crate::cost::ModelPrice>, seen: &mut HashSet<String>) -> BTreeMap<String, Day> {
    let mut days: BTreeMap<String, Day> = BTreeMap::new();
    let Ok(f) = fs::File::open(path) else { return days };
    for line in BufReader::new(f).lines().map_while(Result::ok) {
        if !line.contains("\"usage\"") {
            continue;
        }
        let Ok(v) = serde_json::from_str::<Value>(&line) else { continue };
        let u = &v["message"]["usage"];
        if !u.is_object() {
            continue;
        }
        let key = format!("{}:{}", v["message"]["id"].as_str().unwrap_or(""), v["requestId"].as_str().unwrap_or(""));
        if key != ":" && !seen.insert(key) {
            continue;
        }
        let Some(date) = v["timestamp"].as_str().and_then(local_date) else { continue };
        let n = |k: &str| u[k].as_u64().unwrap_or(0);
        let mut d = Day { input: n("input_tokens"), output: n("output_tokens"), cache_write: n("cache_creation_input_tokens"), cache_read: n("cache_read_input_tokens"), cost: 0.0, messages: 1 };
        d.cost = v["costUSD"].as_f64().unwrap_or_else(|| cost_of(&d, price(prices, v["message"]["model"].as_str().unwrap_or(""))));
        days.entry(date).or_default().add(&d);
    }
    days
}

/// Agent runs (no session logs): usage from each run's result.json.
fn runs_usage(root: &Path) -> BTreeMap<String, Day> {
    let mut days: BTreeMap<String, Day> = BTreeMap::new();
    for r in crate::runs::list(root).unwrap_or_default() {
        if r.chat.is_some() {
            continue; // chat keeps a session log, already counted
        }
        let Ok(t) = fs::read_to_string(root.join("runs").join(&r.run_id).join("result.json")) else { continue };
        let Ok(v) = serde_json::from_str::<Value>(&t) else { continue };
        let u = &v["usage"];
        let date = r.finished_at.as_deref().and_then(local_date).unwrap_or_default();
        if date.is_empty() || !u.is_object() {
            continue;
        }
        let n = |k: &str| u[k].as_u64().unwrap_or(0);
        let d = Day { input: n("input_tokens"), output: n("output_tokens"), cache_write: n("cache_creation_input_tokens"), cache_read: n("cache_read_input_tokens"), cost: v["total_cost_usd"].as_f64().unwrap_or(0.0), messages: v["num_turns"].as_u64().unwrap_or(1) };
        days.entry(date).or_default().add(&d);
    }
    days
}

/// Every day with usage, newest last, plus lifetime totals.
pub fn heatmap(root: &Path) -> Value {
    let prices = crate::cost::load_pricing(root).map(|p| p.models).unwrap_or_default();
    let cache_path = root.join(".cache/usage-cache.json");
    let mut cache: HashMap<String, (u64, u128, BTreeMap<String, Day>)> = fs::read_to_string(&cache_path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let mut files = vec![];
    for d in claude_dirs() {
        jsonl_files(&d, &mut files);
    }
    let mut days: BTreeMap<String, Day> = BTreeMap::new();
    let mut seen = HashSet::new();
    let mut fresh = HashMap::new();
    for f in &files {
        let key = f.display().to_string();
        let meta = fs::metadata(f).ok();
        let (size, mtime) = meta.map(|m| (m.len(), m.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_millis()).unwrap_or(0))).unwrap_or((0, 0));
        let per = match cache.remove(&key) {
            Some((s, t, d)) if s == size && t == mtime => d,
            _ => parse_log(f, &prices, &mut seen),
        };
        for (k, v) in &per {
            days.entry(k.clone()).or_default().add(v);
        }
        fresh.insert(key, (size, mtime, per));
    }
    let _ = fs::create_dir_all(root.join(".cache"));
    let _ = fs::write(&cache_path, serde_json::to_string(&fresh).unwrap_or_default());
    for (k, v) in runs_usage(root) {
        days.entry(k).or_default().add(&v);
    }
    let mut life = Day::default();
    for d in days.values() {
        life.add(d);
    }
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let t = days.get(&today).cloned().unwrap_or_default();
    let busiest = days.iter().max_by_key(|(_, d)| d.total()).map(|(k, d)| json!({ "date": k, "tokens": d.total() }));
    // Streak of days in a row with usage, ending today or yesterday.
    let mut streak = 0;
    let mut day = chrono::Local::now().date_naive();
    if !days.contains_key(&today) {
        day = day.pred_opt().unwrap_or(day);
    }
    while days.get(&day.format("%Y-%m-%d").to_string()).map(|d| d.total() > 0).unwrap_or(false) {
        streak += 1;
        day = day.pred_opt().unwrap_or(day);
    }
    json!({
        "days": days.iter().map(|(k, d)| json!({ "date": k, "tokens": d.total(), "input": d.input, "output": d.output, "cache_read": d.cache_read, "cache_write": d.cache_write, "cost": (d.cost * 100.0).round() / 100.0, "messages": d.messages })).collect::<Vec<_>>(),
        "lifetime": { "tokens": life.total(), "input": life.input, "output": life.output, "cache_read": life.cache_read, "cache_write": life.cache_write, "cost": (life.cost * 100.0).round() / 100.0, "messages": life.messages, "days_active": days.values().filter(|d| d.total() > 0).count() },
        "today": { "tokens": t.total(), "cost": (t.cost * 100.0).round() / 100.0 },
        "busiest": busiest,
        "streak": streak,
        "sources": files.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts_each_message_once_by_day() {
        let d = std::env::temp_dir().join(format!("usage-t-{}.jsonl", std::process::id()));
        let line = |id: &str, ts: &str| format!("{{\"timestamp\":\"{ts}\",\"requestId\":\"r-{id}\",\"message\":{{\"id\":\"{id}\",\"model\":\"claude-sonnet-5-5\",\"usage\":{{\"input_tokens\":10,\"output_tokens\":20,\"cache_creation_input_tokens\":30,\"cache_read_input_tokens\":40}}}}}}\n");
        fs::write(&d, format!("{}{}{}{{\"type\":\"user\"}}\n", line("a", "2026-11-14T12:00:00Z"), line("a", "2026-11-14T12:00:00Z"), line("b", "2026-11-14T13:00:00Z"))).unwrap();
        let mut prices = BTreeMap::new();
        prices.insert("claude-sonnet-5-5".to_string(), crate::cost::ModelPrice { input: 2.0, output: 10.0, cache_read: 0.2, cache_write_multiplier: 2.0 });
        let days = parse_log(&d, &prices, &mut HashSet::new());
        let day = days.values().next().unwrap();
        assert_eq!(day.messages, 2);
        assert_eq!(day.total(), 200);
        assert!(day.cost > 0.0);
        let _ = fs::remove_file(&d);
    }
}
