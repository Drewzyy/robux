from common import note
S = "40-monetization"
R = "Research report (Nov 9 2026) C4 and D1"

note(f"{S}/money-math", "Robux to dollars: 70/30 split, DevEx $0.0038 standard and about $0.0054 US 18+, 30,000 minimum, Creator Rewards. Worked examples with the arithmetic shown.", "reference", "draft",
 R + "; moonetix.com; obby.fun creator rewards guide; loottally.com; gamertagmythras.com DevEx guide",
 "Rates are from secondary sources dated 2025-2026. Re-check every figure in Creator Dashboard (DevEx portal) before any cashout; the US 18+ avatar condition is UNVERIFIED.", r"""
# Money math

Re-check every number here in Creator Dashboard before the first cashout.

## The funnel
1. A player spends Robux in the game.
2. Marketplace fee 30%: the creator keeps **70%** as earned Robux.
3. Earned Robux cash out through **DevEx** (or Roblox Wallet for eligible US creators, see [[40-monetization/roblox-wallet]]).

## DevEx
- Standard rate **$0.0038 per earned Robux** since Sep 5, 2025 (was $0.0035).
- **US 18+ rate about $0.0054** per earned Robux (about 42% higher), from June 8, 2026, for qualifying spend by age-checked US players 18+ in eligible games. UNVERIFIED condition: eligible games use R15-only avatars, a qualifying custom rig, or no visible character.
- Minimum cashout **30,000 earned Robux** = $114 at standard rate ($162 at the US 18+ rate).
- Needs: 13+, verified email, DevEx portal account, W-9 or W-8, account in good standing.
- Only earned Robux cash out. Bought, gift-card, or stipend Robux never do.
- Since Nov 1, 2026 US DevEx is treated as royalties; with a valid W-9 nothing is withheld, without one 24% backup withholding (secondary reporting).

## Worked example (arithmetic shown)
A player spends 1,000 Robux.
- Creator receives 1,000 x 0.70 = **700 earned Robux**.
- Standard DevEx: 700 x $0.0038 = **$2.66**.
- US 18+ rate: 700 x $0.0054 = **$3.78**.
- To reach the 30,000 minimum at the standard rate: 30,000 / 0.70 = about **42,858 Robux of player spending**.

## Creator Rewards (replaced Premium Payouts on July 24, 2025)
- **Daily Engagement:** 5 Robux per day per Active Spender who plays 10+ minutes when the game is one of the first three they play that day. Active Spender: spent at least $9.99 anywhere on Roblox in the past 60 days and not new or reactivated in that time.
- **Audience Expansion:** 35% revenue share on the first $100 of qualifying purchases a new or reactivated player makes anywhere on Roblox in their first 60 days, if they arrived through the game's share link, direct link, or search, played 10+ minutes that day, and the game averages 100+ DAU for the 60 days after they join. Needs an ID-verified account and a valid DevEx account.
- Both paid as earned Robux after a 60-day hold.
- Do not estimate Daily Engagement as DAU x 5. Use the "daily rewarded active spenders" metric in Creator Dashboard.

## Correction to the god plan
The god plan's Audience Expansion example ($10.64) does not follow from 35% of $100 (report D1 item 10). The 35% share is verified in secondary sources; the dollar value of the Robux it pays is not stated, so do not put a dollar figure on it until Creator Dashboard shows one.
""")

