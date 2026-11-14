---
description: Using collections, sets, codex pages and completion rewards to drive long-term retention on Roblox: set sizes, rarity tiers, progress display, milestone rewards, and ethics of RNG.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: collection, completion, codex, set bonus, rarity, pets, index, milestone, completionist, rng, drop rate, album, progress bar, retention
sources: vault notes 02-Game-Design/rng-and-drop-tables, 16-Market-Research/teardown-pokemon-go-and-collecting, 03-Game-Psychology/reward-schedules, consulted 2026-11-14; general design practice (model knowledge)
verify: Set sizes and percentages are heuristics, UNVERIFIED. Check Roblox rules on paid random items (odds disclosure) before any paid RNG.
tags: [retention, playbook]
---
# Collection and completion

A visible set with gaps pulls players back. The incomplete page is the hook. Teardown context: [[16-Market-Research/teardown-pokemon-go-and-collecting]].

## Do this
- Show an index or codex with silhouettes for undiscovered items and a count ("12 of 60"). Gaps drive the return.
- Group items into small sets of 6 to 12 with a set bonus, so completion feels reachable every few sessions (UNVERIFIED sizes).
- Give completion rewards at 25, 50, 75 and 100 percent of the whole index, each bigger than the last.
- Use 4 to 6 rarity tiers with clear colors; the top tier should feel rare but be obtainable by an active free player within weeks.
- Make some items earnable by skill or activity (not only RNG): boss drops, quests, events. Pure RNG frustrates.
- Add pity or bad-luck protection: guaranteed rare after N pulls (see [[02-Game-Design/rng-and-drop-tables]]).
- Rotate limited items through [[15-LiveOps/event-design]] so the collection grows, but let players see which are missable and which return.
- Never sell completion directly; keep paid options out of a zero-spend plan.

## Structure
| Layer | Example size | Purpose |
|---|---|---|
| Item | 1 | A drop |
| Set | 6 to 12 | Short goal, set bonus |
| Category or zone | 30 to 60 | Mid goal, title reward |
| Whole index | 100 to 300 | Long goal, prestige cosmetic |

## Show progress well
- Progress bar and numeric count on the main collection button.
- Notification dot on new, unseen items.
- "Closest to complete" highlight: show the set with one missing item first.
- Hint where to find a missing item (zone name, event name), never leave the player guessing. Hints make completion feel fair.

## How to build it in Roblox
- Store owned item ids as a table in the profile (a bitmask or set of ids, keep DataStore size small).
- Server grants items and checks set completion; client renders the codex via UI from a static item table.
- Use ViewportFrame to show 3D item previews in the codex (see [[08-UI-UX/roblox-ui-production]]).
- Log item_found with source and rarity; check actual drop rates against design rates.

## Numbers
- Common to rare ratio of about 60/25/10/4/1 percent is a typical shape for 5 tiers (UNVERIFIED, tune by feel).
- A free active player should see a new collectible every 5 to 10 minutes early and every few sessions late.
- Check that completion of the full index takes weeks to months, not days (UNVERIFIED target).

## Ethics and policy
- If any item comes from a random roll tied to Robux, Roblox requires odds disclosure (verify current policy). A zero-spend plan should avoid paid rolls.
- Do not use countdown pressure on rare pulls.
- Allow players to see odds in the UI even for free rolls; it builds trust.

## Mistakes
- Index too large too early, discouraging players.
- Duplicate-heavy RNG with no use for duplicates (give duplicate conversion into currency or progress).
- Missable items with no return path, causing resentment.
- Hidden drop rates.

## For our Roblox games
Copy the principle: visible gaps, small sets, tiered milestones, pity protection, fair hints. Never copy another game's creatures, item names, rarity art, music or logos; design original collectibles.
