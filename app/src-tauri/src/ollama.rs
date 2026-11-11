//! Free local models through Ollama: is it installed and running, which
//! models are on this computer, and which one fits this computer's memory.
//! Nothing is downloaded unless Andrew presses Download for a named model.

use serde::Serialize;
use std::io::{BufRead, BufReader};

pub const BASE: &str = "http://localhost:11434";
/// Oldest Ollama that speaks the Anthropic API Claude Code uses.
pub const MIN_VERSION: (u32, u32) = (0, 14);

#[derive(Serialize, Clone, Debug)]
pub struct Pick {
    pub model: String,
    pub size_gb: f64,
    pub why: String,
}

#[derive(Serialize, Debug)]
pub struct Status {
    pub running: bool,
    pub version: Option<String>,
    pub version_ok: bool,
    pub installed: Vec<String>,
    pub memory_gb: Option<f64>,
    pub recommended: Pick,
    pub choices: Vec<Pick>,
    pub message: String,
}

/// Models that handle tool calls well enough for the agents, smallest first.
/// Sizes are the download size of the default quantisation (about).
pub fn choices() -> Vec<Pick> {
    vec![
        Pick { model: "qwen3:4b".into(), size_gb: 2.5, why: "Fits 8 GB computers. Fine for chat; weak for building games.".into() },
        Pick { model: "qwen3:8b".into(), size_gb: 5.2, why: "Good for 16 GB computers. Handles most single-agent jobs.".into() },
        Pick { model: "qwen3:14b".into(), size_gb: 9.3, why: "For 32 GB computers. Noticeably better at multi-step work.".into() },
        Pick { model: "qwen3-coder:30b".into(), size_gb: 19.0, why: "For 48 GB or more. The strongest local option for writing Luau.".into() },
    ]
}

/// The biggest model that leaves room for the OS and Roblox Studio.
pub fn recommend(memory_gb: Option<f64>) -> Pick {
    let c = choices();
    let m = memory_gb.unwrap_or(8.0);
    let i = if m >= 48.0 {
        3
    } else if m >= 28.0 {
        2
    } else if m >= 14.0 {
        1
    } else {
        0
    };
    c[i].clone()
}

pub fn memory_gb() -> Option<f64> {
    let bytes: f64 = if cfg!(target_os = "linux") {
        let t = std::fs::read_to_string("/proc/meminfo").ok()?;
        let kb: f64 = t.lines().find(|l| l.starts_with("MemTotal:"))?.split_whitespace().nth(1)?.parse().ok()?;
        kb * 1024.0
    } else if cfg!(target_os = "macos") {
        let o = crate::paths::command("sysctl").args(["-n", "hw.memsize"]).output().ok()?;
        String::from_utf8_lossy(&o.stdout).trim().parse().ok()?
    } else if cfg!(windows) {
        let o = crate::paths::command("powershell")
            .args(["-NoProfile", "-Command", "(Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory"])
            .output()
            .ok()?;
        String::from_utf8_lossy(&o.stdout).trim().parse().ok()?
    } else {
        return None;
    };
    Some((bytes / 1_073_741_824.0 * 10.0).round() / 10.0)
}

fn parse_version(v: &str) -> Option<(u32, u32)> {
    let mut it = v.trim_start_matches('v').split('.');
    Some((it.next()?.parse().ok()?, it.next()?.split(|c: char| !c.is_ascii_digit()).next()?.parse().ok()?))
}

fn get(path: &str) -> Result<serde_json::Value, String> {
    ureq::get(&format!("{BASE}{path}"))
        .timeout(std::time::Duration::from_secs(3))
        .call()
        .map_err(|e| e.to_string())?
        .into_json()
        .map_err(|e| e.to_string())
}

pub fn status() -> Status {
    let memory_gb = memory_gb();
    let recommended = recommend(memory_gb);
    let version = get("/api/version").ok().and_then(|v| v.get("version").and_then(|x| x.as_str()).map(String::from));
    let running = version.is_some();
    let version_ok = version.as_deref().and_then(parse_version).map(|v| v >= MIN_VERSION).unwrap_or(false);
    let mut installed: Vec<String> = get("/api/tags")
        .ok()
        .and_then(|v| v.get("models").and_then(|m| m.as_array()).cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(String::from))
        .collect();
    installed.sort();
    let message = if !running {
        "Ollama is not running. Install it from ollama.com (free), open it, then come back.".to_string()
    } else if !version_ok {
        format!("Ollama {} is too old. Update to {}.{} or newer.", version.clone().unwrap_or_default(), MIN_VERSION.0, MIN_VERSION.1)
    } else if installed.is_empty() {
        format!("Ollama is ready. No models yet: {} is the best fit for this computer.", recommended.model)
    } else {
        format!("Ollama is ready with {} model(s).", installed.len())
    };
    Status { running, version, version_ok, installed, memory_gb, recommended, choices: choices(), message }
}

/// Download one model. Only models from the list above are accepted, so a
/// typo can never start a surprise multi-gigabyte download.
pub fn pull(model: &str, on_line: &mut dyn FnMut(&str)) -> Result<(), String> {
    if !choices().iter().any(|c| c.model == model) {
        return Err(format!("{model} is not on the list of supported local models"));
    }
    let resp = ureq::post(&format!("{BASE}/api/pull"))
        .timeout(std::time::Duration::from_secs(6 * 3600))
        .send_json(serde_json::json!({ "model": model, "stream": true }))
        .map_err(|e| format!("could not reach Ollama: {e}"))?;
    let mut last = String::new();
    for line in BufReader::new(resp.into_reader()).lines().map_while(Result::ok) {
        let v: serde_json::Value = serde_json::from_str(&line).unwrap_or_default();
        if let Some(e) = v.get("error").and_then(|x| x.as_str()) {
            return Err(e.to_string());
        }
        let st = v.get("status").and_then(|x| x.as_str()).unwrap_or("");
        let pct = match (v.get("completed").and_then(|x| x.as_f64()), v.get("total").and_then(|x| x.as_f64())) {
            (Some(c), Some(t)) if t > 0.0 => format!(" {:.0}%", c / t * 100.0),
            _ => String::new(),
        };
        let msg = format!("{st}{pct}");
        if msg != last {
            on_line(&msg);
            last = msg;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_fit_memory() {
        assert_eq!(recommend(Some(8.0)).model, "qwen3:4b");
        assert_eq!(recommend(Some(16.0)).model, "qwen3:8b");
        assert_eq!(recommend(Some(32.0)).model, "qwen3:14b");
        assert_eq!(recommend(Some(64.0)).model, "qwen3-coder:30b");
        assert_eq!(parse_version("0.14.2"), Some((0, 14)));
        assert!(parse_version("0.13.9").unwrap() < MIN_VERSION);
    }

    #[test]
    fn unknown_model_is_refused() {
        assert!(pull("some-huge-model:400b", &mut |_| {}).is_err());
    }
}
