//! Read-only views for the app: agent status, the vault, run details,
//! artifact previews, plus importing an existing game project.

use crate::{cost, pipeline, queue, runs};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct AgentStatus {
    pub name: String,
    pub description: String,
    pub model: String,
    pub output: Option<String>,
    pub sections: Vec<String>,
    /// idle | working | waiting | blocked
    pub status: String,
    pub detail: String,
    pub last_run: Option<String>,
}

pub fn agent_statuses(root: &Path) -> Result<Vec<AgentStatus>, String> {
    let cfg = cost::load_agents(root)?;
    let all = runs::list(root)?; // newest first
    let items = queue::list(root)?;
    let mut out = vec![];
    const TEAM: [&str; 12] = ["pulse", "lead", "architect", "builder", "scenic", "librarian", "frame", "playtest", "splash", "merchant", "liveops", "producer"];
    let mut names: Vec<&String> = cfg.agents.keys().filter(|n| n.as_str() != "bridge-test").collect();
    names.sort_by_key(|n| TEAM.iter().position(|t| t == n).unwrap_or(99));
    for name in names {
        let a = &cfg.agents[name];
        let mine: Vec<&runs::RunRequest> = all.iter().filter(|r| &r.agent == name).collect();
        let (status, detail) = if let Some(r) = mine.iter().find(|r| r.status == "running") {
            ("working", format!("running {}", r.run_id))
        } else if let Some(r) = mine.iter().find(|r| r.status == "awaiting-approval" || r.status == "approved") {
            ("waiting", format!("run {} waits for your approval", r.run_id))
        } else if items.iter().any(|i| i.status == "pending" && i.created_by == *name) {
            ("waiting", "output waits in the review queue".to_string())
        } else if let Some(r) = mine.iter().find(|r| r.status == "done" || r.status == "failed") {
            if r.status == "failed" {
                ("blocked", format!("last run failed: {}", r.error.clone().unwrap_or_default().chars().take(120).collect::<String>()))
            } else {
                ("idle", format!("last run {}", r.finished_at.clone().unwrap_or_default()))
            }
        } else {
            ("idle", "no runs yet".to_string())
        };
        out.push(AgentStatus {
            name: name.clone(),
            description: a.description.clone(),
            model: a.model.clone(),
            output: a.output.clone(),
            sections: a.allowed_sections.clone(),
            status: status.into(),
            detail,
            last_run: mine.first().map(|r| r.run_id.clone()),
        });
    }
    Ok(out)
}

#[derive(Debug, Clone, Serialize)]
pub struct VaultNote {
    pub path: String,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VaultSection {
    pub name: String,
    pub notes: Vec<VaultNote>,
}

fn fm_field(text: &str, key: &str) -> String {
    text.lines()
        .skip(1)
        .take_while(|l| *l != "---")
        .find_map(|l| l.strip_prefix(&format!("{key}: ")).map(|s| s.to_string()))
        .unwrap_or_default()
}

pub fn vault(root: &Path) -> Result<Vec<VaultSection>, String> {
    let v = root.join("vault");
    let mut secs: Vec<PathBuf> = fs::read_dir(&v)
        .map_err(|e| e.to_string())?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir() && p.file_name().map(|n| n.to_string_lossy().chars().next().unwrap_or('.').is_ascii_digit()).unwrap_or(false))
        .collect();
    secs.sort();
    let mut out = vec![];
    for s in secs {
        let mut notes = vec![];
        let mut stack = vec![s.clone()];
        while let Some(d) = stack.pop() {
            for e in fs::read_dir(&d).map_err(|e| e.to_string())?.flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().and_then(|x| x.to_str()) == Some("md") {
                    let t = fs::read_to_string(&p).unwrap_or_default();
                    let title = t.lines().find(|l| l.starts_with("# ")).map(|l| l[2..].to_string()).unwrap_or_default();
                    notes.push(VaultNote {
                        path: p.strip_prefix(&v).unwrap().display().to_string(),
                        title,
                        description: fm_field(&t, "description"),
                        status: fm_field(&t, "status"),
                    });
                }
            }
        }
        notes.sort_by(|a, b| (a.path.ends_with("index.md") as u8).cmp(&(b.path.ends_with("index.md") as u8)).reverse().then(a.path.cmp(&b.path)));
        out.push(VaultSection { name: s.file_name().unwrap().to_string_lossy().to_string(), notes });
    }
    Ok(out)
}

