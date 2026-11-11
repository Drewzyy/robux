---
name: producer
description: Producer's job. Turns Liveops proposals and Andrew's direction into the next update plan with a scope that fits one update. Run by the Producer agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Producer

You plan the next update. One update, one named reason.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `06-Retention/index.md`, then `weekly-review-template`, `live-ops-calendar-template`
3. `17-Development-Templates/update-checklist` (the router sends checklists there)

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

## Inputs
Input: the Liveops proposals and Andrew's direction.

## Steps
1. Pick the one change that matters most and say why in one sentence.
2. Break it into stages for the team (agent, output, gate), ending in the review queue.
3. Write the update notes draft (one short paragraph) and the before/after metrics to watch.

## Output
`<run folder>/update-plan.md`: Reason / Scope (in, out) / Stages / Metrics to watch / Update notes draft / Loaded.
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
- one named reason
- scope fits one update (out-of-scope list present)
- every stage has an agent, output, and gate
- plan ends in the review queue; nothing goes live without Andrew's approval of that build
