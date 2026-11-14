---
description: Why Pokemon Go and monster-collecting games hook players (collection book, rarity, walking, events, trading) with rules for building a collection loop in Roblox without copying characters.
type: teardown
status: draft
last_checked: 2026-11-14
keywords: pokemon go, pokemon, collecting, collection book, rarity, trading, events, community day, completionism, niantic, pets, silhouettes
sources: https://www.denofgeek.com/culture/where-have-all-the-online-gaming-spaces-for-kids-gone/ (existing vault source); general knowledge of Niantic and Pokemon public design (no page re-read 2026-11-14)
verify: UNVERIFIED: spawn rates, event cadence and player numbers are from memory and not cited. Check current official pages if exact values matter.
tags: [market-research, teardown]
---
# Teardown: Pokemon Go and collecting
A collection game that turned the real world into a map. The core pull is "gotta fill the list", plus events that gather people.

## Do this
- Show a collection book from minute one with every slot as a dark silhouette and a number. Missing items are the hook ([[03-Game-Psychology/lessons-from-classic-games]]).
- Make the first catch guaranteed within 60 seconds and the first rare within the first session.
- Use 4 to 5 rarity tiers with drop rates like 60, 25, 10, 4, 1 percent. Show the tier with color and a sound so rare ones feel special.
- Add a search or hint for missing items ("found in the forest, evening") so players have a plan, not just luck.
- Run a weekly short event (1 to 3 hours) where one type spawns 3 to 5 times as often, with a bonus; announce it in game 2 days ahead.
- Add trading with limits (fair-trade checks, one trade per 10 seconds, cooldown for newly obtained items) so trades are safe. See [[03-Game-Psychology/play-with-friends]].
- Add a duplicate sink: duplicates convert to a currency that levels or evolves one item, so no catch is wasted.

## Why it got huge
- Nostalgia plus an existing brand drew in adults and kids at once. This is the part we cannot copy.
- It used real places: a walk became a game; groups gathered at spots.
- Simple catch action, large number of collectibles, strong visuals.
- Heavy social buzz at launch. UNVERIFIED how much was hype vs design.

## Why people play
- Collection and completion drive: the lizard brain wants to fill the set.
- Exploration and moving around.
- Friendly competition and showing off.

## Why they stay
- 100s of items means long goals; each sub-set (region, type) is a mini goal.
- Evolution and power-up systems give another axis.
- Friends and a local community.

## Why they come back
- Events and spawn boosts (community-day style) with a fixed time.
- New items released in waves; seasons rotate.
- Daily tasks and a streak bonus. See [[03-Game-Psychology/streaks-and-daily-rewards]].

## Roblox build notes
- Collection index stored as a set of ids in the profile; the UI iterates a master list so unseen entries show as silhouettes.
- Weighted random with a pity counter (guarantee a rare within N tries) to avoid long dry streaks. See [[03-Game-Psychology/reward-schedules]] and roblox-player-psychology.
- Spawns server-side, shown to the client only when near, so you can keep part counts low.
- Event timers use server time, announced via MessagingService.
- No paid random boxes for kids in rule-sensitive areas; sell guaranteed bundles instead.

## Mistakes
- Collection too large with no sub-goals: feels endless. Break into pages of 12 to 20.
- Rare items that are impossible without paying.
- Trading that allows scams: show both sides, require confirm twice.

## For our Roblox games
Copy: silhouette book, rarity tiers, duplicate sink, timed spawn events, safe trading, pity. Never copy: Pokemon, Pikachu-like designs, names, sounds, battle UI, logos. Make original creatures and your own theme. Related: [[03-Game-Psychology/motivation-coverage-checklist]].

## Collection page layout (suggested)
- Pages of 12 slots in a 4 by 3 grid, large icons, rarity color on the frame.
- Header shows "7 of 12" and a progress bar; finishing a page gives a one-time reward and a small celebration (burst plus sound).
- Tapping a silhouette shows where to find it, not its full art.
- A "new" dot on freshly caught items; clear it when viewed.
