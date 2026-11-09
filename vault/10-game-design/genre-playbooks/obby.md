---
description: Obby playbook: readable stages, checkpoints as mercy, skill vs progression obby, first stage trivially easy, first success within 30 seconds.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1; rocode.app/guides/roblox-game-genres; Tower of Hell teardown (70-competitors)
verify: Design rules are community and practitioner guidance, not Roblox rules. Check against Creator Analytics after launch: first-play bounce under 60s and D1 vs similar-game benchmarks.
---
# Obby playbook

**Core loop:** jump numbered stages, reach the next checkpoint.
**Hard part:** a difficulty curve that does not lose players early.

## Two models, pick one on purpose
| | Skill obby (Tower of Hell) | Progression obby |
|---|---|---|
| Checkpoints | none | every stage or every few |
| Round | short, timed (ToH: 8 minutes, random tower) | open-ended |
| Retention from | mastery, rivalry, spectating, "one more run" | coins, checkpoints, finish line, best times |
| Who finishes | most never do | most can |

A middle model (checkpoints plus timed races, best times, leaderboards) is what the god plan picks for Sky Tower Obby (god plan not in this repo; flagged UNVERIFIED here).

## Rules
- Stage 1 is trivially easy. First success within 30 seconds of spawn.
- Every stage has a readable solution before the player jumps: telegraph hazards, readable silhouettes, contrast against the background.
- Sawtooth difficulty (see [[10-game-design/difficulty-curve-worksheet]]): each block of 5 to 10 stages introduces one new hazard, dips easy, practices, peaks, then resets with the next mechanic.
- One new idea at a time. Never a new mechanic and a new difficulty jump together.
- Death to retry under 2 seconds, one input, no screens in between. Checkpoints are the mercy system: failure costs seconds, not progress.
- Hidden mercy after 3 to 5 deaths at one spot (extra checkpoint, slower hazard), never announced. UNVERIFIED for Roblox specifically; it is a general design pattern (Crash Bandicoot).
- Progression rewards on fixed schedules (coins per stage). Variable only for bonus surprises (secret coins, rare drop).
- Checkpoints are never sold. Skip Stage as a developer product is a judgment call; see [[40-monetization/do-not-sell-list]].

## Motivation coverage
Achievers: stages, leaderboards, collections. Explorers: secret coins, hidden paths, new tower themes. Socializers: co-play, parties, visiting. Killers: timed races and leaderboards; keep PvP optional and friendly for kids.

## What kills an obby
Long walk to stage 1; unreadable stage 3; a shop pop-up on death; lag on join; a new mechanic introduced at a difficulty peak.
