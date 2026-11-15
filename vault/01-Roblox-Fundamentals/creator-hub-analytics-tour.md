---
description: Tour of Creator Hub analytics: where each dashboard lives, eligibility (10 DAU, 100 DAU, 1000 DAU), filters, Explore, custom events and dashboards, alerts, and a weekly reading order.
type: reference
status: draft
last_checked: 2026-11-15
keywords: creator hub, analytics, dashboard, retention, engagement, acquisition, monetization, performance, explore, custom dashboards, alerts, benchmarks, insights, funnel
sources: creator-docs content/en-us/production/analytics/analytics-dashboard.md (read 2026-11-15); creator-docs content/en-us/production/analytics/index.md (read 2026-11-15); creator-docs content/en-us/production/analytics/insights.md (read 2026-11-15); creator-docs content/en-us/production/analytics/performance.md (read 2026-11-15); creator-docs content/en-us/production/analytics/alerts.md (read 2026-11-15); creator-docs content/en-us/production/analytics/custom-dashboards.md (read 2026-11-15); creator-docs content/en-us/production/analytics/demographics.md (read 2026-11-15)
verify: Page names, thresholds and limits read in docs 2026-11-15. Left menu labels move often. Benchmark peer sets are not published: UNVERIFIED how similar games are chosen.
tags: [roblox-fundamentals, reference]
---
# Creator Hub analytics tour

Where to look in Creator Hub (create.roblox.com > Creations > your game) and what each page answers. Our own event naming lives in [[06-Retention/event-taxonomy]] and reading habits in [[06-Retention/analytics-reading-guide]].

## Do this
- Activate analytics early: verified email, a 2-step method, then accept the Terms on the game overview page. Group games: only the owner and roles with analytics permission see it.
- Expect full KPIs once a game has more than 10 daily active users and 10 play hours for 7 straight days.
- Use the order from the docs: fix D1 retention and session time first, then D7 and D30, then payer conversion and ARPPU, then drive acquisition.
- Compare against benchmarks (similar games) but treat them as a rough guide only.
- Filter before concluding: age group, country, language, source, platform, OS and first-played date change the answer. Filters apply on every page until turned off.
- Check Performance after every update, breaking down by Place Version to see if the newest build regressed.
- Set 1 or 2 alerts before launch (crash rate, memory) so you are told instead of looking.

## The pages
| Page | Answers |
|---|---|
| Overview and Insights | Big changes vs similar games; achievement panel (six month highs); AI weekly and monthly reports at 1,000+ DAU |
| Retention | D1, D7, D30 of cohorts; dips after big new user waves are normal |
| Engagement | Session time, playtime, DAU and how active users are |
| Acquisition | Sources (Home recommendation, search, teleports, ads), play through rate, Home Recommendations tab |
| Monetization | Revenue, payer conversion, ARPPU, sales data download |
| Demographics | Age, gender, country, language of monthly users (small groups hidden) |
| Feedback | Likes and comments |
| Performance (Monitoring) | Client and server charts, need 100+ DAU |
| Crashes, Error Report | Server out-of-memory snapshots; Luau errors and warnings with stack traces |
| Funnel, Custom events, Custom fields | Your own steps and economy events |
| Safety | Abuse reports per 1,000 playtime hours by channel |
| Audience Reach | Progress toward the all-ages evaluation |

## Filters worth knowing
- Age group buckets such as under 9, 9 to 12, 13 to 15, 18+.
- Active payer status is percentile based inside your game (top 15, middle 35, casual 50, lapsed, never). Platform spender status uses a fixed spend of 9.99 USD in 60 days.
- In-experience activity (early, active, lapsed, reactivated) and user engagement (0, 1, 2 to 3, 4 to 7 days in the last 7).
- Memory group (under 2 GB to over 8 GB) for performance charts.

## Explore, custom dashboards and alerts
- Explore builds charts from standard or custom metrics with time interval, breakdown and filter. Calculated metrics use + - * / (win rate, sources minus sinks for economy health).
- Custom dashboards: up to 20 per game. A dashboard saves only when you press Publish.
- Alerts: up to 20 per game, need 100+ DAU. Example from the docs: client crash rate above 5 percent for 5 minutes, sent to a webhook.
- Event limits: custom events have rate limits and field cardinality caps (values beyond 8,000 combined custom field values group as Other). Design few, reusable currencies and fields.

## Weekly reading order (our habit)
1. Acquisition: did plays from Home move? See [[06-Retention/discovery-signals]].
2. Retention and engagement: D1 and session time, split by platform.
3. Performance and Error Report: any regression at the newest place version?
4. Monetization: conversion and ARPPU by age group.
5. Safety: any channel spike after a feature launch.
6. Write one change to test next, and one metric that proves it.

## For our Roblox games
Copy the principle: ask a question, slice by cohort, change one thing, measure. Pair with a short onboarding funnel (ftue-analytics-funnel) from day one.

Never copy: other creators' dashboards or private numbers. Never use analytics to target children with pressure tactics, and keep benchmarks as context rather than goals.
