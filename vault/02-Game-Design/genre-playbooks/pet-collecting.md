---
description: Pet-collecting playbook for Roblox: hatching, rarity, evolution and trading loops, pet power to economy link, with fairness rules for odds disclosure and trade safety.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: pets, hatch, egg, rarity, evolve, trading, collection, companion, gacha, odds, pet simulator, shiny, fuse
sources: Author working knowledge of pet collection games (no pages opened 2026-11-14); Roblox Creator Hub policy on paid random items (title only, not opened 2026-11-14)
verify: UNVERIFIED: the exact current Roblox rules for paid random items; check the Creator Hub policy before shipping any paid egg. Odds below are examples.
tags: [game-design, playbook]
---
# Pet-collecting playbook

Loop: earn currency, hatch pets, equip the best, grow stronger, hatch better pets. See also [[02-Game-Design/rng-and-drop-tables]] and [[02-Game-Design/genre-playbooks/simulator]].

## Do this
- Hatch time under 5 s for the first egg and give a clear reveal animation. Let players skip after 10 hatches.
- Use 4-6 rarity tiers with visible odds in the egg UI; disclose odds before purchase.
- Each pet has a power or multiplier that feeds the core loop (coins per click, speed). Better pets are measurably better.
- Add evolve or fuse: 5 of the same pet become a stronger version. This gives duplicates a use.
- Add a pity counter (guaranteed rare after N hatches) so bad luck ends.
- Let players equip 3-6 pets, and show them following. Pets must look good in a crowd. See [[10-Art-Environment/npc-and-character-design]].
- Trading: use trade windows with confirm steps and a 5-10 s lock on the final button.
- Collection book with silhouettes of unfound pets. See [[05-Progression/achievements-and-badges]].

## Why it works
- Variable rewards and collecting completeness. See [[03-Game-Psychology/reward-schedules]].
- Pets give identity and flex. See [[03-Game-Psychology/social-status-and-flex]].
- Upgrade path: nobody finishes everything, so players return.

## How to build it in Roblox
- Rolls on the server only; never trust a client roll.
- Pet data stored as ID plus a few flags (tier, shiny); not full instances.
- Followers: simple CFrame lerp with a small offset circle; limit per-frame cost on the client.
- Inventory cap with upgradable slots; show a clear "full" warning.
- Trading: both sides lock, review, confirm; log trades for support.
- Mark pets with unique IDs for trade safety and duplicate-bug audits.

## Numbers (example odds, UNVERIFIED as market norms)
- Common 60, uncommon 28, rare 9, epic 2.5, legendary 0.5 percent.
- Pity at 100 hatches for rare, 500 for epic.
- Evolve: 5 pets to 1, power gain 2.5 times.
- Egg cost: each tier 4 to 8 times the previous. See [[02-Game-Design/economy-math]].
- Equip slots start at 3; +1 per milestone.

## Mistakes
- Hidden or inconsistent odds; trust loss and policy risk.
- Pets that look the same at small size.
- Dupe bugs in trading; test with two accounts.
- A single best pet that makes others useless; keep role variety (coin, speed, luck).
- Egg paywall for core progress.

## Collection depth
- Aim for 40-60 pets at launch grouped in 4-5 worlds, with a few cheap seasonal additions.
- Give each pet a name and a one-line story to make it loved.
- Track "pets ever owned" separately from current inventory so trading does not erase the book.

## Playtest check
- Simulate 100000 hatches and compare observed odds with the table.
- Trade the same pet from two accounts at once to hunt for dupes.

## For our Roblox games
Copy the structure: rarity tiers, evolve, pity, disclosed odds, safe trades. Never copy another game's pet names, models, egg art, or sounds. Design an original menagerie; check current Roblox rules on paid random items before selling eggs.
