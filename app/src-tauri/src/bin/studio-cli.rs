//! Headless companion to the desktop app, for verification and scripting.
//! It uses the same core code and the same approval rules as the app.
//!
//!   studio-cli info
//!   studio-cli queue list
//!   studio-cli queue decide <id> approved|rejected [note]
//!   studio-cli estimate <agent> <prompt>
//!   studio-cli run <agent> <prompt> --approve-cap <usd>
//!
//! `run` prepares the request, prints the estimate, and only starts when
//! `--approve-cap` equals the cap it just printed. Without the flag it stops
//! after printing the estimate.

use studio_core::{bridge, cost, paths, queue, runs};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
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
            if !out.ok {
                println!("--- stderr tail ---\n{}", out.stderr_tail);
                return Err("run failed".into());
            }
            Ok(())
        }
        _ => Err("usage: studio-cli info | queue list | queue decide <id> approved|rejected [note] | estimate <agent> <prompt> | run <agent> <prompt> [--approve-cap <usd>]".into()),
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
