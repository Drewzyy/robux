from common import note
S = "80-assets"
R = "Research report (Nov 9 2026) C8"

note(f"{S}/pipeline-overview", "Asset pipelines at a glance: Blender to Studio, Studio capture to thumbnail, audio to SoundService, VFX and SFX, and where each lands in the Rojo project.", "reference", "draft",
 R, "Overview; each step is detailed and sourced in its own note.", r"""
# Pipeline overview

| Asset | Path | Note |
|---|---|---|
| 3D models | primitives in Studio, or Blender (optionally driven through Blender MCP) to FBX/OBJ, import to Studio | [[80-assets/3d-model-checklist]] |
| Creator Store models | search and insert through the Studio MCP server (Andrew's Studio must be open), license recorded | [[80-assets/asset-sources-and-licenses]] |
| Thumbnails and icon | Studio Free Camera capture, then layout | [[80-assets/thumbnail-pipeline]] |
| Audio | copyright-free, licensed, moderated on upload | [[80-assets/audio-pipeline]] |
| VFX and SFX | small feedback on every action, within particle caps | [[80-assets/vfx-sfx-library]] |
| Lighting | Lighting and Atmosphere values in a Rojo config module | [[80-assets/lighting-and-mood]] |

## Two lanes (god plan model, as summarized in the report)
- **Lane A, headless:** Rojo, Lune, Open Cloud. Code and data live in the repo. Lasting changes go here.
- **Lane B, live:** Studio MCP server in Andrew's open Studio. Scene reads, inserts, playtests, screenshots. MCP edits to code are not persistent; copy lasting changes back through Lane A.
A cloud session cannot touch Studio or Blender; the repo is the handoff.
""")

note(f"{S}/3d-model-checklist", "3D models: fewer larger parts, shared MeshIds, union triangle checks, anchoring, LevelOfDetail, texture sizes (256 minor, 512 standard).", "checklist", "draft",
 R + "; c-s-labs luckbound toolchain; 3daistudio free MCP servers", "Performance guidance from community references; verify each model in Studio Scene Analysis (triangles) and MicroProfiler.", r"""
# 3D model checklist

- [ ] Fewer, larger parts.
- [ ] MeshParts share a MeshId where possible (instanced, cheap).
- [ ] Unions checked for triangle count in Scene Analysis (they can be expensive).
- [ ] Static parts anchored.
- [ ] `Model.LevelOfDetail` set for static distant models (reported value name `SLIM`: UNVERIFIED, check the enum).
- [ ] Textures: 256x256 for minor objects, 512x512 standard. Memory scales with the square: 1024 is 4x a 512.
- [ ] Collision simplified where players never touch it.
- [ ] License recorded ([[80-assets/asset-sources-and-licenses]]); no Toolbox free models.
- [ ] Within [[20-luau-rojo/performance-budgets]].
""")

note(f"{S}/thumbnail-pipeline", "Thumbnail workflow: Studio Free Camera (Shift+P) base shots, enhance, export 1920x1080 under 3 MB; icon is a separate 512x512 crop.", "playbook", "draft",
 R + "; views.terabox.com thumbnail creator guide", "Capture steps from secondary guides; confirm the Free Camera shortcut in Studio. Sizes per 30-ui-ux/thumbnail-icon-spec.", r"""
# Thumbnail pipeline

1. Splash writes the shot list: one promise per image, shown in the first 10 seconds of play.
2. Capture in Studio with Free Camera (Shift+P) for high-fidelity base shots. Needs Andrew's Studio (or the Studio MCP screenshot tool).
3. Enhance and lay out: avatar visible, one focal point, text under 4 words.
4. Export 1920x1080 .png under 3 MB.
5. Icon: a separate 512x512 crop, readable at 150x150.
6. Upload is Andrew's action. Every image is moderated.

Spec: [[30-ui-ux/thumbnail-icon-spec]].
""")

note(f"{S}/audio-pipeline", "Audio: copyright-free only, license recorded, Roblox moderation on upload; silence beats a bad loop; gentle sounds over jarring ones.", "playbook", "draft",
 R + " (C8 audio; Pawmodoro rules as cited)", "Rules from the report. License for each track is checked when it is chosen (Creator Store audio license page).", r"""
# Audio pipeline

- 100% copyright-free audio only. Record the source and license for every track.
- No copyrighted music. Check Creator Store audio licences.
- Uploads are moderated.
- Silence is better than a bad loop.
- Fit the mood: soft sounds at transitions; loud or jarring sounds break the mood.
- Every core action gets a short sound ([[80-assets/vfx-sfx-library]]).
""")

note(f"{S}/vfx-sfx-library", "Juice on every action: checkpoint light and sound, coin spin and chime, finish fanfare; within particle and Highlight caps.", "reference", "draft",
 R + " (C8 VFX/SFX)", "Design guidance; caps are community figures (UNVERIFIED limits), measure on a phone.", r"""
# VFX and SFX library

Small feedback, constantly, is what makes a game feel alive.

| Event | Visual | Sound |
|---|---|---|
| Checkpoint touched | checkpoint lights up | short rising tone |
| Coin collected | coin spins out | chime |
| Finish | burst | fanfare |
| Level up / rare drop | the session's peak moment | bigger sting |

## Caps (community figures, measure on a phone)
- ParticleEmitter rate about 100/s mobile, 400 desktop (one reference); about 50 active emitters total (another).
- `Highlight` instances: 255 max total (one reference).
- Limit beams and trails.
""")

