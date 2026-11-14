---
description: Reuse instances instead of Clone/Destroy: a typed pool for projectiles, hit effects, damage numbers and enemies, with prewarm, reset rules, max size and leak guards.
type: reference
status: draft
last_checked: 2026-11-14
keywords: object pool, pooling, Clone, Destroy, projectile, effects, garbage collection, GC, reuse, prewarm, ReplicatedStorage, instance count, hitch
sources: creator-docs content/en-us/studio/optimization/memory-usage.md (Instances, Signals, LuaHeap categories, read 2026-11-14); creator-docs content/en-us/studio/optimization/scriptprofiler.md (read 2026-11-14)
verify: Benefit of pooling is workload dependent and UNVERIFIED here: measure with the MicroProfiler and ScriptProfiler before and after. Code written from model knowledge, not executed.
tags: [luau-engineering, reference]
---
# Object pooling

A pool keeps ready-made instances and hands them out. Creating and destroying many short-lived instances (bullets, sparks, damage numbers) causes hitches and memory churn. Pooling trades a little memory for steady frame time.

## Do this
- Pool only things spawned often (more than about 5 per second) and short-lived. Do not pool one-off or rare objects.
- Prewarm: create N objects at load (for example 20 per pool) in a loop with `task.wait()` every 5 to avoid a start-up hitch.
- Park idle objects out of the world: `Parent = nil` or a hidden folder, `Anchored = true`, `CanCollide = false`, `CanQuery = false`, `CanTouch = false`. Pick one and keep it consistent.
- Define a `reset` step that returns every property to its default: transparency, size, velocity, attachments, particle emitters (`Clear()`), sounds stopped, attributes cleared, connections disconnected.
- Set a max size (for example 64). If the pool is empty at the max, either recycle the oldest active item or skip the effect. Never grow without limit.
- Release on a timer or event, with a guard so an item is never released twice.
- On the server, pool NPC rigs and projectile parts; on the client, pool visual effects and UI rows (damage numbers, scroll list items).
- Verify with the Developer Console Memory tab: `Instances` should stay flat during a long fight (see [[14-QA-Optimization/microprofiler-and-stats-guide]]).

## Pool implementation

```lua
--!strict
local Pool = {}
Pool.__index = Pool

export type Pool<T> = typeof(setmetatable(
	{} :: {
		template: T & Instance,
		free: { T & Instance },
		active: { [Instance]: boolean },
		maxSize: number,
		total: number,
		reset: (T & Instance) -> (),
	},
	Pool
))

function Pool.new<T>(template: T & Instance, prewarm: number, maxSize: number, reset: (T & Instance) -> ()): Pool<T>
	local self = setmetatable({
		template = template,
		free = {},
		active = {},
		maxSize = maxSize,
		total = 0,
		reset = reset,
	}, Pool)
	for _ = 1, prewarm do
		local item = template:Clone() :: T & Instance
		reset(item)
		table.insert(self.free, item)
		self.total += 1
	end
	return self :: any
end

function Pool.get<T>(self: Pool<T>): (T & Instance)?
	local item = table.remove(self.free)
	if not item then
		if self.total >= self.maxSize then
			return nil
		end
		item = self.template:Clone() :: T & Instance
		self.total += 1
	end
	self.active[item] = true
	return item
end

function Pool.release<T>(self: Pool<T>, item: T & Instance)
	if not self.active[item] then
		return
	end
	self.active[item] = nil
	self.reset(item)
	item.Parent = nil
	table.insert(self.free, item)
end

return Pool
```

Usage: `local item = pool:get()`, set its CFrame, parent it to Workspace, then `task.delay(1.5, function() pool:release(item) end)`.

## Pitfalls
- Stale state: a reused bullet keeps old velocity or a connected `Touched` handler. Reset must be complete; prefer connecting handlers once at creation and reading state from attributes.
- Double release puts the same object in the free list twice (two users share it). The `active` check above prevents it.
- A destroyed item (by another script) in the free list. On `get`, check `item.Parent` is not locked and `item:IsDescendantOf(game)` rules fit your design, or listen for `Destroying` and drop it.
- Pooling humanoid rigs: reset Humanoid health, state and animations; or skip, since rigs are costly to reset correctly.
- Particle emitters: call `Emit` then wait for the lifetime before releasing, or sparks cut off.
- Replication: reparenting a server object into Workspace replicates it each time; for pure visuals, pool on the client instead.

## Measuring
Compare frame time spikes during a heavy fight with and without the pool using the MicroProfiler. If no spike disappears, remove the pool; less code is better (see [[13-Luau-Engineering/smallest-complete-change]]).

## Related
[[12-Multiplayer-NPCs/enemy-waves-and-spawners]], [[12-Multiplayer-NPCs/pets-and-followers]], [[13-Luau-Engineering/memory-leak-checklist]], [[14-QA-Optimization/lag-sources-ranked]]

## For our Roblox games
Copy the principle: prewarm, reset fully, cap size, measure. No names, art or assets are involved. Zero spend: plain Luau.
