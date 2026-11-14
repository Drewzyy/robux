---
description: Early churn signals in Roblox games (funnel cliffs, shrinking sessions, stalled progress, skipped dailies) and the fix to try first for each. Use when retention drops or a cohort weakens.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: churn, drop-off, funnel, lapse, at risk, signals, stuck, wall, burnout, retention drop, cohort, diagnosis, fix, session shrink
sources: vault notes 06-Retention/decision-tree, analytics-reading-guide, event-taxonomy, consulted 2026-11-14; general analytics practice (model knowledge)
verify: Thresholds are heuristics, UNVERIFIED. Validate each signal against your own cohorts before acting on it.
tags: [retention, playbook]
---
# Churn signals and fixes

Churn is predictable before it happens. Watch behavior, then fix the cause, not the symptom. Start from [[06-Retention/decision-tree]].

## Do this
- Define "at risk" in data: no session in 2x the player's usual gap, or a falling trend over 3 sessions.
- Review churn by cohort weekly in [[15-LiveOps/weekly-review-template]].
- Ask "where did they stop" first (funnel step, level, zone), then "why" (difficulty, boredom, bug, wall).
- Fix the biggest cluster of quitters before polishing anything else.
- After each fix, compare the next cohort's curve, not the daily average.
- Log a `last_goal_id` and `last_zone` on leave so you know where players quit.
- Pair numbers with watching 5 real sessions of quitters; numbers say where, watching says why.

## Signals and fixes
| Signal | Likely cause | First fix |
|---|---|---|
| Steep D0 drop (quit in first 2 min) | Load, confusing start | [[04-Onboarding/onboarding-mistakes]] |
| Quit cluster at one level or zone | Difficulty or wall | Lower the curve, add a boost, see [[05-Progression/progression-curves]] |
| Session length shrinking per player | Boredom, repetition | Add variety, new goal chain ([[06-Retention/session-length-levers]]) |
| Sessions per day falling | Routine broken | Daily reward, quests, [[06-Retention/day-1-day-7-day-30-playbook]] |
| Daily reward skipped twice | Streak lost, low value | Free streak save, better day-3 reward ([[03-Game-Psychology/streaks-and-daily-rewards]]) |
| No currency spent for many sessions | Nothing worth buying | Add a visible next purchase and new item |
| Late-game players leave after an update | Content ran out | Raise the cap, add prestige, see [[15-LiveOps/content-treadmill-vs-systems]] |
| Spike after bug or patch | Regression | Hotfix, post a note, reward the affected players |
| Friends leave together | Social ties broke | Party features, shared goals ([[06-Retention/social-retention]]) |
| Mobile only drop | Controls, performance | [[08-UI-UX/mobile-first-checklist]] |

## Types of churn
- Early churn (first session): onboarding.
- Wall churn (day 2 to 7): difficulty or pacing.
- Burnout churn (day 14 plus): repetition, grind.
- Content churn (day 30 plus): finished everything.
- Event or update churn: a bad change or a broken event.
Handle each with the matching fix above; one fix does not cover all.

## How to build it in Roblox
- Log events listed in [[06-Retention/event-taxonomy]] with step ids. Use Creator Analytics funnels and custom events.
- Store `sessionCount`, `lastSessionLength`, `lastSeen` in the profile for in-game decisions (like [[04-Onboarding/returning-player-welcome-back]]).
- Create a simple "stuck detector": if a player stays on one goal over 10 minutes with no progress, show a hint or a free boost (UNVERIFIED number).

## Mistakes
- Reacting to a one-day dip. Look at 7 days and at the cohort.
- Fixing with more rewards when the real problem is difficulty or clarity.
- Treating all churn as one number. Segment by tenure and device.
- Spending on ads to refill a leaky bucket; zero spend means fix the bucket.

## Weekly churn review (15 minutes)
1. Open D1, D7 and session-length by cohort.
2. List the top 3 quit points by step or zone.
3. Pick one, watch 3 sessions that ended there.
4. Write the suspected cause and one fix.
5. Ship, then compare the next cohort at the same step.

## For our Roblox games
Copy the principle: detect where and when players leave, match the fix to the cause, test on the next cohort. Never copy any other game's churn tactics that rely on pressure, guilt or deceptive timers, and never use others' art, names, music or logos.
