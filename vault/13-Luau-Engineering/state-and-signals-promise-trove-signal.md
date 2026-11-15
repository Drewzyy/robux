---
description: How Signal, Promise and Trove (from RbxUtil and evaera's Promise) handle events, async work and cleanup in Roblox games, plus Charm and Silo for state, with stars, licenses and examples.
type: reference
status: draft
last_checked: 2026-11-15
keywords: signal, goodsignal, promise, trove, janitor, maid, cleanup, rbxutil, charm, silo, async, memory leak, connections, event
sources: https://github.com/Sleitnick/RbxUtil (modules/signal, modules/trove, read 2026-11-15); https://github.com/evaera/roblox-lua-promise (README, lib/init.lua, read 2026-11-15); https://github.com/littensy/charm (README, read 2026-11-15); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. API names read from cloned source. Janitor (howmanysmall) did not show up in search, so its stars are UNVERIFIED; Trove is the covered alternative.
tags: [luau-engineering, reference]
---
# State and signals: Signal, Promise, Trove, Charm

Three small ideas remove most Roblox bugs: custom events (Signal), values that arrive later (Promise), and one object that cleans up everything (Trove).

| Repo | Stars | License | Use |
|---|---|---|---|
| Sleitnick/RbxUtil | 477 | MIT | Collection: Signal, Trove, Timer, Spring, Shake, TaskQueue, Silo, WaitFor, TableUtil |
| evaera/roblox-lua-promise | 356 | MIT | Promise with cancel, retry, timeout |
| stravant/goodsignal | 75 | MIT | Reference fast Signal |
| Data-Oriented-House/LemonSignal | 51 | MIT | Another fast Signal |
| littensy/charm | 275 | MIT | Atomic state (signals, computed, effects), roblox-ts first |

## Do this
- Use a Signal for any event your own code fires (RoundStarted, CoinsChanged). Do not use BindableEvents for this; they copy arguments.
- Make one Trove per owner (a player, a round, a UI screen, an NPC). Add every connection, instance, thread and promise to it. Call trove:Destroy() when the owner ends.
- Use Trove:Connect(signal, fn) instead of signal:Connect so the connection is tracked automatically.
- Wrap anything that yields and can fail (DataStore calls, MarketplaceService lookups, asset loads) in a Promise, then end the chain with :catch so errors are not silent.
- Use Promise.retryWithDelay(fn, 3, 2) for flaky calls: 3 tries, 2 seconds apart. Do not retry purchases or anything that is not safe to repeat.
- Use promise:timeout(seconds) for waits on the client (for example waiting for a character part), so a missing instance cannot hang a script forever.
- Use Promise.all({...}) to wait for several loads before opening a screen.
- Keep Signals and Promises out of data saved to DataStores. They are runtime only.

## Trove example (our style)
```luau
--!strict
local Trove = require(Packages.Trove)
local Signal = require(Packages.Signal)

local RoundService = {}
RoundService.RoundEnded = Signal.new()

function RoundService.StartRound(players: { Player })
	local trove = Trove.new()
	for _, player in players do
		trove:Connect(player.CharacterRemoving, function() RoundService.Eliminate(player) end)
	end
	trove:Add(task.delay(120, function() RoundService.RoundEnded:Fire() end))
	trove:Add(function() print("round cleaned up") end)
	return trove -- caller runs trove:Destroy() when the round ends
end
```

## Promise example
```luau
local Promise = require(Packages.Promise)

Promise.retryWithDelay(function()
	return Promise.new(function(resolve, reject)
		local ok, result = pcall(function() return store:GetAsync(key) end)
		if ok then resolve(result) else reject(result) end
	end)
end, 3, 2)
	:timeout(15)
	:andThen(function(data) applyData(player, data) end)
	:catch(function(err) warn("load failed", err) player:Kick("Please rejoin") end)
```

## Charm and Silo (state containers)
- Charm: store a value in a signal, derive values with computed, run code on change with effect. Written for roblox-ts, so Luau-only projects may prefer Fusion's Value or a plain Signal.
- Silo (in RbxUtil): a Redux-like state slice with actions. Useful when many systems read one table, such as a round state.
- Keep the authoritative copy on the server and send diffs to clients. A client state container is a display cache only.

## Pitfalls
- A Signal connection to an object that lives forever (a service) leaks if the handler captures a dead player or instance. Always Disconnect or use a Trove.
- Promise :await() yields and returns (ok, value). Mixing it with :andThen in one chain confuses error handling; pick one style per function.
- An unhandled Promise rejection prints a warning later, far from the cause. Always add :catch, or use Promise.onUnhandledRejection during testing.
- Trove:Clean() empties the trove but keeps it usable; Destroy() ends it. Do not add objects after Destroy.
- Cancelling a Promise does not stop a pcall already running inside it.
- Signal:Fire runs handlers immediately in order; use FireDeferred if a handler must not run mid-update.

## For our Roblox games
- Copy the principle: one owner, one cleanup object, no free-floating connections; wrap yielding calls so failure is handled in one place.
- Take the packages from Wally with licenses intact (all MIT). Do not paste whole modules into our code.
- Never copy another game's event names, data keys or economy tables.
- Related: [[13-Luau-Engineering/memory-leak-checklist]], [[13-Luau-Engineering/datastore-playbook]], [[13-Luau-Engineering/tween-and-animation-cleanup]].
