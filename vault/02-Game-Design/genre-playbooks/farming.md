---
description: Farming (Grow a Garden style): plant, wait, harvest; offline growth from os.time; 5-minute clock restocks; four reward schedules at once.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) Part B and C1; rocode.app/guides/how-to-make-a-grow-a-garden-game; geeiq.com/resources/blog/grow-a-garden; tabooharmony roblox-brain player psychology
verify: Mechanics are from community breakdowns of Grow a Garden, not its source code. Verify the offline growth code with a Lune test (growth from planted time) before shipping.
tags: [game-design, playbook]
---
# Farming playbook (Grow a Garden style)

**Loop in one line:** buy seeds, plant, wait, harvest, sell, buy better seeds. The wait is the design.

## Mechanics that make it work
- **Shop restocks every 5 minutes**, same stock on every server, rolled from the clock. A rare seed in stock for 5 minutes is why players come back and talk about the game.
- **Plants grow offline.** Save `Seed` and `PlantedAt = os.time()`. Growth = current time minus planted time. Never a countdown that only runs in a server.
- **Harvest rolls on the server:** weight and rare mutations (example: 2% Golden, worth 5x). Starting values, tune them.
- **Hourly weather mutates plants.**
- **Events keep the core loop** and add new things to collect. They are not separate modes.

## Four reward schedules at once
| Schedule | Here |
|---|---|
| Fixed interval | crops ready on a clock |
| Fixed ratio | coins per harvest sold |
| Variable ratio | mutations |
| Variable interval | weather, unannounced |

## Economy failure modes
- Rare seeds in stock too often kills the waiting.
- Harvests that sell for too much let players finish the seed list in a weekend.
Model both in [[02-Game-Design/economy-worksheet]].

## Notes
Exclusive items claimable only while online and pets from loot boxes exist in Grow a Garden; for game 1 the studio does not sell paid random items ([[07-Monetization/do-not-sell-list]]).
