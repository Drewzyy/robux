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
async fn settings_patch(patch: Value) -> Result<settings::Settings, String> {
    settings::patch(&patch)
}

/// Pick what answers in Ask Claude: a Claude model ("claude:<id>", empty id
/// for the default) or a local Ollama model ("ollama:<name>").
#[tauri::command]
async fn chat_model_set(choice: String) -> Result<settings::Settings, String> {
    if let Some(m) = choice.strip_prefix("ollama:") {
        if !ollama::ensure_running() {
            return Err("Ollama is not installed or would not start. Install it free from ollama.com, then try again.".into());
        }
        settings::patch(&json!({ "engine": "ollama", "ollama_model": m }))
    } else {
        settings::patch(&json!({ "engine": "claude", "chat_model": choice.strip_prefix("claude:").unwrap_or("") }))
    }
}

/// Claude models the app has prices for, cheapest first.
#[tauri::command]
async fn models_list() -> Result<Vec<Value>, String> {
    let p = cost::load_pricing(&root()?)?;
    let mut v: Vec<Value> = p.models.iter().map(|(k, m)| json!({ "id": k, "input": m.input, "output": m.output })).collect();
    v.sort_by(|a, b| a["output"].as_f64().partial_cmp(&b["output"].as_f64()).unwrap_or(std::cmp::Ordering::Equal));
    Ok(v)
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
    spawn_run(app, r, bin, run_id);
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
    spawn_pipeline(app, r, bin, pipeline_id);
    Ok(())
}

fn spawn_run(app: AppHandle, r: std::path::PathBuf, bin: std::path::PathBuf, run_id: String) {
    std::thread::spawn(move || {
        let id = run_id.clone();
        let app2 = app.clone();
        // A crash inside the run must still end it in the UI.
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bridge::run_approved(&r, &run_id, &bin, |line| {
                let _ = app2.emit("run-line", json!({ "run_id": id, "line": line }));
            })
        }))
        .unwrap_or_else(|_| Err("the run crashed inside the app".into()));
        if let Err(e) = &res {
            runs::fail_if_unfinished(&r, &run_id, e);
        }
        let item = pipeline::finish_single(&r, &run_id).ok().flatten();
        handoff(&app, &r, &run_id, None);
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
}

fn spawn_pipeline(app: AppHandle, r: std::path::PathBuf, bin: std::path::PathBuf, pipeline_id: String) {
    std::thread::spawn(move || {
        let app2 = app.clone();
        let pid = pipeline_id.clone();
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pipeline::run(&r, &pipeline_id, &bin, |ev| match ev {
            pipeline::Event::StageStart { index, agent, run_id } => {
                let _ = app2.emit("pipe-stage", json!({"pipeline_id": pid, "index": index, "agent": agent, "run_id": run_id, "state": "start"}));
            }
            pipeline::Event::Line { run_id, line } => {
                let _ = app2.emit("run-line", json!({"run_id": run_id, "pipeline_id": pid, "line": line}));
            }
            pipeline::Event::StageDone { index, agent, gate } => {
                let _ = app2.emit("pipe-stage", json!({"pipeline_id": pid, "index": index, "agent": agent, "state": "done", "gate": gate}));
            }
        })))
        .unwrap_or_else(|_| Err("the pipeline crashed inside the app".into()));
        if let Err(e) = &res {
            pipeline::stop_unfinished(&r, &pipeline_id, e);
        }
        let payload = match res {
            Ok(p) => serde_json::to_value(p).unwrap_or(Value::Null),
            Err(e) => json!({"pipeline_id": pipeline_id, "status": "error", "error": e}),
        };
        let _ = app.emit("pipe-done", payload);
    });
}

