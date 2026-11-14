---
description: Tiering pets, weapons, vehicles and gear in a Roblox game: tier count, power spacing, upgrade vs replace, sources per tier, and rules that keep old items useful and paid items fair.
type: reference
status: draft
last_checked: 2026-11-14
keywords: gear tiers, pets, weapons, rarity, power curve, upgrade, fuse, crafting, item levels, vehicles, equipment, sidegrade, power spacing
sources: Author working knowledge of progression design (no pages opened 2026-11-14); Roblox Creator Hub docs, badges and DataStore pages (titles only, not opened 2026-11-14)
verify: Ratios are common conventions, UNVERIFIED as standards. Simulate with the economy worksheet before shipping.
tags: [progression, reference]
---
# Pets and gear tiers

Tiers give players a ladder to climb and a shared language for status. See [[02-Game-Design/economy-math]] and [[02-Game-Design/rng-and-drop-tables]].

## Do this
- Use 5-7 tiers: common, uncommon, rare, epic, legendary, mythic. Add more only with new content.
- Space power by 1.5 to 3 times per tier so each jump is clearly felt.
- Make drop or craft cost grow faster than power (3 to 6 times per tier) so higher tiers feel rare.
- Give each tier a source: tier 1 from shop, tier 2 from zone, tier 3 from crafting, tier 4 from boss, tier 5 from event or luck.
- Let duplicates matter: fuse 3 to 5 into a higher version or convert to upgrade currency.
- Make sideways options: items with different traits (speed, luck, coins) at the same tier.
- Keep the old tier useful: low-tier items still help in fusion or collections.
- Sell cosmetic variants and convenience, not higher tiers. See [[02-Game-Design/pvp-fairness-and-balance]].

## Why it works
- Clear ladder, clear goals. See [[05-Progression/progression-systems]].
- Rarity supports flex and trading. See [[03-Game-Psychology/social-status-and-flex]].
- Variety adds build decisions beyond raw power.

## A sample tier table (illustrative)
- Tier 1: power 1, cost 1, drop 60 percent.
- Tier 2: power 2, cost 4, drop 25 percent.
- Tier 3: power 5, cost 18, drop 10 percent.
- Tier 4: power 12, cost 80, drop 4 percent.
- Tier 5: power 30, cost 350, drop 1 percent.
All UNVERIFIED example numbers; tune from simulation.

## How to build it in Roblox
- One ModuleScript item database: id, tier, power, traits, source, sell value.
- Store owned items as IDs plus counts or upgrade levels.
- Equip rules with slot limits (3 pets, 1 weapon, 1 armor).
- UI shows tier color, power, and "compare to equipped" in one tap.
- Fuse and upgrade done on the server with the cost shown first.
- Version items with a stable ID so rebalances do not break saves.

## Numbers (starting points)
- Upgrade levels per item: 10, each +8 to 12 percent.
- Slots: start at 3, reach 6-8 by 3 hours.
- Item sell value 10-20 percent of cost to avoid exploit loops.
- Launch library: 5 tiers, 6-10 items per tier.

## Mistakes
- One best-in-slot item per slot; players stop exploring.
- Power creep each update that makes older items worthless.
- Hiding tier requirements.
- Selling stronger tiers with real money.
- Inventory overflow with no sort or favorite lock.

## Maintenance rules
- Add a new tier only when you can add a new zone or activity for it.
- When rebalancing, prefer buffing the weak to nerfing the strong, and announce changes.
- Keep a power-per-minute-of-play chart; each tier should raise it by about 2 times.

## For our Roblox games
Copy the structure: 5-7 tiers, power spacing, fusion, sideways traits, free earn paths. Never copy another game's item names, models, or exact stat tables. Create original gear and pets.