note(f"{S}/lighting-and-mood", "Scenic's note: lighting goals (readability first, then mood) and where values live. Specific property values are UNVERIFIED until tested in Studio.", "reference", "unverified",
 "Research report has no lighting section; goals derived from readability rules in C1 and performance rules in C2",
 "UNVERIFIED: the research does not cover lighting. Property names must be checked against the Lighting and Atmosphere Engine API pages before use, and values tested in Studio on a phone.", r"""
# Lighting and mood

The research behind this vault has no lighting section. This note holds goals only.

## Goals (from verified rules elsewhere)
1. **Readability first.** Hazards and the next goal stand out from the background in every zone ([[10-game-design/genre-playbooks/obby]]).
2. **Phone performance.** Limit shadow-casting lights ([[20-luau-rojo/performance-budgets]]).
3. **Mood second.** Match the design doc's mood words.

## Where values live
A Rojo module (example `src/shared/Configs/Lighting.luau`) applied by the server at start, so the look is in the repo, not only in Studio.

## Before writing values
UNVERIFIED: which properties exist on `Lighting` and `Atmosphere` and their ranges. Read the Engine API pages, then test in Studio on a phone. Record what worked here, after a successful run.
""")

note(f"{S}/tool-setup", "Setup for Studio MCP (built into Studio), Blender MCP, and the headless toolchain; what each is for and what needs Andrew.", "playbook", "draft",
 R + "; c-s-labs luckbound; vibestackweekly; agentgid; quanticsoul4772 mcp-roblox README",
 "Studio MCP command and menu path are from secondary sources: UNVERIFIED until Andrew connects it on his Mac. Headless tools verified in 20-luau-rojo/toolchain.", r"""
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
`claude mcp add blender -- uvx blender-mcp`, then enable "Interface: Blender MCP" in Blender (reported). Pulls from Poly Haven, Sketchfab, Poly Pizza.

## Headless
See [[20-luau-rojo/toolchain]]. Script Sync (built into Studio) syncs scripts only and is a fallback if Rojo breaks.
""")

note(f"{S}/free-tool-table", "Every free tool in the studio and its cost (zero): Studio, Rojo, Lune, Open Cloud, Studio MCP, Assistant, Blender, Wally and friends, GitHub Actions, trackers.", "reference", "draft",
 R + " (C8 free tool table)", "Costs are $0 per the report; GitHub Actions free tier limits are UNVERIFIED. Rojo 7.7.1 and Lune 0.11.1 versions verified locally (see toolchain).", r"""
# Free tool table

| Tool | What it does | Cost |
|---|---|---|
| Roblox Studio | build, playtest, publish manually, screenshots, Free Camera | $0 |
| Rojo 7.7.1 | file-based builds, live sync | $0 |
| Lune 0.11.1 | headless Luau tests and scripts | $0 |
| Open Cloud API | publishing, gamepasses, assets, DataStores, Luau Execution | $0 |
| Studio MCP server (built in) | agent access to an open Studio place | $0 |
| Roblox Assistant and Code Assist | built-in assistant, autocomplete, asset generation | $0 |
| Blender and Blender MCP | 3D modelling | $0 |
| Wally, StyLua, Selene, luau-lsp, Rokit | packages, format, lint, types, version pinning | $0 |
| Git and GitHub Actions (free tier) | version control, CI | $0 |
| Rolimons, RoVitals, BLOXG | competitor tracking | $0 |
| Creator Dashboard Analytics | own data | $0 |

Not free: Claude usage to run the agents (see the app's cost estimate).
""")

note(f"{S}/asset-sources-and-licenses", "Where assets may come from (primitives, team-made, Creator Store, built-in generators, Blender, paid marketplaces) and the license check before publishing.", "checklist", "draft",
 R + " (C8)", "Each asset's license is checked on its source page when chosen; Librarian's gate requires a license line per asset.", r"""
# Asset sources and licenses

| Source | Notes |
|---|---|
| Primitives and code | safest; no license needed |
| Team-made (Blender) | record the file |
| Roblox built-in generators (mesh, material) | good for materials and simple props |
| Creator Store | fastest prototype; insert via Studio MCP; check license before publishing |
| Paid marketplaces (Sketchfab, Kitbash3D, Quixel) | studio build budget is $0: not used without Andrew's approval; license must allow Roblox publishing |
| Toolbox free models | **never** ([[80-assets/no-toolbox-rule]]) |

## For every asset record
Need, choice, source, asset ID or file, license, who approved.
""")

note(f"{S}/no-toolbox-rule", "No Toolbox free models, ever: they can hide malicious scripts. Build from primitives, team assets, or licensed Creator Store items.", "reference", "draft",
 R + " (C6, C8)", "Studio rule; checked by Librarian's gate and the pre-launch policy checklist.", r"""
# No Toolbox rule

No Toolbox free models. They can contain hidden malicious scripts (backdoors, require-by-ID loaders).

Instead: primitives and code, team-made assets, or licensed Creator Store items with the license recorded ([[80-assets/asset-sources-and-licenses]]).
""")
print("80 done")
