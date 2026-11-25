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
use std::process::Stdio;

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
/// Why Claude Code ended a run early, from its result line.
pub fn stop_reason_of(result: &serde_json::Value) -> Option<&'static str> {
    let sub = result.get("subtype").and_then(|s| s.as_str()).unwrap_or("");
    let errs = result.get("errors").map(|e| e.to_string().to_lowercase()).unwrap_or_default();
    if sub == "error_max_turns" || errs.contains("maximum number of turns") {
        Some("turn_limit")
    } else if sub.contains("budget") || errs.contains("budget") {
        Some("budget_cap")
    } else {
        None
    }
}

/// What a run really used, from Claude Code's result line.
pub fn usage_of(result: &serde_json::Value) -> serde_json::Value {
    let u = &result["usage"];
    serde_json::json!({
        "turns": result["num_turns"],
        "duration_ms": result["duration_ms"],
        "input_tokens": u["input_tokens"],
        "cache_read_tokens": u["cache_read_input_tokens"],
        "cache_write_tokens": u["cache_creation_input_tokens"],
        "output_tokens": u["output_tokens"],
        "thinking_tokens": u["output_tokens_details"]["thinking_tokens"],
        "api_equivalent_usd": result["total_cost_usd"],
    })
}

/// Move the long parts off the command line: the prompt (after -p) is
/// returned to send on stdin, the run context is written to a file and
/// passed with --append-system-prompt-file.
pub fn to_launch(mut args: Vec<String>, dir: &Path) -> Result<(Vec<String>, String), String> {
    let mut prompt = String::new();
    if let Some(i) = args.iter().position(|a| a == "-p") {
        if i + 1 < args.len() && !args[i + 1].starts_with("--") {
            prompt = args.remove(i + 1);
        }
    }
    if let Some(i) = args.iter().position(|a| a == "--append-system-prompt") {
        if i + 1 < args.len() {
            let f = dir.join("run-context.md");
            fs::write(&f, &args[i + 1]).map_err(|e| format!("could not write the run context: {e}"))?;
            args[i] = "--append-system-prompt-file".into();
            args[i + 1] = f.display().to_string();
        }
    }
    Ok((args, prompt))
}

/// How any agent starts other agents: the app reads requests.json when the
/// run ends (changes::ingest_requests). Never tell Andrew to start them.
pub const DISPATCH: &str = " You can start other agents yourself; never tell Andrew he has to. Write requests.json in your run folder: \
[{\"to\": \"<agent>\", \"task\": \"<what to do, specific>\", \"why\": \"<why>\", \"after\": \"<agent that must finish first, optional>\", \"game\": \"<game-slug, optional>\"}]. \
The app starts them when you finish (up to 3 at a time, in your order) and gives them your run folder to read. \
{\"stop\": \"<agent>\"} cancels that agent's queued work. Agents: architect, builder, frame, scenic, motion, modeler, librarian, \
merchant, tuner, crowd, splash, playtest, simulator, director, planner, producer, liveops, release.";

