//! The Claude Code CLI bridge. Runs one agent as `claude -p --agent <name>`
//! inside the vault folder, streams its output, and records the result.
//!
//! Guard rails enforced here, not in the UI:
//! - the run must be `approved` (see `runs::approve`);
//! - `--max-budget-usd` is always set to the approved cap;
//! - permission mode is `dontAsk` with an explicit per-agent tool allowlist,
//!   so tools outside the list are denied instead of prompting;
//! - every file the agent reads is logged by the vault's PostToolUse hook to
//!   `runs/<run_id>/loads.jsonl`.

use crate::cost;
use crate::runs::{self, RunRequest};
use serde::Serialize;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize)]
pub struct RunOutcome {
    pub run_id: String,
    pub ok: bool,
    pub result_text: String,
    pub actual_cost_usd: Option<f64>,
    pub num_turns: Option<u64>,
    pub exit_code: Option<i32>,
    pub stderr_tail: String,
}

/// Build the exact argument list. Kept separate so it can be unit tested
/// and shown in the run log.
/// If the agent has a skill in `vault/.claude/skills/<agent>/`, the prompt is
/// sent as `/<agent> <instruction>` so Claude Code loads that skill
/// deterministically (skills are `disable-model-invocation`, so agents cannot
/// pull in other agents' skills on their own).
pub fn build_args(
    req: &RunRequest,
    allowed_tools: &[String],
    add_dirs: &[PathBuf],
    run_dir: &Path,
    has_skill: bool,
) -> Vec<String> {
    let prompt = if has_skill {
        format!("/{} {}", req.agent, req.prompt)
    } else {
        req.prompt.clone()
    };
    let context = format!(
        "Run context. Run id: {}. Your run folder is {} (absolute path). Write every output file there, \
         including gate.json. The studio's game projects folder is {}. You are running unattended: \
         nobody can answer questions mid-run, so put questions in your output under Open questions.",
        req.run_id,
        run_dir.display(),
        add_dirs.first().map(|p| p.display().to_string()).unwrap_or_default()
    );
    let mut a: Vec<String> = vec![
        "-p".into(),
        prompt,
        "--append-system-prompt".into(),
        context,
        "--agent".into(),
        req.agent.clone(),
        "--model".into(),
        req.estimate.model.clone(),
        "--output-format".into(),
        "stream-json".into(),
        "--verbose".into(),
        "--max-budget-usd".into(),
        format!("{:.2}", req.estimate.cap_usd),
        "--no-session-persistence".into(),
        // Only the vault's own settings (hooks, deny rules) and no MCP
        // servers: agents do not inherit plugins or tools from the user's
        // personal Claude Code setup.
        "--setting-sources".into(),
        "project".into(),
        "--strict-mcp-config".into(),
        "--permission-mode".into(),
        "dontAsk".into(),
    ];
    if !allowed_tools.is_empty() {
        a.push("--allowedTools".into());
        a.push(allowed_tools.join(" "));
    }
    for d in add_dirs {
        a.push("--add-dir".into());
        a.push(d.display().to_string());
    }
    a
}

/// Run an approved request to completion. `on_line` receives every stdout
/// line (stream-json) as it arrives.
pub fn run_approved<F: FnMut(&str)>(
    root: &Path,
    run_id: &str,
    claude_bin: &Path,
    on_line: F,
) -> Result<RunOutcome, String> {
    run_approved_with(root, run_id, claude_bin, &[], on_line)
}

