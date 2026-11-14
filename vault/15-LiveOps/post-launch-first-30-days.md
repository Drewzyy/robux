---
description: A day-by-day plan for the first 30 days after a zero-spend Roblox launch: watch metrics, hotfix, first update, first event, community setup, and what decisions to make at day 7, 14 and 30.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: post launch, first 30 days, launch week, hotfix, first update, metrics, day 7 review, day 30 review, soft launch, feedback, algorithm, checklist, plan
sources: vault notes 15-LiveOps/liveops-playbook, live-ops-calendar-template, weekly-review-template, 06-Retention/benchmarks, consulted 2026-11-14; general launch practice (model knowledge)
verify: Day targets and traffic effects are practice, UNVERIFIED against Roblox data. Benchmarks come from the vault benchmarks note and are UNVERIFIED externally.
tags: [liveops, checklist]
---
# Post-launch: first 30 days

The first month sets the algorithm signals and the player base. Plan the work before launch so you spend launch days watching, fixing and talking to players.

## Do this
- Before launch: save-data test, mobile test, exploit check, analytics events live ([[06-Retention/event-taxonomy]]), community channels open.
- Days 1 to 3: watch, do not redesign. Fix crashes, data loss, exploits and blockers within hours.
- Days 1 to 3: log the funnel (join, first input, first reward, minute 5) and watch for drops ([[04-Onboarding/onboarding-mistakes]]).
- Day 3 to 4: ship the first small update (bugs, one quality-of-life fix). It shows the game is alive.
- Day 7: first retention read (D1 cohort, D7 for launch day). Compare to [[06-Retention/benchmarks]] and similar games in Creator Analytics.
- Day 7 to 10: choose one major fix by the decision tree ([[06-Retention/decision-tree]]).
- Day 10 to 14: ship the first content or system update plus a small event ([[15-LiveOps/event-design]]).
- Day 14: second review; decide whether to double down on what players use or fix what they skip.
- Day 21 to 30: ship the first themed event or mini-season; start a code drop for the community ([[15-LiveOps/code-and-reward-drops]]).
- Day 30: full review: D1, D7, D30 for first cohorts, playtime, favorites, likes ratio, bug list. Write the next 30-day plan.

## Timeline table
| Day | Focus | Output |
|---|---|---|
| 0 | Launch checks | Pre-launch checklist complete |
| 1 to 3 | Stability | Hotfixes, error log clean |
| 3 to 4 | First update | Patch notes everywhere |
| 7 | Review 1 | D1 and funnel read, one fix chosen |
| 10 to 14 | Update 2 | Fix plus one new feature |
| 14 | Review 2 | Keep, change or cut |
| 21 to 30 | Event | First limited-time event |
| 30 | Review 3 | Next 30-day plan |

## Signals to watch
- Funnel step drops, session length, sessions per day ([[06-Retention/session-length-levers]]).
- Like ratio and favorites; low ratio means a promise mismatch or bugs.
- Error rate and server health (Developer Console, Creator Dashboard).
- Community feedback themes ([[15-LiveOps/community-management]]).
- Exploit reports; patch fast and quietly, thank reporters.

## Decisions
- Day 7: if D1 is far below similar games, stop adding content and fix the first 5 minutes.
- Day 14: if D1 is fine but D7 is weak, build a routine (daily reward, quests; see [[06-Retention/day-1-day-7-day-30-playbook]]).
- Day 30: if retention is solid but numbers are small, invest in discovery levers like thumbnail and title tests ([[08-UI-UX/thumbnail-icon-spec]]).
- If nothing moves after two cycles, reconsider the core loop ([[02-Game-Design/core-loop-template]]).

## How to build it in Roblox
- Keep a staging place and feature flags so you can ship hourly fixes ([[15-LiveOps/update-cadence]]).
- Keep a `GAME_VERSION` string in every analytics event.
- Use a shared task board (a free tool or a vault note) with columns: now, next, later.

## Mistakes
- Redesigning on launch-day noise.
- Shipping no update for two weeks.
- Skipping the day-7 review.
- Paying for ads to fill a leaky first session; the zero-spend plan fixes the funnel first.

## For our Roblox games
Copy the principle: stabilize, update early, review on fixed days, decide from data. Never copy another game's launch events, names, art, music or logos.
