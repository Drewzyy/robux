---
description: Sources and sinks balance, inflation control, idle and offline earnings caps, and additive versus multiplicative multiplier stacking, with a server-side Luau offline-earnings function.
type: reference
status: draft
last_checked: 2026-11-14
keywords: economy, sources, sinks, inflation, faucet, idle, offline earnings, cap, multiplier stacking, additive, multiplicative, simulator, tycoon, balance, Machinations
sources: Machinations.io game economy design articles, https://machinations.io/articles (searched 2026-11-14, not opened); Roblox Creator Docs on DataStore and os.time (not reachable 2026-11-14, from memory, UNVERIFIED); existing vault note economy-worksheet
verify: Math is standard and hand-checked. Faucet/sink ratios and cap hours are working heuristics, UNVERIFIED against published data. Roblox API names from memory; confirm in Creator Docs.
tags: [game-design, reference]
---
# Economy math

The numbers behind [[02-Game-Design/economy-worksheet]]. Curve shapes are in [[05-Progression/progression-curves]].

## Do this
- Model every currency as a stock with flows: `balance(t+1) = balance(t) + sources - sinks`. List each source and sink with a per-minute rate before coding.
- Target a net flow slightly positive for active players (earn about 1.1-1.5x what mandatory sinks cost per session) so progress feels real but prices still matter (heuristic, UNVERIFIED).
- Every source scales with progress, so every sink must too: sinks that scale (upgrade cost x1.15 per level, repair costs, rebirth fees, consumables, cosmetic shops) stop late inflation.
- Offline earnings: store `lastSeen` as `os.time()` UTC seconds; compute on join; cap at 8-12 hours and at 25-50% of the active rate (heuristic). Never run a server timer per offline player.
- Stack multipliers with a documented rule: additive within a category, multiplicative between categories (see below). Total multiplier hard-capped in one tuning module.
- Server computes all currency changes; clamp each grant per event and log it with balance-after ([[06-Retention/event-taxonomy]]).
- Run a Lune simulation of first hour and day 3 for a casual, a median, and a grinder player before launch; confirm time-to-next stays in target.

## Sources, sinks, inflation
| Term | Meaning | Examples |
|---|---|---|
| Faucet/source | currency created | collect, quest, daily reward, sell |
| Sink/drain | currency destroyed | upgrades, rebirth fee, egg cost, repairs |
| Converter | trades one currency for another | sell backpack, gem exchange |
| Inflation | prices lag income, balances balloon | players hold millions, nothing to buy |

Symptoms of inflation: median balance rising week over week, purchases per player falling, one item bought by everyone. Fixes, in order: add scaling sinks, add a new prestige tier, raise prices of the next tier only (never nerf past earnings), remove the weakest faucet.

Dashboards: log source and sink totals per day per currency; ratio `sources / sinks` near 1.1-1.3 is healthy for a soft currency (UNVERIFIED target).

## Offline earnings (server)
```lua
--!strict
local MAX_OFFLINE_SEC = 8 * 3600
local OFFLINE_RATE = 0.35 -- fraction of active income

local function offlineEarnings(lastSeen: number, now: number, incomePerSec: number): number
	local away = math.clamp(now - lastSeen, 0, MAX_OFFLINE_SEC)
	return math.floor(away * incomePerSec * OFFLINE_RATE)
end

-- on PlayerAdded after data loads:
-- local gain = offlineEarnings(data.lastSeen, os.time(), incomePerSecond(data))
-- data.coins += gain; data.lastSeen = os.time()
```
- Clamp negative `away` (clock skew) to 0. Show a "welcome back, +X" popup so the player sees the reward; the popup is a return trigger ([[03-Game-Psychology/why-players-return]]).
- Longer cap helps whales and hurts return frequency; 8-12 h means one check-in per day, the intended habit.

## Multiplier stacking
- Additive: `total = base * (1 + a1 + a2 + a3)`. Three +50% boosts give x2.5. Easy to reason about, hard to explode.
- Multiplicative: `total = base * m1 * m2 * m3`. Three x1.5 boosts give x3.375. Powerful, explodes with many sources.
- Hybrid (recommended): sum inside a category, multiply across categories.
```lua
--!strict
export type Boosts = { permanent: {number}, temporary: {number}, rebirth: number }
local MAX_TOTAL = 1000

local function totalMult(b: Boosts): number
	local perm = 1
	for _, v in b.permanent do perm += v end
	local temp = 1
	for _, v in b.temporary do temp += v end
	return math.min(perm * temp * b.rebirth, MAX_TOTAL)
end
```
- Tell the player the formula in the UI ("x2.5 = 1 + 0.5 + 1.0"). Hidden stacking reads as a bug when numbers jump.
- Paid boosts: keep them timesavers, not required; policy and pricing in the monetization skill.

## Mistakes
- Offline cap missing: players leave a week, return to a finished game.
- A faucet added in an update with no matching sink.
- Multipliers computed on the client for display only drifting from the server value.
- Currency that can be duped through a remote with no clamp (see the roblox-security skill).

## For our Roblox games
Copy: faucet/sink ledger, scaled sinks, offline cap, hybrid stacking, a Lune sim. Never copy another game's currency names, price tables, or boost art; derive numbers from our own telemetry.
