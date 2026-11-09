// Prevents an extra console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use serde_json::{json, Value};
use std::path::PathBuf;
use studio_core::{budget, bridge, claude, cost, notify, paths, pipeline, queue, runs, settings, studio, update};
use tauri::{AppHandle, Emitter};

fn root() -> Result<PathBuf, String> {
    paths::studio_root()
}

// ---------- app info, Claude, spend ----------

#[tauri::command]
fn studio_info() -> Value {
    let root = root();
    let pricing = root.as_ref().ok().and_then(|r| cost::load_pricing(r).ok());
    json!({
        "root": root.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        "vault": root.as_ref().map(|p| p.join("vault").display().to_string()).unwrap_or_default(),
        "root_error": root.err(),
        "build": update::build_commit(),
        "pricing_checked": pricing.as_ref().map(|p| p.checked.clone()),
        "subscription_note": pricing.as_ref().map(|p| p.subscription_note.clone()),
        "config_file": settings::path().display().to_string(),
    })
}

#[tauri::command]
async fn claude_status() -> claude::Connection {
    claude::status()
}

#[tauri::command]
fn claude_sign_in() -> Result<String, String> {
    claude::sign_in()
}

#[tauri::command]
fn spend() -> Result<budget::Spend, String> {
    Ok(budget::spend(&root()?, settings::load().monthly_budget_usd))
}

// ---------- settings and notifications ----------

#[tauri::command]
fn settings_get() -> settings::Settings {
    settings::load()
}

#[tauri::command]
fn settings_save(value: settings::Settings) -> Result<settings::Settings, String> {
    settings::save(&value)?;
    Ok(settings::load())
}

#[tauri::command]
async fn notify_test() -> Result<String, String> {
    let s = settings::load();
    if s.discord_webhook_url.is_empty() {
        return Err("Add a Discord webhook URL first.".into());
    }
    notify::send(&s, notify::Kind::Test, "Robux Studio is connected", &["You will get messages here when a build finishes or something needs your approval.".into()])?;
    Ok("Test message sent to Discord.".into())
}

// ---------- review queue ----------

#[tauri::command]
fn queue_list() -> Result<Vec<queue::QueueItem>, String> {
    queue::list(&root()?)
}

#[tauri::command]
fn queue_decide(id: String, verdict: String, note: String) -> Result<queue::QueueItem, String> {
    queue::decide(&root()?, &id, &verdict, &note)
}

/// Text of a file in runs/ or projects/ (folders list their entries).
#[tauri::command]
fn artifact_text(path: String) -> Result<String, String> {
    let r = root()?;
    let full = r.join(&path);
    if full.is_dir() && (path.starts_with("projects/") || path.starts_with("runs/")) && !path.contains("..") {
        let mut names: Vec<String> = std::fs::read_dir(&full)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        return Ok(format!("(folder)\n{}", names.join("\n")));
    }
    let bytes = studio::read_artifact(&r, &path)?;
    Ok(String::from_utf8_lossy(&bytes).chars().take(80_000).collect())
}

