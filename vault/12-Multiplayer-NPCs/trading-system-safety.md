---
description: Safe player-to-player trading: two-step confirm, server-held escrow, atomic swaps, anti-scam delays, logging and rate limits. Prevents dupes, bait-and-switch and rollback exploits.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: trading, trade, escrow, dupe, duplication, scam, confirm, atomic, swap, inventory, trade lock, session lock, DataStore, audit log
sources: creator-docs content/en-us/production/game-design/balance-virtual-economies.md (read 2026-11-14); creator-docs content/en-us/safety.md (read 2026-11-14)
verify: Design is standard practice from model knowledge, UNVERIFIED against any specific Roblox policy. Check Roblox Community Standards and any current trading rules for your game type before launch. Cross-server trade is advanced and not covered.
tags: [multiplayer-npcs, playbook]
---
# Trading system safety

Trading moves value between two players. Every bug becomes a dupe or a theft, so the server holds all state and the swap is atomic.

## Do this
- Keep the trade as a server object: `{ a, b, offerA, offerB, confirmA, confirmB, state }`. The client only sends "add item id", "remove item id", "confirm", "cancel".
- Validate each item on the server at the moment of offering AND at the moment of the swap: the player owns it, it is tradable, it is not equipped, it is not already in another trade.
- Lock offered items (mark `locked = true`) so they cannot be sold, used or offered elsewhere while the trade is open.
- Two-step confirm: both press Accept, then a 5 s countdown, then both press Confirm. Any change to either offer resets both confirms and restarts the countdown.
- Apply the swap in one synchronous block with no yields between removing from A, removing from B and adding. If anything fails, roll back in memory before saving.
- Save both inventories together right after the swap. Do not let a player leave mid-save and keep the item (see [[13-Luau-Engineering/datastore-playbook]]).
- Rate-limit: one open trade per player, max 1 new request per 3 s, auto-cancel after 120 s idle.
- Log every completed trade (both ids, items, time, place version) to a DataStore or your analytics; disputes and dupe hunts depend on it.

## Swap code (in memory, after validation)

```lua
--!strict
type Inventory = { [string]: number }

local function hasAll(inv: Inventory, offer: Inventory): boolean
	for itemId, count in offer do
		if (inv[itemId] or 0) < count then
			return false
		end
	end
	return true
end

local function applySwap(invA: Inventory, invB: Inventory, offerA: Inventory, offerB: Inventory): boolean
	if not hasAll(invA, offerA) or not hasAll(invB, offerB) then
		return false
	end
	for itemId, count in offerA do
		invA[itemId] -= count
		invB[itemId] = (invB[itemId] or 0) + count
	end
	for itemId, count in offerB do
		invB[itemId] -= count
		invA[itemId] = (invA[itemId] or 0) + count
	end
	return true
end
```

Nothing yields inside, so no other script can interleave. Call it only after both confirms and a final re-check that both players are still in the server.

## Anti-scam rules
- Show the exact item name, icon and count from the SERVER, not from client text. Never let a client send display names.
- Highlight changes: flash the other player's offer for 1 s when it changes after a confirm.
- Block trading for brand new accounts or for players under N minutes in game (default 10 min) to slow alt-account dupes. UNVERIFIED as effective, tune.
- Cap value gaps with a warning (not a block) when one side offers 10x less value.
- Add a report button on the trade window that saves the trade log.

## Failure cases to test
- Player leaves during countdown: cancel, unlock all items.
- Player dies or teleports mid-trade: cancel.
- Both press confirm in the same frame: process once, guard with `state == "Open"`.
- Server shutdown during trade: items are still in the saved inventory because the swap never ran.
- Inventory full: reject before swap.

## Mistakes
- Removing the item from A, then yielding on a DataStore call before adding to B (lost item on crash).
- Trusting a client-sent price or item name.
- Letting a locked item be sold to a shop NPC.
- Not clearing locks on cancel.

## Related
[[13-Luau-Engineering/remote-validation-cookbook]], [[14-QA-Optimization/exploit-testing]], [[13-Luau-Engineering/remotes-and-security]], [[12-Multiplayer-NPCs/teams-and-parties]]

## For our Roblox games
Copy the principle: server escrow, double confirm, atomic swap, audit log. Never copy another game's item names, rarity art, trade UI skins or currency branding. Do not add paid items to the trade flow without checking current Roblox rules on trading and real-money items. Zero spend: pure Luau.
