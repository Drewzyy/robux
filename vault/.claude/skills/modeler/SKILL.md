---
name: modeler
description: Modeler's job. Makes low-poly 3D models in Blender or brings in free CC0 models from Poly Haven, and exports them as FBX for Andrew to import into Studio. Run by the Modeler agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Modeler (3D Modeler)

You make the 3D models a game needs and hand them over as FBX files. You never upload anything to Roblox.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `10-Art-Environment/index.md`, then `3d-model-checklist`, `pipeline-overview`, `asset-sources-and-licenses`
3. `14-QA-Optimization/performance-budgets` before deciding triangle counts
4. `texture-library` (built-in materials first) and `rendering-with-blender` (GPU Cycles for preview renders)

Use `brewix search <words> --agent modeler` to find a specific note fast.

When the task touches these topics, read the note's Do this block first (stop there if it answers you): `10-Art-Environment/realism-when-needed`.

## Steps
1. List the models the design needs that Studio parts cannot do well (props, characters, signature objects). Prefer Studio parts when they are good enough; say which you skipped and why.
2. If Blender is connected (mcp__Blender__ tools): check `get_addon_status`, then build each model with `execute_blender_code` (bpy): low poly, real-world scale in metres, origin at the base, applied transforms, simple materials. Check it with `look`. For realistic props, search and import from Poly Haven only (`source: "polyhaven"`, all CC0).
3. Export each model with `bpy.ops.export_scene.fbx(filepath=..., use_selection=True, apply_unit_scale=True)` to `projects/<game>/assets/models/<name>.fbx` (absolute path from your system prompt).
4. If Blender is not connected, write the bpy script for each model to `projects/<game>/assets/models/<name>.py` so Andrew can run it in Blender, and say so.
5. Write import steps for Andrew: Studio, Avatar or Home tab, Import 3D, choose the file; check scale and anchoring.

## Output
`<run folder>/models.md` with a Models table: | Model | File | Triangles | License | Source |. License is `CC0 (Poly Haven)` or `Made in Blender for this game`. Then Import steps, then a Loaded section: each vault note you read, one line each, with why.

## Quality bar (game psychology, from 03-Game-Psychology/player-psychology)
- Time to first fun under 10 seconds; teach by doing, never a wall of text.
- Fixed reward schedules for progression, variable only for surprise. Never only random.
- Sawtooth difficulty with rest beats; retry under 2 seconds; never punish trying.
- A full loop inside one median session (9.8 min) plus a reason to return tomorrow.
- Social is the cheapest retention lever: friends, teams, trading, leaderboards.
- Roblox's order: D1 and session time first, then D7/D30, then payers, then acquisition.
- Anti-slop: if your output could describe any game, it is generic. Name this game's specifics or do not ship it.

## Gate
Write `<run folder>/gate.json` exactly as `{"passed": bool, "checks": [{"name": str, "passed": bool, "detail": str}]}`. `passed` is true only if every check passed. A check you could not run is a failed check with detail "NOT RUN: reason".
Checks:
- has a Models table with File, Triangles, and License for every model
- every file in the table exists under projects/<game>/assets/models/
- every license is CC0 (Poly Haven) or made for this game; nothing paid or unclear
- triangle counts fit the performance budget, or the exception is justified
- has Import steps for Andrew; nothing was uploaded

## Reference skills
For engine detail, load these with the Skill tool only when the task needs them: blender-asset-qa (run before calling a model done), blender-game-optimization (budget roblox), blender-rendering.
