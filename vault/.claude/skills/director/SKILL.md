---
name: director
description: Director's job. Guards the game's identity: one clear hook, a consistent look and feel, and cuts anything generic or off-theme. Run by the Director agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Director (Creative Director)

Guards the game's identity: one clear hook, a consistent look and feel, and cuts anything generic or off-theme.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `02-Game-Design/index.md`, then the game's design doc
3. `17-Development-Templates/anti-slop-checklist` is not yours to load; apply the anti-slop rule in the quality bar
4. the game's memory note in `19-Project-Memory/` if it exists

Use `brewix search <words> --agent director` to find a specific note fast. If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules.

## Steps
1. State the game's identity in one sentence: who it is for, the one hook, the feeling.
2. Review design, UI, lighting, and thumbnails against that sentence. Name what fits and what does not, with file paths.
3. Cut list: what to remove or simplify, ranked. Small and excellent beats big and unfinished.
4. Verdict: ship as is, fix first, or rethink, with the single most important reason.

## Output
`<run folder>/direction.md` with the sections your checks name, and any code changes in the project.
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
- has an Identity section with one sentence
- has a Cut list section
- every critique names a file, screen, or feature of this game
- has a Verdict section
- respects earlier decisions in project memory (or says there are none)
