---
description: Return triggers ranked by strength (friends, timers, updates, collections, social status, daily rewards) with evidence quality for each and how to build the top ones in Roblox.
type: reference
status: draft
last_checked: 2026-11-14
keywords: retention, return triggers, D1, D7, D30, friends, timers, updates, collections, social status, habit, notifications, daily reward, why players return, Self-Determination Theory
sources: Ryan, Rigby and Przybylski, The Motivational Pull of Video Games: A Self-Determination Theory Approach, Motivation and Emotion 2006 (from memory, not opened this session); Zeigarnik 1927 (from memory); Roblox Creator Docs on retention and notifications (not reachable 2026-11-14); existing vault notes player-psychology, play-with-friends, streaks-and-daily-rewards, benchmarks
verify: The RANKING is our judgment from design literature and common practice, UNVERIFIED by our own data and by Roblox-published figures. Check against our D1/D7/D30 per game ([[06-Retention/analytics-reading-guide]]). No player-count or revenue claims made.
tags: [game-psychology, reference]
---
# Why players return

Players come back because something external reminds them, a goal is open, or someone is waiting. Ranked by how strongly each tends to pull, with evidence quality noted. Related: [[03-Game-Psychology/player-psychology]], [[03-Game-Psychology/play-with-friends]], [[03-Game-Psychology/streaks-and-daily-rewards]].

## Do this
- Build triggers in rank order: friends, then timers, then updates, then collections, then status. Cover at least three in game 1.
- Friends: add join-friend prompts, shared rewards for playing together, and a visible friends-online list.
- Timers: at least one running timer a player starts before leaving (crop, egg, build, cooldown) of 20 min to 24 h, and show it on join.
- Updates: ship a visible update on a fixed cadence (every 1-2 weeks small, monthly big) with an in-game news panel.
- Collections: an index with empty slots and a completion reward; show "12/60".
- Status: leaderboards among friends, titles, rarest-item display, rebirth tier visible on the character.
- Measure each trigger by D1, D7 return and by event (timer_collected, friend_joined) ([[06-Retention/event-taxonomy]]).
- Do not use guilt, loss threats against kids, or paid FOMO to pull people back.

## Ranked triggers
| Rank | Trigger | Mechanism | Evidence quality | Build in Roblox |
|---|---|---|---|---|
| 1 | Friends | relatedness; obligation and fun with known people | Strong in theory (SDT relatedness, Ryan et al. 2006); platform behaviour UNVERIFIED here | friend join, party rewards, friends-first leaderboard |
| 2 | Timers and unfinished goals | Zeigarnik open loop; fixed-interval reward | Strong in theory; effect size in games UNVERIFIED | timestamps in the saved profile, not server countdown |
| 3 | Updates and events | novelty, new goals reset boredom | Practitioner consensus; UNVERIFIED metrics | cadence calendar, limited-time free events |
| 4 | Collections | completion drive, endowed progress | Moderate (goal gradient studies, see [[05-Progression/goal-gradient-and-session-goals]]) | index UI, set bonuses |
| 5 | Social status | competence plus recognition | Moderate in theory | titles, leaderboards, trophies, flex items |
| 6 | Daily rewards and streaks | habit, loss aversion | Mixed; can annoy | [[03-Game-Psychology/streaks-and-daily-rewards]] |

Why friends rank first: relatedness is one of the three needs in SDT (autonomy, competence, relatedness). A friend waiting in a server is an immediate, personal reason that no timer matches. Why timers beat daily rewards: a timer the player started is a goal they own; a daily reward is a rule we set.

## Luau: welcome-back timer check
```lua
--!strict
type Timer = { id: string, readyAt: number }

local function readyTimers(timers: {Timer}, now: number): {string}
	local out: {string} = {}
	for _, t in timers do
		if now >= t.readyAt then table.insert(out, t.id) end
	end
	return out
end
-- on join: local ready = readyTimers(profile.timers, os.time())
-- show "2 things are ready" as the first UI after load
```
Use `os.time()` (UTC seconds) so it works across servers and offline.

## How the triggers stack in a week
- Day 0: first timer set before the player leaves; one friend invite shown.
- Day 1: timer ready on join; one new collection slot filled; friends list visible.
- Day 3-7: update lands; second tier of collection; leaderboard rank change.
- Day 14+: status goals (rank, rebirth tier).
Low D1 suggests missing timers or friends; low D7 weak progression; low D30 missing endgame status ([[06-Retention/decision-tree]]).

## Mistakes
- Relying on a single trigger (only daily rewards).
- Timers that need the player to stay online.
- Updates with no player-facing hook (news panel, event area).
- Dark patterns: fake urgency, shame, spend pressure. Keep it honest for a young audience.

## For our Roblox games
Copy: the set of triggers and their order. Never copy another game's event names, cosmetics, characters, jingles, or UI art. Verify the ranking with our own cohort data after launch.