note(f"{S}/roblox-wallet", "Roblox Wallet: Dec 2026 opt-in for US 18+ sole proprietors, daily USD payouts at DevEx rates; Roblox plans to move away from DevEx in the US in mid-2027.", "reference", "draft",
 R + " (D1 item 1); gamertagmythras.com RDC 2026 announcements; livecards.net Roblox Wallet explained; en.creatorshub.info Roblox Wallet",
 "Announced at RDC 2026 per secondary sources. UNVERIFIED until Andrew sees an invite in Creator Hub; check in December 2026.", r"""
# Roblox Wallet

**Correction to the god plan:** the god plan treats DevEx as the only cashout. For Andrew (22, US-based), Roblox Wallet is the cashout route once invited.

## What was announced (RDC 2026, secondary sources)
- From **December 2026**, Roblox invites US-based independent creators (sole proprietors) aged 18+ to opt in.
- Pays earnings in **USD every business day** at the same DevEx rates, after a hold of up to 30 days depending on the transaction.
- No cashout request and no monthly DevEx cooldown.
- Wallet holders can buy Robux at $0.0038 per Robux (examples: 10,000 Robux for $38, 30,000 for $114, plus tax).
- Roblox says it will **transition away from DevEx in mid-2027 in the US**. DevEx stays longer in other regions. Existing earned Robux remain eligible for DevEx during the transition.
- Provided through Airwallex US. A Roblox Card follows in select markets in 2027 and spends only the Wallet balance.
- Groups, businesses, 13+ creators, and other countries follow in 2027.

## Andrew's action
- December 2026: check Creator Hub for the Wallet invite. Do not assume eligibility; it depends on being a sole proprietor and on the invite.
- Needs Andrew's account and tax details. No agent does this.
""")

note(f"{S}/product-catalog-template", "Catalog template: gamepass vs developer product vs subscription vs private server, with ID, price, exactly what it does, and why it is fair.", "template", "draft",
 R + "; moonetix.com; spawnblox.com; blancmathis monetization reference",
 "Template. A filled copy is checked by Merchant's gate and by scripts/gates/check_stage.py (catalog and do-not-sell sections present).", r"""
# Product catalog template

| Name | Type | Product ID | Price (R$) | Exactly what it does | Why it is fair | Do-not-sell check |
|---|---|---|---|---|---|---|
| | gamepass / dev product / subscription / private server | (from Creator Dashboard) | | | | pass / fail |

## Which type
- **Gamepass**: one-time, permanent (VIP, 2x coins forever, speed coil, trail, extra inventory, area access). Free to create in Creator Dashboard or the Open Cloud Game Passes API.
- **Developer product**: repeatable consumables (skip stage, coin pack, temporary boost, revive). **Needs `ProcessReceipt`** ([[40-monetization/processreceipt-template]]) or purchases fail and players are charged for nothing.
- **Subscription**: recurring monthly benefit; not for game 1 ([[40-monetization/roblox-plus-and-subscriptions]]).
- **Private server**: recurring; around 100 R$/month when added later (god plan, not in repo).

## Rules
- Sell each pass inside its own experience. **Cross-game pass sales were disabled May 30, 2026.** Portals and links to other games are still fine.
- Creating products needs Andrew's account. Agents list them; Andrew creates them.
- No third-party ad SDKs or off-platform reward schemes.
""")

note(f"{S}/pricing-ladder", "Pricing tiers (impulse, core, premium, decoy), ending in 9 or 5, anchoring, bundles. Community guidance with ranges, not Roblox rules.", "reference", "draft",
 R + "; spawnblox.com; Andy A. G. Medium 2026 guide; pragma.vision; axltaylor2013-star roblox-monetization",
 "Community guidance. Revenue-share claims (250 R$ tier carries 40-55%; 14-minute visitors 30x more likely to spend) are UNVERIFIED hypotheses. Test with Roblox Price Optimization once traffic allows.", r"""
# Pricing ladder

Community guidance, not Roblox rules. Ranges differ by source.

## Tiers
| Tier | Range (R$) | Example use |
|---|---|---|
| Impulse | under 50 | first purchase |
| Small perk | 25 to 99 | skip stage, small boost |
| Core | 149 to 499 | 2x coins, VIP |
| Premium | 499 to 999 | big permanent perk |
| Deluxe | 2,000 to 6,000+ | whales (1 to 5% of players) |

One example ladder: 50, 250, 1,000, plus a 2,500 decoy above the top tier. The god plan's Sky Tower Obby ladder (Skip Stage 25, 2x Coins 79, Speed Coil 99, Low Gravity 149, VIP 249) fits these ranges (god plan not in repo).

## Patterns
- End prices in 9 or 5 (99 not 100). Avoid odd numbers like 73.
- Anchor with a high price so the middle tier reads as a deal.
- Bundles, a one-time first-purchase bonus, limited season exclusives.
- Show value before the prompt (let players try a limited version).
- Awkward pack increments push players to the next pack. Allowed only if prices are visible in Robux before purchase and nothing misleads.

## UNVERIFIED claims
- "The 250 R$ middle tier carries 40-55% of revenue."
- "A visitor who plays 14 minutes is about 30x more likely to spend than one who plays 30 seconds."

## Later
Roblox Price Optimization tests price points on subsets of players. Use it instead of guessing once there is traffic.
""")

