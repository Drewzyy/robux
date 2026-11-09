from common import note
S = "50-retention-analytics"
R = "Research report (Nov 9 2026) B8 and C5"

note(f"{S}/analytics-reading-guide", "How to read Creator Analytics: D1 and session first, then D7/D30, acquisition, monetization, performance. Cohorts, data delays, the 72-hour rule.", "reference", "draft",
 R + "; yeno.games; rowatcher.com (two articles); craftane.com", "Secondary guides. Access thresholds and data delays are UNVERIFIED; confirm in Creator Analytics itself once the game is live.", r"""
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
- Log the time of every significant change (update notes, [[50-retention-analytics/weekly-review-template]]).
- **Wait a full 72 hours** before drawing conclusions about a change.
- Compare your own cohorts before and after a change. That is the most honest comparison.
- A game with 2,000 CCU and 35% D1 will usually beat one with 8,000 CCU and 12% D1 over 30 days.

## Diagnosis
Use [[50-retention-analytics/decision-tree]].

## Tools
Third-party: RoWatcher, BloxMetrics, GameAnalytics, Dive Games (all need custom server-side instrumentation). Open Cloud Analytics API with a read-only `universe.analytics:read` key; community `roblox-analytics-mcp`. Any of these need Andrew's account and key.
""")

note(f"{S}/benchmarks", "Honest benchmarks: GameAnalytics 2026 medians (D1 10.3%, D7 1.6%, D30 0.5%, session 9.8 min, payer 3.8%) vs god plan and community targets, labeled.", "reference", "draft",
 R + "; gameanalytics.com/reports/2026-roblox-report; bloxg.com; god plan Section 3 (not in repo)",
 "GameAnalytics figures as quoted in the report; re-read the 2026 report before quoting externally. Community tables are labeled community. Use Creator Analytics similar-game benchmarks for decisions.", r"""
# Benchmarks

Roblox does not publish universal targets. Use **similar-game benchmarks in Creator Analytics** for decisions. The tables below are context.

## GameAnalytics 2026 Roblox report (the distribution)
500+ titles with 1M+ MAU, Aug 1 2025 to Jul 31 2026, 4.76 billion sessions. Large established games, not typical new releases.

| Metric | p25 | Median | p75 | p90 | p99 |
|---|---|---|---|---|---|
| D1 retention | 8.4% | **10.3%** | 12.9% | 15.9% | 22.2% |
| D7 retention | 1.0% | **1.6%** | | | 9.1% |
| D30 retention | 0.3% | **0.5%** | | | 4.7% |
| Session length per game | | **9.8 min** | 12.0 | 14.7 | 21.1 |
| Sessions per player per day per game | | 1.56 | 2.07 | 3.22 | 8.30 |

- Payer rate: **3.80%** of tracked players paid anything. Median ARPPU per game $0.70, p90 $2.33.
- Player level across Roblox: 3 sessions a day, median session 7.28 min, 22.4 min a day.
- **A D1 of 15% is around the 90th percentile, not "solid".**

## Other tables (labeled)
| Metric | God plan (game-specific targets) | BLOXG (community, stretch) |
|---|---|---|
| D1 | weak <10%, solid 15-25%, excellent 30%+ | good 20%, great 30%, excellent 40%+ |
| D7 | weak <3%, solid 8-15%, excellent 15%+ | good 8%, great 15%, excellent 20%+ |
| D30 | weak <1%, solid 3-8%, excellent 8%+ | good 3%, great 7%, excellent 10%+ |
| Session | <5 min weak, 15-20 solid, 30-45 excellent | n/a |

A D1 of 20% would be near p99 in the GameAnalytics sample. Treat community numbers as stretch goals for a hit, not a pass/fail bar.

## Sky Tower Obby target (god plan)
D1 15%+, session 10+ min, like ratio 85%+. That is about p90 on D1: a strong-game target, not a median. (God plan not in repo; figures from the report.)

## Do not use
Mobile medians (GameAnalytics mobile: D1 about 22%) do not apply to Roblox.
""")

note(f"{S}/discovery-signals", "Recommended for You: retrieval then ranking; play-through, first-play bounce, play days, playtime most important; co-play next. June 2026: 28-day window, QPTR retired.", "reference", "verified",
 R + "; github.com/Roblox/creator-docs/blob/main/content/en-us/discovery.md (read Nov 9 2026 per report); allthings.how; gmmarket.me",
 "Signal list from Roblox's own Creator Docs (discovery.md) as read for the report; the June 2026 window and QPTR change are from Roblox's announcement as summarized. Re-read discovery.md before relying on weights.", r"""
# Discovery signals

## How Recommended for You works
1. **Retrieval:** picks candidates using engagement, retention, and monetization signals.
2. **Ranking:** orders them for each player.
How far a game goes depends on the engagement and retention of players who arrived **through Recommended for You**. Players acquired from ads, curation, friends, search, social media, or elsewhere do not count in the ranking stage (they can speed up consideration).

## Most important signals
| Signal | Meaning |
|---|---|
| Play-through rate | rate users play after seeing the game in Recommended for You |
| First-play bounce rate (negative) | rate users leave after a short first session; segments under 60 s and 61 to 180 s |
| Play days per user | unique days played; windows Day 1, Day 2-7, Day 8-28 |
| Playtime per user | average time, capped at 60 minutes per user per game per day |

## Important signals
Intentional co-play days per user (join, invites, private servers); qualified play sessions per user; spend days per user; Robux spent per user.

## Other points
- Signals are per-user averages, not totals: small games with engaged players are not disadvantaged.
- Explore then expand: an update triggers an explore phase; good cohorts expand.
- Benchmarks do not affect ranking; they are for comparison.
- Best practices: accurate metadata, original imagery and names, your own spin on trends, a custom thumbnail.
- Limits exposure: leading with giveaways, metadata that does not match the game, repetitive titles, images, or place files.

## June 2026 change (correction to the god plan)
Window expanded **from 7 to 28 days**. **Qualified Play-Through Rate (QPTR) retired**, replaced by standard play-through rate plus first-play bounce rate, both shown in Creator Analytics. Any note quoting QPTR or a 7-day window is out of date.
""")

