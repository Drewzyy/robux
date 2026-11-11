//! Monthly spend budget. Runs stop and ask once this month's spend plus the
//! cap of the run being approved would pass Andrew's budget (Settings).
//! Spend is API-equivalent dollars as reported by Claude Code per run; caps
//! of approved and running runs count as reserved.

use crate::runs;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct Spend {
    pub month: String,
    pub spent_usd: f64,
    pub reserved_usd: f64,
    pub budget_usd: f64,
    /// Expected cost of runs in progress right now.
    pub running_expected_usd: f64,
    pub running: Vec<String>,
}

pub fn current_month() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}

pub fn spend(root: &Path, budget_usd: f64) -> Spend {
    let month = current_month();
    let mut spent = 0.0;
    let mut reserved = 0.0;
    let mut running_expected = 0.0;
    let mut running = vec![];
    for r in runs::list(root).unwrap_or_default() {
        match r.status.as_str() {
            "done" | "failed" => {
                let when = r.finished_at.clone().unwrap_or_else(|| r.created_at.clone());
                if when.starts_with(&month) {
                    spent += r.actual_cost_usd.unwrap_or(0.0);
                }
            }
            "approved" => reserved += r.estimate.cap_usd,
            "running" => {
                reserved += r.estimate.cap_usd;
                running_expected += r.estimate.expected_usd;
                running.push(r.run_id.clone());
            }
            _ => {}
        }
    }
    Spend {
        month,
        spent_usd: round2(spent),
        reserved_usd: round2(reserved),
        budget_usd,
        running_expected_usd: (running_expected * 10000.0).round() / 10000.0,
        running,
    }
}

/// Refuse when spent + reserved + this cap would pass the budget.
pub fn check(root: &Path, new_cap_usd: f64) -> Result<(), String> {
    let s = crate::settings::load();
    if s.monthly_budget_usd <= 0.0 {
        return Ok(());
    }
    let sp = spend(root, s.monthly_budget_usd);
    let after = sp.spent_usd + sp.reserved_usd + new_cap_usd;
    if after > s.monthly_budget_usd + 1e-9 {
        let msg = format!(
            "Monthly budget reached: ${:.2} spent and ${:.2} reserved this month, this run's cap is ${:.2}, budget is ${:.2}. Raise the budget in Settings to run it.",
            sp.spent_usd, sp.reserved_usd, new_cap_usd, s.monthly_budget_usd
        );
        crate::notify::send_quiet(crate::notify::Kind::Budget, "Budget stopped a run", std::slice::from_ref(&msg));
        return Err(msg);
    }
    Ok(())
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
