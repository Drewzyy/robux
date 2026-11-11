---
description: Currencies, multipliers, rebirth/prestige, collections, levels, offline progress, scarcity: what each does and when to use it.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1 (B2, B5, C1)
verify: Design guidance; no numeric claims. Verify per game through D7 retention (low D7 usually means weak progression, per Roblox guidance as summarized in the report).
tags: [progression, reference]
---
# Progression systems

| System | What it gives the player | Use when | Watch out |
|---|---|---|---|
| Soft currency | visible progress every action | almost always | one currency in game 1; more currencies means more confusion |
| Levels / XP bar | competence feedback (fixed ratio) | any game with repeat play | low early thresholds so the first level-up lands in session 1 |
| Multipliers | numbers going up faster | simulators, tycoons | must live in one tuning module |
| Rebirth / prestige | a fresh start with a permanent bonus | after content runs out (tycoon, TD, simulator) | show the payoff before the reset |
| Collections / index | long goals, completion drive | pets, RNG, farming | completion should give something (boost, title) |
| Offline progress / timers | a reason to come back tomorrow | farming, idle | compute from saved timestamps, never server countdowns |
| Scarcity (restocks, limited items) | urgency, talk | farming, events | free scarcity is fine; paid FOMO on kids is not |
| Best times / leaderboards | mastery and rivalry | obby, racing | friends leaderboards beat global ones |

## Rules
- Fixed schedules carry progression; variable schedules are for surprise ([[03-Game-Psychology/reward-schedules]]).
- Endowed progress: start bars partly filled, never at zero (goal gradient effect).
- Low D7 usually means a weak progression system; low D30 means missing end-game goals ([[06-Retention/decision-tree]]).
