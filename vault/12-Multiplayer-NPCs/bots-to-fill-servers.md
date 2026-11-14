---
description: Fill empty or small servers with bots so solo players still get a lively match: when to add and remove bots, honest labelling, difficulty scaling, and server-side bot cost limits.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: bots, fill servers, empty server, low population, AI players, difficulty scaling, bot replace, backfill, NPC players, honest labels, pathfinding
sources: creator-docs content/en-us/characters/pathfinding.md (read 2026-11-14); creator-docs content/en-us/production/game-design/design-for-roblox.md (read 2026-11-14)
verify: Design guidance from model knowledge, UNVERIFIED as a retention lever for any specific genre. Check Roblox Community Standards on deceptive bots before shipping; this note assumes bots are labelled.
tags: [multiplayer-npcs, playbook]
---
# Bots to fill servers

Empty lobbies make new players leave. Bots give a match a minimum shape until real players arrive, then step aside.

## Do this
- Set a target population per match (for example 8). Bots fill `target - realPlayers`, up to a cap of 75% of slots so real players always matter.
- Label bots. Give them a visible tag (name prefix or icon) and a real-looking but clearly non-player name list. Do not pass them off as people; deceptive bots break trust and may break platform rules (check current rules).
- Add bots only at round start. Remove one bot per joining player at the next safe moment (death, respawn or round end), never by deleting a bot mid-fight.
- Scale difficulty to the real players: start bots slightly below the median real player skill (accuracy 60%, reaction 0.4 s, defaults UNVERIFIED), raise or lower by win rate over the last 3 rounds.
- Cap total active bots per server (default 12) and total server NPC think cost (see budget below).
- Bots should share the same rules as players: same speed, damage, cooldowns. Do not give them extra health to compensate for bad AI.
- Give bots flaws on purpose: aim error, reaction delay, occasional wrong target. Perfect bots feel unfair.
- Bots never earn leaderboard credit, trade, or count toward matchmaking queue size (see [[12-Multiplayer-NPCs/leaderboards]]).

## Backfill logic

```lua
--!strict
local Players = game:GetService("Players")

local TARGET = 8
local MAX_BOT_RATIO = 0.75

local bots: { Model } = {}

local function desiredBots(): number
	local real = #Players:GetPlayers()
	local want = TARGET - real
	local cap = math.floor(TARGET * MAX_BOT_RATIO)
	return math.clamp(want, 0, cap)
end

local function spawnBot(): Model
	local bot = Instance.new("Model")
	bot.Name = "Bot"
	bot:SetAttribute("IsBot", true)
	bot.Parent = workspace
	return bot
end

local function reconcile()
	local want = desiredBots()
	while #bots < want do
		table.insert(bots, spawnBot())
	end
	while #bots > want do
		local bot = table.remove(bots) :: Model
		bot:Destroy()
	end
end
```

Call `reconcile()` at round start and when players join or leave during `Intermission`. Replace the bare `Model` with your real bot rig; the attribute `IsBot` lets every system skip bots.

## AI cost budget
- Run one shared bot-brain loop (for example 10 Hz), not a `while true` per bot. Steps are staggered so only about a third of bots think each tick.
- Path recompute no more than every 0.5 s per bot, and only when the target moved more than 4 studs (see [[12-Multiplayer-NPCs/npc-ai]]).
- Set network ownership of bot rigs to the server so physics does not hop between clients: `part:SetNetworkOwner(nil)` on the root part.
- Disable bot collisions with each other if it causes jams (collision groups).

## Why it works
New players meet a populated world instantly. Soloists get practice, and the bot-to-player swap hides low population from the matchmaker (UNVERIFIED effect size).

## Mistakes
- Bots with infinite accuracy and no reaction time.
- Counting bots in DataStore stats or quests.
- Spawning all bots in one frame (hitch). Spread over 5 to 10 frames with `task.wait()`.
- Removing a bot that a real player is fighting.
- Hiding that they are bots.

## Related
[[12-Multiplayer-NPCs/npc-ai]], [[12-Multiplayer-NPCs/matchmaking-and-lobbies]], [[12-Multiplayer-NPCs/server-size-and-streaming]], [[14-QA-Optimization/lag-sources-ranked]]

## For our Roblox games
Copy the principle: labelled bots, low-skill start, cap by ratio, one shared brain loop. Never copy another game's bot names, avatars, voice lines or behavior scripts. Zero spend: use Roblox built-in rigs and PathfindingService only.
