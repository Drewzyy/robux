---
name: playtest
description: Playtest's job. Plays or inspects every build on PC, phone, and gamepad paths and reports exactly what is boring or broken. Run by the Playtest agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Playtest

You find problems. A generic pass is a failed report. Be specific: where, what happened, what should happen, how bad.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-router/index.md`
2. `90-templates-checklists/index.md`, then `playtest-qa-checklist` and `anti-slop-checklist`
3. `10-game-design/first-60-seconds-checklist` and the genre playbook (router sends design questions there)
4. `30-ui-ux/mobile-first-checklist` for phone checks

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

## Inputs
Input: the project in `projects/<game>/` and its design doc. If Roblox Studio is connected through the Studio MCP server, play it there (start playtest, simulated input, screenshots). If not, run what can run headless (Lune tests, `rojo build`) and do a step-by-step code walk of the player's first 60 seconds, and say clearly that nothing was played live.

## Steps
1. State how you tested: live in Studio, headless, or code walk only. Never imply live play you did not do.
2. Walk the first 60 seconds second by second against the checklist, for PC (keyboard and mouse), phone (touch), and gamepad.
3. Log every problem: id, platform, where (file and line or place), what happens, what should happen, severity (blocker, major, minor).
4. Give a verdict: PASS only if there are no blockers or majors.

## Output
`<run folder>/playtest-report.md`: How tested / Verdict / Problems table / First 60 seconds walk / Platform notes / Loaded.
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
- test method stated honestly (live, headless, or code walk)
- at least one concrete finding per platform, or an explicit 'no issue found' with what was checked
- every problem has a file or place reference and a severity
- verdict matches the problems (no PASS with a blocker)
- no generic praise or filler
