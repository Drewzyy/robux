---
description: What to fix first: low play-through means packaging, high bounce means first 60 seconds, D1 ok but D7 falling means progression, then D30, payers, acquisition.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) B8 and C5 (B8b, C5)
verify: Roblox's priority order as summarized from Creator Docs and creator guides. Each branch is checked against your own before/after cohorts, waiting 72 hours.
---
# Decision tree

Work top to bottom. Fix one thing, wait 72 hours, compare cohorts.

1. **Play-through rate low?** The packaging is the problem: icon, thumbnail, title, description ([[08-UI-UX/thumbnail-icon-spec]]).
2. **First-play bounce high (under 60 s or 61-180 s)?** Fix loading, spawn flow, controls, tutorial friction, unclear goals, or a misleading thumbnail ([[04-Onboarding/first-60-seconds-checklist]]).
3. **D1 and D7 both weak?** Fix onboarding and the core loop first.
4. **D1 fine but D7 falls sharply?** Players run out of things to do: progression and content ([[05-Progression/progression-systems]]).
5. **D1 stable but Day 2-7 or Day 8-28 declining?** Check save data, content pacing, return goals, and whether updates add value.
6. **D30 low?** Missing end-game goals; live ops and seasons close this gap ([[15-LiveOps/live-ops-calendar-template]]).
7. **Session short?** The core loop lacks depth.
8. **Retention healthy, payers low?** Then (and only then) the shop ([[07-Monetization/pricing-ladder]]).
9. **Everything healthy?** Then acquisition.

Roblox's order: D1 and session time, then D7 and D30, then payer conversion, then acquisition.