pub fn build_args(
    req: &RunRequest,
    allowed_tools: &[String],
    add_dirs: &[PathBuf],
    run_dir: &Path,
    has_skill: bool,
) -> Vec<String> {
    // A continued run: tell the agent it was stopped partway.
    let cont = if req.continued > 0 && req.chat.is_none() {
        "[You were stopped partway through this task (Claude's usage limit, or the app closed) and are now continuing. Your run folder already holds what you wrote before; read it first, keep what is finished, and finish the task. Do not start over.]\n\n"
    } else {
        ""
    };
    let prompt = if has_skill {
        format!("/{} {cont}{}", req.agent, req.prompt)
    } else if req.prompt.starts_with('-') {
        format!(" {cont}{}", req.prompt) // a leading dash would be read as an option
    } else {
        format!("{cont}{}", req.prompt)
    };
    let context = format!(
        "Run context. Run id: {}. Your run folder is {} (absolute path). Write every output file there, \
         including gate.json. The studio's game projects folder is {}. You are running unattended: \
         nobody can answer questions mid-run, so put questions in your output under Open questions. \
         Never write that Andrew decided, asked or approved something he did not.{}",
        req.run_id,
        run_dir.display(),
        add_dirs.first().map(|p| p.display().to_string()).unwrap_or_default(),
        // Pipelines run their own fixed order of agents.
        if req.chat.is_none() && req.pipeline_id.is_none() { DISPATCH } else { "" }
    );
    let shell: Vec<String> = allowed_tools
        .iter()
        .filter_map(|t| t.strip_prefix("Bash(").and_then(|r| r.strip_suffix(")")))
        .map(|r| format!("`{}`", r.trim_end_matches(":*").trim_end_matches('*').trim_end()))
        .collect();
    let context = if shell.is_empty() {
        format!("{context} You have no shell access in this run.")
    } else {
        format!(
            "{context} Shell commands you may run, each as ONE command that starts exactly like this (arguments after it are fine): {}. Anything else is refused, including pipes, &&, ;, cd, heredocs, and python3 -c. Do not try other commands.",
            shell.join(", ")
        )
    };
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
        // Local runs cost nothing; give them a cap that never stops them.
        // A continued run only gets what is left of the approved cap.
        if req.estimate.cap_usd > 0.0 { format!("{:.2}", (req.estimate.cap_usd - req.prior_cost_usd).max(0.01)) } else { "1000".into() },
        // Only the vault's own settings (hooks, deny rules) and no MCP
        // servers: agents do not inherit plugins or tools from the user's
        // personal Claude Code setup.
        "--setting-sources".into(),
        "project".into(),
        "--strict-mcp-config".into(),
        "--permission-mode".into(),
        "dontAsk".into(),
    ];
    match (&req.chat, &req.resume) {
        (Some(_), Some(session)) if !session.is_empty() => {
            a.push("--resume".into());
            a.push(session.clone());
        }
        (Some(_), _) => {}
        _ => a.push("--no-session-persistence".into()),
    }
    if req.chat.is_some() {
        // Ask Claude shows the reply as it is written, word by word.
        a.push("--include-partial-messages".into());
    }
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
    // Videos watched with `brewix watch` (transcripts and frames).
    let videos = crate::video::dir(root);
    let _ = fs::create_dir_all(&videos);
    add_dirs.push(videos);
    if req.chat.is_some() {
        let f = crate::chat::files_dir(root);
        let _ = fs::create_dir_all(&f);
        add_dirs.push(f);
    }
    add_dirs.extend(extra_dirs.iter().cloned());
    let has_skill = vault
        .join(".claude/skills")
        .join(&req.agent)
        .join("SKILL.md")
        .is_file();
    let mut tools = profile.allowed_tools.clone();
    let mut servers = serde_json::Map::new();
    let mut notes = String::new();
    // Name the folders of the team's earlier work, with what is in each, so
    // a hand-off like "read design.md" finds the right file.
    let team: Vec<String> = extra_dirs.iter().filter(|d| d.is_dir()).map(|d| {
        let files: Vec<String> = fs::read_dir(d).into_iter().flatten().flatten().map(|e| e.file_name().to_string_lossy().to_string())
            .filter(|n| n.ends_with(".md") || n.ends_with(".json") && !["request.json", "result.json", "command.json", "gate.json"].contains(&n.as_str()) || n == "mockups").take(12).collect();
        format!("{} ({})", d.display(), files.join(", "))
    }).collect();
    if !team.is_empty() {
        notes.push_str(&format!(" Your teammates' finished work, readable now (a file named in your task is in one of these, not in your own folder): {}.", team.join("; ")));
    }
    // Studio and Blender tools go to every engine: the local model used to
    // get neither, so its agents reported them "unreachable" while both
    // were open and connected.
    {
        // Open Roblox Studio / Blender first if this agent uses them.
        crate::connect::ready_for(&req.agent);
        let st = crate::settings::load();
        // Playtest needs the game itself in Studio, not an empty baseplate:
        // build it and open it once per game per session.
        if req.agent == "playtest" && st.studio_mcp == "studio" {
            static OPENED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(vec![]);
            if let Some(slug) = crate::changes::list(root).into_iter().find(|c| c.run_id == req.run_id).and_then(|c| c.autopilot) {
                let mut o = OPENED.lock().unwrap_or_else(|e| e.into_inner());
                if !o.contains(&slug) {
                    if let Ok(place) = crate::studio::open_in_studio(root, &format!("projects/{slug}")) {
                        o.push(slug.clone());
                        notes.push_str(&format!(" The game was just built and opened in Roblox Studio from {place}. If list_roblox_studios shows more than one Studio, use the one with that place, not an empty Baseplate."));
                        std::thread::sleep(std::time::Duration::from_secs(25));
                    }
                }
            }
        }
        if crate::studio_mcp::AGENTS.contains(&req.agent.as_str()) {
            if let Some(cmd) = crate::studio_mcp::server_command(&st) {
                servers.insert(crate::studio_mcp::SERVER.into(), serde_json::json!({ "type": "stdio", "command": cmd.0, "args": cmd.1 }));
                tools.extend(crate::studio_mcp::TOOLS.iter().map(|t| format!("mcp__{}__{t}", crate::studio_mcp::SERVER)));
                notes.push_str(" Roblox Studio is connected (mcp__Roblox_Studio__ tools). With Studio's built-in server, call list_roblox_studios first and pass that studio_id on every call; explore with search_game_tree and inspect_instance, edit scripts with multi_edit, run code with execute_luau, playtest with start_stop_play, get_console_output, and screen_capture. With the older server, use run_code and insert_model. Uploads, AI asset generation, Studio's own subagent, and running code that publishes, buys, calls the web, or writes DataStores are blocked; do not try them.");
            }
        }
        if crate::blender_mcp::AGENTS.contains(&req.agent.as_str()) {
            if let Some(cmd) = crate::blender_mcp::server_command(&st) {
                servers.insert(crate::blender_mcp::SERVER.into(), crate::blender_mcp::config_entry(&cmd));
                tools.extend(crate::blender_mcp::tool_names());
                notes.push_str(" Blender is connected (mcp__Blender__ tools). Model with execute_blender_code (bpy), check with look, and export FBX into the game's assets/models folder. Only Poly Haven (CC0) assets are allowed; paid generators, other asset sites, and network access are blocked.");
            }
        }
    }
    if req.engine.as_deref() != Some("ollama") && crate::settings::load().figma_mcp && ["frame", "planner"].contains(&req.agent.as_str()) {
        servers.insert("Figma".into(), serde_json::json!({ "type": "http", "url": "https://mcp.figma.com/mcp" }));
        tools.push("mcp__Figma".into());
        notes.push_str(" Figma is connected (mcp__Figma__ tools) through Andrew's own Figma login. Draw each screen as a frame in the Figma file Andrew names, with layers named as 08-UI-UX/figma-handoff says. Reads are limited on Figma's free plan: read a file at most once per run, and never read when you can write from the plan. If the tools are missing or refuse, fall back to the SVG handoff.");
    }
    // Local model: its big-window copy, so the whole task fits.
    if req.engine.as_deref() == Some("ollama") {
        req.estimate.model = crate::ollama::with_context(&req.estimate.model);
    }
    let mut args = build_args(&req, &tools, &add_dirs, &dir, has_skill);
    // A real bound on the run: Claude Code stops at this many turns and
    // reports subtype error_max_turns (checked with Claude Code 2.1.326).
    let max_turns = crate::cost::turn_limit(&profile);
    args.push("--max-turns".into());
    args.push(max_turns.to_string());
    req.turn_limit = Some(max_turns);
    if !servers.is_empty() {
        let cfg = dir.join("mcp.json");
        fs::write(&cfg, serde_json::to_string_pretty(&serde_json::json!({ "mcpServers": servers })).unwrap()).map_err(|e| e.to_string())?;
        if let Some(i) = args.iter().position(|a| a == "--append-system-prompt") {
            args[i + 1].push_str(&std::mem::take(&mut notes));
        }
        args.push("--mcp-config".into());
        args.push(cfg.display().to_string());
    }
    // Team folders (and anything else noted) even when no tool server is on.
    if !notes.is_empty() {
        if let Some(i) = args.iter().position(|a| a == "--append-system-prompt") {
            args[i + 1].push_str(&notes);
        }
    }
    // Windows refuses command lines over 32,767 characters (8,191 through a
    // .cmd wrapper), and a game's brief, notes and team folders get long. So
    // the prompt goes in on stdin and the run context in a file; only short
    // flags stay on the command line.
    let (args, stdin_prompt) = to_launch(args, &dir)?;
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

    // Paused or stopped while it was starting: honour that now.
    let early = PENDING.lock().unwrap_or_else(|e| e.into_inner()).remove(run_id).filter(|(_, at)| at.elapsed() < std::time::Duration::from_secs(120)).map(|(w, _)| w);
    if let Some(what) = early {
        if what == "pause" {
            req.status = "paused".into();
            req.resume_at = Some("9999-12-31T00:00:00Z".into());
            req.error = Some("Paused by you before it started.".into());
        } else {
            req.status = "cancelled".into();
            req.error = Some("Stopped by you before it started.".into());
        }
        runs::save(root, &req)?;
        return Ok(RunOutcome { run_id: run_id.to_string(), ok: false, result_text: String::new(), actual_cost_usd: None, num_turns: None, exit_code: None, stderr_tail: String::new() });
    }
    req.status = "running".into();
    runs::save(root, &req)?;

    let mut cmd = crate::paths::command(claude_bin);
    if req.engine.as_deref() == Some("ollama") {
        if let Some(note) = crate::ollama::ensure_big_context() {
            on_line(&serde_json::json!({"type": "system", "subtype": "note", "text": note}).to_string());
        }
    }
    if req.engine.as_deref() == Some("ollama") {
        // Free local engine: Ollama 0.14+ speaks the Anthropic Messages API.
        cmd.env("ANTHROPIC_BASE_URL", crate::ollama::base()).env("ANTHROPIC_AUTH_TOKEN", "ollama");
        // A system or corporate proxy must never sit between Claude Code and
        // Ollama on this computer.
        let np = std::env::var("NO_PROXY").unwrap_or_default();
        let np = if np.is_empty() { "127.0.0.1,localhost".to_string() } else { format!("{np},127.0.0.1,localhost") };
        cmd.env("NO_PROXY", &np).env("no_proxy", &np);
    }
    let mut child = cmd
        .args(&args)
        .current_dir(&vault)
        .env("PATH", agent_path())
        .env("ROBUX_RUN_ID", run_id)
        .env("ROBUX_RUN_DIR", &dir)
        .env("ROBUX_AGENT", &req.agent)
        .env("ROBUX_STUDIO_ROOT", root)
        // Agents always run on Andrew's Claude login (his subscription),
        // never on an API key, even if one is set in the environment.
        .env_remove("ANTHROPIC_API_KEY")
        .stdin(Stdio::piped())
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
    LIVE.lock().unwrap_or_else(|e| e.into_inner()).insert(run_id.to_string(), child.id());
    // Watchdog: an agent that prints nothing for this long is stuck (a hung
    // Claude Code, or a local model that does not fit in memory). End it so
    // it can be retried instead of showing "Working" forever.
    let quiet_limit = std::time::Duration::from_secs(std::env::var("ROBUX_QUIET_SECS").ok().and_then(|v| v.parse().ok()).unwrap_or(if req.engine.as_deref() == Some("ollama") { 20 * 60 } else { 10 * 60 }));
    let last_line = std::sync::Arc::new(std::sync::Mutex::new(std::time::Instant::now()));
    {
        let (last, id) = (last_line.clone(), run_id.to_string());
        std::thread::spawn(move || loop {
            std::thread::sleep(quiet_limit.min(std::time::Duration::from_secs(15)));
            if !LIVE.lock().unwrap_or_else(|e| e.into_inner()).contains_key(&id) {
                return;
            }
            if last.lock().unwrap_or_else(|e| e.into_inner()).elapsed() > quiet_limit {
                end_run(&id, "stall");
                return;
            }
        });
    }
    // The prompt, on stdin (then closed so Claude starts).
    if let Some(mut si) = child.stdin.take() {
        std::thread::spawn(move || {
            use std::io::Write;
            let _ = si.write_all(stdin_prompt.as_bytes());
        });
    }

    // Drain stderr on a thread so a full pipe never blocks the child.
    let stderr = child.stderr.take().unwrap();
    let err_path = dir.join("stderr.log");
    let err_thread = std::thread::spawn(move || {
        let mut buf = String::new();
        let mut f = fs::File::create(&err_path).ok();
        for line in lossy_lines(stderr) {
            if let Some(f) = f.as_mut() {
                let _ = writeln!(f, "{line}");
            }
            buf.push_str(&line);
            buf.push('\n');
        }
        buf
    });

    let stdout = child.stdout.take().unwrap();
    // If the log cannot be written (disk full, locked), keep watching the
    // agent anyway: never leave it running unobserved.
    let mut stream_log: Box<dyn Write> = match fs::File::create(dir.join("stream.jsonl")) {
        Ok(f) => Box::new(f),
        Err(e) => {
            eprintln!("could not write the run log: {e}");
            Box::new(std::io::sink())
        }
    };
    let mut final_result: Option<serde_json::Value> = None;
    for line in lossy_lines(stdout) {
        let _ = writeln!(stream_log, "{line}");
        on_line(&line);
        *last_line.lock().unwrap_or_else(|e| e.into_inner()) = std::time::Instant::now();
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
            if v.get("type").and_then(|t| t.as_str()) == Some("result") {
                final_result = Some(v);
            }
        }
    }
    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            let _ = child.kill();
            LIVE.lock().unwrap_or_else(|e| e.into_inner()).remove(run_id);
            req.status = "failed".into();
            req.error = Some(format!("could not start Claude properly: {e}"));
            let _ = runs::save(root, &req);
            return Err(e.to_string());
        }
    };
    LIVE.lock().unwrap_or_else(|e| e.into_inner()).remove(run_id);
    // A local job ended: free the model's memory now unless another runs.
    if req.engine.as_deref() == Some("ollama") && LIVE.lock().unwrap_or_else(|e| e.into_inner()).is_empty() {
        std::thread::spawn(crate::ollama::unload_all);
    }
    let asked = ASKED.lock().unwrap_or_else(|e| e.into_inner()).remove(run_id);
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
    let process_ok = status.success() && !is_error && final_result.is_some();
    // Claude Code exiting cleanly is not the same as the work being done:
    // an agent that stops without writing its output file (a local model
    // that asks a question back, for one) did not finish.
    let missing_output = profile.output.as_ref().filter(|o| process_ok && req.chat.is_none() && req.change.is_none() && !dir.join(o.as_str()).is_file()).cloned();
    let ok = process_ok && missing_output.is_none() && !(req.engine.as_deref() == Some("ollama") && lost_task(&result_text));
    req.usage = final_result.as_ref().map(usage_of);
    req.stop_reason = final_result.as_ref().and_then(stop_reason_of).map(String::from);

    req.status = if ok { "done".into() } else { "failed".into() };
    req.finished_at = Some(crate::now_rfc3339());
    req.actual_cost_usd = cost.map(|c| c + req.prior_cost_usd).or(Some(req.prior_cost_usd).filter(|c| *c > 0.0));
    req.result_text = Some(result_text.clone());
    if !ok {
        req.error = Some(if req.stop_reason.as_deref() == Some("turn_limit") {
            let out = profile.output.clone().unwrap_or_else(|| "its output".into());
            let missing: Vec<&str> = [out.as_str(), "gate.json"].into_iter().filter(|f| !dir.join(f).is_file()).collect();
            format!(
                "Stopped at the turn limit ({} turns) before it finished. Everything it wrote is kept in runs/{run_id}. {}",
                turns.unwrap_or(max_turns as u64),
                if missing.is_empty() { "Its files exist but it did not get to check them.".to_string() } else { format!("Not written yet: {}.", missing.join(", ")) }
            )
        } else if req.stop_reason.as_deref() == Some("budget_cap") {
            format!("Stopped at the approved cap (${:.2}, API-equivalent) before it finished. Everything it wrote is kept in runs/{run_id}.", req.estimate.cap_usd)
        } else if req.engine.as_deref() == Some("ollama") && lost_task(&result_text) {
            format!("The local model did not get the task (its memory window cut it off) and asked what to do instead. What it said: {}", result_text.chars().take(160).collect::<String>())
        } else if let Some(o) = &missing_output {
            format!("The agent ended without writing {o}. What it said last: {}", result_text.chars().take(300).collect::<String>())
        } else if final_result.is_none() {
            format!("no result line from claude (exit {:?}). stderr:\n{stderr_tail}", status.code())
        } else {
            // Keep Claude's own words: they tell a network or sign-in problem
            // apart from bad work (changes::env_problem).
            format!("claude reported an error (exit {:?}): {}", status.code(), result_text.chars().take(300).collect::<String>())
        });
    }
    // Andrew pressed Stop or Pause: say so, keep the files, no diagnosis.
    if asked.as_deref() == Some("stall") && !ok {
        req.status = "failed".into();
        req.error = Some(format!("No response for {} minutes: the agent was stuck, so it was ended. It is tried again by itself.", quiet_limit.as_secs() / 60));
        if req.engine.as_deref() == Some("ollama") {
            req.error = Some(format!("{} Local engine: the model may not fit in memory; a smaller model in Settings helps.", req.error.clone().unwrap_or_default()));
        }
        runs::save(root, &req)?;
        return Ok(RunOutcome { run_id: run_id.to_string(), ok: false, result_text, actual_cost_usd: req.actual_cost_usd, num_turns: turns, exit_code: status.code(), stderr_tail });
    }
    if let Some(what) = asked.filter(|_| !ok) {
        if what == "pause" {
            req.status = "paused".into();
            req.resume_at = Some("9999-12-31T00:00:00Z".into()); // only Andrew resumes it
            req.prior_cost_usd = req.actual_cost_usd.unwrap_or(req.prior_cost_usd);
            req.error = Some("Paused by you. Files it made are kept; Resume continues from where it stopped.".into());
        } else {
            req.status = "cancelled".into();
            req.error = Some("Stopped by you. Files it made are kept.".into());
        }
        runs::save(root, &req)?;
        return Ok(RunOutcome { run_id: run_id.to_string(), ok: false, result_text, actual_cost_usd: req.actual_cost_usd, num_turns: turns, exit_code: status.code(), stderr_tail });
    }
    // Local engine failed: find out which part failed and say what to do.
    if !ok && req.engine.as_deref() == Some("ollama") {
        let tail = crate::ollama::diagnose(&dir, &req.estimate.model, &stderr_all, &result_text, final_result.as_ref());
        req.error = Some(format!("{} {tail}", req.error.clone().unwrap_or_default()).trim().to_string());
    }
    runs::save(root, &req)?;
    // Stopped by a usage limit: pause (keep the work) instead of failing.
    let limit_text = format!("{result_text}\n{stderr_tail}");
    // Not after a turn or budget stop, and at most 6 continues in a row.
    if !ok && req.stop_reason.is_none() && req.continued < 6 && runs::is_limit(&limit_text) {
        let _ = runs::pause(root, run_id, &limit_text);
    }

    Ok(RunOutcome {
        run_id: run_id.to_string(),
        ok,
        result_text,
        actual_cost_usd: req.actual_cost_usd,
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
    fn long_text_never_goes_on_the_command_line() {
        let dir = std::env::temp_dir().join(format!("launch-t-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let long = "x".repeat(60_000);
        let args = vec!["-p".to_string(), format!("/planner {long}"), "--append-system-prompt".into(), long.clone(), "--agent".into(), "planner".into()];
        let (a, prompt) = to_launch(args, &dir).unwrap();
        assert!(prompt.starts_with("/planner ") && prompt.len() > 60_000);
        assert_eq!(a[0], "-p");
        assert_eq!(a[1], "--append-system-prompt-file");
        assert_eq!(fs::read_to_string(&a[2]).unwrap().len(), 60_000);
        assert!(a.iter().map(|x| x.len() + 1).sum::<usize>() < 2_000, "command line stays short");
        let _ = fs::remove_dir_all(&dir);
    }

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
            chat: None,
            resume: None,
            engine: None,
            resume_at: None,
            continued: 0,
            prior_cost_usd: 0.0,
            change: None,
            stop_reason: None,
            usage: None,
            turn_limit: None,
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
        assert!(joined.contains("no shell access"), "agents without Bash rules are told so");
        let b = build_args(&req, &["Bash(rojo:*)".into()], &[], Path::new("/r"), true).join(" ");
        assert!(b.contains("`rojo`") && b.contains("Do not try other commands"), "{b}");
        assert!(joined.contains("--max-budget-usd 0.37"));
        assert!(joined.contains("--permission-mode dontAsk"));
        assert!(joined.contains("--setting-sources project"));
        assert!(joined.contains("--strict-mcp-config"));
        assert!(joined.contains("--agent lead"));
        assert!(joined.contains("--allowedTools Read Glob"));
        assert!(!joined.contains("skip-permissions"));
    }
}

/// PATH for agent runs: a folder holding `brewix` (a link back to this app,
/// so agents and the vault's hook can run the built-in checks), the app's
/// bundled Roblox tools, then the normal PATH.
/// Agent processes running now (run id to process id), and runs Andrew
/// asked to stop or pause ("stop" or "pause").
static LIVE: std::sync::Mutex<std::collections::BTreeMap<String, u32>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
static ASKED: std::sync::Mutex<std::collections::BTreeMap<String, String>> = std::sync::Mutex::new(std::collections::BTreeMap::new());

/// Stop (or pause) a running agent now. Ends its Claude process and
/// everything it started. Returns false when that run is not running.
pub fn stop_run(run_id: &str, pause: bool) -> bool {
    end_run(run_id, if pause { "pause" } else { "stop" })
}

/// End a running agent for a reason: "pause", "stop" or "stall".
fn end_run(run_id: &str, why: &str) -> bool {
    let Some(pid) = LIVE.lock().unwrap_or_else(|e| e.into_inner()).get(run_id).copied() else { return false };
    ASKED.lock().unwrap_or_else(|e| e.into_inner()).insert(run_id.to_string(), why.into());
    let quiet = |c: &mut std::process::Command| { let _ = c.stdout(Stdio::null()).stderr(Stdio::null()).status(); };
    if cfg!(windows) {
        quiet(crate::paths::command("taskkill").args(["/T", "/F", "/PID", &pid.to_string()]));
    } else {
        quiet(crate::paths::command("pkill").args(["-TERM", "-P", &pid.to_string()]));
        quiet(crate::paths::command("kill").args(["-TERM", &pid.to_string()]));
        // Still there after 5 seconds: force it (and what it started).
        let id = run_id.to_string();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(5));
            if LIVE.lock().unwrap_or_else(|e| e.into_inner()).get(&id) == Some(&pid) {
                let quiet = |c: &mut std::process::Command| { let _ = c.stdout(Stdio::null()).stderr(Stdio::null()).status(); };
                quiet(crate::paths::command("pkill").args(["-KILL", "-P", &pid.to_string()]));
                quiet(crate::paths::command("kill").args(["-KILL", &pid.to_string()]));
            }
        });
    }
    true
}

