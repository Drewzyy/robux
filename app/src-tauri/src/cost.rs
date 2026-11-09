//! Running-cost estimates shown before every agent run.
//!
//! Prices come from `studio/pricing.json` (per million tokens). The formula
//! was checked against two real `claude -p` runs: Claude Code writes prompt
//! content to the 1-hour cache (billed at 2x the input price) and re-reads it
//! on later turns (billed at the cache-read price). See LOG.md, Phase 0.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPrice {
    pub input: f64,
    pub output: f64,
    pub cache_read: f64,
    /// Multiplier on `input` for a cache write. Claude Code uses the 1-hour
    /// cache, which is 2x input.
    #[serde(default = "two")]
    pub cache_write_multiplier: f64,
}
fn two() -> f64 {
    2.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub checked: String,
    pub source: String,
    pub subscription_note: String,
    pub models: BTreeMap<String, ModelPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub model: String,
    /// Expected number of model turns (tool round trips) in one run.
    pub turns: u32,
    /// Tokens of vault and project files the agent is expected to read.
    pub context_tokens: u64,
    pub output_tokens_per_turn: u64,
    /// System prompt plus tool definitions, before any file is read.
    #[serde(default = "default_base")]
    pub base_tokens: u64,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub description: String,
    /// Vault sections this agent may read. Checked against the load log.
    #[serde(default)]
    pub allowed_sections: Vec<String>,
    /// The output file the agent's skill must write in its run folder.
    #[serde(default)]
    pub output: Option<String>,
}
fn default_base() -> u64 {
    8000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub agents: BTreeMap<String, AgentProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Estimate {
    pub agent: String,
    pub model: String,
    pub low_usd: f64,
    pub expected_usd: f64,
    pub high_usd: f64,
    /// Hard cap passed to `claude --max-budget-usd`. The run stops if it
    /// reaches this amount.
    pub cap_usd: f64,
    pub cache_write_tokens: u64,
    pub cache_read_tokens: u64,
    pub output_tokens: u64,
    pub assumptions: Vec<String>,
    pub pricing_checked: String,
    pub subscription_note: String,
}

pub fn load_pricing(root: &Path) -> Result<Pricing, String> {
    let p = root.join("studio/pricing.json");
    let t = std::fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
    serde_json::from_str(&t).map_err(|e| format!("parse {}: {e}", p.display()))
}

pub fn load_agents(root: &Path) -> Result<AgentsConfig, String> {
    let p = root.join("studio/agents.json");
    let t = std::fs::read_to_string(&p).map_err(|e| format!("read {}: {e}", p.display()))?;
    serde_json::from_str(&t).map_err(|e| format!("parse {}: {e}", p.display()))
}

pub fn estimate(
    agent: &str,
    profile: &AgentProfile,
    prompt: &str,
    pricing: &Pricing,
) -> Result<Estimate, String> {
    let price = pricing
        .models
        .get(&profile.model)
        .ok_or_else(|| format!("no price for model {} in studio/pricing.json", profile.model))?;
    let turns = profile.turns.max(1) as u64;
    let base = profile.base_tokens;
    let ctx = profile.context_tokens + (prompt.chars().count() as u64) / 4 + 1;
    let out_turn = profile.output_tokens_per_turn;

    // Everything the model sees is written to cache once.
    let write = base + ctx + turns * out_turn;
    // Each turn re-reads the system prompt plus the history so far. Context
    // and outputs accumulate, so on average half of them are re-read per turn.
    let read = turns * base + ctx * (turns + 1) / 2 + out_turn * turns * turns.saturating_sub(1) / 2;
    let out = turns * out_turn;

    let m = 1_000_000.0;
    let expected = write as f64 * price.input * price.cache_write_multiplier / m
        + read as f64 * price.cache_read / m
        + out as f64 * price.output / m;
    let low = expected * 0.5;
    let high = expected * 2.5;
    let cap = ((high * 100.0).ceil() / 100.0).max(0.05);

    Ok(Estimate {
        agent: agent.to_string(),
        model: profile.model.clone(),
        low_usd: round4(low),
        expected_usd: round4(expected),
        high_usd: round4(high),
        cap_usd: cap,
        cache_write_tokens: write,
        cache_read_tokens: read,
        output_tokens: out,
        assumptions: vec![
            format!("{turns} model turns, about {out_turn} output tokens each (thinking included)"),
            format!("about {ctx} tokens of files and prompt read, {base} tokens of system prompt and tools"),
            format!(
                "{} at ${}/M input, ${}/M output, ${}/M cache read, cache write {}x input",
                profile.model, price.input, price.output, price.cache_read, price.cache_write_multiplier
            ),
            "Range is 0.5x to 2.5x of expected. The run is stopped if it reaches the cap.".into(),
        ],
        pricing_checked: pricing.checked.clone(),
        subscription_note: pricing.subscription_note.clone(),
    })
}

/// Cost of a finished run from its reported token usage, for checking the
/// estimate against reality.
pub fn cost_from_usage(
    price: &ModelPrice,
    input: u64,
    cache_write: u64,
    cache_read: u64,
    output: u64,
) -> f64 {
    let m = 1_000_000.0;
    input as f64 * price.input / m
        + cache_write as f64 * price.input * price.cache_write_multiplier / m
        + cache_read as f64 * price.cache_read / m
        + output as f64 * price.output / m
}

fn round4(x: f64) -> f64 {
    (x * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn haiku() -> ModelPrice {
        ModelPrice { input: 0.10, output: 0.50, cache_read: 0.01, cache_write_multiplier: 2.0 }
    }

    /// Real run 1 (Phase 0): plain `claude -p`, Haiku 5.5. The CLI reported
    /// total_cost_usd = 0.0035613 for 2 input, 16467 cache write, 26220 cache
    /// read, 11 output tokens.
    #[test]
    fn usage_formula_matches_real_run_1() {
        let c = cost_from_usage(&haiku(), 2, 16467, 26220, 11);
        assert!((c - 0.0035613).abs() < 1e-6, "got {c}");
    }

    /// Real run 2 (Phase 0): `--agent tester`, Haiku 5.5. Reported 0.0009911
    /// for 4 input, 3836 cache write, 3550 cache read, 376 output tokens.
    #[test]
    fn usage_formula_matches_real_run_2() {
        let c = cost_from_usage(&haiku(), 4, 3836, 3550, 376);
        assert!((c - 0.0009911).abs() < 1e-6, "got {c}");
    }

    #[test]
    fn estimate_range_contains_real_run_2() {
        let mut models = BTreeMap::new();
        models.insert("claude-haiku-5-5".to_string(), haiku());
        let pricing = Pricing {
            checked: "test".into(),
            source: "test".into(),
            subscription_note: String::new(),
            models,
        };
        let profile = AgentProfile {
            model: "claude-haiku-5-5".into(),
            turns: 2,
            context_tokens: 300,
            output_tokens_per_turn: 200,
            base_tokens: 7000,
            allowed_tools: vec![],
            description: String::new(),
            allowed_sections: vec![],
            output: None,
        };
        let e = estimate("tester", &profile, "What is the probe phrase?", &pricing).unwrap();
        assert!(e.low_usd <= 0.0009911 && 0.0009911 <= e.high_usd, "{e:?}");
        assert!(e.cap_usd >= e.high_usd);
    }
}
