---
description: How to gather players in a lobby, start a match at the right size, and teleport them to a reserved server; covers queue timers, fill rules, teleport failure handling and a ready-check.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: matchmaking, lobby, queue, teleport, reserved server, TeleportService, party, ready check, minimum players, fill timer, TeleportResult, MemoryStore
sources: creator-docs content/en-us/resources/battle-royale/installation-and-setup.md (lobby place plus queue place pattern, read 2026-11-14); creator-docs content/en-us/reference/engine/enums/TeleportResult.yaml (read 2026-11-14); creator-docs content/en-us/reference/engine/enums/MatchmakingType.yaml (read 2026-11-14)
verify: Enum names verified in local docs. TeleportService, ReserveServer and MemoryStoreService method signatures are NOT in the local clone: UNVERIFIED, written from model knowledge, check the live API reference before shipping. Timer numbers are design defaults, UNVERIFIED as optimal.
tags: [multiplayer-npcs, playbook]
---
# Matchmaking and lobbies

A lobby is a waiting room that turns scattered players into a match. Two shapes work: an in-server lobby (same server, players walk to a pad) and a separate lobby place that teleports groups to a match server.

## Do this
- Pick the shape by match size. Under 12 players per match: run the lobby and the match in the same server and just reset the map. Over 12 or when matches need a fresh map: lobby place plus reserved match servers.
- Start rule: start when `players >= minPlayers` AND the fill timer ends. Defaults: min 2, fill timer 20 s, hard cap on the timer 45 s. Reset the timer to 10 s when the match becomes full.
- Show the queue state to everyone: count, needed, seconds left. A silent queue reads as a broken game.
- Make the server choose the group. The client may only say "join queue" or "leave queue". Never let a client send the list of teammates or a place ID.
- Teleport groups with one `TeleportAsync` call using a `TeleportOptions` with a reserved server code, never one call per player. Wrap it in `pcall` and retry once after 2 s.
- Handle every `Enum.TeleportResult` that is not `Success`: `GameFull` and `Flooded` mean retry later with backoff, `Unauthorized` means drop the player from the group and continue.
- Carry only small data across teleport: a match id, team assignment, mode. Load real data (inventory, currency) in the destination from DataStores, not from teleport data (a client can tamper with what it holds).
- Always let a solo player play something while waiting: bots or a practice dummy. See [[12-Multiplayer-NPCs/bots-to-fill-servers]].

## Queue model
One table on the server, keyed by `Player`. Remove on `PlayerRemoving`. Never keep a dead player in the count.

```lua
--!strict
local Players = game:GetService("Players")

local MIN_PLAYERS = 2
local MAX_PLAYERS = 8
local FILL_SECONDS = 20

local queue: { Player } = {}
local fillDeadline: number? = nil

local function removeFromQueue(player: Player)
	local index = table.find(queue, player)
	if index then
		table.remove(queue, index)
	end
	if #queue < MIN_PLAYERS then
		fillDeadline = nil
	end
end

local function addToQueue(player: Player)
	if table.find(queue, player) then
		return
	end
	table.insert(queue, player)
	if #queue >= MIN_PLAYERS and fillDeadline == nil then
		fillDeadline = os.clock() + FILL_SECONDS
	end
end

local function shouldStart(): boolean
	if #queue >= MAX_PLAYERS then
		return true
	end
	return fillDeadline ~= nil and os.clock() >= fillDeadline
end

Players.PlayerRemoving:Connect(removeFromQueue)
```

A single loop with `task.wait(1)` checks `shouldStart()`. One loop, not one thread per queue entry.

## Teleport a group

```lua
--!strict
local TeleportService = game:GetService("TeleportService")

local MATCH_PLACE_ID = 0 -- set to your match place id

local function sendGroup(group: { Player }, mode: string): boolean
	local code = TeleportService:ReserveServer(MATCH_PLACE_ID)
	local options = Instance.new("TeleportOptions")
	options.ReservedServerAccessCode = code
	options:SetTeleportData({ mode = mode, size = #group })
	for attempt = 1, 2 do
		local ok, err = pcall(function()
			TeleportService:TeleportAsync(MATCH_PLACE_ID, group, options)
		end)
		if ok then
			return true
		end
		warn("teleport failed", attempt, err)
		task.wait(2)
	end
	return false
end
```

If it returns false, put the group back in the queue and tell them. Do not lose them.

## Ready check (optional)
- Needed only for modes where a late player ruins the match (ranked, 4v4).
- Send each queued player a "Match found" prompt for 10 s. Accept removes nothing. Decline or timeout removes that player and re-queues the rest.
- Do not punish declines with penalties on a casual game.

## Why it works
Short, visible waits keep players in the server. Players leave when they see nothing happening for more than about 30 s (UNVERIFIED rule of thumb). A fill timer trades match quality against wait time; the cap stops it from stalling.

## Mistakes
- Teleporting inside `PlayerAdded` before data loaded.
- Using `Players:GetPlayers()` as the queue (includes players who never opted in).
- One reserved server per player (splits the group).
- Trusting `TeleportData` for currency or rank.
- No retry path, so one `Flooded` result strands the group.

## Related
[[12-Multiplayer-NPCs/round-system-pattern]], [[12-Multiplayer-NPCs/teams-and-parties]], [[12-Multiplayer-NPCs/server-size-and-streaming]], [[12-Multiplayer-NPCs/multiplayer-patterns]], [[13-Luau-Engineering/datastore-playbook]]

## For our Roblox games
Copy the principle: server-owned queue, visible countdown, fill timer with a cap, grouped teleport with retry. Never copy another game's lobby layout, mode names, UI art, music or branding. Zero spend: use only free built-in services (TeleportService, optionally MemoryStoreService within free quotas), no paid matchmaking tools.
