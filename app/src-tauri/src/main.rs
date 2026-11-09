// Prevents an extra console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{json, Value};
use std::path::PathBuf;
use studio_core::{bridge, cost, paths, pipeline, queue, runs};
use tauri::{AppHandle, Emitter};

fn root() -> Result<PathBuf, String> {
    paths::studio_root()
}

#[tauri::command]
fn studio_info() -> Value {
    let root = root();
    let claude = paths::claude_bin();
    let pricing = root.as_ref().ok().and_then(|r| cost::load_pricing(r).ok());
    json!({
        "root": root.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        "root_error": root.err(),
        "claude_bin": claude.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
        "claude_error": claude.err(),
        "pricing_checked": pricing.as_ref().map(|p| p.checked.clone()),
        "subscription_note": pricing.as_ref().map(|p| p.subscription_note.clone()),
    })
}

#[tauri::command]
fn queue_list() -> Result<Vec<queue::QueueItem>, String> {
    queue::list(&root()?)
}

#[tauri::command]
fn queue_decide(id: String, verdict: String, note: String) -> Result<queue::QueueItem, String> {
    queue::decide(&root()?, &id, &verdict, &note)
}

#[tauri::command]
fn queue_artifact(id: String, path: String) -> Result<String, String> {
    let r = root()?;
    let item = queue::get(&r, &id)?;
    if !item.artifacts.iter().any(|a| a == &path) {
        return Err("not an artifact of this item".into());
    }
    let full = r.join(&path);
    let canon = full.canonicalize().map_err(|e| e.to_string())?;
    if !canon.starts_with(r.canonicalize().map_err(|e| e.to_string())?) {
        return Err("artifact outside studio root".into());
    }
    if canon.is_dir() {
        let mut names: Vec<String> = std::fs::read_dir(&canon)
            .map_err(|e| e.to_string())?
            .flatten()
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        names.sort();
        return Ok(format!("(folder)\n{}", names.join("\n")));
    }
    let text = std::fs::read_to_string(&canon).map_err(|e| e.to_string())?;
    Ok(text.chars().take(60_000).collect())
}

#[tauri::command]
fn agents_list() -> Result<Value, String> {
    let r = root()?;
    let cfg = cost::load_agents(&r)?;
    Ok(serde_json::to_value(cfg.agents).map_err(|e| e.to_string())?)
}

#[tauri::command]
fn run_prepare(agent: String, prompt: String) -> Result<runs::RunRequest, String> {
    runs::prepare(&root()?, &agent, &prompt)
}

#[tauri::command]
fn run_cancel(run_id: String) -> Result<runs::RunRequest, String> {
    runs::cancel(&root()?, &run_id)
}

#[tauri::command]
fn runs_list() -> Result<Vec<runs::RunRequest>, String> {
    runs::list(&root()?)
}

#[tauri::command]
fn run_loads(run_id: String) -> Result<Vec<Value>, String> {
    let p = runs::run_dir(&root()?, &run_id)?.join("loads.jsonl");
    let text = std::fs::read_to_string(&p).unwrap_or_default();
    Ok(text
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .collect())
}

/// Approve the run at the exact cap shown, then start it on a background
/// thread. Output lines arrive as `run-line` events; the end as `run-done`.
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
        let payload = match res {
            Ok(o) => serde_json::to_value(o).unwrap_or(Value::Null),
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
fn pipeline_prepare(name: String, input: String, project: Option<String>) -> Result<pipeline::PipelineRun, String> {
    let project = project.filter(|p| !p.is_empty());
    pipeline::prepare(&root()?, &name, &input, project.as_deref())
}

#[tauri::command]
fn pipeline_cancel(pipeline_id: String) -> Result<pipeline::PipelineRun, String> {
    pipeline::cancel(&root()?, &pipeline_id)
}

#[tauri::command]
fn pipeline_runs() -> Result<Vec<pipeline::PipelineRun>, String> {
    Ok(pipeline::list(&root()?))
}

/// Approve the whole pipeline at the exact total cap shown, then run its
/// stages in order on a background thread. Events: `pipe-stage`,
/// `run-line`, `pipe-done`.
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            studio_info,
            queue_list,
            queue_decide,
            queue_artifact,
            agents_list,
            run_prepare,
            run_cancel,
            runs_list,
            run_loads,
            run_approve_and_start,
            pipelines_list,
            projects_list,
            pipeline_prepare,
            pipeline_cancel,
            pipeline_runs,
            pipeline_approve_and_start
        ])
        .run(tauri::generate_context!())
        .expect("error while running Robux Studio");
}
