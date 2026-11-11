---
description: AnalyticsService events: custom, economy (sources and sinks with balance after), and funnels (onboarding, purchase). Naming and limits to verify.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) B8 and C5; craftane.com; jimmygit1977 and tabooharmony roblox-analytics skills
verify: Method names and limits are community-reported. Copy exact signatures and limits from the AnalyticsService Engine API page before coding: UNVERIFIED here.
---
# Event taxonomy

Built-in `AnalyticsService`; no third-party SDK needed. Copy exact method signatures from the Engine API reference, not old examples. Methods reported: `LogOnboardingFunnelStepEvent`, `LogFunnelStepEvent`, `LogEconomyEvent`, `LogCustomEvent`.

## Funnels
| Funnel | Steps |
|---|---|
| Onboarding | Spawned, FirstCoreVerb, FirstReward, FirstUpgrade, FirstCheckpoint (game-specific) |
| Purchase | OpenedShop, ViewedItem, ClickedBuy, Confirmed, Granted |
| Stage progress (obby) | one step per stage reached |
Funnel steps need not fire in order; skipped steps back-fill (reported).

## Economy
- Log sources (earned) and sinks (spent) separately, one SKU per source or sink.
- Always pass `balanceAfterTransaction`.
- Health = sources minus sinks. A missing sink is the classic inflation bug.

## Custom
Example: `ItemUnlocked` with an item ID field.

## Rules
- Log after a successful operation, not on the attempt.
- Batch where possible.
- Creator Rewards has no AnalyticsService event; payouts live in Creator Dashboard.

## Limits (community-reported, UNVERIFIED)
120 + (20 x CCU) calls per minute; 100 custom events; 5 currencies; 10 funnels; 3 custom fields per event.
