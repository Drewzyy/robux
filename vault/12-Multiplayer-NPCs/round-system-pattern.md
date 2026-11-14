---
description: Server-side round loop (intermission, map load, play, results, cleanup) with exact timings, win conditions, late-join rules and a typed Luau skeleton that cannot get stuck.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: round system, game loop, intermission, round timer, map vote, late join, win condition, spawn, results screen, state machine, cleanup, attributes
sources: creator-docs content/en-us/resources/battle-royale/core-scripts.md (state-driven game modes, read 2026-11-14); creator-docs content/en-us/production/game-design/core-loops.md (read 2026-11-14)
verify: Pattern is standard practice, written from model knowledge. Timings are design defaults, UNVERIFIED as optimal; tune with your own session-length analytics.
tags: [multiplayer-npcs, playbook]
---
# Round system pattern

A round game is one server loop that walks through fixed phases. Clients only display the phase; the server decides every transition.

## Do this
- Use five phases: `Intermission`, `Loading`, `Playing`, `Results`, `Cleanup`. Store the current phase and end time in Workspace or ReplicatedStorage attributes so clients can render a countdown without remotes.
- Default timings: intermission 15 s, loading 3 s, play 120 to 240 s (shorter for casual), results 8 s. A full cycle under 5 minutes keeps "one more round" cheap.
- Start the clock from `workspace:GetServerTimeNow()` and send the end timestamp, not "seconds left". Clients compute the remainder, so lag does not desync the timer.
- Every phase must have a timeout and an exit when players drop below the minimum. A round that waits forever is the number one stuck-server bug.
- Decide the winner on the server from server-tracked state (score, last alive). Never accept "I won" from a client.
- Late joiners: spawn them as spectators in `Playing`, full participants in `Intermission`. Do not drop them into a live round mid-map.
- Players who die or leave stay in a `participants` set that you edit on `PlayerRemoving`; check win condition each time it changes.
- Cleanup always runs: destroy the map clone, reset player state, disconnect round connections (see [[13-Luau-Engineering/memory-leak-checklist]]).

## Skeleton

```lua
--!strict
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

type Phase = "Intermission" | "Loading" | "Playing" | "Results" | "Cleanup"

local MIN_PLAYERS = 2
local DURATIONS: { [Phase]: number } = {
	Intermission = 15,
	Loading = 3,
	Playing = 180,
	Results = 8,
	Cleanup = 1,
}

local participants: { [Player]: boolean } = {}

local function setPhase(phase: Phase)
	ReplicatedStorage:SetAttribute("Phase", phase)
	ReplicatedStorage:SetAttribute("PhaseEnds", workspace:GetServerTimeNow() + DURATIONS[phase])
end

local function aliveCount(): number
	local n = 0
	for _ in participants do
		n += 1
	end
	return n
end

Players.PlayerRemoving:Connect(function(player)
	participants[player] = nil
end)

local function runRound()
	setPhase("Intermission")
	task.wait(DURATIONS.Intermission)
	if #Players:GetPlayers() < MIN_PLAYERS then
		return
	end

	setPhase("Loading")
	for _, player in Players:GetPlayers() do
		participants[player] = true
	end
	task.wait(DURATIONS.Loading)

	setPhase("Playing")
	local deadline = os.clock() + DURATIONS.Playing
	while os.clock() < deadline and aliveCount() > 1 do
		task.wait(0.5)
	end

	setPhase("Results")
	task.wait(DURATIONS.Results)

	setPhase("Cleanup")
	table.clear(participants)
	task.wait(DURATIONS.Cleanup)
end

while true do
	local ok, err = pcall(runRound)
	if not ok then
		warn("round failed", err)
		table.clear(participants)
		task.wait(3)
	end
end
```

The `pcall` is the safety net: any error inside a phase resets the loop instead of killing it.

## Client side
- Read `Phase` and `PhaseEnds` with `GetAttributeChangedSignal`. Render the remaining time from `workspace:GetServerTimeNow()`.
- Play phase-change sound and UI banner locally only; no gameplay logic.

## Win conditions (pick one per mode)
- Last alive: end when `aliveCount() <= 1`.
- Score race: first to N, with the timer as tiebreak.
- Objective: team holds point or carries item for X seconds, tracked on the server.
- Survive: end when the timer runs out and at least one player lives.

## Mistakes
- Using `wait()` loops per player instead of one authoritative loop.
- Starting the round and then a player leaves, so the round never ends.
- Rewarding in `Results` without checking the player is still in the server.
- Not clearing map state, so round 2 starts with leftovers.
- Granting wins from client remotes.

## Related
[[12-Multiplayer-NPCs/matchmaking-and-lobbies]], [[13-Luau-Engineering/state-machines]], [[12-Multiplayer-NPCs/enemy-waves-and-spawners]], [[12-Multiplayer-NPCs/leaderboards]]

## For our Roblox games
Copy the principle: server-owned phase loop with timeouts, timestamps over countdowns, late-join as spectator. Never copy another game's mode names, map layouts, win banners, jingles or UI skins. Zero spend: all of this is plain Luau, no paid plugins.
