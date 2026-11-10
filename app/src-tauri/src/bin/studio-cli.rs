//! Headless companion to the desktop app, for verification and scripting.
//! It uses the same core code and the same approval rules as the app.
//!
//!   studio-cli info
//!   studio-cli queue list
//!   studio-cli queue decide <id> approved|rejected [note]
//!   studio-cli estimate <agent> <prompt>
//!   studio-cli run <agent> <prompt> --approve-cap <usd>
//!   studio-cli pipeline <name> <input> [--project projects/<game>] [--approve-cap <usd>]
//!   studio-cli spend | claude | settings show | settings budget <usd> | settings webhook <url|none>
//!   studio-cli notify test | update check | update apply | import <path>
//!
//! `run` prepares the request, prints the estimate, and only starts when
//! `--approve-cap` equals the cap it just printed. Without the flag it stops
//! after printing the estimate.

use studio_core::{budget, bridge, claude, cost, notify, paths, pipeline, queue, runs, settings, studio, update};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(code) = studio_core::gates::cli(&args) {
        std::process::exit(code);
    }
    if let Err(e) = real_main(&args) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn real_main(args: &[String]) -> Result<(), String> {
    let root = paths::studio_root()?;
    match args.iter().map(|s| s.as_str()).collect::<Vec<_>>().as_slice() {
        ["info"] => {
            println!("studio root: {}", root.display());
            match paths::claude_bin() {
                Ok(p) => println!("claude: {}", p.display()),
                Err(e) => println!("claude: NOT FOUND ({e})"),
            }
            let p = cost::load_pricing(&root)?;
            println!("pricing checked: {}", p.checked);
            println!("agents: {}", cost::load_agents(&root)?.agents.keys().cloned().collect::<Vec<_>>().join(", "));
            Ok(())
        }
        ["queue", "list"] => {
            let items = queue::list(&root)?;
            if items.is_empty() {
                println!("review queue is empty");
            }
            for i in items {
                println!(
                    "{}  [{}]  {}  gate={}  by={}  {}",
                    i.id,
                    i.status,
                    i.kind,
                    if i.gate.passed { "PASS" } else { "FAIL" },
                    i.created_by,
                    i.title
                );
            }
            Ok(())
        }
        ["queue", "decide", id, verdict, rest @ ..] => {
            let item = queue::decide(&root, id, verdict, &rest.join(" "))?;
            println!("{} -> {}", item.id, item.status);
            Ok(())
        }
        ["estimate", agent, prompt @ ..] => {
            let req = runs::prepare(&root, agent, &prompt.join(" "))?;
            runs::cancel(&root, &req.run_id)?;
            print_estimate(&req);
            Ok(())
        }
        ["run", agent, rest @ ..] => {
            let mut cap: Option<f64> = None;
            let mut words = vec![];
            let mut it = rest.iter();
            while let Some(w) = it.next() {
                if *w == "--approve-cap" {
                    cap = it.next().and_then(|v| v.parse().ok());
                } else {
                    words.push(*w);
                }
            }
            let req = runs::prepare(&root, agent, &words.join(" "))?;
            print_estimate(&req);
            let Some(cap) = cap else {
                runs::cancel(&root, &req.run_id)?;
                println!("not approved: rerun with --approve-cap {:.2} to start", req.estimate.cap_usd);
                return Ok(());
            };
            runs::approve(&root, &req.run_id, cap).map_err(|e| {
                let _ = runs::cancel(&root, &req.run_id);
                e
            })?;
            let bin = paths::claude_bin()?;
            println!("approved; running {} ...", req.run_id);
            let out = bridge::run_approved(&root, &req.run_id, &bin, |_| {})?;
            println!("ok: {}", out.ok);
            println!("actual cost: {:?}", out.actual_cost_usd);
            println!("turns: {:?}", out.num_turns);
            println!("--- result ---\n{}", out.result_text);
            if let Some(item) = pipeline::finish_single(&root, &req.run_id)? {
                println!("review queue item: {} (gate {})", item.id, if item.gate.passed { "PASS" } else { "FAIL" });
            }
            if !out.ok {
                println!("--- stderr tail ---\n{}", out.stderr_tail);
                return Err("run failed".into());
            }
            Ok(())
        }
        ["pipeline", name, rest @ ..] => {
            let mut cap: Option<f64> = None;
            let mut project: Option<String> = None;
            let mut words = vec![];
            let mut it = rest.iter();
            while let Some(w) = it.next() {
                match *w {
                    "--approve-cap" => cap = it.next().and_then(|v| v.parse().ok()),
                    "--project" => project = it.next().map(|s| s.to_string()),
                    _ => words.push(*w),
                }
            }
            let p = pipeline::prepare(&root, name, &words.join(" "), project.as_deref())?;
            println!("pipeline: {}", p.pipeline_id);
            for s in &p.stages {
                let r = runs::load(&root, &s.run_id)?;
                println!("  stage {:10} est ${:.4}  cap ${:.2}  -> {}", s.agent, r.estimate.expected_usd, r.estimate.cap_usd, s.output);
            }
            println!("total: ${:.4} expected, up to ${:.4}; hard cap ${:.2}", p.expected_usd, p.high_usd, p.cap_usd);
            let Some(cap) = cap else {
                pipeline::cancel(&root, &p.pipeline_id)?;
                println!("not approved: rerun with --approve-cap {:.2} to start", p.cap_usd);
                return Ok(());
            };
            pipeline::approve(&root, &p.pipeline_id, cap).map_err(|e| {
                let _ = pipeline::cancel(&root, &p.pipeline_id);
                e
            })?;
            let bin = paths::claude_bin()?;
            println!("approved; running");
            let done = pipeline::run(&root, &p.pipeline_id, &bin, |ev| match ev {
                pipeline::Event::StageStart { index, agent, .. } => println!("[{}] {} started", index + 1, agent),
                pipeline::Event::StageDone { index, agent, gate } => {
                    println!("[{}] {} gate {}", index + 1, agent, if gate.passed { "PASS" } else { "FAIL" });
                    for c in &gate.checks {
                        println!("      {} {}: {}", if c.passed { "ok  " } else { "FAIL" }, c.name, c.detail.chars().take(160).collect::<String>());
                    }
                }
                pipeline::Event::Line { .. } => {}
            })?;
            println!("pipeline {}: {}  actual ${:.4}", done.pipeline_id, done.status, done.actual_cost_usd.unwrap_or(0.0));
            println!("review queue item: {}", done.queue_item.unwrap_or_default());
            Ok(())
        }
        ["recheck", run_id] => {
            match pipeline::recheck_single(&root, run_id)? {
                Some(i) => {
                    println!("{} gate {}", i.id, if i.gate.passed { "PASS" } else { "FAIL" });
                    for c in i.gate.checks {
                        println!("  {} {}: {}", if c.passed { "ok  " } else { "FAIL" }, c.name, c.detail.chars().take(120).collect::<String>());
                    }
                }
                None => println!("this agent has no output file; nothing to check"),
            }
            Ok(())
        }
        ["version"] => {
            println!("build {}", update::build_commit());
            Ok(())
        }
        ["spend"] => {
            let st = settings::load();
            let sp = budget::spend(&root, st.monthly_budget_usd);
            println!("{}", serde_json::to_string_pretty(&sp).unwrap());
            Ok(())
        }
        ["claude"] => {
            println!("{}", serde_json::to_string_pretty(&claude::status()).unwrap());
            Ok(())
        }
        ["settings", "show"] => {
            let mut st = settings::load();
            if !st.discord_webhook_url.is_empty() {
                st.discord_webhook_url = "(set)".into();
            }
            println!("{}\n(file: {})", serde_json::to_string_pretty(&st).unwrap(), settings::path().display());
            Ok(())
        }
        ["settings", "budget", v] => {
            let mut st = settings::load();
            st.monthly_budget_usd = v.parse().map_err(|_| "budget must be a number")?;
            settings::save(&st)?;
            println!("monthly budget: ${:.2}", st.monthly_budget_usd);
            Ok(())
        }
        ["settings", "webhook", v] => {
            let mut st = settings::load();
            st.discord_webhook_url = if *v == "none" { String::new() } else { v.to_string() };
            settings::save(&st)?;
            println!("webhook {}", if st.discord_webhook_url.is_empty() { "cleared" } else { "saved" });
            Ok(())
        }
        ["notify", "test"] => {
            let st = settings::load();
            let sent = notify::send(&st, notify::Kind::Test, "BrewixGames is connected", &["Test message from studio-cli.".into()])?;
            println!("{}", if sent { "sent" } else { "not sent (no webhook set)" });
            Ok(())
        }
        ["update", "check"] => {
            println!("{}", serde_json::to_string_pretty(&update::check(&root)?).unwrap());
            Ok(())
        }
        ["update", "apply"] => {
            let mut on = |l: &str| println!("  {l}");
            let exe = update::apply(&root, &mut on)?;
            println!("updated; restart into {}", exe.display());
            Ok(())
        }
        ["import", path] => {
            println!("{}", studio::import_project(&root, path)?);
            Ok(())
        }
        _ => Err("usage: studio-cli info | queue list | queue decide <id> approved|rejected [note] | estimate <agent> <prompt> | run <agent> <prompt> [--approve-cap <usd>] | pipeline <name> <input> [--project p] [--approve-cap <usd>]".into()),
    }
}

fn print_estimate(req: &runs::RunRequest) {
    let e = &req.estimate;
    println!("run: {}", req.run_id);
    println!("agent: {}  model: {}", e.agent, e.model);
    println!(
        "estimate: ${:.4} expected (range ${:.4} to ${:.4}); hard cap ${:.2}",
        e.expected_usd, e.low_usd, e.high_usd, e.cap_usd
    );
    for a in &e.assumptions {
        println!("  - {a}");
    }
    println!("  - prices checked {}", e.pricing_checked);
    println!("  - {}", e.subscription_note);
}
