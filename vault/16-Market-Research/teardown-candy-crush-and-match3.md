---
description: Why Candy Crush and match-3 games keep millions playing: lives, level difficulty curves, boosters, and live ops, with numbers for a Roblox puzzle or level-based game.
type: teardown
status: draft
last_checked: 2026-11-14
keywords: candy crush, match 3, lives, energy, difficulty curve, levels, boosters, blockers, live ops, king, soft currency, near miss
sources: https://www.pocketgamer.biz/crafting-candy-crushs-difficulty-blockers-level-design-ai-and-the-complexity-staircase (search result read 2026-11-14, snippet only); https://mobilegamer.biz/how-king-defines-a-good-candy-crush-saga-level-and-why-it-constantly-prunes-the-bad-ones/ (snippet only); https://developer.microsoft.com/en-us/games/articles/2026/05/art-and-science-of-evergreen-games-minecraft-candy-crush/ (snippet only); GDC session "Blockers: Analyzing Difficulty Drivers in Candy Crush Games" (title)
verify: UNVERIFIED: life count, refill timer and level numbers are from memory and change over time. Article bodies were not read in full.
tags: [market-research, teardown]
---
# Teardown: Candy Crush and match-3
A casual puzzle game on a tuned difficulty curve with a soft limit on play (lives) and thousands of levels.

## Do this
- Start with 5 to 10 very easy levels (win rate over 90 percent) that teach one new idea each. Teach by doing, with a pointing hand.
- Raise difficulty in a staircase: a new element appears, a few easy levels let players learn it, then it joins harder levels. Do not raise every variable at once.
- Insert a hard level every 8 to 12 levels (win rate 25 to 40 percent) followed by 2 to 3 easy ones for relief.
- Track win rate and attempts per level; flag any level where more than half of players quit after 5 tries. Fix or remove it. King reports it prunes bad levels (see sources).
- Use "near miss" design: the player should often finish with 1 to 3 moves worth of progress short, so a retry feels one tap away. Do not fake it; tune real levels.
- Offer a limited lives system only if you can give a free fallback: 5 lives, one regenerates every 20 to 30 minutes, ask a friend or watch the timer. Never block the first session.
- Boosters: earn some free from play; sell the rest as convenience. Never gate progress behind a spend.

## Why it got huge
- Simple one-finger match rule, fast results, bright reward feedback.
- Long level list and a story map made progress visible.
- Social features (friend leaderboards, sending lives) used existing networks.
- Heavy testing and data on every level.

## Why people play
- Quick wins in 1 to 3 minutes; sound and cascades feel good (see [[09-Animation-VFX/game-feel-juice]]).
- Light problem solving with no fail penalty beyond a life.
- Relaxation while waiting.

## Why they stay
- Level map and unlock gates create a visible path. Episode ends create goals.
- Blockers and new mechanics refresh the puzzle every few weeks.
- Variable rewards from cascades and special pieces. See [[03-Game-Psychology/reward-schedules]].

## Why they come back
- Lives refill: a timer is a built-in reminder. Use it gently.
- Daily bonuses and timed events. See [[03-Game-Psychology/streaks-and-daily-rewards]].
- New level packs weekly.

## Economy shape (suggested starting values, UNVERIFIED vs real game)
| Piece | Starting idea |
|---|---|
| Lives | 5, refill 25 min each |
| Soft coin per level | 10 to 30 |
| Booster price | 100 to 300 coins |
| Free booster per episode | 2 to 3 |
| Level win rate target | 60 percent average, range 20 to 95 |

## Roblox build notes
- Store only level number and booster counts; levels as data tables so you can tune without code changes.
- Log level, attempts, win with AnalyticsService (see roblox-analytics) to find spikes.
- Lives timer: save os.time of last refill on the server; compute regeneration on join so no cheating by changing the client clock.
- Avoid paid random items (policy); sell booster bundles with fixed contents.

## Mistakes
- Hard walls that push spending: they hurt trust, and Roblox kids will quit.
- Lives that end the whole session at minute 3.
- Too many boosters that trivialize levels.

## For our Roblox games
Copy: staircase difficulty, relief levels, data-pruned levels, soft lives with free fallback, fixed-content boosters. Never copy: candy theme, characters, names, level layouts, audio. Related: [[03-Game-Psychology/player-psychology]].
