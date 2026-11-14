---
description: How to keep Roblox PvP fair: matchmaking, power caps, counterplay, spawn rules, anti-grief, pay-to-win limits, and a balance review loop with win-rate targets.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: pvp, fairness, balance, matchmaking, pay to win, win rate, counterplay, spawn protection, ranked, anti grief, exploits, handicap
sources: Author working knowledge of game design practice (no pages opened 2026-11-14); Roblox Creator Hub docs (create.roblox.com/docs, titles only, not opened 2026-11-14)
verify: Targets such as 45-55 percent win rate are common practice, UNVERIFIED as a Roblox rule. Re-measure on live data.
tags: [game-design, checklist]
---
# PvP fairness and balance

Players forgive losing a fair fight and never forgive a rigged one. Use this checklist before launching any PvP mode.

## Do this
- Match by skill or progress band, not only by server luck. Start with plus or minus 100 rating, widen over time.
- Cap power gaps: gear or level difference above 20 percent should not be matched; or scale stats in PvP modes.
- Never sell raw power. Sell cosmetics, convenience, and speed of collection, not extra damage. See [[05-Progression/pets-and-gear-tiers]].
- Give every offense a counter: block, dodge, range, cover, or a timing window.
- Protect new players: 10 minutes of protection or a safe tutorial arena.
- Spawn fairly: 2-3 s spawn shield, away from enemy lines.
- Limit stun and crowd control chains to 1.5-2 s total, then immunity of 3 s.
- Run a weekly review: win rate per weapon or class, kills per minute, time to kill.

## Why it works
- Fairness is the base of long-term PvP retention.
- Clear rules reduce reports and moderation load.
- Cosmetic monetization keeps skill meaningful and flex available. See [[03-Game-Psychology/social-status-and-flex]].

## Balance review loop
1. Collect win and pick rates for 7 days (at least 500 matches per option when possible).
2. Flag anything above 55 percent win with high pick, or below 40 percent.
3. Change one variable (damage, cooldown, range) by 5-10 percent.
4. Re-measure for another week; keep a public changelog.

## How to build it in Roblox
- Server authoritative combat; reject impossible speed, reach, and fire rate.
- Ranked queue via MemoryStoreService or a simple lobby queue; teleport to reserved servers.
- Anti-grief: team damage off by default, spawn camping detection (kills within 5 s of spawn flagged), and a report button.
- Log match results to DataStore for later review.
- Add a rematch or surrender option to cut frustration.

## Numbers (playtest starting points)
- Target win rates 45-55 percent per option.
- Spawn protection 2 s; respawn 3-5 s.
- Rating change per match about plus or minus 15-25 for a close match.
- Matchmaking wait cap 30 s, then fill with bots or widen the band.
- New player protection 10 min or first 5 matches.

## Mistakes
- Pay-to-win items that change damage or defense.
- Single meta weapon for months.
- Matching a level 5 with a level 80 because the queue was empty.
- Allowing spawn camping or kill-farming alts.
- Ignoring mobile players' input disadvantage; add aim assist and larger buttons.

## Playtest check
- Review 20 reports; if over a quarter are about spawns or one weapon, fix that before anything else.
- Test queue behavior with 2 players and with 20.
- Try the game as a new account for 10 minutes to feel the protection window.

## For our Roblox games
Copy the process: banded matching, no paid power, counterplay for every move, weekly numbers review. Never copy another game's rank names, badges, or balance values wholesale. Tune from your own match data.
