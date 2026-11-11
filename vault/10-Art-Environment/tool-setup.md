---
description: Setup for Studio MCP (built into Studio), Blender MCP, and the headless toolchain; what each is for and what needs Andrew.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C8; c-s-labs luckbound; vibestackweekly; agentgid; quanticsoul4772 mcp-roblox README
verify: Studio MCP command and menu path are from secondary sources: UNVERIFIED until Andrew connects it on his Mac. Headless tools verified in 13-Luau-Engineering/toolchain.
tags: [art-environment, playbook]
---
# Tool setup

## Roblox Studio MCP server (built into Studio)
- Studio: Assistant, Manage MCP Servers, turn on "Enable Studio as MCP server", Quick connect for Claude Code.
- Command-line form on Mac (UNVERIFIED path): `claude mcp add --scope user --transport stdio roblox-studio -- /Applications/RobloxStudio.app/Contents/MacOS/StudioMCP`
- Config lives in `~/.claude.json` (user) or `.mcp.json` (project).
- Tools (25+ in 8 categories, reported): scripts, instance tree, Luau execution, Creator Store search and insert, mesh generation, playtests, simulated input, console, viewport screenshots.
- Needs Studio open with the place loaded; local only.
- Roblox warns MCP clients can read and modify open places. Only connect trusted clients; work on a copy or rely on version history.
- MCP edits to code are not persistent. Lasting changes go through Rojo.
- The older `Roblox/studio-rust-mcp-server` repo is archived (April 2026); use the built-in server.
- Studio agent runs launched by this app use `--strict-mcp-config`, so they get no MCP servers unless the app passes one. Wiring Studio MCP into Playtest is Phase 4 work and needs Andrew's Studio.

## Blender MCP (free)
BrewixGames runs it for you: Settings, Blender, choose Blender. The package was renamed from `blender-mcp` to `mcp-for-blender`; BrewixGames pins `mcp-for-blender==2.2.4` and runs it with `uvx` (free, from astral.sh/uv) with telemetry switched off. Get the Blender Add-on (Settings) saves the add-on file; install it in Blender with Edit, Preferences, Add-ons, Install from Disk, then click Connect.
Allowed: scene info, Python in Blender (no network, no other programs, no deleting files), viewport look, Poly Haven search and import (CC0). Blocked: the paid 3D generators (Tripo, Hunyuan3D, Hyper3D, Meshy), Sketchfab and Poly Pizza (licenses vary), and the feedback upload tool. Only the Modeler agent and Ask Claude get Blender.

## Headless
See [[13-Luau-Engineering/toolchain]]. Script Sync (built into Studio) syncs scripts only and is a fallback if Rojo breaks.
