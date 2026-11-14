// Prevents an extra console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use serde_json::{json, Value};
use std::path::PathBuf;
use studio_core::{budget, bridge, chat, claude, cost, notify, ollama, paths, pipeline, queue, runs, settings, studio, update};
use tauri::{AppHandle, Emitter};

fn root() -> Result<PathBuf, String> {
    paths::studio_root()
}

// ---------- app info, Claude, spend ----------

#[tauri::command]
async fn studio_info() -> Value {
    let root = root();
    let pricing = root.as_ref().ok().and_then(|r| cost::load_pricing(r).ok());
    json!({
        "root": root.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        "vault": root.as_ref().map(|p| p.join("vault").display().to_string()).unwrap_or_default(),
        "root_error": root.err(),
        "build": update::release_tag().map(|t| t.to_string()).unwrap_or_else(update::build_commit),
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
async fn claude_sign_in() -> Result<String, String> {
    claude::sign_in()
}

#[tauri::command]
async fn claude_install() -> Result<String, String> {
    claude::install()
}

#[tauri::command]
async fn uv_install() -> Result<String, String> {
    claude::install_uv()
}

#[tauri::command]
async fn spend() -> Result<budget::Spend, String> {
    Ok(budget::spend(&root()?, settings::load().monthly_budget_usd))
}

// ---------- settings and notifications ----------

#[tauri::command]
async fn settings_get() -> settings::Settings {
    settings::load()
}

#[tauri::command]
async fn settings_save(value: settings::Settings) -> Result<settings::Settings, String> {
    settings::save(&value)?;
    Ok(settings::load())
}

#[tauri::command]
async fn notify_test() -> Result<String, String> {
    let s = settings::load();
    if s.discord_webhook_url.is_empty() {
        return Err("Add a Discord webhook URL first.".into());
    }
    notify::send(&s, notify::Kind::Test, "BrewixGames is connected", &["You will get messages here when a build finishes or something needs your approval.".into()])?;
    Ok("Test message sent to Discord.".into())
}

// ---------- review queue ----------

#[tauri::command]
async fn queue_list() -> Result<Vec<queue::QueueItem>, String> {
    queue::list(&root()?)
}

#[tauri::command]
async fn queue_decide(id: String, verdict: String, note: String) -> Result<queue::QueueItem, String> {
    queue::decide(&root()?, &id, &verdict, &note)
}

#[tauri::command]
async fn queue_request_changes(id: String, note: String) -> Result<Value, String> {
    studio::request_changes(&root()?, &id, &note)
}

/// Text of a file in runs/ or projects/ (folders list their entries).
#[tauri::command]
async fn artifact_text(path: String) -> Result<String, String> {
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
async fn artifact_image(path: String) -> Result<String, String> {
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
async fn export_png(svg_path: String, png_base64: String) -> Result<String, String> {
    let png = base64::engine::general_purpose::STANDARD
        .decode(png_base64.trim_start_matches("data:image/png;base64,"))
        .map_err(|e| e.to_string())?;
    studio::save_export(&root()?, &svg_path, &png)
}

// ---------- agents, vault, runs ----------

#[tauri::command]
async fn agents_status() -> Result<Vec<studio::AgentStatus>, String> {
    studio::agent_statuses(&root()?)
}

#[tauri::command]
async fn agents_list() -> Result<Value, String> {
    let cfg = cost::load_agents(&root()?)?;
    serde_json::to_value(cfg.agents).map_err(|e| e.to_string())
}

#[tauri::command]
async fn vault_tree() -> Result<Vec<studio::VaultSection>, String> {
    studio::vault(&root()?)
}

#[tauri::command]
async fn vault_search(q: String) -> Result<Vec<studio_core::search::Hit>, String> {
    studio_core::search::search(&root()?, &q, &[], 25)
}

#[tauri::command]
async fn vault_note(path: String) -> Result<String, String> {
    studio::vault_note(&root()?, &path)
}

#[tauri::command]
async fn runs_overview() -> Result<Vec<studio::RunSummary>, String> {
    studio::run_list(&root()?)
}

#[tauri::command]
async fn run_detail(id: String) -> Result<studio::RunDetail, String> {
    studio::run_detail(&root()?, &id)
}

#[tauri::command]
async fn projects_list() -> Result<Vec<String>, String> {
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
async fn sounds_get(project: String) -> Result<Value, String> {
    studio::sounds_get(&root()?, &project)
}

#[tauri::command]
async fn sounds_set_asset(project: String, id: String, asset_id: String, license: String) -> Result<Value, String> {
    studio::sounds_set_asset(&root()?, &project, &id, &asset_id, &license)
}

#[tauri::command]
async fn project_import(path: String) -> Result<String, String> {
    studio::import_project(&root()?, &path)
}

// ---------- starting runs (always estimate first, then approve) ----------

#[tauri::command]
async fn run_prepare(agent: String, prompt: String) -> Result<runs::RunRequest, String> {
    runs::prepare(&root()?, &agent, &prompt)
}

#[tauri::command]
async fn run_cancel(run_id: String) -> Result<runs::RunRequest, String> {
    runs::cancel(&root()?, &run_id)
}

#[tauri::command]
async fn run_approve_and_start(app: AppHandle, run_id: String, confirmed_cap_usd: f64) -> Result<(), String> {
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
async fn pipelines_list() -> Result<Value, String> {
    let cfg = pipeline::load_config(&root()?)?;
    serde_json::to_value(cfg.pipelines).map_err(|e| e.to_string())
}

#[tauri::command]
async fn pipeline_prepare(name: String, input: String, project: Option<String>) -> Result<pipeline::PipelineRun, String> {
    let project = project.filter(|p| !p.is_empty());
    pipeline::prepare(&root()?, &name, &input, project.as_deref())
}

#[tauri::command]
async fn pipeline_cancel(pipeline_id: String) -> Result<pipeline::PipelineRun, String> {
    pipeline::cancel(&root()?, &pipeline_id)
}

#[tauri::command]
async fn pipeline_approve_and_start(app: AppHandle, pipeline_id: String, confirmed_cap_usd: f64) -> Result<(), String> {
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

// ---------- Ask Claude (chat) ----------

#[tauri::command]
async fn chat_get() -> Result<Value, String> {
    let r = root()?;
    Ok(json!({ "chat": chat::load(&r), "cap_usd": chat::cap(&r)?, "engine": settings::load().engine }))
}

#[tauri::command]
async fn chat_reset() -> Result<(), String> {
    chat::reset(&root()?)
}

/// Send one message. Pressing Send approves this message at the cap shown.
#[tauri::command]
async fn chat_send(app: AppHandle, text: String) -> Result<String, String> {
    let r = root()?;
    let bin = paths::claude_bin()?;
    let run_id = chat::begin(&r, &text)?;
    let id = run_id.clone();
    std::thread::spawn(move || {
        let app2 = app.clone();
        let id2 = id.clone();
        let msg = chat::finish(&r, &id, &bin, |line| {
            let _ = app2.emit("chat-line", json!({ "run_id": id2, "line": line }));
        });
        let _ = app.emit("chat-done", json!({ "run_id": id, "message": msg }));
    });
    Ok(run_id)
}

// ---------- free local models (Ollama) ----------

#[tauri::command]
async fn ollama_status() -> ollama::Status {
    ollama::status()
}

/// Download one model from the supported list. Andrew presses Download for
/// a named model with its size shown; nothing downloads otherwise.
#[tauri::command]
async fn ollama_pull(app: AppHandle, model: String) -> Result<(), String> {
    std::thread::spawn(move || {
        let app2 = app.clone();
        let res = ollama::pull(&model, &mut |l| {
            let _ = app2.emit("ollama-line", json!({ "model": model, "line": l }));
        });
        let _ = app.emit("ollama-done", json!({ "model": model, "error": res.err() }));
    });
    Ok(())
}

// ---------- Roblox Studio connection ----------

#[tauri::command]
async fn studio_mcp_status() -> Value {
    let found = studio_core::studio_mcp::default_path();
    json!({ "found": found.as_ref().map(|p| p.display().to_string()), "agents": studio_core::studio_mcp::AGENTS })
}

#[tauri::command]
async fn blender_mcp_status() -> Value {
    json!({ "uvx": studio_core::blender_mcp::uvx().map(|p| p.display().to_string()), "package": studio_core::blender_mcp::PACKAGE })
}

#[tauri::command]
async fn blender_addon_get() -> Result<String, String> {
    studio_core::blender_mcp::fetch_addon(&root()?).map(|p| p.display().to_string())
}

#[tauri::command]
async fn project_open_in_studio(project: String) -> Result<String, String> {
    studio::open_in_studio(&root()?, &project)
}

#[tauri::command]
async fn studio_move(dest: String) -> Result<String, String> {
    studio::move_studio(&root()?, &dest)
}

#[tauri::command]
async fn tools_dir_get() -> String {
    paths::tools_dir().display().to_string()
}

#[tauri::command]
async fn pitches_latest() -> Result<Value, String> {
    Ok(studio::latest_pitches(&root()?))
}

/// The system's folder picker; returns the chosen folder or nothing.
#[tauri::command]
async fn pick_folder(app: AppHandle) -> Option<String> {
    use tauri_plugin_dialog::DialogExt;
    app.dialog().file().blocking_pick_folder().map(|f| f.to_string())
}

#[tauri::command]
async fn apps_status() -> studio_core::apps::Status {
    studio_core::apps::status()
}

/// Install Blender or Obsidian into the Downloads folder (progress events).
#[tauri::command]
async fn app_install(app: AppHandle, name: String) -> Result<(), String> {
    std::thread::spawn(move || {
        let app2 = app.clone();
        let res = studio_core::apps::install(&name, &mut |l| {
            let _ = app2.emit("app-line", json!({ "app": name, "line": l }));
        });
        let _ = app.emit("app-done", json!({ "app": name, "message": res.as_ref().ok(), "error": res.err() }));
    });
    Ok(())
}

#[tauri::command]
async fn app_open(name: String) -> Result<(), String> {
    studio_core::apps::open(&name, &root()?.join("vault"))
}

#[tauri::command]
async fn refs_list(project: String) -> Result<Vec<studio_core::references::Reference>, String> {
    studio_core::references::list(&root()?, &project)
}

#[tauri::command]
async fn refs_add(project: String, name: String, data: String, category: String, note: String, source: String) -> Result<studio_core::references::Reference, String> {
    studio_core::references::add(&root()?, &project, &name, &data, &category, &note, &source)
}

#[tauri::command]
async fn refs_remove(project: String, file: String) -> Result<(), String> {
    studio_core::references::remove(&root()?, &project, &file)
}

#[tauri::command]
async fn refs_image(project: String, file: String) -> Result<String, String> {
    studio_core::references::image(&root()?, &project, &file)
}

#[tauri::command]
async fn money_summary() -> Result<studio_core::earnings::Summary, String> {
    Ok(studio_core::earnings::summary(&root()?, settings::load().devex_rate, chrono::Local::now().date_naive()))
}

#[tauri::command]
async fn money_add(date: String, game: String, robux: i64, note: String) -> Result<(), String> {
    studio_core::earnings::add(&root()?, studio_core::earnings::Entry { date, game, robux, note })
}

#[tauri::command]
async fn money_remove(index: usize) -> Result<(), String> {
    studio_core::earnings::remove(&root()?, index)
}

#[tauri::command]
async fn money_import(text: String, game: String) -> Result<usize, String> {
    studio_core::earnings::import_csv(&root()?, &text, &game)
}

// ---------- simulator and thumbnails ----------

#[tauri::command]
async fn simulations(project: String) -> Result<Vec<Value>, String> {
    studio::simulations(&root()?, &project)
}

#[tauri::command]
async fn thumbnails(project: String) -> Result<Value, String> {
    studio::thumbnails(&root()?, &project)
}

#[tauri::command]
async fn screenshot_add(project: String, path: String) -> Result<String, String> {
    studio::add_screenshot(&root()?, &project, &path)
}

// ---------- updates ----------

#[tauri::command]
async fn update_check() -> Result<update::Status, String> {
    update::check(&root()?)
}

/// Download (git pull), rebuild, then restart the app automatically.
/// Progress arrives as `update-line` events.
#[tauri::command]
async fn update_apply(app: AppHandle) -> Result<(), String> {
    let r = root()?;
    std::thread::spawn(move || {
        let app2 = app.clone();
        let mut on = |l: &str| {
            let _ = app2.emit("update-line", json!({ "line": l }));
        };
        match update::apply(&r, &mut on) {
            Ok(_) => {
                on("Update installed. Restarting...");
                notify::send_quiet(notify::Kind::Update, "BrewixGames updated", &[format!("Now on {}", update::check(&r).map(|s| s.local).unwrap_or_default())]);
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
    // `brewix gate ...` / `brewix hook-log`: headless helpers agents call.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(code) = studio_core::gates::cli(&args) {
        std::process::exit(code);
    }
    // Downloaded app, first launch: set up ~/BrewixGames from the bundled template.
    if let Err(e) = studio::ensure_studio() {
        eprintln!("could not set up the studio folder: {e}");
    }
    // First launch after an update: refresh the vault and agents (backed up first).
    if let Ok(r) = paths::studio_root() {
        if let Some(m) = studio::sync_release(&r) {
            eprintln!("{m}");
        }
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            studio_info,
            claude_status,
            claude_sign_in,
            claude_install,
            spend,
            settings_get,
            settings_save,
            notify_test,
            queue_list,
            queue_decide,
            queue_request_changes,
            artifact_text,
            artifact_image,
            export_png,
            agents_status,
            agents_list,
            vault_tree,
            vault_search,
            vault_note,
            runs_overview,
            run_detail,
            projects_list,
            project_import,
            sounds_get,
            sounds_set_asset,
            run_prepare,
            run_cancel,
            run_approve_and_start,
            pipelines_list,
            pipeline_prepare,
            pipeline_cancel,
            pipeline_approve_and_start,
            chat_get,
            ollama_status,
            studio_mcp_status,
            blender_mcp_status,
            blender_addon_get,
            project_open_in_studio,
            studio_move,
            uv_install,
            tools_dir_get,
            pitches_latest,
            apps_status,
            pick_folder,
            refs_list,
            money_summary,
            money_add,
            money_remove,
            money_import,
            refs_add,
            refs_remove,
            refs_image,
            app_install,
            app_open,
            ollama_pull,
            chat_reset,
            chat_send,
            simulations,
            thumbnails,
            screenshot_add,
            update_check,
            update_apply
        ])
        .run(tauri::generate_context!())
        .expect("error while running BrewixGames");
}
