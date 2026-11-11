---
description: Idempotent ProcessReceipt in Luau: NotProcessedYet when the player or data is missing, grant once per PurchaseId, never grant from PromptProductPurchaseFinished.
type: template
status: verified
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C4 and D1 (developer products, ProcessReceipt rules)
verify: Verified 2026-11-09: the decision module and its Lune test pass scripts/gates/build_gate.py. The wrapper's Roblox API use (MarketplaceService.ProcessReceipt, Enum.ProductPurchaseDecision) must be tested with a real purchase in Studio: UNVERIFIED live.
tags: [monetization, template]
---
# ProcessReceipt template

## Rules
- Every developer product is granted in `MarketplaceService.ProcessReceipt`. Never grant from `PromptProductPurchaseFinished`.
- Return `PurchaseGranted` only after the grant is recorded. Otherwise Roblox keeps retrying (good) or, if the callback errors, the purchase loops.
- Player not in this server, or data not loaded: return `NotProcessedYet`; Roblox retries when they rejoin.
- Idempotent: record each `PurchaseId`; a replayed receipt returns `PurchaseGranted` without granting twice.
- The grant and the record must be saved with the player's session-locked data ([[13-Luau-Engineering/datastore-playbook]]). How to force a save is ProfileStore-specific: read its docs (UNVERIFIED here).

## Pure decision logic (testable in Lune)
`src/shared/ReceiptLogic.luau`
```lua
local ReceiptLogic = {}

export type State = {
	playerPresent: boolean,
	dataLoaded: boolean,
	alreadyGranted: boolean,
	hasHandler: boolean,
}

-- Returns "retry", "already", or "grant".
function ReceiptLogic.decide(s: State): string
	if not s.playerPresent or not s.dataLoaded then
		return "retry"
	end
	if s.alreadyGranted then
		return "already"
	end
	if not s.hasHandler then
		return "retry"
	end
	return "grant"
end

return table.freeze(ReceiptLogic)
```

## Server wrapper
`src/server/Services/Receipts.luau`
```lua
local MarketplaceService = game:GetService("MarketplaceService")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local ReceiptLogic = require(ReplicatedStorage.Shared.ReceiptLogic)

-- Game-provided: product id -> function(player, data) that changes data.
local Products: { [number]: (Player, any) -> () } = {}
-- Game-provided: returns the player's loaded, session-locked data table or nil.
local function getData(_player: Player): any
	return nil
end

local Receipts = {}

function Receipts:Init() end

function Receipts:Start()
	MarketplaceService.ProcessReceipt = function(receiptInfo)
		local player = Players:GetPlayerByUserId(receiptInfo.PlayerId)
		local data = player and getData(player)
		local purchases = data and data.Purchases
		local verdict = ReceiptLogic.decide({
			playerPresent = player ~= nil,
			dataLoaded = data ~= nil,
			alreadyGranted = purchases ~= nil and purchases[receiptInfo.PurchaseId] == true,
			hasHandler = Products[receiptInfo.ProductId] ~= nil,
		})
		if verdict == "already" then
			return Enum.ProductPurchaseDecision.PurchaseGranted
		elseif verdict == "retry" then
			return Enum.ProductPurchaseDecision.NotProcessedYet
		end
		local ok, err = pcall(Products[receiptInfo.ProductId], player, data)
		if not ok then
			warn("grant failed", receiptInfo.ProductId, err)
			return Enum.ProductPurchaseDecision.NotProcessedYet
		end
		data.Purchases = data.Purchases or {}
		data.Purchases[receiptInfo.PurchaseId] = true
		return Enum.ProductPurchaseDecision.PurchaseGranted
	end
end

return Receipts
```

## Test
`tests/run.luau`
```lua
local ReceiptLogic = require("../src/shared/ReceiptLogic")

local base = { playerPresent = true, dataLoaded = true, alreadyGranted = false, hasHandler = true }
local function with(changes)
	local s = table.clone(base)
	for k, v in changes do
		s[k] = v
	end
	return s
end

assert(ReceiptLogic.decide(base) == "grant", "normal purchase grants")
assert(ReceiptLogic.decide(with({ playerPresent = false })) == "retry", "player gone: retry")
assert(ReceiptLogic.decide(with({ dataLoaded = false })) == "retry", "data not loaded: retry")
assert(ReceiptLogic.decide(with({ alreadyGranted = true })) == "already", "replay: no double grant")
assert(ReceiptLogic.decide(with({ hasHandler = false })) == "retry", "unknown product: retry")
print("all tests passed")
```

`data.Purchases` grows with every purchase. Trim old ids in a real game (keep the last few hundred).
