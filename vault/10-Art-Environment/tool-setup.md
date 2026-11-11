---
description: Setup for Studio MCP (built into Studio), Blender MCP, and the headless toolchain; what each is for and what needs Andrew.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C8; c-s-labs luckbound; vibestackweekly; agentgid; quanticsoul4772 mcp-roblox README
verify: Studio MCP command, menu path, and tools checked against Roblox/creator-docs on 2026-11-11; untested on a real Studio. Headless tools verified in 13-Luau-Engineering/toolchain.
tags: [art-environment, playbook]
---
# Tool setup

## Roblox Studio MCP server (built into Studio)
From Roblox's docs (Roblox/creator-docs repo, content/en-us/studio/mcp.md, read 2026-11-11):
- Turn it on in Studio: Assistant, the ... menu, Manage MCP Servers, Enable Studio as MCP server. A green dot shows connected clients.
- BrewixGames connects to `/Applications/RobloxStudio.app/Contents/MacOS/StudioMCP` on Mac and `cmd.exe /c %LOCALAPPDATA%\Roblox\mcp.bat` on Windows (Settings, Roblox Studio).
- Tools: list_roblox_studios (each call needs its studio_id), get_studio_state, search_game_tree, inspect_instance, script_read, script_search, script_grep, multi_edit, execute_luau, start_stop_play, get_console_output, screen_capture, character_navigation, user_keyboard_input, user_mouse_input, search_asset, insert_asset, generate_procedural_model, http_get (Roblox docs only), skill.
- BrewixGames blocks upload_image, generate_mesh, generate_material, and subagent, and checks code run by execute_luau for publishing, purchases, web calls, and DataStore writes.
- The older standalone server (Roblox/studio-rust-mcp-server) is no longer developed; BrewixGames still supports it.

## Blender MCP (free)
BrewixGames runs it for you: Settings, Blender, choose Blender. The package was renamed from `blender-mcp` to `mcp-for-blender`; BrewixGames pins `mcp-for-blender==2.2.4` and runs it with `uvx` (free, from astral.sh/uv) with telemetry switched off. Get the Blender Add-on (Settings) saves the add-on file; install it in Blender with Edit, Preferences, Add-ons, Install from Disk, then click Connect.
Allowed: scene info, Python in Blender (no network, no other programs, no deleting files), viewport look, Poly Haven search and import (CC0). Blocked: the paid 3D generators (Tripo, Hunyuan3D, Hyper3D, Meshy), Sketchfab and Poly Pizza (licenses vary), and the feedback upload tool. Only the Modeler agent and Ask Claude get Blender.

## Headless
See [[13-Luau-Engineering/toolchain]]. Script Sync (built into Studio) syncs scripts only and is a fallback if Rojo breaks.
