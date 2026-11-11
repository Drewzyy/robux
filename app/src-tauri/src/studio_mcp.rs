//! Roblox Studio connection through Roblox's own Studio MCP server, which
//! runs on this computer next to an open Studio and needs no Roblox login,
//! cookie, or key. Off by default. Only the agents that build things get it,
//! and every Studio call goes through `guard` first (a PreToolUse hook), which
//! blocks publishing, uploading, selling, web requests, and live data writes.
//!
//! `brewix mock-studio-mcp` is a stand-in server with the same tools, for
//! tests and for trying the connection without Studio.

use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

/// The MCP server name; tools appear to agents as mcp__Roblox_Studio__<tool>.
pub const SERVER: &str = "Roblox_Studio";
/// Agents that may use Studio when the connection is on.
pub const AGENTS: &[&str] = &["builder", "scenic", "frame", "playtest", "motion", "crowd", "tuner", "assistant"];
/// Tools agents may call. The first group is Studio's built-in MCP server
/// (Assistant, Manage MCP Servers); the second is the older standalone
/// server (studio-rust-mcp-server), kept for people who still run it.
pub const TOOLS: &[&str] = &[
    "list_roblox_studios", "get_studio_state", "search_game_tree", "inspect_instance",
    "script_read", "script_search", "script_grep", "multi_edit", "execute_luau",
    "start_stop_play", "get_console_output", "screen_capture",
    "character_navigation", "user_keyboard_input", "user_mouse_input",
    "search_asset", "insert_asset", "store_image", "generate_procedural_model", "wait_job_finished",
    "http_get", "skill",
    "run_code", "insert_model", "run_script_in_play_mode", "get_studio_mode",
];
/// Tools that upload to or create assets on Andrew's account, or hand work
/// to Studio's own assistant outside our guard.
pub const BLOCKED: &[&str] = &["upload_image", "generate_mesh", "generate_material", "subagent"];
/// Tools that run code right now in Studio (checked against DENY).
const RUNS_CODE: &[&str] = &["execute_luau", "run_code", "run_script_in_play_mode"];

/// Where the server lives: Studio's built-in one first (Roblox docs,
/// studio/mcp), then the older standalone app.
pub fn default_path() -> Option<PathBuf> {
    let c: Vec<PathBuf> = if cfg!(target_os = "macos") {
        let mut v = vec![PathBuf::from("/Applications/RobloxStudio.app/Contents/MacOS/StudioMCP")];
        if let Some(h) = crate::paths::home() {
            v.push(h.join("Applications/RobloxStudio.app/Contents/MacOS/StudioMCP"));
        }
        v.push(PathBuf::from("/Applications/RobloxStudioMCP.app/Contents/MacOS/rbx-studio-mcp"));
        v
    } else if cfg!(windows) {
        std::env::var("LOCALAPPDATA").map(|d| vec![Path::new(&d).join("Roblox").join("mcp.bat")]).unwrap_or_default()
    } else {
        vec![]
    };
    c.into_iter().find(|p| p.is_file())
}

/// Command and arguments for the server, or None when the connection is off.
pub fn server_command(s: &crate::settings::Settings) -> Option<(String, Vec<String>)> {
    match s.studio_mcp.as_str() {
        "mock" => Some((std::env::current_exe().ok()?.display().to_string(), vec!["mock-studio-mcp".into()])),
        "studio" => {
            let p = if s.studio_mcp_path.trim().is_empty() { default_path()? } else { PathBuf::from(s.studio_mcp_path.trim()) };
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
            if name.ends_with(".bat") || name.ends_with(".cmd") {
                // Roblox docs: cmd.exe /c %LOCALAPPDATA%\Roblox\mcp.bat
                Some(("cmd.exe".into(), vec!["/c".into(), p.display().to_string()]))
            } else if name.starts_with("rbx-studio-mcp") {
                Some((p.display().to_string(), vec!["--stdio".into()]))
            } else {
                Some((p.display().to_string(), vec![]))
            }
        }
        _ => None,
    }
}

