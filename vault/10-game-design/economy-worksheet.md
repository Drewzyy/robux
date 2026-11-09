---
description: Economy worksheet: sources, sinks, prices in one ModuleScript, time-to-next-goal column, and the failure modes that break games.
type: template
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1 (C1, C5)
verify: Template. Verify a filled copy with a Lune test that simulates the first hour of income and prints time-to-next-goal per purchase.
---
# Economy worksheet

## Sources and sinks
| Source (earned) | Amount | Per | Sink (spent) | Amount | Notes |
|---|---|---|---|---|---|
| | | | | | |

A missing sink is the classic hidden inflation bug. Log both with AnalyticsService economy events, with balance after each transaction ([[50-retention-analytics/event-taxonomy]]).

## Price ladder
| Item | Price | Income per minute at that point | Time to afford (min) | Feels close? |
|---|---|---|---|---|
| | | | | |

Keep early time-to-afford short (under a minute in session 1); let it grow slowly.

## Rules
- All prices and rates in one frozen shared ModuleScript, read by server and client.
- Server does all currency math; client only displays.
- Pure functions (price for level, income for upgrades) live in `src/shared` so Lune can test them.

## Failure modes
- Rare items available too often: no reason to wait or return.
- Sell prices too high: players finish the content in a weekend.
- Prices that jump: a wall that reads as bait and switch.