/// Same as `run_approved`, with extra folders the agent may read (earlier
/// pipeline stages).
pub fn run_approved_with<F: FnMut(&str)>(
    root: &Path,
    run_id: &str,
    claude_bin: &Path,
    extra_dirs: &[PathBuf],
    mut on_line: F,
) -> Result<RunOutcome, String> {
    let mut req = runs::load(root, run_id)?;
    if req.status != "approved" {
        return Err(format!(
            "run {run_id} is {}; it must be approved before it can start",
            req.status
        ));
    }
    let agents = cost::load_agents(root)?;
    let profile = agents
        .agents
        .get(&req.agent)
        .ok_or_else(|| format!("unknown agent {}", req.agent))?;
    let dir = runs::run_dir(root, run_id)?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let vault = root.join("vault");
    let mut add_dirs = vec![root.join("projects"), dir.clone()];
    add_dirs.extend(extra_dirs.iter().cloned());
    let has_skill = vault
        .join(".claude/skills")
        .join(&req.agent)
        .join("SKILL.md")
        .is_file();
    let args = build_args(&req, &profile.allowed_tools, &add_dirs, &dir, has_skill);
    fs::write(
        dir.join("command.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "bin": claude_bin.display().to_string(),
            "cwd": vault.display().to_string(),
            "args": args,
        }))
        .unwrap(),
    )
    .map_err(|e| e.to_string())?;

    req.status = "running".into();
    runs::save(root, &req)?;

    let mut child = Command::new(claude_bin)
        .args(&args)
        .current_dir(&vault)
        .env("ROBUX_RUN_ID", run_id)
        .env("ROBUX_RUN_DIR", &dir)
        .env("ROBUX_AGENT", &req.agent)
        .env("ROBUX_STUDIO_ROOT", root)
        // Agents always run on Andrew's Claude login (his subscription),
        // never on an API key, even if one is set in the environment.
        .env_remove("ANTHROPIC_API_KEY")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            let msg = format!("could not start {}: {e}", claude_bin.display());
            req.status = "failed".into();
            req.error = Some(msg.clone());
            let _ = runs::save(root, &req);
            msg
        })?;

    // Drain stderr on a thread so a full pipe never blocks the child.
    let stderr = child.stderr.take().unwrap();
    let err_path = dir.join("stderr.log");
    let err_thread = std::thread::spawn(move || {
        let mut buf = String::new();
        let mut f = fs::File::create(&err_path).ok();
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            if let Some(f) = f.as_mut() {
                let _ = writeln!(f, "{line}");
            }
            buf.push_str(&line);
            buf.push('\n');
        }
        buf
    });

    let stdout = child.stdout.take().unwrap();
    let mut stream_log = fs::File::create(dir.join("stream.jsonl")).map_err(|e| e.to_string())?;
    let mut final_result: Option<serde_json::Value> = None;
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        let _ = writeln!(stream_log, "{line}");
        on_line(&line);
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
            if v.get("type").and_then(|t| t.as_str()) == Some("result") {
                final_result = Some(v);
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    let stderr_all = err_thread.join().unwrap_or_default();
    let stderr_tail: String = stderr_all
        .lines()
        .rev()
        .take(20)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");

    let (result_text, cost, turns, is_error) = match &final_result {
        Some(v) => (
            v.get("result").and_then(|x| x.as_str()).unwrap_or("").to_string(),
            v.get("total_cost_usd").and_then(|x| x.as_f64()),
            v.get("num_turns").and_then(|x| x.as_u64()),
            v.get("is_error").and_then(|x| x.as_bool()).unwrap_or(true),
        ),
        None => (String::new(), None, None, true),
    };
    if let Some(v) = &final_result {
        let _ = fs::write(dir.join("result.json"), serde_json::to_string_pretty(v).unwrap());
    }
    let ok = status.success() && !is_error && final_result.is_some();

    req.status = if ok { "done".into() } else { "failed".into() };
    req.finished_at = Some(crate::now_rfc3339());
    req.actual_cost_usd = cost;
    req.result_text = Some(result_text.clone());
    if !ok {
        req.error = Some(if final_result.is_none() {
            format!("no result line from claude (exit {:?}). stderr:\n{stderr_tail}", status.code())
        } else {
            format!("claude reported an error (exit {:?})", status.code())
        });
    }
    runs::save(root, &req)?;

    Ok(RunOutcome {
        run_id: run_id.to_string(),
        ok,
        result_text,
        actual_cost_usd: cost,
        num_turns: turns,
        exit_code: status.code(),
        stderr_tail,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cost::Estimate;

    #[test]
    fn args_always_carry_cap_and_dont_ask() {
        let req = RunRequest {
            run_id: "run-x".into(),
            agent: "lead".into(),
            prompt: "hi".into(),
            estimate: Estimate {
                agent: "lead".into(),
                model: "claude-sonnet-5-5".into(),
                low_usd: 0.0,
                expected_usd: 0.0,
                high_usd: 0.0,
                cap_usd: 0.37,
                cache_write_tokens: 0,
                cache_read_tokens: 0,
                output_tokens: 0,
                assumptions: vec![],
                pricing_checked: String::new(),
                subscription_note: String::new(),
            },
            status: "approved".into(),
            created_at: String::new(),
            approved_at: None,
            finished_at: None,
            actual_cost_usd: None,
            result_text: None,
            error: None,
            pipeline_id: None,
        };
        let a = build_args(
            &req,
            &["Read".into(), "Glob".into()],
            &[PathBuf::from("/s/projects")],
            Path::new("/s/runs/run-x"),
            true,
        );
        assert_eq!(a[1], "/lead hi", "skill must be invoked by slash command");
        let joined = a.join(" ");
        assert!(joined.contains("/s/runs/run-x"));
        assert!(joined.contains("--max-budget-usd 0.37"));
        assert!(joined.contains("--permission-mode dontAsk"));
        assert!(joined.contains("--setting-sources project"));
        assert!(joined.contains("--strict-mcp-config"));
        assert!(joined.contains("--agent lead"));
        assert!(joined.contains("--allowedTools Read Glob"));
        assert!(!joined.contains("skip-permissions"));
    }
}
