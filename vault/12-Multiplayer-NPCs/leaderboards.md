---
description: Build global and per-server leaderboards with OrderedDataStore, throttled writes, cached reads, in-server boards and anti-cheat; includes update intervals and request budgets.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: leaderboard, OrderedDataStore, GetSortedAsync, top 100, ranking, weekly reset, SurfaceGui, leaderstats, cache, request budget, global board, anti-cheat
sources: creator-docs content/en-us/production/game-design/liveops-essentials.md (read 2026-11-14); creator-docs content/en-us/production/game-design/core-loops.md (read 2026-11-14)
verify: OrderedDataStore method names and limits are from model knowledge, UNVERIFIED because the class pages are not in the local clone. Check the live DataStore limits table before choosing intervals.
tags: [multiplayer-npcs, playbook]
---
# Leaderboards

A leaderboard turns a number into a goal. Two kinds: the in-server `leaderstats` list (live, free, per server) and a global board backed by an `OrderedDataStore` (persistent, slower).

## Do this
- Show both. In-server `leaderstats` gives instant social proof; a global board gives a long-term target.
- Write to the global board rarely: on player leaving, on a milestone, or every 60 to 120 s at most per player. Never on every kill.
- Read the board on a timer (every 60 to 120 s), cache the result in a table, and let every display read the cache. Do not call `GetSortedAsync` per player or per frame.
- Wrap every DataStore call in `pcall` and retry with backoff (see [[13-Luau-Engineering/datastore-playbook]]).
- Store integers only. For time scores, store milliseconds or negate the value if lower is better.
- Add a period to the key for resets: `Wins_2026W46`. Weekly boards create fresh races; all-time boards are discouraged for new players because they can never catch up.
- Validate the score on the server before writing. Only server code writes to the store; clients never.
- Display top 10 on a world `SurfaceGui` plus the player's own rank. Own rank motivates more than a distant top.

## In-server leaderstats

```lua
--!strict
local Players = game:GetService("Players")

local function setupStats(player: Player)
	local folder = Instance.new("Folder")
	folder.Name = "leaderstats"

	local wins = Instance.new("IntValue")
	wins.Name = "Wins"
	wins.Value = 0
	wins.Parent = folder

	folder.Parent = player
end

Players.PlayerAdded:Connect(setupStats)
```

The folder must be named exactly `leaderstats` and be a child of the Player for the default list to show it.

## Global board refresh

```lua
--!strict
local DataStoreService = game:GetService("DataStoreService")

local store = DataStoreService:GetOrderedDataStore("Wins_2026W46")
local REFRESH_SECONDS = 90

type Row = { userId: number, score: number }
local cache: { Row } = {}

local function refresh()
	local ok, pages = pcall(function()
		return store:GetSortedAsync(false, 10)
	end)
	if not ok then
		warn("leaderboard read failed", pages)
		return
	end
	local rows: { Row } = {}
	for _, entry in pages:GetCurrentPage() do
		table.insert(rows, { userId = tonumber(entry.key) or 0, score = entry.value })
	end
	cache = rows
end

task.spawn(function()
	while true do
		refresh()
		task.wait(REFRESH_SECONDS)
	end
end)
```

Names: resolve user ids to names with `Players:GetNameFromUserIdAsync` in a `pcall`, and cache them. Never inside the loop for every row every refresh.

## Rewards and reset
- Pay top 3 at the period end with an in-game reward (cosmetic, boost). Never promise Robux or paid items.
- Run the payout on one server only (use a MemoryStore lock or a DataStore flag) so players are not paid twice.
- Announce the reset time on the board.

## Anti-cheat
- Cap per-update gains (plausible max per minute). Reject jumps beyond it and log.
- Compare to a server-tracked counter, not a client number.
- Remove banned users on read (skip rows whose userId is in a ban list).

## Mistakes
- Writing on every event and hitting request limits.
- Calling `GetSortedAsync` once per player join.
- Letting one value type (float) leak into an integer store.
- A top-only board, so 99% of players see no path to the list.

## Related
[[12-Multiplayer-NPCs/round-system-pattern]], [[13-Luau-Engineering/cooldowns-and-rate-limits]], [[12-Multiplayer-NPCs/multiplayer-patterns]], [[14-QA-Optimization/exploit-testing]]

## For our Roblox games
Copy the principle: cached reads, throttled writes, periodic seasons, own-rank display. Never copy another game's rank titles, trophy art or board layouts. Zero spend: OrderedDataStore is free.