fn spawn_chat(app: AppHandle, r: std::path::PathBuf, bin: std::path::PathBuf, run_id: String) {
    let id = run_id.clone();
    std::thread::spawn(move || {
        let app2 = app.clone();
        let id2 = id.clone();
        // A crash must still send chat-done, or Send stays disabled.
        let msg = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            chat::finish(&r, &id, &bin, |line| {
                let _ = app2.emit("chat-line", json!({ "run_id": id2, "line": line }));
            })
        }));
        if msg.is_err() {
            runs::fail_if_unfinished(&r, &id, "the chat crashed inside the app");
        }
        let msg = msg.unwrap_or_else(|_| chat::Message { role: "assistant".into(), text: "That didn't finish: the app hit an error. Try again.".into(), at: studio_core::now_rfc3339(), cost_usd: None, run_id: Some(id.clone()), failed: true, attachments: vec![] });
        let _ = app.emit("chat-done", json!({ "run_id": id, "message": msg }));
    });
}

/// Start every change request that may start now (changes.rs decides how
/// many at once); each one starts the next when it ends.
fn dispatch_changes(app: &AppHandle) {
    let Ok(r) = root() else { return };
    let Ok(bin) = paths::claude_bin() else { return };
    for c in studio_core::changes::take_next(&r) {
        spawn_change(app.clone(), r.clone(), bin.clone(), c);
    }
}

/// Start the agents a finished run asked for (requests.json).
fn handoff(app: &AppHandle, r: &std::path::Path, run_id: &str, parent: Option<&studio_core::changes::Change>) {
    let (made, notes) = studio_core::changes::ingest_requests(r, run_id, parent);
    if !made.is_empty() || !notes.is_empty() {
        let names: Vec<String> = made.iter().map(|c| c.agent.clone()).collect();
        let _ = app.emit("handoff", json!({ "run_id": run_id, "started": names, "notes": notes }));
        dispatch_changes(app);
    }
}

fn spawn_change(app: AppHandle, r: std::path::PathBuf, bin: std::path::PathBuf, c: studio_core::changes::Change) {
    std::thread::spawn(move || {
        let _ = app.emit("change-update", json!({ "id": c.id, "status": "working", "agent": c.agent }));
        let app2 = app.clone();
        let (id, run_id) = (c.id.clone(), c.run_id.clone());
        let extra = studio_core::changes::extra_dirs(&r, &c);
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bridge::run_approved_with(&r, &run_id, &bin, &extra, |line| {
                let _ = app2.emit("run-line", json!({ "run_id": run_id, "line": line }));
            })
        }))
        .unwrap_or_else(|_| Err("the run crashed inside the app".into()));
        if let Err(e) = &res {
            runs::fail_if_unfinished(&r, &run_id, e);
        }
        let done = studio_core::changes::finish(&r, &id);
        // Autopilot: retry with feedback, start agents it asked for, move on.
        if let Some(c) = &done {
            if c.autopilot.is_some() {
                studio_core::autopilot::on_finish(&r, c);
            } else if studio_core::changes::launch_failed(c) {
                // Not the agent's fault (Claude could not start, no
                // connection, signed out): queue it again to wait it out.
                studio_core::changes::retry_launch(&r, c);
            } else {
                handoff(&app, &r, &c.run_id, Some(c));
            }
        }
        let _ = app.emit("change-update", serde_json::to_value(&done).unwrap_or(Value::Null));
        dispatch_changes(&app);
    });
}

// ---------- Autopilot ----------

#[tauri::command]
async fn autopilot_list() -> Result<Value, String> {
    let r = root()?;
    Ok(json!(studio_core::autopilot::list(&r).into_iter().map(|a| json!({ "slug": a.slug, "title": a.title, "status": a.status, "milestones": a.milestones })).collect::<Vec<_>>()))
}

#[tauri::command]
async fn autopilot_estimate(idea: String) -> Result<Value, String> {
    studio_core::autopilot::estimate(&root()?, &idea)
}

/// Andrew approves an idea and its budget: the agents take it from here.
#[tauri::command]
async fn autopilot_start(app: AppHandle, title: String, idea: String, budget_usd: f64, slug: Option<String>) -> Result<Value, String> {
    let r = root()?;
    paths::claude_bin()?;
    let a = studio_core::autopilot::start(&r, &title, &idea, budget_usd, slug.as_deref().filter(|s| !s.is_empty()))?;
    dispatch_changes(&app);
    let _ = app.emit("change-update", json!({ "autopilot": a.slug }));
    Ok(json!({ "slug": a.slug }))
}

