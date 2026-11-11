---
description: Simulator playbook: click, upgrade, rebirth loop; every price in one ModuleScript; build order; no trading in game 1.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1; rocode.app/guides/roblox-game-genres
verify: Community guidance. Verify the curve with the economy worksheet before launch and with economy events (AnalyticsService) after.
tags: [game-design, playbook]
---
# Simulator playbook

**Core loop:** click (or act) for currency, buy upgrades, rebirth for a multiplier.
**Hard part:** numbers that grow for weeks without breaking.

## Rules
- Every price, rate, and multiplier lives in one shared ModuleScript (`src/shared/Data/Economy.luau`, frozen with `table.freeze`), so the whole curve is tuned in one place and the server and client agree.
- Build order: core loop on a baseplate first, then progression, then saving, then UI, then monetization.
- Rebirth resets base progress for a permanent multiplier. Show the multiplier before the player commits.
- Do not build trading in game 1. Trading needs a working economy, moderation, and scam protection.
- Model the curve in [[02-Game-Design/economy-worksheet]] before coding: time to next upgrade must stay short early and grow slowly.

## Reward schedules
Fixed ratio: currency per action. Fixed: upgrade ladder. Variable: rare drops, eggs (free only in game 1; paid random items need [[07-Monetization/paid-random-items-checklist]]).
