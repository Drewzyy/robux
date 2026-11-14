//! Core logic for BrewixGames, shared by the desktop app and `studio-cli`.
//!
//! Nothing in this crate publishes, posts, or contacts anyone. Agent runs go
//! through `bridge`, which refuses to start without an approved estimate.

pub mod apps;
pub mod blender_mcp;
pub mod bridge;
pub mod budget;
pub mod chat;
pub mod earnings;
pub mod claude;
pub mod cost;
pub mod gates;
pub mod notify;
pub mod ollama;
pub mod settings;
pub mod studio;
pub mod studio_mcp;
pub mod textures;
pub mod plan;
pub mod update;
pub mod paths;
pub mod pipeline;
pub mod queue;
pub mod references;
pub mod runs;
pub mod search;

pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

/// Short unique id: UTC timestamp plus a counter-free random suffix.
pub fn new_id(prefix: &str) -> String {
    let now = chrono::Utc::now();
    let nanos = now.timestamp_subsec_nanos();
    let pid = std::process::id();
    format!(
        "{}-{}-{:04x}",
        prefix,
        now.format("%Y%m%d-%H%M%S"),
        (nanos ^ pid) & 0xffff
    )
}
