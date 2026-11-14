---
description: Pets and follower NPCs that trail the owner: server-owned data, client-side smooth following, a typed follow loop, spacing offsets, and performance limits for many pets per server.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: pets, followers, companion, follow, offset, lerp, AlignPosition, network ownership, equip, pet limit, client rendering, Heartbeat
sources: creator-docs content/en-us/studio/optimization/scriptprofiler.md (Heartbeat/Stepped task view, read 2026-11-14); creator-docs content/en-us/production/game-design/design-for-roblox.md (read 2026-11-14)
verify: Pattern from model knowledge. Pet count and update rates are design defaults, UNVERIFIED; profile on a low-end phone with the MicroProfiler.
tags: [multiplayer-npcs, playbook]
---
# Pets and followers

A pet is a cosmetic or helper that trails its owner. The cheapest and smoothest design keeps ownership on the server and motion on each client.

## Do this
- Server owns the data: which pet, level, equipped slot. Validate equips against the saved inventory.
- Server creates no physical follower. It sets an attribute or value (`EquippedPets` as a folder of StringValues) that replicates to all clients.
- Each client builds visual pet models for every nearby player and moves them locally in `RunService.Heartbeat`. No physics, no remotes per frame.
- Position = owner root CFrame * offset, smoothed with a lerp (alpha about 0.15 per frame at 60 fps, convert with dt). Add a small sine bob for life.
- Spacing: place pets on an arc behind the owner. Slot `i` of `n`: angle = `(i - (n + 1) / 2) * 0.5` rad, radius 4 studs, height 1.5 studs.
- Hard limits: 3 to 6 equipped pets per player, and render pets only for players within 80 studs or the 20 nearest (defaults UNVERIFIED).
- Make pet parts `Anchored = true`, `CanCollide = false`, `CanQuery = false`, `CanTouch = false`, `Massless` irrelevant if anchored.
- If pets must affect gameplay (collect coins), the server computes it from the owner and the pet's stats, not from pet position.

## Client follow loop

```lua
--!strict
local RunService = game:GetService("RunService")
local Players = game:GetService("Players")

local localPlayer = Players.LocalPlayer

type PetState = { model: Model, slot: number, total: number }
local pets: { PetState } = {}

local function targetCFrame(root: BasePart, slot: number, total: number, t: number): CFrame
	local angle = (slot - (total + 1) / 2) * 0.5
	local offset = CFrame.Angles(0, angle, 0) * CFrame.new(0, 1.5 + math.sin(t * 2 + slot) * 0.2, 4)
	return root.CFrame * offset
end

RunService.Heartbeat:Connect(function(dt: number)
	local character = localPlayer.Character
	local root = character and character:FindFirstChild("HumanoidRootPart")
	if not root or not root:IsA("BasePart") then
		return
	end
	local alpha = 1 - math.exp(-10 * dt)
	local t = os.clock()
	for _, pet in pets do
		local goal = targetCFrame(root, pet.slot, pet.total, t)
		pet.model:PivotTo(pet.model:GetPivot():Lerp(goal, alpha))
	end
end)
```

`1 - math.exp(-k * dt)` makes smoothing frame-rate independent. Extend the loop to other players' pets the same way, within the distance cap.

## Edge cases
- Owner teleports or respawns: snap pets to the target (skip lerp) when the distance exceeds 30 studs.
- Owner leaves: destroy their pet models on `PlayerRemoving` on every client.
- Unequip: destroy the model and disconnect any per-pet connections.
- Pet needs to go somewhere (fetch): lerp toward a server-sent target position for 1 to 2 s, then return.

## Performance
- One Heartbeat connection total, not one per pet.
- Reuse pet models with a pool (see [[13-Luau-Engineering/object-pooling]]).
- Pets on low-end mobile: show only your own pets plus 5 nearest others.
- Keep each pet under about 2000 triangles and few unique textures (see [[14-QA-Optimization/mobile-performance-cookbook]]).

## Mistakes
- Spawning real physics pets on the server for every player (huge replication and physics cost).
- Using `wait()` loops for follow.
- Letting clients tell the server "give me coins from pet".
- No distance cull, so a 40-player server draws 200 pets.

## Related
[[12-Multiplayer-NPCs/npc-ai]], [[12-Multiplayer-NPCs/server-size-and-streaming]], [[13-Luau-Engineering/tween-and-animation-cleanup]], [[12-Multiplayer-NPCs/multiplayer-patterns]]

## For our Roblox games
Copy the principle: server data, client-side motion, distance cull, pooled models. Never copy another game's pet designs, names, rarity tiers as branded art, egg animations or sounds. Zero spend: original low-poly pets built in Studio.