pub fn vault_note(root: &Path, rel: &str) -> Result<String, String> {
    if rel.contains("..") || !rel.ends_with(".md") {
        return Err("not a vault note".into());
    }
    fs::read_to_string(root.join("vault").join(rel)).map_err(|e| e.to_string())
}

#[derive(Debug, Clone, Serialize)]
pub struct Step {
    pub agent: String,
    pub run_id: String,
    pub status: String,
    pub output: Option<String>,
    pub estimate_usd: f64,
    pub cap_usd: f64,
    pub actual_usd: Option<f64>,
    pub gate: Option<queue::Gate>,
    pub loaded: Vec<String>,
    /// What the agent said at the end of its run (first 1,500 characters).
    pub result_text: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunDetail {
    pub id: String,
    pub kind: String, // pipeline | run
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub estimate_usd: f64,
    pub cap_usd: f64,
    pub actual_usd: Option<f64>,
    pub queue_item: Option<String>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunSummary {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub estimate_usd: f64,
    pub actual_usd: Option<f64>,
}

fn loaded(root: &Path, run_id: &str) -> Vec<String> {
    let p = root.join("runs").join(run_id).join("loads.jsonl");
    fs::read_to_string(p)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter(|v| v.get("tool").and_then(|t| t.as_str()) == Some("Read"))
        .filter_map(|v| v.get("target").and_then(|t| t.as_str()).map(|s| s.to_string()))
        .collect()
}

fn step_for(root: &Path, r: &runs::RunRequest, gate: Option<queue::Gate>, output: Option<String>) -> Step {
    let output = output.and_then(|o| {
        let rel = format!("runs/{}/{}", r.run_id, o);
        root.join(&rel).is_file().then_some(rel)
    });
    Step {
        agent: r.agent.clone(),
        run_id: r.run_id.clone(),
        status: r.status.clone(),
        output,
        estimate_usd: r.estimate.expected_usd,
        cap_usd: r.estimate.cap_usd,
        actual_usd: r.actual_cost_usd,
        gate,
        loaded: loaded(root, &r.run_id),
        result_text: r.result_text.as_ref().map(|t| t.chars().take(1500).collect()),
    }
}

/// Pipelines and standalone runs, newest first (stage runs are folded into
/// their pipeline). Cancelled estimates are left out.
pub fn run_list(root: &Path) -> Result<Vec<RunSummary>, String> {
    let mut out: Vec<RunSummary> = pipeline::list(root)
        .into_iter()
        .filter(|p| p.status != "cancelled")
        .map(|p| RunSummary {
            id: p.pipeline_id.clone(),
            kind: "pipeline".into(),
            title: format!("{}: {}", p.pipeline, p.input.chars().take(90).collect::<String>()),
            status: p.status.clone(),
            created_at: p.created_at.clone(),
            estimate_usd: p.expected_usd,
            actual_usd: p.actual_cost_usd,
        })
        .collect();
    for r in runs::list(root)? {
        if r.pipeline_id.is_some() || r.status == "cancelled" {
            continue;
        }
        out.push(RunSummary {
            id: r.run_id.clone(),
            kind: "run".into(),
            title: format!("{}: {}", r.agent, r.prompt.chars().take(90).collect::<String>()),
            status: r.status.clone(),
            created_at: r.created_at.clone(),
            estimate_usd: r.estimate.expected_usd,
            actual_usd: r.actual_cost_usd,
        });
    }
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(out)
}

pub fn run_detail(root: &Path, id: &str) -> Result<RunDetail, String> {
    let agents = cost::load_agents(root)?;
    if root.join("runs").join(id).join("pipeline.json").is_file() {
        let p = pipeline::load(root, id)?;
        let steps = p
            .stages
            .iter()
            .filter_map(|s| runs::load(root, &s.run_id).ok().map(|r| step_for(root, &r, s.gate.clone(), Some(s.output.clone()))))
            .collect();
        return Ok(RunDetail {
            id: id.into(),
            kind: "pipeline".into(),
            title: format!("{}: {}", p.pipeline, p.input),
            status: p.status,
            created_at: p.created_at,
            estimate_usd: p.expected_usd,
            cap_usd: p.cap_usd,
            actual_usd: p.actual_cost_usd,
            queue_item: p.queue_item,
            steps,
        });
    }
    let r = runs::load(root, id)?;
    let qid = format!("q-{id}");
    let item = queue::get(root, &qid).ok();
    let output = agents.agents.get(&r.agent).and_then(|a| a.output.clone());
    Ok(RunDetail {
        id: id.into(),
        kind: "run".into(),
        title: format!("{}: {}", r.agent, r.prompt),
        status: r.status.clone(),
        created_at: r.created_at.clone(),
        estimate_usd: r.estimate.expected_usd,
        cap_usd: r.estimate.cap_usd,
        actual_usd: r.actual_cost_usd,
        queue_item: item.as_ref().map(|i| i.id.clone()),
        steps: vec![step_for(root, &r, item.map(|i| i.gate), output)],
    })
}

/// Read a studio file for preview, only from runs/ or projects/.
pub fn read_artifact(root: &Path, rel: &str) -> Result<Vec<u8>, String> {
    if rel.contains("..") || !(rel.starts_with("runs/") || rel.starts_with("projects/")) {
        return Err("only files under runs/ or projects/ can be opened".into());
    }
    fs::read(root.join(rel)).map_err(|e| e.to_string())
}

/// Save a PNG exported in the app next to its SVG source.
pub fn save_export(root: &Path, svg_rel: &str, png: &[u8]) -> Result<String, String> {
    if svg_rel.contains("..") || !svg_rel.starts_with("runs/") || !svg_rel.ends_with(".svg") {
        return Err("exports only next to an SVG under runs/".into());
    }
    if png.len() < 8 || &png[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("not a PNG".into());
    }
    let out = svg_rel.trim_end_matches(".svg").to_string() + ".png";
    fs::write(root.join(&out), png).map_err(|e| e.to_string())?;
    Ok(out)
}

/// Copy an existing Rojo game (for example ~/workspace/roblox-games/sky-tower-obby)
/// into projects/<name>/ so agents can work on it. Skips .git, node_modules,
/// Packages, and build output. Never overwrites an existing project.
pub fn import_project(root: &Path, src: &str) -> Result<String, String> {
    let src = if let Some(rest) = src.strip_prefix("~/") {
        PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(rest)
    } else {
        PathBuf::from(src)
    };
    if !src.join("default.project.json").is_file() {
        return Err(format!("{} has no default.project.json; is it a Rojo project?", src.display()));
    }
    let name: String = src
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '-' })
        .collect();
    if name.is_empty() {
        return Err("could not name the project".into());
    }
    let dest = root.join("projects").join(&name);
    if dest.exists() {
        return Err(format!("projects/{name} already exists; remove it first if you want a fresh copy"));
    }
    let mut copied = 0usize;
    copy_dir(&src, &dest, &mut copied)?;
    Ok(format!("Imported {copied} files to projects/{name}"))
}

