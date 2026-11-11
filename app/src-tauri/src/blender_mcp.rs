//! Blender connection through the free, open-source MCP for Blender server
//! (PyPI `mcp-for-blender`, pinned below), which talks to an add-on inside an
//! open Blender. Off by default. Only the Modeler agent gets it.
//!
//! Zero spend and safe by default:
//! - telemetry is switched off through the server's own environment flags;
//! - the paid 3D generators (Tripo, Hunyuan3D, Hyper3D, Meshy) are never
//!   offered, and the feedback-upload tool is not allowed;
//! - asset search and import are limited to Poly Haven (all CC0);
//! - Python run inside Blender may not reach the network, start programs,
//!   install add-ons, or delete folders (`guard`, a PreToolUse hook).
//!
//! `brewix mock-blender-mcp` is a stand-in server with the same tools.

use std::io::{BufRead, Write};

pub const SERVER: &str = "Blender";
pub const PACKAGE: &str = "mcp-for-blender==2.2.4";
pub const AGENTS: &[&str] = &["modeler", "assistant"];
/// Tools the agents may call. generate_3d (paid services) and
/// record_trajectory_feedback (uploads data) are deliberately left out.
pub const TOOLS: &[&str] = &["get_addon_status", "get_scene_info", "execute_blender_code", "look", "get_viewport_screenshot", "search_assets", "import_asset"];
pub const TELEMETRY_OFF: &[&str] = &["DISABLE_TELEMETRY", "BLENDER_MCP_DISABLE_TELEMETRY", "MCP_DISABLE_TELEMETRY"];

/// `uvx` (from uv, free) runs the pinned server without a separate install.
pub fn uvx() -> Option<std::path::PathBuf> {
    let name = if cfg!(windows) { "uvx.exe" } else { "uvx" };
    let mut dirs: Vec<std::path::PathBuf> = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default();
    if let Some(h) = crate::paths::home() {
        dirs.push(h.join(".local/bin"));
        dirs.push(h.join(".cargo/bin"));
        dirs.push(h.join("AppData/Local/uv/bin"));
    }
    dirs.push("/opt/homebrew/bin".into());
    dirs.push("/usr/local/bin".into());
    dirs.into_iter().map(|d| d.join(name)).find(|p| p.is_file())
}

/// Copy the Blender add-on that ships inside the pinned package to
/// <studio>/tools/mcp_for_blender_addon.py, for Andrew to install in Blender
/// (Edit, Preferences, Add-ons, Install from Disk). Returns the path.
pub fn fetch_addon(root: &std::path::Path) -> Result<std::path::PathBuf, String> {
    let uvx = uvx().ok_or("uv is not installed. It is free: astral.sh/uv")?;
    let mut cmd = crate::paths::command(uvx);
    cmd.args(["--from", PACKAGE, "python", "-c", "import os, blender_mcp.bundled as b; print(os.path.join(os.path.dirname(b.__file__), 'addon.py'))"]);
    for k in TELEMETRY_OFF {
        cmd.env(k, "1");
    }
    let o = cmd.output().map_err(|e| format!("could not run uvx: {e}"))?;
    let src = String::from_utf8_lossy(&o.stdout).trim().to_string();
    if !o.status.success() || src.is_empty() {
        return Err(format!("could not get the add-on: {}", String::from_utf8_lossy(&o.stderr).lines().last().unwrap_or("")));
    }
    let dest = root.join("tools").join("mcp_for_blender_addon.py");
    std::fs::create_dir_all(dest.parent().unwrap()).map_err(|e| e.to_string())?;
    std::fs::copy(&src, &dest).map_err(|e| format!("copy add-on: {e}"))?;
    Ok(dest)
}

pub fn server_command(s: &crate::settings::Settings) -> Option<(String, Vec<String>)> {
    match s.blender_mcp.as_str() {
        "mock" => Some((std::env::current_exe().ok()?.display().to_string(), vec!["mock-blender-mcp".into()])),
        "blender" => Some((uvx()?.display().to_string(), vec![PACKAGE.into()])),
        _ => None,
    }
}

/// The mcpServers entry for this server (telemetry off).
pub fn config_entry(cmd: &(String, Vec<String>)) -> serde_json::Value {
    let env: serde_json::Map<String, serde_json::Value> = TELEMETRY_OFF.iter().map(|k| (k.to_string(), serde_json::json!("1"))).collect();
    serde_json::json!({ "type": "stdio", "command": cmd.0, "args": cmd.1, "env": env })
}

pub fn tool_names() -> Vec<String> {
    TOOLS.iter().map(|t| format!("mcp__{SERVER}__{t}")).collect()
}

