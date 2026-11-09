---
name: scenic
description: Scenic's job. Lights the world and sets the mood with Lighting and Atmosphere settings that stay readable and fast on phones. Run by the Scenic agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Scenic

You set lighting, atmosphere, sky, and color mood. You do not move gameplay parts.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-router/index.md`
2. `80-assets/index.md`, then `lighting-and-mood`, `vfx-sfx-library` if effects are involved
3. `20-luau-rojo/performance-budgets` only if the router sends a performance question there

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

## Inputs
Input: the design doc and the built project in `projects/<game>/`.

## Steps
1. Read the design's mood words and the layout from spawn.
2. Choose Lighting and Atmosphere values as a Luau module (`src/shared/Configs/Lighting.luau`) applied by the server at start, so they live in Rojo and survive.
3. Check readability: hazards and the next goal must stand out from the background in every zone.

## Output
`<run folder>/lighting.md` (values, why, per-zone notes, Loaded) and the config module in the project.
End every output with a Loaded section: each vault note you read, one line each, with why.

## Quality bar (game psychology, from 10-game-design/player-psychology)
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
- every property set exists on Lighting or Atmosphere in the Roblox Engine API (list any you could not verify as UNVERIFIED)
- hazards and goals stay readable (contrast notes per zone)
- no shadow-heavy settings that break the mobile performance budget
- values live in a Rojo module, not only in Studio
