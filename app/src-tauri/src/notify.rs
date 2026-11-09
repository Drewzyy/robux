//! Discord webhook notifications. Andrew's own webhook, set in Settings.
//! Messages never include file contents, prompts, or paths outside the
//! studio, and never ping anyone (allowed_mentions is empty).

use crate::settings::Settings;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    NeedsApproval,
    Finished,
    Failed,
    Budget,
    Update,
    Test,
}

impl Kind {
    fn enabled(self, s: &Settings) -> bool {
        match self {
            Kind::NeedsApproval => s.notify.needs_approval,
            Kind::Finished => s.notify.finished,
            Kind::Failed => s.notify.failed,
            Kind::Budget => s.notify.budget,
            Kind::Update => s.notify.updates,
            Kind::Test => true,
        }
    }
    fn color(self) -> u32 {
        match self {
            Kind::NeedsApproval | Kind::Budget | Kind::Failed => 0xFF3B30,
            _ => 0x8E8E93,
        }
    }
}

/// Only real Discord webhook URLs. A loopback URL is allowed only when
/// ROBUX_ALLOW_LOCAL_WEBHOOK=1 (used by tests with a local mock server).
pub fn validate_url(url: &str) -> Result<(), String> {
    let ok_hosts = [
        "https://discord.com/api/webhooks/",
        "https://discordapp.com/api/webhooks/",
        "https://canary.discord.com/api/webhooks/",
        "https://ptb.discord.com/api/webhooks/",
    ];
    if ok_hosts.iter().any(|h| url.starts_with(h)) && url.len() > 40 {
        return Ok(());
    }
    if std::env::var("ROBUX_ALLOW_LOCAL_WEBHOOK").as_deref() == Ok("1") && url.starts_with("http://127.0.0.1:") {
        return Ok(());
    }
    Err("that is not a Discord webhook URL (it should start with https://discord.com/api/webhooks/)".into())
}

pub fn payload(kind: Kind, title: &str, lines: &[String]) -> serde_json::Value {
    let description: String = lines.join("\n").chars().take(1800).collect();
    serde_json::json!({
        "username": "Robux Studio",
        "allowed_mentions": { "parse": [] },
        "embeds": [{
            "title": title.chars().take(240).collect::<String>(),
            "description": description,
            "color": kind.color(),
            "footer": { "text": "Robux Studio · nothing is published without your approval" }
        }]
    })
}

/// Send if a webhook is set and this kind is switched on. Returns Ok(false)
/// when nothing was sent because notifications are off.
pub fn send(s: &Settings, kind: Kind, title: &str, lines: &[String]) -> Result<bool, String> {
    if s.discord_webhook_url.is_empty() || !kind.enabled(s) {
        return Ok(false);
    }
    validate_url(&s.discord_webhook_url)?;
    let body = payload(kind, title, lines);
    match ureq::post(&s.discord_webhook_url)
        .timeout(std::time::Duration::from_secs(10))
        .send_json(body)
    {
        Ok(_) => Ok(true),
        Err(ureq::Error::Status(code, _)) => Err(format!("Discord answered {code}")),
        Err(e) => Err(format!("could not reach Discord: {e}")),
    }
}

/// Fire-and-forget for background events: failures are written to stderr
/// and never stop a run.
pub fn send_quiet(kind: Kind, title: &str, lines: &[String]) {
    let s = crate::settings::load();
    if let Err(e) = send(&s, kind, title, lines) {
        eprintln!("discord notification failed: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_discord_urls() {
        assert!(validate_url("https://discord.com/api/webhooks/123456789012345678/abcdefghijklmnop").is_ok());
        assert!(validate_url("https://evil.example.com/api/webhooks/123").is_err());
        assert!(validate_url("http://discord.com/api/webhooks/123456789012345678/abc").is_err());
    }

    #[test]
    fn payload_never_pings() {
        let p = payload(Kind::NeedsApproval, "t", &["a".into()]);
        assert_eq!(p["allowed_mentions"]["parse"].as_array().unwrap().len(), 0);
        assert_eq!(p["embeds"][0]["color"], 0xFF3B30);
    }
}