#[tauri::command]
async fn autopilot_view(slug: String) -> Result<Value, String> {
    studio_core::autopilot::view(&root()?, &slug)
}

#[tauri::command]
async fn autopilot_note(app: AppHandle, slug: String, text: String, agent: Option<String>) -> Result<(), String> {
    studio_core::autopilot::note(&root()?, &slug, &text, agent.as_deref(), None)?;
    studio_core::autopilot::advance(&root()?, &slug);
    dispatch_changes(&app);
    let _ = app.emit("change-update", json!({ "autopilot": slug }));
    Ok(())
}

#[tauri::command]
async fn autopilot_review(app: AppHandle, slug: String, milestone: String, approve: bool, notes: String) -> Result<(), String> {
    studio_core::autopilot::review(&root()?, &slug, &milestone, approve, &notes)?;
    dispatch_changes(&app);
    let _ = app.emit("change-update", json!({ "autopilot": slug }));
    Ok(())
}

#[tauri::command]
async fn autopilot_control(app: AppHandle, slug: String, action: String, budget_usd: Option<f64>) -> Result<(), String> {
    studio_core::autopilot::control(&root()?, &slug, &action, budget_usd)?;
    dispatch_changes(&app);
    let _ = app.emit("change-update", json!({ "autopilot": slug }));
    Ok(())
}

#[tauri::command]
async fn changes_list() -> Result<Value, String> {
    let r = root()?;
    Ok(json!({ "changes": studio_core::changes::list(&r), "max_at_once": studio_core::changes::MAX_AT_ONCE }))
}

#[tauri::command]
async fn changes_preview(target: String, requests: Vec<Value>) -> Result<Value, String> {
    let reqs: Vec<studio_core::changes::Request> = requests.into_iter().filter_map(|v| serde_json::from_value(v).ok()).collect();
    studio_core::changes::preview(&root()?, &target, &reqs)
}

/// Pressing Send approves each request at the cap the preview showed.
#[tauri::command]
async fn changes_send(app: AppHandle, target: String, requests: Vec<Value>) -> Result<Value, String> {
    let r = root()?;
    paths::claude_bin()?;
    let reqs: Vec<studio_core::changes::Request> = requests.into_iter().filter_map(|v| serde_json::from_value(v).ok()).collect();
    let made = studio_core::changes::submit(&r, &target, &reqs)?;
    dispatch_changes(&app);
    Ok(json!(made))
}

/// Continue paused work (usage limit or app closed). `id` is a run or a
/// pipeline; with no id, everything whose limit has reset.
fn continue_paused(app: &AppHandle, id: Option<&str>) -> Result<usize, String> {
    let manual = id.is_some();
    let r = root()?;
    let bin = paths::claude_bin()?;
    let mut n = 0;
    let pipes: Vec<pipeline::PipelineRun> = match id {
        Some(i) if i.starts_with("pipe-") => vec![pipeline::load(&r, i)?],
        Some(_) => vec![],
        None => pipeline::due(&r),
    };
    for p in pipes {
        pipeline::ready_to_continue(&r, &p.pipeline_id)?;
        spawn_pipeline(app.clone(), r.clone(), bin.clone(), p.pipeline_id);
        n += 1;
    }
    let solo: Vec<runs::RunRequest> = match id {
        Some(i) if !i.starts_with("pipe-") => {
            let x = runs::load(&r, i)?;
            if let Some(pid) = x.pipeline_id.clone() {
                return continue_paused(app, Some(&pid));
            }
            vec![x]
        }
        Some(_) => vec![],
        None => runs::due(&r),
    };
    for x in solo {
        if let Some(cid) = x.change.clone() {
            // A game Andrew paused (or that hit its budget) waits for him.
            let held = studio_core::changes::list(&r).into_iter().find(|c| c.id == cid).and_then(|c| c.autopilot).and_then(|g| studio_core::autopilot::load(&r, &g).ok()).map(|a| a.status != "running").unwrap_or(false);
            if held && id.is_none() {
                continue;
            }
            runs::ready_to_continue(&r, &x.run_id)?;
            studio_core::changes::resumed(&r, &cid);
            if let Some(c) = studio_core::changes::list(&r).into_iter().find(|c| c.id == cid) {
                spawn_change(app.clone(), r.clone(), bin.clone(), c);
            }
        } else if x.chat.is_some() {
            chat::ready_to_continue(&r, &x.run_id)?;
            spawn_chat(app.clone(), r.clone(), bin.clone(), x.run_id);
        } else {
            runs::ready_to_continue(&r, &x.run_id)?;
            spawn_run(app.clone(), r.clone(), bin.clone(), x.run_id);
        }
        n += 1;
    }
    if n > 0 {
        let _ = app.emit("work-resumed", json!({ "count": n, "manual": manual }));
    }
    Ok(n)
}

