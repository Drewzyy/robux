---
description: Wave-based enemy spawning with budgets, spawn point rules, alive caps, difficulty curves and cleanup; includes a typed wave runner and scaling for solo versus full servers.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: enemy waves, spawner, wave system, spawn budget, alive cap, difficulty curve, horde, tower defense, CollectionService, spawn points, scaling, cleanup
sources: creator-docs content/en-us/resources/battle-royale/the-storm.md (timed escalating phases, read 2026-11-14); creator-docs content/en-us/characters/pathfinding.md (read 2026-11-14); creator-docs content/en-us/production/game-design/core-loops.md (read 2026-11-14)
verify: Numbers are design defaults from model knowledge, UNVERIFIED as optimal. Verify enemy count limits with the MicroProfiler on a target low-end device.
tags: [multiplayer-npcs, playbook]
---
# Enemy waves and spawners

A wave is a batch of enemies released over time. The server owns spawning, counts and cleanup. The goal: constant pressure that fits the player count and device budget.

## Do this
- Define each wave as data: a list of `{ kind, count, interval }` plus a total enemy budget. Waves live in a ModuleScript, not inside the loop.
- Use a budget, not raw counts. Each enemy kind has a cost (grunt 1, runner 2, brute 5). Wave budget = `base + wave * growth` (start base 8, growth 3, UNVERIFIED defaults).
- Scale by players: `budget * (0.6 + 0.4 * playerCount)` so a solo player is not overrun and a full server is not bored.
- Cap alive enemies: 25 on mobile-heavy games, 40 on desktop-leaning ones (UNVERIFIED defaults). When at the cap, queue spawns; never skip them.
- Spawn at tagged parts (`CollectionService` tag `EnemySpawn`), outside every player's view and at least 40 studs from any player. Pick the point furthest from players that passes the rule.
- Stagger spawns: 0.3 to 1.5 s between enemies, never a whole wave in one frame.
- Intermission between waves of 8 to 12 s, with a visible countdown and a small reward. Boss every 5th or 10th wave (see [[12-Multiplayer-NPCs/boss-fight-patterns]]).
- Wave ends when alive == 0 and the queue is empty, or after a hard timeout (90 s) that despawns leftovers.
- Always clean up: destroy enemies on wave end, round end and server shutdown.

## Wave runner

```lua
--!strict
local CollectionService = game:GetService("CollectionService")
local Players = game:GetService("Players")

type EnemyKind = { name: string, cost: number }
local KINDS: { [string]: EnemyKind } = {
	grunt = { name = "grunt", cost = 1 },
	runner = { name = "runner", cost = 2 },
	brute = { name = "brute", cost = 5 },
}

local ALIVE_CAP = 25
local alive = 0

local function farthestSpawn(): BasePart?
	local best: BasePart? = nil
	local bestDist = 40
	for _, spawn in CollectionService:GetTagged("EnemySpawn") do
		if spawn:IsA("BasePart") then
			local nearest = math.huge
			for _, player in Players:GetPlayers() do
				local root = player.Character and player.Character:FindFirstChild("HumanoidRootPart")
				if root and root:IsA("BasePart") then
					nearest = math.min(nearest, (root.Position - spawn.Position).Magnitude)
				end
			end
			if nearest > bestDist then
				best = spawn
				bestDist = nearest
			end
		end
	end
	return best
end

local function runWave(waveIndex: number, spawnEnemy: (kind: string, at: Vector3) -> Model)
	local budget = (8 + waveIndex * 3) * (0.6 + 0.4 * #Players:GetPlayers())
	local deadline = os.clock() + 90
	while budget > 0 and os.clock() < deadline do
		if alive < ALIVE_CAP then
			local spawnPart = farthestSpawn()
			local kind = KINDS.grunt
			if spawnPart and budget >= kind.cost then
				local enemy = spawnEnemy(kind.name, spawnPart.Position)
				alive += 1
				budget -= kind.cost
				enemy.Destroying:Once(function()
					alive -= 1
				end)
			end
		end
		task.wait(0.8)
	end
end
```

Pick `kind` from a weighted table in a real game; the example keeps one kind for clarity.

## Difficulty curve
- Waves 1 to 3: teach (one enemy kind, slow).
- Waves 4 to 9: add a second kind, rising speed.
- Wave 10 and every 5th: mini-boss or elite.
- Spikes then rests: a hard wave followed by an easy one reads as fair.

## Mistakes
- Spawning inside walls or in sight of players.
- Counting `alive` by `GetChildren()` every frame (use a counter plus `Destroying`).
- Leaving dead enemies in Workspace (memory and physics cost).
- Same budget for 1 player and 30 players.
- A runaway loop with no deadline when enemies get stuck.

## Related
[[12-Multiplayer-NPCs/npc-ai]], [[12-Multiplayer-NPCs/round-system-pattern]], [[13-Luau-Engineering/object-pooling]], [[14-QA-Optimization/lag-sources-ranked]], [[12-Multiplayer-NPCs/server-size-and-streaming]]

## For our Roblox games
Copy the principle: budgeted waves, player scaling, alive cap, safe spawn rules. Never copy another game's enemy designs, names, stat tables or wave scripts. Zero spend: free rigs and tags only.
