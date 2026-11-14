---
description: Survival playbook for Roblox: needs meters, gather-craft-build loop, danger curve across day and night, base building limits, and server-persistence rules for a fair, low-grind survival game.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: survival, roblox survival, crafting, gather, hunger, thirst, day night cycle, base building, resource, wilderness, persistence, grind
sources: Author working knowledge of survival game design (no pages opened 2026-11-14); Roblox Creator Hub docs, DataStore and StreamingEnabled pages (titles only, not opened 2026-11-14)
verify: General design practice. Timings are starting points to playtest. UNVERIFIED: genre popularity or earnings on Roblox right now.
tags: [game-design, playbook]
---
# Survival playbook

Core loop: gather, craft, build, survive the threat, expand to the next biome. Pressure comes from needs and night, not from punishment.

## Do this
- Keep needs to 2 or 3 meters (health, hunger, stamina or warmth). Each drains to empty in 8-15 minutes, so one session has real decisions but no constant chores.
- First 3 minutes: punch a tree, craft one tool, see a resource node and a hint of a threat. Reward the first craft in under 90 s.
- Use a day and night cycle of 10-20 minutes. Day is gather, night is threat. See [[02-Game-Design/loop-stack]].
- Tier tools (wood, stone, iron) so each tier opens a new biome or node. Show the next tier recipe early as a goal.
- Make crafting recipes 2-4 inputs with an on-screen list; no wiki needed.
- Limit base building by a part or budget cap per player, such as 500 parts, so servers stay fast.
- Death rule: drop only a share of items (for example the backpack, not the hotbar) so losing is a setback, not a wipe. See [[03-Game-Psychology/loss-aversion-used-fairly]].
- Add a co-op reason: bigger crafts or defenses need 2 or more players.

## Why it works
- Clear escalating goals (tier up, bigger base) give long sessions and return reasons.
- Needs create small urgent choices without requiring skill.
- Base building gives ownership and a flex surface for friends. See [[03-Game-Psychology/social-status-and-flex]].

## How to build it in Roblox
- Resource nodes: server-spawned with respawn timers (60-300 s); store node state on the server only.
- Inventory: table per player, replicate a copy to the client UI; validate every craft on the server.
- Saves: DataStore for inventory and base layout, saved on leave and every 60-120 s; serialize placed parts as compact data (type, CFrame), not instances.
- Threats: simple NPCs with PathfindingService; scale spawn count to player count.
- Enable StreamingEnabled for big maps; keep spawn points near water and trees.
- Weather and day cycle: tween Lighting over time, one script, replicated by time value.

## Numbers (playtest starting points)
- Gather rate: a stone tool yields about 3x the wood yield per hit.
- Tier-up every 20-40 min of play at launch.
- Base part cap 300-800; respawn 5-10 s.
- Server size 8-20 players for a map that does not feel empty.
- Hunger drain 1 point per 6-10 s on a 100 scale.

## Mistakes
- Needs that drain so fast they feel like chores; players quit during the tutorial.
- Full inventory loss on death with no protection for newbies; add a 5-minute spawn shield.
- Unlimited base parts causing lag.
- PvP everywhere on day one; let players choose a PvE or safe zone. See [[02-Game-Design/pvp-fairness-and-balance]].
- No ending goal; add a boss or final biome for the first month.

## Retention hooks
- Daily resource refresh that rewards logging in without forcing it.
- Seasonal biome or event with a limited free craft recipe.
- Shared base mode: a group can combine part budgets and decorate together.

## Playtest check
- Time how long until a new player crafts a first tool. Over 2 minutes means the opening hint is missing.
- Check server frame rate at the part cap with 12 players.

## For our Roblox games
Copy the loop shape: gather, craft, tier, night threat, capped building, partial death loss. Never copy another game's creature designs, item names, map layouts, UI art, or recipes verbatim. Name and model your own tiers and biomes.