/// Things Studio code must never do from an agent run.
const DENY: &[(&str, &str)] = &[
    (r"(?i)\bpublish", "publishing"),
    (r"(?i)saveplace|createplace|savetoroblox|uploadasset|createasset", "saving or uploading to Roblox"),
    (r"(?i)assetservice", "AssetService (creates or uploads assets)"),
    (r"(?i)marketplaceservice", "MarketplaceService (purchases and products)"),
    (r"(?i)httpservice|request\s*\(|opencloud|\bhttps?://", "web requests"),
    (r"(?i)(setasync|updateasync|removeasync|incrementasync)", "writing to DataStores (live player data)"),
    (r"(?i)roblosecurity|cookie|getsecret|api[_ ]?key", "credentials"),
    (r"(?i)\bloadstring\b|getfenv|setfenv", "running hidden code"),
];

/// Ok when a Studio tool call is safe to run; Err with the reason otherwise.
pub fn guard(tool: &str, tool_input: &serde_json::Value) -> Result<(), String> {
    let short = tool.rsplit("__").next().unwrap_or(tool);
    if BLOCKED.contains(&short) {
        return Err(format!("Blocked by BrewixGames: the Studio tool {short} uploads or creates assets on Andrew's account, or works outside BrewixGames' checks. Build it another way or list it under Open questions."));
    }
    if !RUNS_CODE.contains(&short) {
        // Writing scripts into the game (multi_edit) may use purchases and
        // saving: that code only runs when Andrew plays. Reading, exploring,
        // and playtesting are safe.
        return Ok(());
    }
    fn strings(v: &serde_json::Value, out: &mut Vec<String>) {
        match v {
            serde_json::Value::String(s) => out.push(s.clone()),
            serde_json::Value::Array(a) => a.iter().for_each(|x| strings(x, out)),
            serde_json::Value::Object(o) => o.values().for_each(|x| strings(x, out)),
            _ => {}
        }
    }
    let mut all = vec![];
    strings(tool_input, &mut all);
    let text = all.join("\n");
    for (re, what) in deny_compiled_deny() {
        if re.is_match(&text) {
            return Err(format!(
                "Blocked by BrewixGames: Studio code from an agent may not touch {what}. Nothing is published, uploaded, sold, or sent online without Andrew. Do this without it, or list it under Open questions."
            ));
        }
    }
    Ok(())
}

/// `brewix studio-guard`: PreToolUse hook. Reads the hook JSON on stdin and
/// exits 2 (block, reason on stderr) when guard refuses.
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

