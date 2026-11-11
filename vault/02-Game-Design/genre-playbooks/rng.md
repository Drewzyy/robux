---
description: RNG playbook: roll, collect, index boosts; free rolls fine everywhere; paid rolls need the Paid Random Items rules and region gating.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1; rocode.app/guides/roblox-game-genres; Paid Random Items reporting (see 07-Monetization)
verify: Odds rules are Roblox policy as reported in 2026 secondary sources. Re-read the Paid Random Items policy in Creator Hub before building any paid roll.
---
# RNG playbook

**Core loop:** roll for rare results, collect, roll again.
**Hard part:** odds design and the Paid Random Items rules.

## Rules
- Free rolls are allowed everywhere.
- Paid rolls are Paid Random Items: numeric odds for every outcome shown before purchase, summing to 100%; every outcome has value; region gating with PolicyService; fail closed. Full list: [[07-Monetization/paid-random-items-checklist]]. Luck boosts, pity systems, rerolls, and keys bought with Robux all count as paid random items.
- Not in game 1 (studio rule, see [[07-Monetization/do-not-sell-list]]).
- Example structure (Tycoon RNG, from RoCode): odds displayed like 1/94.5B, an index that unlocks permanent boosts as you collect, rebirth unlocking new tiers of rarer machines.
- Progress must not depend only on luck. Pair the variable reward with a fixed track (index completion, levels) so competence is visible. See [[03-Game-Psychology/reward-schedules]].
- Audience is mostly minors. Do not build the core retention loop on paid RNG.