/// A local model that never saw its task answers like a fresh chat.
pub fn lost_task(text: &str) -> bool {
    let t = text.to_lowercase();
    ["please provide", "please clarify", "ready to help", "what would you like", "could you specify", "specify the file", "your query does not specify", "how can i help", "how can i assist"].iter().any(|p| t.contains(p))
}

/// Lines of a pipe, never stopping at text that is not valid UTF-8 (which
/// would leave the agent looking stuck until the watchdog ends it).
fn lossy_lines<R: std::io::Read>(r: R) -> impl Iterator<Item = String> {
    let mut rd = BufReader::new(r);
    std::iter::from_fn(move || {
        let mut b = Vec::new();
        match rd.read_until(b'\n', &mut b) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(String::from_utf8_lossy(&b).trim_end_matches(['\r', '\n']).to_string()),
        }
    })
}

/// Before the app closes or restarts: pause every working agent (files
/// kept) and mark it to continue by itself when the app is back.
pub fn pause_all_for_restart(root: &Path) -> usize {
    let busy = running();
    for id in &busy {
        stop_run(id, true);
    }
    for _ in 0..20 {
        if running().is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    for id in &busy {
        if let Ok(mut r) = runs::load(root, id) {
            if r.status == "paused" {
                r.resume_at = None;
                r.error = Some("Paused while BrewixGames closed; continues by itself.".into());
                let _ = runs::save(root, &r);
            }
        }
    }
    busy.len()
}

/// Pause/stop asked for a run that was still starting (expires after 2 min,
/// so a late click never hits a later resume of the same run).
static PENDING: std::sync::Mutex<std::collections::BTreeMap<String, (String, std::time::Instant)>> = std::sync::Mutex::new(std::collections::BTreeMap::new());

/// Pause or stop a run whether or not its process is up yet: one that is
/// still starting is paused (or stopped) the moment it would launch.
/// Always succeeds.
pub fn request_end(run_id: &str, pause: bool) {
    if !stop_run(run_id, pause) {
        PENDING.lock().unwrap_or_else(|e| e.into_inner()).insert(run_id.to_string(), (if pause { "pause" } else { "stop" }.into(), std::time::Instant::now()));
    }
}

/// Run ids of agents working right now.
pub fn running() -> Vec<String> {
    LIVE.lock().unwrap_or_else(|e| e.into_inner()).keys().cloned().collect()
}

/// C:\\Users\\a\\x.exe as Git Bash writes it: /c/Users/a/x.exe.
pub fn git_bash_path(p: &std::path::Path) -> String {
    let s = p.display().to_string();
    let s = s.strip_prefix(r"\\?\").unwrap_or(&s).replace('\\', "/");
    match s.as_bytes() {
        [d, b':', rest @ ..] => format!("/{}{}", (*d as char).to_ascii_lowercase(), String::from_utf8_lossy(rest)),
        _ => s,
    }
}

pub fn agent_path() -> std::ffi::OsString {
    let mut dirs: Vec<PathBuf> = vec![];
    let bin = crate::settings::config_dir().join("bin");
    if let Ok(exe) = std::env::current_exe().and_then(|e| e.canonicalize()) {
        let _ = fs::create_dir_all(&bin);
        let link = bin.join("brewix");
        #[cfg(unix)]
        {
            if fs::read_link(&link).ok().as_deref() != Some(exe.as_path()) {
                let _ = fs::remove_file(&link);
                let _ = std::os::unix::fs::symlink(&exe, &link);
            }
        }
        #[cfg(windows)]
        {
            let _ = fs::write(bin.join("brewix.cmd"), format!("@\"{}\" %*\r\n", exe.display()));
            // Claude Code runs agent commands in Git Bash, which cannot run
            // brewix.cmd by the name "brewix": give Bash its own script.
            let _ = fs::write(bin.join("brewix"), format!("#!/bin/sh\nexec \"{}\" \"$@\"\n", git_bash_path(&exe)));
        }
        dirs.push(bin);
        if let Some(d) = exe.parent() {
            dirs.push(d.join("../Resources/tools"));
            dirs.push(d.join("tools"));
        }
    }
    dirs.push(crate::apps::graphify_bin_dir());
    if let Some(home) = crate::paths::home() {
        dirs.push(home.join(".cargo/bin"));
    }
    if let Some(p) = std::env::var_os("PATH") {
        dirs.extend(std::env::split_paths(&p));
    }
    std::env::join_paths(dirs).unwrap_or_default()
}

#[cfg(test)]
mod git_bash_tests {
    #[test]
    fn windows_paths_become_git_bash_paths() {
        assert_eq!(super::git_bash_path(std::path::Path::new(r"\\?\C:\Users\Andrew\AppData\Local\BrewixGames\brewixgames.exe")), "/c/Users/Andrew/AppData/Local/BrewixGames/brewixgames.exe");
        assert_eq!(super::git_bash_path(std::path::Path::new(r"D:\a b\x.exe")), "/d/a b/x.exe");
        assert_eq!(super::git_bash_path(std::path::Path::new("/usr/bin/x")), "/usr/bin/x");
    }
}

#[cfg(all(test, unix))]
mod stop_tests {
    use super::*;
    #[test]
    fn andrew_can_pause_or_stop_a_running_agent() {
        let d = std::env::temp_dir().join(format!("stop-t-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(d.join("studio")).unwrap();
        for f in ["agents.json", "pricing.json", "pipelines.json"] {
            fs::copy(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../studio").join(f), d.join("studio").join(f)).unwrap();
        }
        fs::create_dir_all(d.join("vault/.claude/agents")).unwrap();
        fs::write(d.join("vault/.claude/agents/architect.md"), "---\nname: architect\n---\n").unwrap();
        let fake = d.join("fake-claude");
        fs::write(&fake, "#!/bin/sh\nsleep 60\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        for pause in [true, false] {
            let r = runs::prepare(&d, "architect", "a design").unwrap();
            runs::approve(&d, &r.run_id, r.estimate.cap_usd).unwrap();
            let (dd, f, id) = (d.clone(), fake.clone(), r.run_id.clone());
            let t = std::time::Instant::now();
            let h = std::thread::spawn(move || run_approved(&dd, &id, &f, |_| {}));
            while !running().contains(&r.run_id) {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            assert!(stop_run(&r.run_id, pause));
            let out = h.join().unwrap().unwrap();
            assert!(!out.ok);
            assert!(t.elapsed().as_secs() < 20, "ended at once, not after the agent's 60 s");
            let after = runs::load(&d, &r.run_id).unwrap();
            if pause {
                assert_eq!(after.status, "paused");
                assert!(runs::due(&d).is_empty(), "a run Andrew paused waits for him");
                runs::ready_to_continue(&d, &r.run_id).unwrap();
            } else {
                assert_eq!(after.status, "cancelled");
            }
            assert!(!running().contains(&r.run_id));
        }
        // A silent, stuck agent is ended by the watchdog and retried as a
        // connection problem (not the agent's fault).
        std::env::set_var("ROBUX_QUIET_SECS", "2");
        let r = runs::prepare(&d, "architect", "a design").unwrap();
        runs::approve(&d, &r.run_id, r.estimate.cap_usd).unwrap();
        let t = std::time::Instant::now();
        let out = run_approved(&d, &r.run_id, &fake, |_| {}).unwrap();
        std::env::remove_var("ROBUX_QUIET_SECS");
        assert!(!out.ok && t.elapsed().as_secs() < 30);
        let after = runs::load(&d, &r.run_id).unwrap();
        assert_eq!(after.status, "failed");
        assert!(after.error.as_deref().unwrap_or("").contains("No response for"), "{:?}", after.error);
        let _ = fs::remove_dir_all(&d);
    }
}
