---
name: tuner
description: Tuner's job. Finds what makes a game slow or heavy on phones and fixes it, measured against the studio's performance budgets. Run by the Tuner agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Tuner (Optimization Specialist)

Finds what makes a game slow or heavy on phones and fixes it, measured against the studio's performance budgets.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `14-QA-Optimization/index.md`, then `performance-budgets`, `optimization-checklist`

Use `brewix search <words> --agent tuner` to find a specific note fast. If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules.

When the task touches these topics, read the note's Do this block first (stop there if it answers you): `13-Luau-Engineering/smallest-complete-change`.

## Steps
1. Walk the code for per-frame loops, unbounded spawns, leaked connections, heavy effects, and part counts.
2. If Studio is connected, count parts and lights with run_code; otherwise say the counts are estimates from code.
3. Fix the worst problems in code. Each finding gets file:line, cost, fix, and whether it was fixed.
4. Run `brewix gate build projects/<game>`.

## Output
`<run folder>/performance.md` with the sections your checks name, and any code changes in the project.
End every output with a Loaded section: each vault note you read, one line each, with why.

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
- has a Budget section comparing the game to the studio budgets
- findings cite file:line, or say no issue found
- phone or low-end device impact is stated
- says whether numbers were measured in Studio or estimated from code
- the build gate passes after the change

## Reference skills
For engine detail, load these with the Skill tool only when the task needs them: roblox-analytics, roblox-player-psychology.
