---
description: Five nested loops (core, session, meta, social, long-term), how each hands off to the next, hit-game examples, and a 60-second test to find which loop is broken.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: core loop, session loop, meta loop, social loop, long-term loop, loops and arcs, nested loops, interaction loop, skill chain, retention, pacing, playtest, 60 second test
sources: Daniel Cook, Lostgarden, "Loops and Arcs" and skill-chain slides, via search summary https://mechanicsofmagic.com/2026/05/13/sketchnote-loops-arcs-51/ (read 2026-11-14, secondary); Michael Sellers, Advanced Game Design (loops chapter), https://www.oreilly.com/library/view/advanced-game-design/9780134668185/ch7.xhtml (seen in search 2026-11-14, not opened); existing vault note core-loop-template
verify: Loop names and timings are a working convention, not an industry standard. Cook's primary essays were NOT opened (search returned secondary summaries). Hit-game examples are from general knowledge, UNVERIFIED in detail.
tags: [game-design, playbook]
---
# Loop stack

A game is several loops nested like gears. Each loop is a cycle of action, feedback, reward that feeds the next, slower one. [[02-Game-Design/core-loop-template]] fills in the table; this note says how the loops connect and how to test them.

## Do this
- Name all five loops in the GDD, each with a cycle time: core 5-30 s, session 5-15 min, meta (tomorrow/this week) 1-7 days, social any time a friend is present, long-term 2-8 weeks.
- Every loop must output something the next loop up consumes. Core outputs currency; session spends it on an upgrade or unlock; meta turns upgrades into a new zone or collection page; long-term turns collection into status.
- Build and test bottom-up. Do not write meta or social loops until the core loop passes the 60-second test below.
- Give each loop its own visible meter (coin counter, session goal bar, collection index, friend list/leaderboard, rank). One missing meter means the loop is invisible to players.
- Each loop must end on a visible payoff and expose the next open loop (Zeigarnik: unfinished goals pull people back).
- Never let a slower loop starve a faster one: meta rewards must make the core verb feel better (stronger, faster, new), not replace it.
- Log one analytics event per loop completion (core_loop_done, session_goal_done, collection_page_done) so a funnel shows which loop breaks ([[06-Retention/event-taxonomy]]).

## The five loops
| Loop | Cycle | Player question | Typical output | Break symptom |
|---|---|---|---|---|
| Core | 5-30 s | What do I do right now? | currency, XP, progress tick | players idle or ask "what now" |
| Session | 5-15 min | What am I finishing this visit? | one upgrade, one zone clear, one level | quit at minute 3-5 with no goal |
| Meta | 1-7 days | Why come back tomorrow? | timer, daily reward, restock, new unlock | D1 low |
| Social | variable | Who do I show or play with? | trade, co-op bonus, gifts, leaderboard rank | solo-only, no invites |
| Long-term | weeks | Who am I in this game? | rank, rebirth tier, index completion, title | D30 low, endgame empty |

## How they nest
```
long-term  [ meta  [ session  [ core core core ] -> upgrade ] -> unlock ] -> status
social loop crosses all of them: friends share the core, trade the meta, compare the long-term.
```
- Cook's framing (secondary source): loops build mastery by repetition; arcs deliver authored content and emotion. A good game wraps loops in arcs (zones, story beats, seasons). Use arcs for the meta and long-term layers, loops for core and session.
- Cook's skill-chain view: a loop is a chain of skills; if a link is broken (player cannot read feedback, cannot do the verb), every loop above it fails. Fix the lowest broken link first.

## Examples (general knowledge, UNVERIFIED specifics)
- Simulator style (click, collect, sell, upgrade, rebirth): core = collect; session = fill the bag and sell; meta = new zone/pet egg; long-term = rebirth tier. Social = pet trading.
- Obby: core = jump a section; session = reach next checkpoint stage band; meta = daily/skip/trails; long-term = tower completion time.
- Pet/trade game: core = a chore task; session = hatch/age an item; social = trade; long-term = rarest item as status.
- Farming/garden: core = plant/harvest; session = one crop timer cycle; meta = restock timer; social = gifting and visiting.

## 60-second loop test
1. Join a fresh server with a stopwatch. Do not read tutorial text.
2. 0-5 s: is the verb clear and does it already give feedback (sound, number, motion)?
3. 5-30 s: has a reward landed that you can name? If not, the core loop is empty.
4. 30-60 s: can you see the next goal and how far you are from it (bar, price tag, locked door)? If not, the session loop is missing.
5. Pass: three yes answers with no help. Fail: fix before building anything slower.
6. Repeat with a first-time player who cannot see your screen; note where they hesitate over 3 seconds.

## Luau: loop event logging (server)
```lua
--!strict
local AnalyticsService = game:GetService("AnalyticsService")

local function logLoop(player: Player, loopName: string, count: number)
	-- custom event; keep names stable and low-cardinality
	AnalyticsService:LogCustomEvent(player, "LoopDone_" .. loopName, count)
end
return logLoop
```
Check the current AnalyticsService signature in Creator Docs before use (UNVERIFIED; docs host was unreachable this session).

## Mistakes
- Building a big meta loop on top of a core loop that is not fun by itself.
- Session goals that take over 15 minutes (median session is about 9.8 min per [[02-Game-Design/core-loop-template]]).
- A social loop that requires friends online to progress at all; make it a bonus.

## For our Roblox games
Copy the structure: five named loops, one meter each, bottom-up testing. Never copy another game's names, characters, pets, art, music, UI skins or logos; reuse only the abstract cycle shape.