note(f"{S}/processreceipt-template", "Idempotent ProcessReceipt in Luau: NotProcessedYet when the player or data is missing, grant once per PurchaseId, never grant from PromptProductPurchaseFinished.", "template", "verified",
 R + " (developer products, ProcessReceipt rules)",
 "Verified 2026-11-09: the decision module and its Lune test pass scripts/gates/build_gate.py. The wrapper's Roblox API use (MarketplaceService.ProcessReceipt, Enum.ProductPurchaseDecision) must be tested with a real purchase in Studio: UNVERIFIED live.", r"""
# ProcessReceipt template

## Rules
- Every developer product is granted in `MarketplaceService.ProcessReceipt`. Never grant from `PromptProductPurchaseFinished`.
- Return `PurchaseGranted` only after the grant is recorded. Otherwise Roblox keeps retrying (good) or, if the callback errors, the purchase loops.
- Player not in this server, or data not loaded: return `NotProcessedYet`; Roblox retries when they rejoin.
- Idempotent: record each `PurchaseId`; a replayed receipt returns `PurchaseGranted` without granting twice.
- The grant and the record must be saved with the player's session-locked data ([[20-luau-rojo/datastore-playbook]]). How to force a save is ProfileStore-specific: read its docs (UNVERIFIED here).

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
""")

note(f"{S}/paid-random-items-checklist", "Paid random items: numeric odds summing to 100% shown before purchase, every outcome has value, PolicyService gating per player, fail closed.", "checklist", "draft",
 R + " (B8e, C4); zehn-studio26.com; we-are-family.com; techtimes.com (Korea loot box rules); antimedia.co; andrian-syh monetization-policy",
 "Roblox policy (live May 26, 2026) as reported by secondary sources. Re-read the official Paid Random Items policy in Creator Hub before building any paid random feature.", r"""
# Paid random items checklist

Studio rule: **none in game 1.** If a later game uses them, every box must be ticked.

## Disclosure
- [ ] Numeric odds for every possible outcome, shown before purchase, as percentages summing to 100%.
- [ ] If a paid item boosts odds (a Lucky Potion), the adjusted odds are shown too.
- [ ] Every outcome gives something of value. No outcome leaves the player with nothing.

## What counts as paid random
- [ ] Indirect purchases count: a key or ticket bought with Robux and then used for a random item.
- [ ] Paid prize wheels, luck boosts, pity systems, rerolls, enhanced drops, and rate-ups all count.

## Region gating
- [ ] On join, `pcall` `PolicyService:GetPolicyInfoForPlayerAsync` once, cache it for the session, and honour `ArePaidRandomItemsRestricted` per player.
- [ ] **Fail closed:** if the call fails, treat the player as most restricted.
- [ ] Reported restrictions: off for under-18s in the UK and Brazil; off for everyone in Australia, Belgium, and the Netherlands.
- [ ] Where blocked, offer another route: earn it, or buy the item directly. Accepted treatments include an unpaid earnable path, a fixed non-random sequence, guaranteed direct purchase, hiding the feature, or blocking with a message.

## Do not copy
Some large RNG games reportedly keep paid random features on in restricted regions and rely on odds disclosure. That is community-reported practice, not policy, and it is not compliant.

Violations can mean game removal and account suspension. Free random rewards are fine everywhere.
""")

