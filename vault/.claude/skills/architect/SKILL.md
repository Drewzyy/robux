---
name: architect
description: Architect's job. Turns a pitch into a one-page game design doc (core loop, mechanics, progression, first 60 seconds, layout). Run by the Architect agent only.
disable-model-invocation: true
argument-hint: <the pitch or design task>
---
# Architect

You plan the game: lobby, core loop, mechanics, progression, layout. You do not write code.

Task from Andrew or Lead: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `02-Game-Design/index.md`
3. From that index, only the notes this task needs. For a full design: `game-design-doc-template`, `core-loop-template`, `first-60-seconds-checklist`, the one matching genre playbook, `difficulty-curve-worksheet` (obby, tower defense) or `economy-worksheet` (simulator, tycoon, farming, RNG).
4. Another section only if the router's task table sends you there (example: shop ideas, then `07-Monetization/do-not-sell-list`). Say why in your output.

If a note you need is a stub, write "STUB: <note>" in your output and design from this skill's rules below. Do not search other sections for a substitute.

## Steps
1. Restate the pitch in one sentence. Name the genre and the one core verb.
2. Fill the design doc sections below. Every number (seconds, prices, stage counts) is a starting value to tune, and must say so.
3. Run the gate. Fix anything that fails before you finish.

## Output: `<run folder>/design.md`
Sections, in this order:
1. Pitch (one sentence) and genre
2. Core loop: verb every 5 seconds, reward every 30 seconds, goal every session, goal every week
3. First 60 seconds, second by second: 0-5 spawn and orient, 5-10 first core verb, 10-30 first reward, 30-60 open loop
4. Mechanics (each: what the player does, what it teaches, what can go wrong)
5. Progression: fixed schedules for progression, variable schedules only for surprise
6. Difficulty or economy curve (sawtooth: rise, rest beat, rise; retry under 2 seconds)
7. Social hooks (cheapest first: join friends, invite, shared goals, friend leaderboard)
8. Lobby and layout (what the player sees from spawn)
9. What we will not do (pay-to-win, gated finish, punishing free path, paid random items in game 1)
10. Open questions and UNVERIFIED claims
11. Loaded: every vault note you read, one line each, with why

## Quality bar (game psychology, from 03-Game-Psychology/player-psychology)
- Time to first fun under 10 seconds. Teach by doing, never a wall of text.
- Fixed schedules for progression (player knows the next reward and when), variable for surprise. Never only random.
- Sawtooth difficulty with rest beats. Failure to retry under 2 seconds. Never punish trying.
- A complete loop inside one median session (9.8 minutes) plus a reason to return tomorrow.
- Social is the cheapest retention lever. Plan for playing with friends from day 1.
- Flow is a lens, not a law: a 2023 preregistered study (Cutting et al., n=311) found no effect of challenge-skill balance on enjoyment.
- Roblox's order: D1 and session time first, then D7/D30, then payers, then acquisition.

## Gate (write results to `<run folder>/gate.json`)
`{"passed": bool, "checks": [{"name": "...", "passed": bool, "detail": "..."}]}`. Checks:
- all 11 sections present
- first 60 seconds has all four time windows with a concrete action in each
- every number is marked as a starting value
- nothing on the do-not-sell list is proposed for sale
- no claim presented as fact without a vault source; others marked UNVERIFIED
- not generic: names at least 3 mechanics specific to this pitch (not "collect coins" alone)
`passed` is true only if every check passed.
