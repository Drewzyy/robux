//! Free local models through Ollama: is it installed and running, which
//! models are on this computer, and which one fits this computer's memory.
//! Nothing is downloaded unless Andrew presses Download for a named model.

use serde::Serialize;
use std::io::{BufRead, BufReader};

/// Where Ollama listens. Uses OLLAMA_HOST when set, and the IPv4 address
/// rather than "localhost": on Windows "localhost" can resolve to IPv6
/// first, which Ollama does not listen on, so it looked "not running".
pub fn base() -> String {
    static B: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    B.get_or_init(|| host_to_base(&std::env::var("OLLAMA_HOST").unwrap_or_default())).clone()
}

pub fn host_to_base(h: &str) -> String {
    let h = h.trim().trim_end_matches('/');
    if h.is_empty() {
        return "http://127.0.0.1:11434".into();
    }
    let (scheme, rest) = h.split_once("://").map(|(s, r)| (s.to_string(), r.to_string())).unwrap_or(("http".into(), h.to_string()));
    let (host, port) = match rest.rsplit_once(':') {
        Some((a, p)) if p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty() => (a.to_string(), p.to_string()),
        _ => (rest.clone(), "11434".into()),
    };
    let host = match host.as_str() {
        "" | "0.0.0.0" | "localhost" | "[::]" | "::" => "127.0.0.1".to_string(),
        h => h.to_string(),
    };
    format!("{scheme}://{host}:{port}")
}
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

/// Total memory in GB, measured once per app run (the Windows check starts
/// PowerShell, which is slow).
pub fn memory_gb() -> Option<f64> {
    static MEM: std::sync::OnceLock<Option<f64>> = std::sync::OnceLock::new();
    *MEM.get_or_init(measure_memory_gb)
}

fn measure_memory_gb() -> Option<f64> {
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
    ureq::get(&format!("{}{path}", base()))
        .timeout(std::time::Duration::from_millis(3000))
        .call()
        .map_err(|e| e.to_string())?
        .into_json()
        .map_err(|e| e.to_string())
}

/// Where Ollama is installed, if it is.
pub fn binary() -> Option<std::path::PathBuf> {
    let mut c = vec![];
    if cfg!(windows) {
        if let Some(l) = std::env::var_os("LOCALAPPDATA") {
            c.push(std::path::PathBuf::from(l).join("Programs/Ollama/ollama.exe"));
        }
    } else if cfg!(target_os = "macos") {
        c.push("/Applications/Ollama.app/Contents/Resources/ollama".into());
        c.push("/opt/homebrew/bin/ollama".into());
        c.push("/usr/local/bin/ollama".into());
    } else {
        c.push("/usr/local/bin/ollama".into());
        c.push("/usr/bin/ollama".into());
    }
    c.into_iter().find(|p| p.is_file()).or_else(|| {
        crate::paths::command(if cfg!(windows) { "where" } else { "which" }).arg("ollama").output().ok().filter(|o| o.status.success()).and_then(|o| String::from_utf8_lossy(&o.stdout).lines().next().map(|l| l.trim().into()))
    })
}

/// Auto-connect: if Ollama is installed but not running, start it quietly
/// (no window). Returns true when Ollama answers.
pub fn ensure_running() -> bool {
    if get("/api/version").is_ok() {
        return true;
    }
    let Some(bin) = binary() else { return false };
    if crate::paths::command(&bin).arg("serve").stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn().is_err() {
        return false;
    }
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        if get("/api/version").is_ok() {
            return true;
        }
    }
    false
}

pub fn status() -> Status {
    let memory_gb = memory_gb();
    let recommended = recommend(memory_gb);
    let probe = get("/api/version");
    let probe_err = probe.as_ref().err().cloned().unwrap_or_default();
    let version = probe.ok().and_then(|v| v.get("version").and_then(|x| x.as_str()).map(String::from));
    let running = version.is_some();
    let version_ok = version.as_deref().and_then(parse_version).map(|v| v >= MIN_VERSION).unwrap_or(false);
    // Skip the second call when Ollama is not there (saves the timeout).
    let mut installed: Vec<String> = running
        .then(|| get("/api/tags").ok())
        .flatten()
        .and_then(|v| v.get("models").and_then(|m| m.as_array()).cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(String::from))
        .collect();
    installed.sort();
    let message = if !running {
        if binary().is_some() {
            format!("Ollama is installed but did not answer at {} ({}). Quit Ollama from its tray icon and open it again, then press Check Again.", base(), probe_err.chars().take(80).collect::<String>())
        } else {
            "Ollama is not running. Install it from ollama.com (free), open it, then come back.".to_string()
        }
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
    let resp = ureq::post(&format!("{}/api/pull", base()))
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
    #[test]
    fn ollama_host_forms() {
        assert_eq!(host_to_base(""), "http://127.0.0.1:11434");
        assert_eq!(host_to_base("0.0.0.0"), "http://127.0.0.1:11434");
        assert_eq!(host_to_base("0.0.0.0:11500"), "http://127.0.0.1:11500");
        assert_eq!(host_to_base("http://localhost:11434/"), "http://127.0.0.1:11434");
        assert_eq!(host_to_base("192.168.1.5"), "http://192.168.1.5:11434");
    }
}
