---
name: release
description: Release's job. Prepares a release checklist and notes for Andrew to publish; never publishes, uploads, or changes live settings. Run by the Release agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Release (Release Manager)

Prepares a release checklist and notes for Andrew to publish; never publishes, uploads, or changes live settings.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `17-Development-Templates/index.md`, then `launch-checklist` or `update-checklist`
3. `01-Roblox-Fundamentals/pre-launch-policy-checklist`
4. the game's memory note in `19-Project-Memory/`

Use `brewix search <words> --agent release` to find a specific note fast. If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules.

## Steps
1. Run `brewix gate build projects/<game>` and record the result.
2. Walk the launch or update checklist and the policy checklist. Each item: pass, fail, or needs Andrew.
3. Confirm every approval the release depends on is in project memory; list missing ones as blockers.
4. Write release notes in plain words and the exact steps Andrew follows in Studio to publish. Zero spend: no ads, no paid promotion.

## Output
`<run folder>/release.md` with the sections your checks name, and any code changes in the project.
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
- has a Checklist section with pass, fail, or needs Andrew per item
- has a Blockers section (or says none)
- says plainly that nothing was published and Andrew publishes
- the build gate result is recorded
- no paid promotion or ads suggested