const DENY_CODE: &[(&str, &str)] = &[
    (r"(?i)\b(requests|urllib|http\.client|httpx|aiohttp|socket|ftplib|smtplib)\b", "the network"),
    (r"(?i)\bsubprocess\b|os\.system|os\.popen|os\.exec|os\.spawn", "starting other programs"),
    (r"(?i)shutil\.rmtree|os\.remove|os\.unlink|os\.rmdir", "deleting files"),
    (r"(?i)addon_install|preferences\.addon|wm\.url_open|wm\.quit_blender", "add-ons, links, or closing Blender"),
    (r"(?i)\bexec\s*\(|\beval\s*\(|__import__|compile\s*\(", "running hidden code"),
    (r"(?i)api[_ ]?key|token|password|cookie", "credentials"),
];

/// Ok when a Blender tool call is safe; Err with the reason otherwise.
pub fn guard(tool: &str, input: &serde_json::Value) -> Result<(), String> {
    let short = tool.strip_prefix(&format!("mcp__{SERVER}__")).unwrap_or(tool);
    let block = |what: &str| Err(format!("Blocked by BrewixGames: {what}. Zero spend and nothing sent online; use Poly Haven (CC0) assets or model it yourself in Blender."));
    if !TOOLS.contains(&short) {
        return block(&format!("the Blender tool {short} is not allowed (paid generators and data uploads are off)"));
    }
    if short == "search_assets" || short == "import_asset" {
        let src = input.get("source").and_then(|s| s.as_str()).unwrap_or("").to_lowercase();
        if src != "polyhaven" {
            return block(&format!("asset source {:?} is not allowed; only polyhaven (CC0) is", if src.is_empty() { "(none)" } else { &src }));
        }
    }
    if short == "execute_blender_code" {
        let code = input.get("code").and_then(|s| s.as_str()).unwrap_or_else(|| input.as_str().unwrap_or(""));
        let code = if code.is_empty() { input.to_string() } else { code.to_string() };
        for (pat, what) in DENY_CODE {
            if regex::Regex::new(pat).unwrap().is_match(&code) {
                return block(&format!("Blender code from an agent may not touch {what}"));
            }
        }
    }
    Ok(())
}

/// `brewix blender-guard`: PreToolUse hook; exit 2 blocks with the reason.
pub fn guard_hook(stdin: &str) -> i32 {
    let v: serde_json::Value = serde_json::from_str(stdin).unwrap_or_default();
    let tool = v.get("tool_name").and_then(|t| t.as_str()).unwrap_or("");
    if !tool.starts_with(&format!("mcp__{SERVER}__")) {
        return 0;
    }
    match guard(tool, v.get("tool_input").unwrap_or(&serde_json::Value::Null)) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            2
        }
    }
}

