---
description: Stage-by-stage difficulty worksheet: sawtooth ramp, one new idea per block, quit-rate column, retune rule, retry under 2 seconds.
type: template
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1 (B6); gamedesign.gg difficulty-curves; Celeste and Super Meat Boy conventions as cited
verify: Template. A filled copy is verified by playtest plus a funnel event per stage (stage reached) after launch.
---
# Difficulty curve worksheet

Two curves: **challenge** (what the game demands) and **capability** (what the player brings). Only the gap is felt. Raise difficulty just after the player shows they have the pattern.

## Shape: sawtooth
Introduce a mechanic, dip easy while the player is a novice again, practice, peak, then introduce the next mechanic and dip again. Linear ramps feel too hard early and too easy late; exponential ramps hit a wall.

## Worksheet
| Stage | New idea (one at most) | Difficulty 1-10 | Rest beat? | Retry cost (seconds) | Players reaching (%) | Quit here (%) | Action |
|---|---|---|---|---|---|---|---|
| 1 | core verb only | 1 | | <2 | | | |
| 2 | | | | | | | |

## Rules
- Stage 1 is nearly unfailable and pays out a win.
- Teach, repeat with one twist, combine. Re-test old skills while teaching new ones.
- Retry under 2 seconds, one input, no confirmation dialogs or death screens between player and retry.
- Failure must be legible: the player can say what went wrong and sees a path.
- Raise difficulty on mastery events, never on a clock.

## Retune rule
If a stage's quit rate is far above its neighbours, first make the solution more readable, then add a checkpoint or rest beat before it, and only then lower difficulty. Change one thing at a time and wait 72 hours before judging ([[50-retention-analytics/analytics-reading-guide]]).
