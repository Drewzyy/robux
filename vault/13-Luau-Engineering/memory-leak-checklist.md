---
description: Find and prevent Roblox memory leaks in Luau: connections, tables keyed by Player, instances kept alive, threads, with a Maid helper and Developer Console steps to confirm growth.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: memory leak, connections, Disconnect, Maid, Trove, PlayerRemoving, weak table, __mode, LuaHeap, Developer Console, Luau heap snapshot, Instances, Signals, out of memory
sources: creator-docs content/en-us/studio/optimization/memory-usage.md (Memory tool, Luau heap tool, read 2026-11-14); creator-docs content/en-us/reference/engine/enums/DeveloperMemoryTag.yaml (read 2026-11-14); creator-docs content/en-us/production/analytics/performance.md (server memory guideline, read 2026-11-14)
verify: Tools and category names verified in local docs. The 3 GB server and 50% guideline is quoted from the performance dashboard doc; client limits by device are UNVERIFIED. Code is from model knowledge, not executed.
tags: [luau-engineering, checklist]
---
# Memory leak checklist

A leak is memory that grows and never comes back. On Roblox it ends in a server shutdown or a client out-of-memory crash, mostly on phones. The cause is almost always something that stays referenced after it should be gone.

## Do this
- Make every `Connect` have an owner that disconnects it. Use a Maid (list of connections and instances) per player, per character, per round, per NPC.
- Clear every table keyed by `Player` in `PlayerRemoving`. Same for tables keyed by `Model` or `Instance` (clear on `Destroying`).
- Use weak tables for caches keyed by instances: `setmetatable({}, { __mode = "k" })`, so a destroyed key can be collected.
- Destroy what you create: `Destroy()` clones, effect parts, GUI rows, Folders made per round. `Destroy` also disconnects events on that instance.
- Do not keep references in upvalues of long-living closures (a connection callback holding a big table or a model).
- Cancel threads you no longer need with `task.cancel(thread)`; loops should exit on a flag or when their owner is gone.
- Cap logs, histories and queues (keep the last 100 entries, not all).
- Confirm with data: watch the Developer Console Memory tab for 10 minutes in a test server with bots joining and leaving. `Instances`, `Signals` and `LuaHeap` should plateau.
- Server memory guideline from the performance dashboard docs: keep it under 50% of the total and under about 3 GB. If the line climbs without a plateau, you have a leak.

## Maid helper

```lua
--!strict
local Maid = {}
Maid.__index = Maid

export type Task = RBXScriptConnection | Instance | (() -> ()) | thread

export type Maid = typeof(setmetatable({} :: { tasks: { Task } }, Maid))

function Maid.new(): Maid
	return setmetatable({ tasks = {} }, Maid)
end

function Maid.give(self: Maid, item: Task): Task
	table.insert(self.tasks, item)
	return item
end

function Maid.clean(self: Maid)
	for _, item in self.tasks do
		local kind = typeof(item)
		if kind == "RBXScriptConnection" then
			(item :: RBXScriptConnection):Disconnect()
		elseif kind == "Instance" then
			(item :: Instance):Destroy()
		elseif kind == "function" then
			(item :: () -> ())()
		elseif kind == "thread" then
			task.cancel(item :: thread)
		end
	end
	table.clear(self.tasks)
end

return Maid
```

Pattern: `local maid = Maid.new()` at player join; `maid:give(player.CharacterAdded:Connect(...))`; `maid:clean()` in `PlayerRemoving`. Create a child maid per character and clean it on `Humanoid.Died` or `CharacterRemoving`.

## Leak sources, ranked by how often they bite
1. `Connect` on a global signal (`Heartbeat`, `Players.PlayerAdded`) inside a function that runs many times.
2. Tables keyed by `Player` never cleared.
3. Per-round objects (maps, effects) not destroyed at round end.
4. Tweens and animation tracks left running (see [[13-Luau-Engineering/tween-and-animation-cleanup]]).
5. Cached instances kept after they are destroyed.
6. `RemoteEvent` listeners connected per request (connect once at start).
7. Unbounded arrays (chat history, damage logs, analytics queues).
8. Threads spawned in a loop that never end.

## Confirm in Studio
1. Run a local server with 2 or more clients (Test tab).
2. Open the Developer Console, Memory tab, choose Server.
3. Run the suspect flow 20 times (join, leave, start round, end round).
4. Take a Luau heap snapshot before and after; compare by object type to see which kind grew (see [[14-QA-Optimization/microprofiler-and-stats-guide]]).
5. If `Signals` grows, it is connections. If `Instances` grows, it is undestroyed objects. If `LuaHeap` grows, it is tables and closures.

## Mistakes
- Disconnecting the connection variable you overwrote (you hold only the last one).
- Using `:Wait()` in a loop on a signal for events, instead of one `Connect`.
- Assuming `Destroy` on a Player's character cleans your handlers (it cleans signals on that instance only).
- Treating `collectgarbage` as a fix.

## Related
[[13-Luau-Engineering/object-pooling]], [[13-Luau-Engineering/common-bugs-checklist]], [[14-QA-Optimization/performance-budgets]], [[14-QA-Optimization/lag-sources-ranked]], [[13-Luau-Engineering/service-controller-template]]

## For our Roblox games
Copy the principle: every connection has an owner, every player-keyed table is cleared, measure over time. No names, art or assets are involved. Zero spend: Developer Console and Luau only.