fn copy_dir(src: &Path, dst: &Path, n: &mut usize) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for e in fs::read_dir(src).map_err(|e| e.to_string())?.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        if [".git", "node_modules", "Packages", "ServerPackages", "build", ".DS_Store"].contains(&name.as_str()) || name.ends_with(".rbxl") || name.ends_with(".rbxlx") {
            continue;
        }
        let p = e.path();
        let ft = e.file_type().map_err(|e| e.to_string())?;
        if ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            copy_dir(&p, &dst.join(&name), n)?;
        } else {
            fs::copy(&p, dst.join(&name)).map_err(|e| e.to_string())?;
            *n += 1;
        }
    }
    Ok(())
}

/// For SVG previews: replace relative `href="...png|jpg"` image links with
/// data URIs so the comp shows (and exports) with the real game captures.
/// Only files inside the studio folder are inlined.
pub fn inline_svg_images(root: &Path, svg_rel: &str, svg: &str) -> String {
    use base64::Engine;
    let base = root.join(svg_rel).parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let root_c = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let mut out = String::with_capacity(svg.len());
    let mut rest = svg;
    while let Some(i) = rest.find("href=\"") {
        let (head, tail) = rest.split_at(i + 6);
        out.push_str(head);
        let end = tail.find('"').unwrap_or(0);
        let href = &tail[..end];
        let lower = href.to_lowercase();
        let is_img = lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg");
        let mut replaced = None;
        if is_img && !href.starts_with("data:") && !href.starts_with("http") {
            if let Ok(full) = base.join(href).canonicalize() {
                if full.starts_with(&root_c) {
                    if let Ok(bytes) = fs::read(&full) {
                        let mime = if lower.ends_with(".png") { "image/png" } else { "image/jpeg" };
                        replaced = Some(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)));
                    }
                }
            }
        }
        out.push_str(replaced.as_deref().unwrap_or(href));
        rest = &tail[end..];
    }
    out.push_str(rest);
    out
}