note(f"{S}/do-not-sell-list", "Hard rules: never sell a win, never gate the finish line, never make the free path punishing, no pay-to-win, no paid random items in game 1, shop never interrupts play.", "checklist", "draft",
 R + " (C4 'What not to sell'); god plan Section 4.3 (not in repo); Roblox Community Standards on deceptive monetization (as reported)",
 "Studio rules (stricter than Roblox policy). Merchant's gate checks every product against this list; Andrew is the final check in the review queue.", r"""
# Do-not-sell list

These are hard rules for this studio, not suggestions.

1. **Never sell a win.** Victory is never the product.
2. **Never gate the finish line.** Free players can finish the whole game.
3. **Never make the free path feel punishing.** No artificial walls to push purchases.
4. **Never sell checkpoints.** They are the retention mechanic.
5. **No paid random items in game 1.**
6. **Nothing that affects other players** (a tool that knocks others off, a troll pass).
7. **No pay-to-win stat items** in competitive modes.
8. **Consumables are developer products, not gamepasses.**
9. **No required-to-progress items.**
10. **Every purchase does exactly what it says.** Deceptive monetization violates Community Standards.
11. **The shop never interrupts play.** No pop-up on join, on death, or in the first 5 minutes. Death is never a sales pitch.
12. **No off-platform payments, ever.** No "DM me for Robux", no external shops.

Free players get the full game. Paying players get speed, style, and convenience.
""")

note(f"{S}/roblox-plus-and-subscriptions", "Roblox Plus (Apr 30 2026, $4.99/month) creator incentive and experience subscriptions. A later addition, not a game 1 feature.", "reference", "draft",
 R + " (D1 item 5, C4); gamertagmythras.com RDC 2026; spawnblox.com",
 "Secondary reporting; payout figures differ between sources (UNVERIFIED). Read the Creator Hub subscription docs before adding either.", r"""
# Roblox Plus and subscriptions

## Roblox Plus
- Launched April 30, 2026 at $4.99/month; new Premium sign-ups stopped.
- Creator incentive: up to **750 Robux per new subscriber** (250 a month for the subscriber's first three months) when they subscribe from inside the experience through `PromptRobloxSubscriptionPurchase`.
- Roblox reported 300 million+ Robux paid to creators in Plus's early period (one source: first four months; another: first twelve weeks). UNVERIFIED exact figure.
- Selling UGC avatar items requires age verification and Plus or Premium.

## Experience subscriptions
- Recurring monthly benefits, priced in Robux (49+) or local-currency points ($2.99, $4.99, $7.99, $9.99, $14.99).
- Creator is paid in Robux: 70% every month for Robux pricing; for local pricing 70% in month one and 100% after, at $0.01 to 1 Robux after fees (secondary, UNVERIFIED).
- Local-currency subscriptions need ID or phone verification.

## Studio timing
Not for game 1 at launch. Add only once the game has regular players (god plan threshold 500+ DAU; god plan not in repo).
""")

note(f"{S}/creator-store-and-ugc", "Creator Store sales (100% of net proceeds) and UGC item revenue splits. Later income lines, not game 1.", "reference", "draft",
 R + " (C4)", "Secondary reporting. Verify splits on the Creator Store and Marketplace policy pages before selling anything.", r"""
# Creator Store and UGC

- **Creator Store:** selling models and plugins to other creators returns 100% of net proceeds, outside platform fees and DevEx rates (as reported).
- **UGC avatar items:** a Marketplace sale pays the item creator 30% and the seller 40%; Roblox takes 30%. Selling your own item inside your own game pays 70% total.
- Selling UGC items needs age verification and a Plus or Premium subscription.
- Both are later income lines. Game 1 focuses on the game.
""")
print("40 done")
