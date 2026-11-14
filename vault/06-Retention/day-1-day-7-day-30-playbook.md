---
description: What to build and ship for D1, D7 and D30 retention on Roblox: the hook each window needs, concrete features, benchmarks to compare against, and what to check when a window is weak.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: d1, d7, d30, retention, day 1, day 7, day 30, daily reward, hook, return, loop, benchmark, cohort, progression, social
sources: vault note 06-Retention/benchmarks (GameAnalytics 2026 Roblox report figures as quoted there), consulted 2026-11-14; vault notes 06-Retention/decision-tree, 03-Game-Psychology/why-players-return
verify: Benchmarks come from the vault benchmarks note, which cites GameAnalytics; not re-opened this session, treat as UNVERIFIED externally. Feature suggestions are practice, not studies.
tags: [retention, playbook]
---
# Day 1, day 7, day 30 playbook

Each window has a different job. D1 asks "was the first session good and is there a reason to return". D7 asks "is there a routine". D30 asks "is there depth and community". Numbers are in [[06-Retention/benchmarks]]; use Creator Analytics similar-game comparison for decisions.

## Do this
- Work in order D1 then D7 then D30. Fixing D30 with a leaky D1 wastes effort (see [[06-Retention/decision-tree]]).
- D1 target: a visible reason to return tomorrow set before the session ends (timer, daily reward, half-done goal, unhatched egg).
- D7 target: a weekly rhythm: daily reward, a weekly goal, one update or event inside the first 7 days.
- D30 target: long-term goals (collection, ranks, rebirth), social ties, and regular content change.
- Compare your D1 against the median of 10.3 percent and p90 of 15.9 percent from the vault benchmarks (large established games, so a new game may sit lower, UNVERIFIED).
- Ship one retention change per update and measure that cohort against the previous one (see [[15-LiveOps/liveops-playbook]]).
- Cohort by install date and by device, never by blended daily averages.

## D1 (first return)
| Lever | How |
|---|---|
| Strong first 5 minutes | [[04-Onboarding/first-session-rewards]] and [[04-Onboarding/tutorial-patterns]] |
| Open loop | Timer or half-built goal visible at session end |
| Daily reward preview | Show tomorrow's reward at session end (see [[03-Game-Psychology/streaks-and-daily-rewards]]) |
| Save and remember | Progress persists, goal restored on rejoin |
| Easy return | Favorite and follow prompts after a win, not at join |

## D7 (routine)
- Daily reward with a 7-day cycle where day 7 is the best prize.
- Daily quests (2 to 3, each under 5 minutes) plus one weekly quest.
- A second progression layer unlocks around day 3 to 5 (new zone, new tool tier) so there is something new inside the week.
- First event or update lands inside the first 7 days (see [[15-LiveOps/post-launch-first-30-days]]).
- Friends: nudge a first friend invite after a win (see [[06-Retention/social-retention]]).

## D30 (depth)
- Prestige or rebirth loop with a visible multiplier, a collection to complete ([[06-Retention/collection-and-completion]]), ranks or leaderboards.
- Monthly or seasonal content ([[15-LiveOps/event-design]]).
- Community presence: group, Discord, DevForum posts ([[15-LiveOps/community-management]]).
- Mid-game gap check: look for the level where progress slows to a crawl and add a boost or new content there.

## Diagnose a weak window
| Weak | First check |
|---|---|
| D1 | Funnel drop in first 5 min, reward timing, load time, first-session length |
| D7 | Day-2 to day-6 curve shape: steep early drop means no hook; flat then cliff means content ran out |
| D30 | Late-game content, social ties, whether regulars hit a wall |

## Measure
- Retention by cohort per update (D1, D7, D30), sessions per day, playtime per user, and session length (see [[06-Retention/session-length-levers]]).
- Retention by feature use: players who used the daily reward versus those who did not (correlation, not proof).
- Churn view: [[06-Retention/churn-signals-and-fixes]].

## Example 30-day arc (generic)
- Day 0: tutorial, first reward, tomorrow's reward shown.
- Day 1: daily reward and a new goal chain; returning card if away over 2 days.
- Day 3: second progression layer unlocks; first friend invite prompt.
- Day 7: weekly quest completes with a bigger prize; first update or event.
- Day 14: collection milestone, prestige preview.
- Day 30: season finale or monthly event, new cohort goal.

## For our Roblox games
Copy the principle: give each window one job and a concrete mechanic for it. Never copy another game's reward names, pets or characters, art, music or logos, and never plan retention around paid ads or paid assets.
