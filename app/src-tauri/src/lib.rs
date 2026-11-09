//! Core logic for Robux Studio, shared by the desktop app and `studio-cli`.
//!
//! Nothing in this crate publishes, posts, or contacts anyone. Agent runs go
//! through `bridge`, which refuses to start without an approved estimate.

pub mod bridge;
pub mod cost;
pub mod paths;
pub mod queue;
pub mod runs;

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
