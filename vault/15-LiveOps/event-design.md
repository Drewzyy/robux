---
description: Designing limited-time events and seasons for Roblox games with no paid assets: structure, length, reward tracks, FOMO ethics, event economy, build tips, and a post-event review checklist.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: event design, limited-time event, season, battle pass, seasonal, holiday, reward track, fomo, event currency, leaderboard, community goal, return rewards
sources: vault notes 15-LiveOps/liveops-playbook, live-ops-calendar-template, consulted 2026-11-14; general live-ops practice (model knowledge)
verify: Event lengths and reward shares are heuristics, UNVERIFIED. Retention lift of events is UNVERIFIED; measure with cohorts.
tags: [liveops, playbook]
---
# Event design (limited-time events and seasons)

Events create reasons to return that the base game cannot. Keep them cheap to build and fair to players who miss them.

## Do this
- Give each event a clear name, theme, start and end time shown in game (UTC and local time).
- Length: 3 to 7 days for a mini-event, 2 to 3 weeks for a major event, 4 to 8 weeks for a season (UNVERIFIED, tune by data).
- Use one core mechanic per event: collect X, defeat boss, community goal, or leaderboard sprint. Do not stack five.
- Add an event currency earned through normal play; spend it in an event shop. Cap daily earnings so casual players can finish.
- Provide a reward track with 8 to 15 steps; first reward within 5 minutes, final reward needs about 60 to 70 percent of an active free player's expected event time (UNVERIFIED).
- Include a return-rewards path: ways to get missed rewards later (a re-run, a lower-tier version) so FOMO is mild.
- Seasons: a theme, a free track, and new items per week; reveal next week's reward in advance.
- Schedule events around your calendar ([[15-LiveOps/live-ops-calendar-template]]) and avoid clashing with big platform dates.
- Never sell event rewards in the zero-spend plan; use play-earned only.

## Event types
| Type | Best for | Cost to build |
|---|---|---|
| Double rewards weekend | Cheap spike | Very low |
| Collect-and-spend event | Regulars | Medium |
| Boss or raid | Co-op games | Medium |
| Community goal (everyone contributes) | Social retention | Medium |
| Holiday reskin | Seasonal theme | Low to medium |
| Leaderboard sprint | Competitive players | Medium |
| Season track | Long-term routine | High |

## FOMO and fairness
- Limited items give urgency, but too many create resentment. Keep at most 1 truly exclusive item per event.
- Show remaining time clearly; never reset timers or extend them without saying so.
- Keep core progression unaffected by events so non-participants are not behind.

## How to build it in Roblox
- Drive events with server time (os.time or workspace server time) and a config table with start, end, rewards. Avoid hard-coded dates inside scripts.
- Store event progress in a separate profile key that clears after the event; keep earned items permanently.
- Publish a preview or teaser 2 to 3 days earlier (banner in game, Discord post).
- Use feature flags to turn the event on and off without a republish ([[15-LiveOps/update-cadence]]).
- Use free decoration reuse: recolor existing assets and lighting instead of new models.

## Review after the event
- Participation rate, completion rate, D1 and D7 of players who joined the event versus others.
- Return rate in the week after the event (is there a hangover?).
- Bugs, exploits, economy inflation, community feedback.
- Decide: repeat, change, or retire. Log in [[15-LiveOps/weekly-review-template]].

## Mistakes
- Event too grindy for casual players; players quit mid-way.
- Rewards that break the economy.
- No end plan: a cliff the day after the event (see [[06-Retention/churn-signals-and-fixes]]).
- Silent launch, so nobody knows (see [[15-LiveOps/community-management]]).

## For our Roblox games
Copy the principle: single mechanic, visible timer, earned currency, fair catch-up. Never copy another game's event names, mascots, items, art, music or logos, and never fake scarcity.
