---
description: Build a collection log or index for a Roblox game: set sizes, discovery states, progress counts, rewards, DataStore and UI limits, and the rule that paid random drops must show odds.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: collection log, index, codex, pokedex, collectibles, completion, discovery, set bonus, progress bar, badges, datastore, pets, paid random items, odds
sources: creator-docs content/en-us/production/game-design/liveops-essentials.md (read 2026-11-15); creator-docs content/en-us/production/game-design/onboarding.md (read 2026-11-15); creator-docs content/en-us/production/game-design/season-pass-design.md (read 2026-11-15); creator-docs content/en-us/production/publishing/badges.md (read 2026-11-15); creator-docs content/en-us/production/game-design/introduction-to-quest-design.md (read 2026-11-15); creator-docs content/en-us/production/monetization/paid-random-items.md (read 2026-11-15)
verify: Roblox docs only name collections as a goal type and set rules for odds and badges. Set sizes, rarity counts and reward steps below are heuristics from our vault, UNVERIFIED. Badge limits (5 free per 24 hours, 100 Robux each extra, 512 by 512 icon) were read in the docs.
tags: [progression, playbook]
---
# Collection logs and indexes

A collection log (index, codex, album) lists every item a player can find, with gaps shown. It gives short, mid and long term goals without extra content. The retention angle is in [[06-Retention/collection-and-completion]]; this note covers building it.

## Do this
- List all items in one data table with id, name, rarity, source and category. The UI and the save data both read from it.
- Save only discovered item ids per player (a compact list or bitset), not the full table, to stay inside DataStore limits ([[13-Luau-Engineering/datastore-playbook]]).
- Show three states: undiscovered (silhouette and hint), discovered, completed or maxed. Add a counter such as 12 of 60.
- Make every item obtainable without paying. Show where to find undiscovered items as a hint (zone, boss, event) and never as a hidden gamble.
- Break the index into small categories of 6 to 12 items with a reward per category (UNVERIFIED size). Completion of the whole index gets the biggest reward.
- Show progress on the main HUD or menu button so the goal is always visible (the onboarding guide says to surface goals in highly visible places).
- Award a badge for key milestones. Roblox allows 5 free badges per game per 24 hours and 100 Robux for extras, so plan the list.
- Teach the index in onboarding: give the first entries within the first minutes ([[04-Onboarding/onboarding-flow]]).

## Why Roblox lists collections
- The liveops guide places "collections and achievements" among major update types, offering short, mid and long term completionist goals such as collectible pets.
- The season pass guide lists "collection" as a mission category: add, trade or level up items in a collection.
- Achievements are badges: single-task quests that take effort and let players show pride. See [[05-Progression/achievements-and-badges]].

## Design rules
| Rule | Why |
|---|---|
| Always show the next goal | Gaps are the pull; players want the empty slot filled |
| Hints for locked entries | Removes guessing and frustration |
| No time-limited entries that vanish forever | Children can feel pressure or loss; use returning events |
| One source per item is clear | Players know what to do |
| Cap new entries per update | Keep a steady drip ([[15-LiveOps/update-cadence]]) |
| Trading needs safety | Collections encourage trades; see [[12-Multiplayer-NPCs/trading-system-safety]] |

## Random drops and odds
- If items come from a random outcome bought with Robux or currency bought with Robux, odds must be shown before purchase (see [[07-Monetization/paid-random-items-checklist]]). That includes keys, tokens and spin tickets.
- Boosts that raise odds must be explained numerically, and the new odds must update live while active.
- A free random reward (a chest from a key you found) needs no odds disclosure per the docs, but showing rates is still good practice.
- For players where ArePaidRandomItemsRestricted is true, offer a direct purchase, a free path or hide it (age-and-region-policies).
- Never show a "near miss" trick or a rigged animation.

## UI checklist
1. Grid of icons with a scroll list; tap an item for details, source and owned count.
2. Filters by category and by found or missing.
3. Progress bars per category and overall.
4. Notification dot when a new entry unlocks; clear it on view.
5. Sounds and a short animation on first discovery ([[08-UI-UX/reward-moments-and-juice]]).
6. Works at phone size with large touch targets.

## Data and integrity
- Validate on the server that an item was really obtained before marking it discovered; never trust a client remote.
- Version the item table so renaming or removing an item does not delete saves.
- Migrate saves when you add categories; test with an old save.

## For our Roblox games
Copy the principle: a visible index with hints, small sets, honest sources and steady reward steps.

Never copy: another game's item names, creature designs, icons or index layout. No pay-to-complete pressure, no hidden odds, and no fake scarcity.
