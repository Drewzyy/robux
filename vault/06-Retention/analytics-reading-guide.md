---
description: How to read Creator Analytics: D1 and session first, then D7/D30, acquisition, monetization, performance. Cohorts, data delays, the 72-hour rule.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) B8 and C5; yeno.games; rowatcher.com (two articles); craftane.com
verify: Secondary guides. Access thresholds and data delays are UNVERIFIED; confirm in Creator Analytics itself once the game is live.
tags: [retention, reference]
---
# Reading Creator Analytics

## Order (Roblox's suggested order, as summarized)
1. D1 retention and average session time.
2. D7 and D30 retention.
3. Acquisition by source.
4. Monetization.
5. Performance.

## How the data works
- Retention is by **cohort**: everyone who first played on a day. D1 = share who came back the next day.
- Data is not live. Retention takes around 48 hours in one source; custom events around 24 hours in another (UNVERIFIED).
- Full analytics reportedly unlock at 10+ DAU and 10+ play hours for seven consecutive days (UNVERIFIED).
- Similar-game benchmarks need at least 100 DAU and show the 50th to 90th percentile.
- Reported features: hourly granularity, two-year history, New User First Session Retention, genre percentile benchmarks.

## Rules
- Log the time of every significant change (update notes, [[15-LiveOps/weekly-review-template]]).
- **Wait a full 72 hours** before drawing conclusions about a change.
- Compare your own cohorts before and after a change. That is the most honest comparison.
- A game with 2,000 CCU and 35% D1 will usually beat one with 8,000 CCU and 12% D1 over 30 days.

## Diagnosis
Use [[06-Retention/decision-tree]].

## Tools
Third-party: RoWatcher, BloxMetrics, GameAnalytics, Dive Games (all need custom server-side instrumentation). Open Cloud Analytics API with a read-only `universe.analytics:read` key; community `roblox-analytics-mcp`. Any of these need Andrew's account and key.