/// Stand-in Studio MCP server on stdio (newline-delimited JSON-RPC). It keeps
/// a pretend workspace so inserted models show up in later code runs, and
/// echoes print() output. Calls are logged to $BREWIX_MOCK_STUDIO_LOG if set.
pub fn mock_server() -> i32 {
    let tools = serde_json::json!([
        {"name": "run_code", "description": "Runs a command in Roblox Studio and returns the printed output.",
         "inputSchema": {"type": "object", "properties": {"command": {"type": "string", "description": "Luau code to run"}}, "required": ["command"]}},
        {"name": "insert_model", "description": "Inserts a model from the Roblox marketplace into the workspace.",
         "inputSchema": {"type": "object", "properties": {"query": {"type": "string", "description": "What to search for"}}, "required": ["query"]}},
        {"name": "get_studio_mode", "description": "Tells whether Studio is in edit or play mode.",
         "inputSchema": {"type": "object", "properties": {}}}
    ]);
    let mut workspace: Vec<String> = vec!["Baseplate".into(), "SpawnLocation".into()];
    let log = std::env::var("BREWIX_MOCK_STUDIO_LOG").ok();
    let prints = regex::Regex::new(r#"print\(\s*["']([^"']*)["']"#).unwrap();
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    for line in stdin.lock().lines().map_while(Result::ok) {
        let Ok(msg) = serde_json::from_str::<serde_json::Value>(&line) else { continue };
        let Some(id) = msg.get("id").cloned() else { continue }; // notifications need no reply
        let method = msg.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let result = match method {
            "initialize" => serde_json::json!({
                "protocolVersion": msg.pointer("/params/protocolVersion").cloned().unwrap_or(serde_json::json!("2025-06-18")),
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "mock-roblox-studio", "version": "1.0.0"}
            }),
            "tools/list" => serde_json::json!({ "tools": tools }),
            "tools/call" => {
                let name = msg.pointer("/params/name").and_then(|n| n.as_str()).unwrap_or("");
                let args = msg.pointer("/params/arguments").cloned().unwrap_or_default();
                if let Some(path) = &log {
                    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
                        let _ = writeln!(f, "{}", serde_json::json!({"tool": name, "arguments": args}));
                    }
                }
                let text = match name {
                    "run_code" => {
                        let code = args.get("command").and_then(|c| c.as_str()).unwrap_or("");
                        let printed: Vec<String> = prints.captures_iter(code).map(|m| m[1].to_string()).collect();
                        let mut t = format!("[mock Studio] ran {} line(s). Workspace children: {}.", code.lines().count(), workspace.join(", "));
                        if !printed.is_empty() {
                            t.push_str(&format!("\nOutput:\n{}", printed.join("\n")));
                        }
                        t
                    }
                    "insert_model" => {
                        let q = args.get("query").and_then(|c| c.as_str()).unwrap_or("Model");
                        let n = format!("{}Model", q.split_whitespace().map(|w| { let mut c = w.chars(); c.next().map(|f| f.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() }).collect::<String>());
                        workspace.push(n.clone());
                        format!("[mock Studio] inserted {n} into Workspace.")
                    }
                    "get_studio_mode" => "[mock Studio] edit mode".into(),
                    other => format!("[mock Studio] unknown tool {other}"),
                };
                serde_json::json!({ "content": [{"type": "text", "text": text}], "isError": false })
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
    fn guard_blocks_publishing_and_web_and_live_data() {
        for bad in [
            "game:GetService('AssetService'):CreatePlaceAsync('x')",
            "plugin:SaveToRoblox()",
            "game:GetService('HttpService'):GetAsync('https://x.com')",
            "store:SetAsync('k', 1)",
            "MarketplaceService:PromptPurchase(p, 1)",
            "loadstring(src)()",
        ] {
            assert!(guard("mcp__Roblox_Studio__run_code", &json!({ "command": bad })).is_err(), "{bad}");
            assert!(guard("mcp__Roblox_Studio__execute_luau", &json!({ "code": bad, "datamodel_type": "Edit" })).is_err(), "{bad}");
        }
        assert!(guard("mcp__Roblox_Studio__execute_luau", &json!({ "code": "local p = Instance.new('Part'); p.Parent = workspace; print('ok')" })).is_ok());
        assert!(guard("mcp__Roblox_Studio__insert_model", &json!({ "query": "street lamp" })).is_ok());
        // game scripts written into the place may use purchases and saving
        assert!(guard("mcp__Roblox_Studio__multi_edit", &json!({ "edits": "MarketplaceService.ProcessReceipt = ... store:UpdateAsync(k, f)" })).is_ok());
        for b in ["upload_image", "generate_mesh", "generate_material", "subagent"] {
            assert!(guard(&format!("mcp__Roblox_Studio__{b}"), &json!({})).is_err(), "{b}");
        }
    }

    #[test]
    fn hook_only_judges_studio_tools() {
        assert_eq!(guard_hook(r#"{"tool_name":"Read","tool_input":{"file_path":"publish.md"}}"#), 0);
        assert_eq!(guard_hook(r#"{"tool_name":"mcp__Roblox_Studio__run_code","tool_input":{"command":"game:Publish()"}}"#), 2);
        assert_eq!(guard_hook(r#"{"tool_name":"mcp__Roblox_Studio__run_code","tool_input":{"command":"print('hi')"}}"#), 0);
    }
}

/// DENY, compiled once.
fn deny_compiled_deny() -> &'static [(regex::Regex, &'static str)] {
    static C: std::sync::OnceLock<Vec<(regex::Regex, &'static str)>> = std::sync::OnceLock::new();
    C.get_or_init(|| DENY.iter().map(|(p, w)| (regex::Regex::new(p).unwrap(), *w)).collect())
}