/// Stand-in Blender MCP server (stdio, newline-delimited JSON-RPC). Keeps a
/// pretend scene; "exports" write a small placeholder file where the code
/// asks (filepath="..."), so the agent's export step can be checked.
pub fn mock_server() -> i32 {
    let tools: Vec<serde_json::Value> = TOOLS
        .iter()
        .chain(["generate_3d"].iter())
        .map(|t| serde_json::json!({"name": t, "description": format!("mock {t}"), "inputSchema": match *t {
            "execute_blender_code" => serde_json::json!({"type":"object","properties":{"code":{"type":"string"}},"required":["code"]}),
            "search_assets" => serde_json::json!({"type":"object","properties":{"source":{"type":"string"},"query":{"type":"string"},"asset_type":{"type":"string"}},"required":["source","query"]}),
            "import_asset" => serde_json::json!({"type":"object","properties":{"source":{"type":"string"},"asset_id":{"type":"string"}},"required":["source","asset_id"]}),
            "generate_3d" => serde_json::json!({"type":"object","properties":{"prompt":{"type":"string"},"provider":{"type":"string"}}}),
            _ => serde_json::json!({"type":"object","properties":{}}),
        }}))
        .collect();
    let mut scene: Vec<String> = vec!["Camera".into(), "Light".into()];
    let log = std::env::var("BREWIX_MOCK_BLENDER_LOG").ok();
    let obj = regex::Regex::new(r#"\.name\s*=\s*["']([^"']+)["']"#).unwrap();
    let export = regex::Regex::new(r#"export_scene\.(fbx|obj|gltf)\([^)]*filepath\s*=\s*r?["']([^"']+)["']"#).unwrap();
    let mut out = std::io::stdout();
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        let Ok(msg) = serde_json::from_str::<serde_json::Value>(&line) else { continue };
        let Some(id) = msg.get("id").cloned() else { continue };
        let result = match msg.get("method").and_then(|m| m.as_str()).unwrap_or("") {
            "initialize" => serde_json::json!({"protocolVersion": msg.pointer("/params/protocolVersion").cloned().unwrap_or(serde_json::json!("2025-06-18")), "capabilities": {"tools": {}}, "serverInfo": {"name": "mock-blender", "version": "1.0.0"}}),
            "tools/list" => serde_json::json!({ "tools": tools }),
            "tools/call" => {
                let name = msg.pointer("/params/name").and_then(|n| n.as_str()).unwrap_or("");
                let args = msg.pointer("/params/arguments").cloned().unwrap_or_default();
                if let Some(p) = &log {
                    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
                        let _ = writeln!(f, "{}", serde_json::json!({"tool": name, "arguments": args}));
                    }
                }
                let text = match name {
                    "get_addon_status" => "[mock Blender] add-on connected, Blender 4.2".to_string(),
                    "get_scene_info" => format!("[mock Blender] objects: {}", scene.join(", ")),
                    "execute_blender_code" => {
                        let code = args.get("code").and_then(|c| c.as_str()).unwrap_or("");
                        for m in obj.captures_iter(code) {
                            scene.push(m[1].to_string());
                        }
                        let mut t = format!("[mock Blender] ran {} line(s). Objects: {}.", code.lines().count(), scene.join(", "));
                        // Exports written with path variables: join the folder
                        // literal ending in assets/models with each file-name
                        // literal in the code. Test mode cannot run Python, so
                        // print() output is not available.
                        if code.contains("export_scene.") && !export.is_match(code) {
                            let dir_lit = regex::Regex::new(r#"["'](/[^"']*?assets/models)/?["']"#).unwrap();
                            let file_lit = regex::Regex::new(r#"["']/?([A-Za-z0-9_.-]+\.(fbx|obj))["']"#).unwrap();
                            if let Some(d) = dir_lit.captures(code) {
                                for f in file_lit.captures_iter(code) {
                                    let p = format!("{}/{}", &d[1], &f[1]);
                                    let _ = std::fs::create_dir_all(&d[1]);
                                    let _ = std::fs::write(&p, format!("mock {} export of {}\n", &f[2], scene.join(", ")));
                                    t.push_str(&format!(" Exported {} to {p}.", &f[2]));
                                }
                            }
                        }
                        t.push_str(" (Test mode: print() output is not shown.)");
                        for m in export.captures_iter(code) {
                            let path = std::path::Path::new(&m[2]);
                            if let Some(d) = path.parent() {
                                let _ = std::fs::create_dir_all(d);
                            }
                            let _ = std::fs::write(path, format!("mock {} export of {}\n", &m[1], scene.join(", ")));
                            t.push_str(&format!(" Exported {} to {}.", &m[1], &m[2]));
                        }
                        t
                    }
                    "look" | "get_viewport_screenshot" => "[mock Blender] viewport: the scene from the front".into(),
                    "search_assets" => "[mock Blender] polyhaven results: wooden_crate_01 (CC0), street_lamp_02 (CC0)".into(),
                    "import_asset" => {
                        let a = args.get("asset_id").and_then(|c| c.as_str()).unwrap_or("asset").to_string();
                        scene.push(a.clone());
                        format!("[mock Blender] imported {a}")
                    }
                    other => format!("[mock Blender] {other} would call a paid service"),
                };
                serde_json::json!({"content": [{"type": "text", "text": text}], "isError": false})
            }
            "ping" => serde_json::json!({}),
            _ => {
                let _ = writeln!(out, "{}", serde_json::json!({"jsonrpc": "2.0", "id": id, "error": {"code": -32601, "message": "method not found"}}));
                let _ = out.flush();
                continue;
            }
        };
        let _ = writeln!(out, "{}", serde_json::json!({"jsonrpc": "2.0", "id": id, "result": result}));
        let _ = out.flush();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn guard_keeps_blender_free_and_offline() {
        let t = |s: &str| format!("mcp__Blender__{s}");
        assert!(guard(&t("generate_3d"), &json!({"prompt": "lamp"})).is_err());
        assert!(guard(&t("record_trajectory_feedback"), &json!({})).is_err());
        assert!(guard(&t("search_assets"), &json!({"source": "sketchfab", "query": "lamp"})).is_err());
        assert!(guard(&t("import_asset"), &json!({"source": "polypizza", "asset_id": "x"})).is_err());
        assert!(guard(&t("search_assets"), &json!({"source": "polyhaven", "query": "lamp"})).is_ok());
        for bad in ["import requests", "import subprocess", "shutil.rmtree('/')", "bpy.ops.preferences.addon_install()", "exec(s)"] {
            assert!(guard(&t("execute_blender_code"), &json!({ "code": bad })).is_err(), "{bad}");
        }
        assert!(guard(&t("execute_blender_code"), &json!({"code": "import bpy\nbpy.ops.mesh.primitive_cube_add()\nbpy.ops.export_scene.fbx(filepath='/p/x.fbx')"})).is_ok());
    }

    #[test]
    fn config_turns_telemetry_off() {
        let e = config_entry(&("uvx".into(), vec![PACKAGE.into()]));
        for k in TELEMETRY_OFF {
            assert_eq!(e["env"][k], "1");
        }
        assert_eq!(e["args"][0], "mcp-for-blender==2.2.4");
    }
}
