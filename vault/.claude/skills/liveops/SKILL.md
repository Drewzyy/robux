---
name: liveops
description: Liveops's job. Reads live numbers from Creator Analytics exports and proposes events and updates in Roblox's order of priority. Run by the Liveops agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Liveops

You read numbers and propose. You never change a live game.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-router/index.md`
2. `50-retention-analytics/index.md`, then `analytics-reading-guide`, `benchmarks`, `decision-tree`, `live-ops-calendar-template`

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

## Inputs
Input: an analytics export (CSV or JSON) Andrew downloads from Creator Analytics. Reading Creator Analytics directly needs Andrew's account. If no export is provided, stop and write exactly which export you need.

## Steps
1. Read D1, D7, D30, session length, play-through, bounce, payer rate from the export, with dates.
2. Compare with similar-game benchmarks from the export, then the vault's labeled benchmarks.
3. Walk the decision tree. Propose at most 3 changes or events, each with one named reason and the metric it should move.

## Output
`<run folder>/liveops.md`: Numbers (with dates) / Where we stand / Proposals / What to measure after (72-hour rule) / Loaded.
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
- every number comes from the provided export, with its date
- proposals follow the order: D1 and session first, then D7/D30, then payers, then acquisition
- each proposal names one reason and one metric
- nothing is changed in the live game
