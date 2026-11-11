---
description: Simulator and tycoon economy numbers: cost curves, buy max, rebirth pacing, offline income, big numbers, and odds panels.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: economy, pacing, cost curve, upgrade cost, rebirth, prestige, offline earnings, idle, simulator, tycoon, big numbers, odds, eggs
sources: https://github.com/gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0); Roblox paid random items guidelines
verify: UNVERIFIED: condensed from the roblox-simulator-economy skill (Apache-2.0) on 2026-11-11; its maths is tested there with Lune but not yet run in this studio.
tags: [monetization, playbook]
---
# Economy pacing

Condensed from the roblox-simulator-economy skill in gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0). Tested code: `13-Luau-Engineering/lib/economy/` (unchanged copy with its license).

## Do this
- Put every number in one config table (base income; per upgrade baseCost, costGrowth, effect, maxLevel; rebirth baseRequirement, requirementGrowth, multiplierPerRebirth; offline maxHours, efficiency). Server and client read it.
- Price of next level = floor(baseCost * costGrowth ^ owned). Use growth 1.07 to 1.15 for upgrades bought many times; 1.5 to 3 only for rare big ones.
- First upgrades take seconds (first 10 levels about 3 to 10 s each).
- Each rebirth only a little longer than the last (x1.2 to x1.5). requirementGrowth 3 with an additive multiplier makes rebirth 8 take months; about 1.8 keeps it near x1.2 to x1.6.
- Offline pays less than online: efficiency 0.25 to 0.5, cap 8 to 12 hours, measured with the server clock only (future or missing timestamps pay 0).
- Buy max: walk real prices until the next one exceeds the balance or maxLevel; never an uncapped loop.
- Big numbers: round before choosing the suffix or "999950" shows as "1000.0K" (formatNumber handles it). Doubles stay exact only to about 9e15; past 1e15 store a mantissa and exponent.
- Repair saves on load (non-numbers, NaN, infinity, negatives become 0; levels clamp to maxLevel); never save NaN.

## Paid random items (eggs, crates, spins bought with Robux or Robux-bought currency)
Show every outcome with numeric odds summing to 100% before purchase; explain luck boosts and pity numerically and update shown odds while a boost is active; put the full list behind a control labelled "Info" or "Details" (an icon alone is not enough); check PolicyService ArePaidRandomItemsRestricted. Show and roll from the same list (oddsPercent and pickWeighted). See [[07-Monetization/paid-random-items-checklist]].
