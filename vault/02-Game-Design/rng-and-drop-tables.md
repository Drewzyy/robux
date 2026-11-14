---
description: Weighted random drop tables, pity timers and bad-luck protection, odds display rules for paid random items on Roblox, and seeded RNG, with typed server-side Luau.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: rng, drop table, weighted random, pity timer, bad luck protection, odds, loot, egg, crate, paid random items, Random.new, seed, probability, luck boost, PolicyService
sources: Roblox Creator Docs, Paid random items policy guidelines, https://create.roblox.com/docs/en-us/production/monetization/paid-random-items (found via search 2026-11-14, page itself could not be fetched, summary from search snippet); Roblox Creator Docs, Random datatype, https://create.roblox.com/docs/reference/engine/datatypes/Random (search summary 2026-11-14); DevForum, Why seeds do not work for Random.new, https://devforum.roblox.com/t/why-seeds-do-not-work-for-randomnew/2938777 (seen in search 2026-11-14, not opened); existing notes reward-schedules
verify: Policy points come from a search summary of the official page (docs metadata said last updated 2026-08-19); READ THE PAGE before shipping any paid random item. Pity numbers are design heuristics, UNVERIFIED. Random seed behavior from docs summary.
tags: [game-design, playbook]
---
# RNG and drop tables

How to roll rewards fairly, soften streaks of bad luck, and stay inside Roblox policy. Schedule theory is in [[03-Game-Psychology/reward-schedules]]. Paid random items are not in game 1; see [[07-Monetization/paid-random-items-checklist]].

## Do this
- Roll on the server only, with one `Random` object per system (create once, reuse). Never roll on the client or trust a client "luck" value.
- Store drop tables as data: `{ id, weight }` entries in a shared ModuleScript. Probability = weight / sum of weights.
- Free random rewards: add pity (guaranteed rare after N misses) so no one is stuck at 0 for hours.
- Paid or Robux-purchasable random outcomes: show every possible outcome and its true probability as a percent, summing to exactly 100%, before purchase (Roblox policy, per Creator Docs summary).
- Luck boosts, pity counters and anything that changes odds are in scope of that policy: explain them before purchase and update displayed odds while active (per docs summary).
- Check `PolicyService:GetPolicyInfoForPlayerAsync(player).ArePaidRandomItemsRestricted`; if true, block paid random purchase and trade of results for that player (per docs summary; confirm field name).
- Log every roll (table id, result, pity count) so you can audit real drop rates against displayed ones.
- Keep the wow-rare tier at 0.1-1% for free drops; with pity at about 2-3x the expected rolls (e.g. 1% drop, hard pity 200). Heuristic, UNVERIFIED.

## Weighted random
```lua
--!strict
export type Entry = { id: string, weight: number }

local function rollWeighted(rng: Random, table_: {Entry}): string
	local total = 0
	for _, e in table_ do total += e.weight end
	local r = rng:NextNumber() * total
	local acc = 0
	for _, e in table_ do
		acc += e.weight
		if r < acc then return e.id end
	end
	return table_[#table_].id -- float edge case
end
```
`Random:NextNumber()` returns 0 to 1 (docs summary), so use `r < acc`. Display odds from the same table: `string.format("%.2f%%", e.weight / total * 100)`. Round so the displayed list still sums to 100.00; put the rounding remainder on the most common item.

## Pity timer
- Counter per player per table, saved in the profile. Each miss on the rare tier increments it; a hit resets it.
- Soft pity: from N_soft, add to the rare weight each roll: `weight_rare = base + (pity - N_soft) * step`. Hard pity: at N_hard force the rare.
```lua
--!strict
local function rareWeight(base: number, pity: number, softStart: number, step: number, hard: number): number
	if pity + 1 >= hard then return math.huge end
	if pity < softStart then return base end
	return base + (pity - softStart) * step
end
```
- Expected rolls with p per roll and hard cap H: below 1/p without cap; pity bounds the worst case at H. Worked: p=1%, no pity: 5% of players need over 298 rolls (1 - 0.99^298 = 0.95). With hard pity 150, nobody does.
- Probability of no hit in n rolls: `(1-p)^n`. Use it to choose H so the unlucky tail is acceptable (e.g. under 5% of players reach 70% of H).
- Bad-luck protection variants: duplicate protection (re-roll duplicates for collection games), streak insurance (first rare in the first 20 rolls is boosted), cooldown on rare repeats.
- Pity counts and effective odds are part of odds disclosure when the roll is paid (docs summary).

## Seeded RNG
- `Random.new(seed)` gives a reproducible stream; the seed is truncated to an integer (0 and 0.99 give the same stream, per docs summary). Use `os.time()` or `math.random(1e9)` stored with the result when you want to replay.
- Seed per round or per player-day for shared "daily" content: `Random.new(dayNumber * 7919 + placeId)`. Same seed, same sequence, on server and client.
- Do not reseed per roll (`Random.new(os.clock())` each call correlates results). Create once.
- Deterministic replays: log seed + call order; any added or reordered `NextNumber` call changes the stream.
- `math.noise` cannot be seeded; add a seed offset to coordinates (community source, see [[02-Game-Design/procedural-generation]]).

## Mistakes
- Displayed odds that differ from rolled odds (policy risk, trust loss).
- Boosts that change odds without updating the display.
- Client-side rolls or client-supplied luck values.
- Pity counter not saved, so rejoining resets it.
- Rolling in a loop with a new `Random` each iteration.

## For our Roblox games
Copy: data-driven tables, server roll, pity, honest displayed percentages, audit logs. Never copy another game's creature or item names, rarity colour schemes, egg art, odds numbers or sounds. Keep paid random out of the core loop for a young audience.
