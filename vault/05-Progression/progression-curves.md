---
description: XP and upgrade-cost curve formulas (linear, polynomial, exponential, piecewise), how to choose one, time-to-next-level targets, and rebirth/prestige math with typed Luau.
type: reference
status: draft
last_checked: 2026-11-14
keywords: xp curve, level curve, cost curve, exponential, polynomial, piecewise, time to level, prestige, rebirth, multiplier, grind, idle game, progression math, Luau
sources: Machinations.io game economy articles, https://machinations.io/articles (searched 2026-11-14, not opened); Lostgarden/Daniel Cook skill-chain material (secondary, see loop-stack); existing vault notes progression-systems, economy-worksheet
verify: Formulas are standard math, checked by hand. Target times (seconds/minutes per level) are our working heuristics, UNVERIFIED against any published benchmark. Test with a Lune simulation before shipping.
tags: [progression, reference]
---
# Progression curves

How much a level or upgrade costs as the player advances, and how to pick the shape. Systems overview is in [[05-Progression/progression-systems]]; prices in [[02-Game-Design/economy-worksheet]]; economy flow in [[02-Game-Design/economy-math]].

## Do this
- Define cost(n) as a pure function in `src/shared`, read by server and client. The server alone applies it.
- Pick the shape from the table below; default is polynomial for XP levels and exponential for repeatable upgrades.
- Set targets first, then fit the curve: level 1 in 10-30 s, levels 2-5 in under 1 min each, then grow so a level takes 2-10 min in the first hour, and stretch to 20-60 min by day 3 (working heuristic, UNVERIFIED).
- Check income growth against cost growth: time-to-next = cost(n) / income(n). Plot it for n=1..100. If it climbs faster than 1.3x per level for long, players hit a wall.
- Use piecewise curves: fast onboarding segment, steady middle, steep endgame that rebirth resets.
- Cap level and cost with math.min so numbers never overflow to inf; format big numbers (1.2K, 3.4M, 1e12) in the UI.
- Rebirth should pay back faster than the previous run took: second run reaches the same level in 50-70% of the first run's time.
- Store levels and counts as integers; store huge currencies as doubles (safe integer range is 2^53, about 9.0e15).

## Curve shapes
| Shape | cost(n) | Growth per level | Use for | Danger |
|---|---|---|---|---|
| Linear | a + b*n | constant add | tutorial, small caps | trivial later, no wall |
| Polynomial | base * n^p, p 1.5-3 | ratio shrinks toward 1 | XP levels, skill ranks | late levels feel flat in ratio |
| Exponential | base * r^n, r 1.05-1.25 | constant ratio | repeatable upgrades, idle games | overflow; wall if income is linear |
| Piecewise | different formula per band | designed | whole-game pacing | visible seams if bands do not join |
| Logistic/soft cap | cap / (1 + e^(-k(n-m))) | slows at the end | stat gain, not cost | confusing if hidden |

Matching rule: if income grows exponentially (multipliers stack), cost should grow exponentially with r slightly above income growth. If income is flat, polynomial cost suffices.

## Time-to-next-level
```
time(n) = cost(n) / income(n)        -- income in currency per second at level n
total(n) = sum_{i=1..n} time(i)
```
Example: cost = 50 * 1.15^n, income = 5 * 1.10^n per s. time(n) = 10 * (1.15/1.10)^n = 10 * 1.0455^n. At n=10 about 15.6 s, n=50 about 90 s, n=100 about 830 s (14 min). Gentle on purpose. If cost ratio is 1.2 vs income 1.1, time(100) is 10 * 1.0909^100, about 56,000 s: a wall.

## Luau
```lua
--!strict
export type Curve = { base: number, ratio: number, power: number }

local Curves = {}

function Curves.linear(n: number, a: number, b: number): number
	return a + b * n
end

function Curves.poly(n: number, base: number, power: number): number
	return math.floor(base * n ^ power)
end

function Curves.expo(n: number, base: number, ratio: number): number
	return math.floor(math.min(base * ratio ^ n, 1e15))
end

-- piecewise: tutorial (n<=5) linear, middle polynomial, late exponential
function Curves.xpToNext(n: number): number
	if n <= 5 then
		return 10 + 10 * n
	elseif n <= 40 then
		return math.floor(50 * (n - 5) ^ 1.8) + 60
	end
	local at40 = 50 * 35 ^ 1.8 + 60
	return math.floor(math.min(at40 * 1.12 ^ (n - 40), 1e15))
end

-- time-to-next report for tuning (run in Lune)
function Curves.timeToNext(cost: number, incomePerSec: number): number
	return cost / math.max(incomePerSec, 1e-9)
end

return Curves
```
Check the seams: xpToNext(5)=60 then xpToNext(6) is about 110: a small jump is fine, a 10x jump is a bug.

## Rebirth / prestige math
- Permanent multiplier from lifetime earnings E, threshold T: `mult = 1 + k * (E / T) ^ 0.5` (square root keeps it from exploding; k 0.1-1).
- Or fixed per rebirth: `mult = 1 + 0.5 * rebirths` (additive, predictable) or `1.5 ^ rebirths` (multiplicative, needs steeper costs).
- Cost to rebirth: `T(r) = T0 * 4 ^ r` makes each rebirth take about as long as one more doubling of income.
- Rule: the second run must be visibly faster (50-70% of the first). Show the preview ("x2.4 after rebirth") before the confirm button.
- Keep one thing across rebirth (cosmetics, index, a currency) so reset does not feel like a loss.
```lua
--!strict
local function rebirthMult(lifetime: number, threshold: number, k: number): number
	return 1 + k * math.sqrt(math.max(lifetime, 0) / threshold)
end
```

## Mistakes
- Fitting cost to a pretty formula instead of a time target.
- Cost grows faster than income with no new source: grind wall, churn.
- Multipliers added in client code; see [[02-Game-Design/economy-math]] for stacking.
- Float level counters that drift; use integers.

## For our Roblox games
Copy the math: pure cost functions, time-to-next tables, preview before reset. Never copy another game's exact numbers, level names, rebirth names, or art; tune from our own simulation and D1/D7 data ([[06-Retention/benchmarks]]).