#[tauri::command]
async fn continue_now(app: AppHandle, id: String) -> Result<usize, String> {
    continue_paused(&app, Some(&id))
}

#[tauri::command]
async fn paused_work() -> Result<Value, String> {
    let r = root()?;
    let runs: Vec<Value> = runs::list(&r).unwrap_or_default().into_iter().filter(|x| x.status == "paused").map(|x| json!({"id": x.pipeline_id.clone().unwrap_or(x.run_id.clone()), "agent": x.agent, "chat": x.chat.is_some(), "resume_at": x.resume_at})).collect();
    Ok(json!(runs))
}

// ---------- Ask Claude (chat) ----------

/// Watch a video link (captions and frames) for Andrew or an agent.
#[tauri::command]
async fn video_watch(app: AppHandle, url: String) -> Result<Value, String> {
    let r = root()?;
    tauri::async_runtime::spawn_blocking(move || studio_core::video::watch(&r, &url, &mut |l| { let _ = app.emit("video-line", json!({ "line": l })); }))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn chat_attach(name: String, data: String) -> Result<String, String> {
    chat::attach(&root()?, &name, &data)
}

#[tauri::command]
async fn chat_file(path: String) -> Result<String, String> {
    // Only files saved by chat_attach, for the chat's thumbnails.
    let r = root()?;
    let p = std::path::PathBuf::from(&path);
    if !p.starts_with(chat::files_dir(&r)) || path.contains("..") {
        return Err("not a chat file".into());
    }
    let bytes = std::fs::read(&p).map_err(|e| e.to_string())?;
    use base64::Engine;
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("png").to_lowercase();
    let mime = match ext.as_str() { "jpg" | "jpeg" => "image/jpeg", "webp" => "image/webp", "gif" => "image/gif", _ => "image/png" };
    Ok(format!("data:{mime};base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

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
async fn chat_send(app: AppHandle, text: String, project: Option<String>, attachments: Option<Vec<String>>) -> Result<String, String> {
    let r = root()?;
    let bin = paths::claude_bin()?;
    let atts = attachments.unwrap_or_default();
    // Video links are watched first (captions and frames), so Claude can
    // actually see and hear them instead of guessing from the title.
    let mut text_full = text.clone();
    for url in studio_core::video::links_in(&text).into_iter().take(3) {
        let _ = app.emit("chat-line", json!({ "run_id": "", "line": json!({ "type": "assistant", "message": { "content": [{ "type": "text", "text": format!("Watching {url}…") }] } }).to_string() }));
        let r2 = r.clone();
        let u2 = url.clone();
        match tauri::async_runtime::spawn_blocking(move || studio_core::video::watch(&r2, &u2, &mut |_| {})).await {
            Ok(Ok(v)) => text_full.push_str(&format!("\n\n[The video {url} was watched for you: \"{}\". Read {} first (transcript with times), then open the frames it lists with Read; they are moments of the video in order.]", v["title"].as_str().unwrap_or(""), v["watch"].as_str().unwrap_or(""))),
            Ok(Err(e)) => text_full.push_str(&format!("\n\n[The video {url} could not be watched: {e}]")),
            Err(_) => {}
        }
    }
    let full = chat::with_context(&r, &text_full, project.as_deref(), &atts);
    let run_id = chat::begin_with(&r, &text, &full, &atts)?;
    spawn_chat(app, r, bin, run_id.clone());
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
async fn studio_mcp_test() -> Result<String, String> {
    let s = settings::load();
    tauri::async_runtime::spawn_blocking(move || studio_core::studio_mcp::test_connection(&s)).await.map_err(|e| e.to_string())?
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

#[tauri::command]
async fn textures_status() -> Result<Value, String> {
    let lib = studio_core::textures::library(&root()?);
    Ok(json!({ "count": lib.len(), "groups": studio_core::textures::GROUPS.iter().map(|(g, _)| *g).collect::<Vec<_>>() }))
}

#[tauri::command]
async fn refs_library() -> Result<Value, String> {
    studio_core::refs::library(&root()?)
}

#[tauri::command]
async fn refs_picture(path: String) -> Result<String, String> {
    studio_core::refs::picture(&root()?, &path)
}

/// Download game reference pictures into the vault (progress events).
#[tauri::command]
async fn refs_download(app: AppHandle, ids: Vec<String>, per_game: usize, refresh: bool) -> Result<(), String> {
    let r = root()?;
    std::thread::spawn(move || {
        let app2 = app.clone();
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            studio_core::refs::download(&r, &ids, per_game.clamp(1, 20), refresh, &mut |l| {
                let _ = app2.emit("ref-line", json!({ "line": l }));
            })
        }))
        .unwrap_or_else(|_| Err("the download crashed inside the app".into()));
        let _ = app.emit("ref-done", match res { Ok(m) => json!({ "ok": true, "message": m }), Err(e) => json!({ "ok": false, "message": e }) });
    });
    Ok(())
}

/// Download CC0 textures from Poly Haven into the vault (progress events).
#[tauri::command]
async fn textures_download(app: AppHandle, groups: Vec<String>, per_group: usize) -> Result<(), String> {
    let r = root()?;
    std::thread::spawn(move || {
        let app2 = app.clone();
        let res = studio_core::textures::download(&r, &groups, per_group.clamp(1, 20), "1k", &mut |l| {
            let _ = app2.emit("tex-line", json!({ "line": l }));
        });
        let _ = app.emit("tex-done", json!({ "message": res.as_ref().ok(), "error": res.err() }));
    });
    Ok(())
}

#[tauri::command]
async fn plans_list() -> Result<Vec<Value>, String> {
    Ok(studio_core::plan::list(&root()?))
}

#[tauri::command]
async fn plan_set(run_id: String, item: String, checked: Option<bool>, note: Option<String>) -> Result<(), String> {
    studio_core::plan::set(&root()?, &run_id, &item, checked, note)
}

#[tauri::command]
async fn plan_mock(run_id: String, path: String) -> Result<String, String> {
    studio_core::plan::mock(&root()?, &run_id, &path)
}

/// Figma handoff: show the plan's mockups folder and open Figma in the browser.
#[tauri::command]
async fn plan_to_figma(run_id: String) -> Result<String, String> {
    if !paths::valid_id(&run_id) {
        return Err("bad run id".into());
    }
    let d = root()?.join("runs").join(&run_id).join("mockups");
    if !d.is_dir() {
        return Err("this plan has no mockups folder".into());
    }
    paths::open_url(&d.to_string_lossy());
    paths::open_url("https://www.figma.com/files");
    Ok(d.to_string_lossy().to_string())
}

/// Prepare the build pipeline for a fully approved plan (cost card follows).
#[tauri::command]
async fn plan_build_prepare(run_id: String) -> Result<pipeline::PipelineRun, String> {
    let r = root()?;
    let input = studio_core::plan::build_input(&r, &run_id)?;
    pipeline::prepare(&r, "plan-to-build", &input, None)
}

#[tauri::command]
async fn reviews_list(engines: Option<bool>) -> Result<Value, String> {
    let r = root()?;
    if !engines.unwrap_or(false) {
        // Fast path (Dashboard): no Ollama or Codex checks, which can wait on timeouts.
        return Ok(json!({ "reviews": studio_core::review::list(&r) }));
    }
    let projects: Vec<String> = std::fs::read_dir(r.join("projects")).into_iter().flatten().flatten().filter(|e| e.path().is_dir()).map(|e| e.file_name().to_string_lossy().to_string()).collect();
    Ok(json!({ "reviews": studio_core::review::list(&r), "projects": projects, "local_model": studio_core::review::local_model(), "codex": studio_core::review::codex_installed() }))
}

/// Second opinion from a non-Claude AI (progress events).
#[tauri::command]
async fn review_start(app: AppHandle, target: String, engine: String) -> Result<(), String> {
    let r = root()?;
    // The release name the build baked in (tauri.conf's version stays 0.1.0).
    let version = update::release_tag().map(|t| t.trim_start_matches('v').to_string()).unwrap_or_default();
    if target == "app" && version.is_empty() {
        return Err("Reviewing the app itself needs an installed release (this is a development build).".into());
    }
    let _ = &app;
    std::thread::spawn(move || {
        let app2 = app.clone();
        let res = studio_core::review::run(&r, &target, &engine, &version, &mut |l| {
            let _ = app2.emit("review-line", json!({ "line": l }));
        });
        let _ = app.emit("review-done", json!({ "id": res.as_ref().ok(), "error": res.err(), "engine": engine }));
    });
    Ok(())
}

#[tauri::command]
async fn review_answer(id: String, text: String) -> Result<(), String> {
    studio_core::review::save_answer(&root()?, &id, &text)
}

#[tauri::command]
async fn review_reveal(id: String) -> Result<(), String> {
    if !paths::valid_id(&id) {
        return Err("bad id".into());
    }
    let d = root()?.join("reviews").join(&id);
    paths::open_url(&d.to_string_lossy());
    Ok(())
}

/// Latest tune-up report; runs one first when it is due (free, no AI).
#[tauri::command]
async fn tuneup_get(force: bool) -> Result<Value, String> {
    let r = root()?;
    let days = settings::load().tuneup_days;
    tauri::async_runtime::spawn_blocking(move || {
        if force {
            return studio_core::tuneup::run(&r);
        }
        Ok(studio_core::tuneup::run_if_due(&r, days).unwrap_or_else(|| studio_core::tuneup::latest(&r)))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn tuneup_fix(project: String) -> Result<String, String> {
    let r = root()?;
    tauri::async_runtime::spawn_blocking(move || studio_core::tuneup::fix(&r, &project)).await.map_err(|e| e.to_string())?
}

/// The vault as a graph: notes, their section, and the links between them.
#[tauri::command]
async fn vault_graph() -> Result<Value, String> {
    let g = studio_core::graph::build(&root()?);
    let nodes: Vec<Value> = g.values().filter(|n| !n.id.ends_with("/index")).map(|n| json!({ "id": n.id, "desc": n.desc, "out": n.out })).collect();
    Ok(json!({ "nodes": nodes }))
}

/// Claude Code usage per day and lifetime (Dashboard heatmap).
#[tauri::command]
async fn usage_heatmap() -> Result<Value, String> {
    let r = root()?;
    tauri::async_runtime::spawn_blocking(move || studio_core::usage::heatmap(&r)).await.map_err(|e| e.to_string())
}

// ---------- find my files ----------
#[tauri::command]
async fn locate_scan() -> Result<Vec<Value>, String> {
    tauri::async_runtime::spawn_blocking(studio_core::locate::scan).await.map_err(|e| e.to_string())
}
#[tauri::command]
async fn locate_use(kind: String, path: String) -> Result<String, String> {
    studio_core::locate::use_found(&kind, &path)
}
#[tauri::command]
async fn locate_merge_vault(path: String) -> Result<String, String> {
    let r = root()?;
    tauri::async_runtime::spawn_blocking(move || studio_core::locate::merge_vault(std::path::Path::new(&path), &r)).await.map_err(|e| e.to_string())?
}

// ---------- game score ----------
#[tauri::command]
async fn scores_list() -> Result<Vec<Value>, String> {
    let r = root()?;
    Ok(std::fs::read_dir(r.join("projects")).into_iter().flatten().flatten().filter(|e| e.path().join("default.project.json").is_file()).map(|e| {
        let n = e.file_name().to_string_lossy().to_string();
        json!({ "project": n, "saved": studio_core::score::saved(&r, &n) })
    }).collect())
}
#[tauri::command]
async fn score_run(project: String) -> Result<Value, String> {
    let r = root()?;
    tauri::async_runtime::spawn_blocking(move || studio_core::score::score(&r, &project)).await.map_err(|e| e.to_string())?
}

// ---------- game brief (idea checklist) ----------
#[tauri::command]
async fn brief_list() -> Result<Vec<Value>, String> {
    Ok(studio_core::brief::list(&root()?))
}
#[tauri::command]
async fn brief_create(title: String) -> Result<String, String> {
    studio_core::brief::create(&root()?, &title)
}
#[tauri::command]
async fn brief_get(slug: String) -> Result<Value, String> {
    studio_core::brief::get(&root()?, &slug)
}
#[tauri::command]
async fn brief_set(slug: String, id: String, text: String) -> Result<(), String> {
    studio_core::brief::set(&root()?, &slug, &id, &text)
}
#[tauri::command]
async fn brief_add_image(slug: String, id: String, name: String, data: String) -> Result<(), String> {
    studio_core::brief::add_image(&root()?, &slug, &id, &name, &data)
}
#[tauri::command]
async fn brief_autofill(slug: String, dump: String) -> Result<usize, String> {
    let r = root()?;
    tauri::async_runtime::spawn_blocking(move || studio_core::brief::autofill(&r, &slug, &dump)).await.map_err(|e| e.to_string())?
}
#[tauri::command]
async fn brief_prompt(slug: String) -> Result<String, String> {
    studio_core::brief::planner_prompt(&root()?, &slug)
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
    claude::export_git_bash();
    if let Ok(r) = paths::studio_root() {
        runs::recover_stale(&r);
        studio_core::changes::recover(&r);
    }
    if settings::load().ollama_autostart {
        std::thread::spawn(ollama::ensure_running);
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
            continue_now,
            changes_list,
            autopilot_list,
            autopilot_estimate,
            autopilot_start,
            autopilot_view,
            autopilot_note,
            autopilot_review,
            autopilot_control,
            changes_preview,
            changes_send,
            paused_work,
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
            textures_status,
            textures_download,
            refs_library,
            refs_picture,
            refs_download,
            plans_list,
            plan_set,
            plan_mock,
            plan_build_prepare,
            plan_to_figma,
            tuneup_get,
            tuneup_fix,
            settings_patch,
            brief_list,
            scores_list,
            locate_scan,
            vault_graph,
            usage_heatmap,
            studio_mcp_test,
            locate_use,
            locate_merge_vault,
            score_run,
            brief_create,
            brief_get,
            brief_set,
            brief_add_image,
            brief_prompt,
            brief_autofill,
            models_list,
            chat_model_set,
            chat_attach,
            video_watch,
            chat_file,
            reviews_list,
            review_start,
            review_answer,
            review_reveal,
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
        .setup(|app| {
            // Work paused by Claude's usage limit continues by itself once
            // the limit resets (checked every minute, first after 20 s).
            let h = app.handle().clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(20));
                // Change requests sent before the app closed carry on.
                dispatch_changes(&h);
                loop {
                    // Keep every running game moving (also after a restart).
                    if let Ok(r) = paths::studio_root() {
                        for a in studio_core::autopilot::list(&r).into_iter().filter(|a| a.status == "running") {
                            studio_core::autopilot::advance(&r, &a.slug);
                        }
                        dispatch_changes(&h);
                    }
                    if let Err(e) = continue_paused(&h, None) {
                        eprintln!("continue paused work: {e}");
                    }
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running BrewixGames");
}