note(f"{S}/event-taxonomy", "AnalyticsService events: custom, economy (sources and sinks with balance after), and funnels (onboarding, purchase). Naming and limits to verify.", "reference", "draft",
 R + "; craftane.com; jimmygit1977 and tabooharmony roblox-analytics skills",
 "Method names and limits are community-reported. Copy exact signatures and limits from the AnalyticsService Engine API page before coding: UNVERIFIED here.", r"""
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
""")

note(f"{S}/decision-tree", "What to fix first: low play-through means packaging, high bounce means first 60 seconds, D1 ok but D7 falling means progression, then D30, payers, acquisition.", "reference", "draft",
 R + " (B8b, C5)", "Roblox's priority order as summarized from Creator Docs and creator guides. Each branch is checked against your own before/after cohorts, waiting 72 hours.", r"""
# Decision tree

Work top to bottom. Fix one thing, wait 72 hours, compare cohorts.

1. **Play-through rate low?** The packaging is the problem: icon, thumbnail, title, description ([[30-ui-ux/thumbnail-icon-spec]]).
2. **First-play bounce high (under 60 s or 61-180 s)?** Fix loading, spawn flow, controls, tutorial friction, unclear goals, or a misleading thumbnail ([[10-game-design/first-60-seconds-checklist]]).
3. **D1 and D7 both weak?** Fix onboarding and the core loop first.
4. **D1 fine but D7 falls sharply?** Players run out of things to do: progression and content ([[10-game-design/progression-systems]]).
5. **D1 stable but Day 2-7 or Day 8-28 declining?** Check save data, content pacing, return goals, and whether updates add value.
6. **D30 low?** Missing end-game goals; live ops and seasons close this gap ([[50-retention-analytics/live-ops-calendar-template]]).
7. **Session short?** The core loop lacks depth.
8. **Retention healthy, payers low?** Then (and only then) the shop ([[40-monetization/pricing-ladder]]).
9. **Everything healthy?** Then acquisition.

Roblox's order: D1 and session time, then D7 and D30, then payer conversion, then acquisition.
""")

note(f"{S}/weekly-review-template", "Weekly review: change log with timestamps, one named reason per change, cohort before and after, 72-hour wait, one hour cap on competitor watching.", "template", "draft",
 R + " (C5)", "Template; no facts to verify.", r"""
# Weekly review template

**Week of:** ____

## Numbers (from Creator Analytics, with the date read)
| Metric | Last week | This week | Similar-game p50 to p90 |
|---|---|---|---|
| D1 | | | |
| D7 | | | |
| Session length | | | |
| Play-through | | | |
| First-play bounce | | | |
| Payer rate | | | |

## Changes shipped (timestamped)
| When (UTC) | What changed | One named reason | Metric it should move | Read after (72 h) |
|---|---|---|---|---|

## Decision
Walk [[50-retention-analytics/decision-tree]]. One change for next week, with its reason.

## Competitors
One hour max ([[70-competitors/tracker-watchlist]]).
""")

note(f"{S}/live-ops-calendar-template", "Live ops: weekly events, content cadence by genre, daily rewards, limited items, double-coin weekends. One human decision per update.", "template", "draft",
 R + " (C5 live-ops, B5); Grow a Garden and Tower of Hell cadence from the report", "Cadence figures are observed from other games (secondary). Verify fit for each game with event spike and decay in your own analytics.", r"""
# Live ops calendar

## Cadence by genre (observed in other games)
| Genre | Content cadence |
|---|---|
| Obby | stage pack or new tower theme every 1 to 2 weeks |
| Tower defense | new towers or maps every 2 to 3 weeks |
| Farming | weekly events (Grow a Garden had 9 update days in the 30 days before Nov 9, 2026) |

Event spikes are big and fall fast, so events must be regular, not one-off.

## Standing systems
- Daily reward with an escalating 7-day streak and an exclusive Day 7 reward. Add grace days or streak repair: a single missed day should not wipe a streak.
- Offline timers (crop, egg, potion) handed to the player on exit.
- Limited-time free items, double-coin weekends, themed towers or seasons.

## Calendar
| Week | Event or content | Reason (one) | Metric to watch | Owner agent |
|---|---|---|---|---|

## Rules
- One human decision per update, with a named reason.
- Update notes: one short paragraph.
- Nothing goes live without Andrew approving that exact build.
""")
print("50 done")