/// An image artifact as a data URL for previews (svg, png, jpg).
#[tauri::command]
fn artifact_image(path: String) -> Result<String, String> {
    let mime = if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else {
        return Err("not an image".into());
    };
    let r = root()?;
    let mut bytes = studio::read_artifact(&r, &path)?;
    if mime == "image/svg+xml" {
        bytes = studio::inline_svg_images(&r, &path, &String::from_utf8_lossy(&bytes)).into_bytes();
    }
    Ok(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

#[tauri::command]
fn export_png(svg_path: String, png_base64: String) -> Result<String, String> {
    let png = base64::engine::general_purpose::STANDARD
        .decode(png_base64.trim_start_matches("data:image/png;base64,"))
        .map_err(|e| e.to_string())?;
    studio::save_export(&root()?, &svg_path, &png)
}

// ---------- agents, vault, runs ----------

#[tauri::command]
fn agents_status() -> Result<Vec<studio::AgentStatus>, String> {
    studio::agent_statuses(&root()?)
}

#[tauri::command]
fn agents_list() -> Result<Value, String> {
    let cfg = cost::load_agents(&root()?)?;
    serde_json::to_value(cfg.agents).map_err(|e| e.to_string())
}

#[tauri::command]
fn vault_tree() -> Result<Vec<studio::VaultSection>, String> {
    studio::vault(&root()?)
}

#[tauri::command]
fn vault_note(path: String) -> Result<String, String> {
    studio::vault_note(&root()?, &path)
}

#[tauri::command]
fn runs_overview() -> Result<Vec<studio::RunSummary>, String> {
    studio::run_list(&root()?)
}

#[tauri::command]
fn run_detail(id: String) -> Result<studio::RunDetail, String> {
    studio::run_detail(&root()?, &id)
}

#[tauri::command]
fn projects_list() -> Result<Vec<String>, String> {
    let mut out = vec![];
    if let Ok(entries) = std::fs::read_dir(root()?.join("projects")) {
        for e in entries.flatten() {
            if e.path().join("default.project.json").is_file() {
                out.push(format!("projects/{}", e.file_name().to_string_lossy()));
            }
        }
    }
    out.sort();
    Ok(out)
}

#[tauri::command]
fn project_import(path: String) -> Result<String, String> {
    studio::import_project(&root()?, &path)
}

// ---------- starting runs (always estimate first, then approve) ----------

#[tauri::command]
fn run_prepare(agent: String, prompt: String) -> Result<runs::RunRequest, String> {
    runs::prepare(&root()?, &agent, &prompt)
}

#[tauri::command]
fn run_cancel(run_id: String) -> Result<runs::RunRequest, String> {
    runs::cancel(&root()?, &run_id)
}

#[tauri::command]
fn run_approve_and_start(app: AppHandle, run_id: String, confirmed_cap_usd: f64) -> Result<(), String> {
    let r = root()?;
    let bin = paths::claude_bin()?;
    runs::approve(&r, &run_id, confirmed_cap_usd)?;
    std::thread::spawn(move || {
        let id = run_id.clone();
        let app2 = app.clone();
        let res = bridge::run_approved(&r, &run_id, &bin, |line| {
            let _ = app2.emit("run-line", json!({ "run_id": id, "line": line }));
        });
        let item = pipeline::finish_single(&r, &run_id).ok().flatten();
        let payload = match res {
            Ok(o) => {
                let mut v = serde_json::to_value(o).unwrap_or(Value::Null);
                v["queue_item"] = json!(item.map(|i| i.id));
                v
            }
            Err(e) => json!({ "run_id": run_id, "ok": false, "error": e }),
        };
        let _ = app.emit("run-done", payload);
    });
    Ok(())
}

#[tauri::command]
fn pipelines_list() -> Result<Value, String> {
    let cfg = pipeline::load_config(&root()?)?;
    serde_json::to_value(cfg.pipelines).map_err(|e| e.to_string())
}

#[tauri::command]
fn pipeline_prepare(name: String, input: String, project: Option<String>) -> Result<pipeline::PipelineRun, String> {
    let project = project.filter(|p| !p.is_empty());
    pipeline::prepare(&root()?, &name, &input, project.as_deref())
}

#[tauri::command]
fn pipeline_cancel(pipeline_id: String) -> Result<pipeline::PipelineRun, String> {
    pipeline::cancel(&root()?, &pipeline_id)
}

#[tauri::command]
fn pipeline_approve_and_start(app: AppHandle, pipeline_id: String, confirmed_cap_usd: f64) -> Result<(), String> {
    let r = root()?;
    let bin = paths::claude_bin()?;
    pipeline::approve(&r, &pipeline_id, confirmed_cap_usd)?;
    std::thread::spawn(move || {
        let app2 = app.clone();
        let pid = pipeline_id.clone();
        let res = pipeline::run(&r, &pipeline_id, &bin, |ev| match ev {
            pipeline::Event::StageStart { index, agent, run_id } => {
                let _ = app2.emit("pipe-stage", json!({"pipeline_id": pid, "index": index, "agent": agent, "run_id": run_id, "state": "start"}));
            }
            pipeline::Event::Line { run_id, line } => {
                let _ = app2.emit("run-line", json!({"run_id": run_id, "pipeline_id": pid, "line": line}));
            }
            pipeline::Event::StageDone { index, agent, gate } => {
                let _ = app2.emit("pipe-stage", json!({"pipeline_id": pid, "index": index, "agent": agent, "state": "done", "gate": gate}));
            }
        });
        let payload = match res {
            Ok(p) => serde_json::to_value(p).unwrap_or(Value::Null),
            Err(e) => json!({"pipeline_id": pipeline_id, "status": "error", "error": e}),
        };
        let _ = app.emit("pipe-done", payload);
    });
    Ok(())
}

// ---------- updates ----------

#[tauri::command]
async fn update_check() -> Result<update::Status, String> {
    update::check(&root()?)
}

/// Download (git pull), rebuild, then restart the app automatically.
/// Progress arrives as `update-line` events.
#[tauri::command]
fn update_apply(app: AppHandle) -> Result<(), String> {
    let r = root()?;
    std::thread::spawn(move || {
        let app2 = app.clone();
        let mut on = |l: &str| {
            let _ = app2.emit("update-line", json!({ "line": l }));
        };
        match update::apply(&r, &mut on) {
            Ok(_) => {
                on("Update installed. Restarting...");
                notify::send_quiet(notify::Kind::Update, "Robux Studio updated", &[format!("Now on {}", update::check(&r).map(|s| s.local).unwrap_or_default())]);
                std::thread::sleep(std::time::Duration::from_millis(600));
                app.restart();
            }
            Err(e) => {
                let _ = app.emit("update-line", json!({ "line": format!("Update failed: {e}"), "error": true }));
            }
        }
    });
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            studio_info,
            claude_status,
            claude_sign_in,
            spend,
            settings_get,
            settings_save,
            notify_test,
            queue_list,
            queue_decide,
            artifact_text,
            artifact_image,
            export_png,
            agents_status,
            agents_list,
            vault_tree,
            vault_note,
            runs_overview,
            run_detail,
            projects_list,
            project_import,
            run_prepare,
            run_cancel,
            run_approve_and_start,
            pipelines_list,
            pipeline_prepare,
            pipeline_cancel,
            pipeline_approve_and_start,
            update_check,
            update_apply
        ])
        .run(tauri::generate_context!())
        .expect("error while running Robux Studio");
}
