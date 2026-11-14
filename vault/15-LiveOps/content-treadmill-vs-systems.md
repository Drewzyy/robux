---
description: Decide between shipping endless new content (treadmill) and building systems that generate play (crafting, prestige, procedural, UGC) for a small Roblox team. Includes a mix ratio and triggers.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: content treadmill, systems, replayability, procedural, prestige, crafting, ugc, content burn, small team, long term, depth, live ops
sources: vault notes 02-Game-Design/procedural-generation, 02-Game-Design/loop-stack, 15-LiveOps/liveops-playbook, consulted 2026-11-14; general design practice (model knowledge)
verify: Ratios are heuristics, UNVERIFIED. No studies cited; treat as a decision aid, not evidence.
tags: [liveops, playbook]
---
# Content treadmill vs systems

Content is consumed; systems keep producing play. A small zero-spend team cannot out-build players' consumption, so invest in systems and use content as the seasoning.

## Do this
- Estimate burn rate: hours of content you ship per week versus hours your regulars consume. If consumption is higher, you are on a treadmill.
- Aim for a mix: about 60 to 70 percent of live-ops effort on systems and tools that multiply play, 30 to 40 percent on fresh content (UNVERIFIED ratio).
- Add a system that turns existing content into new runs: prestige or rebirth, crafting, randomized modifiers, daily seeds, difficulty tiers.
- Make content reusable: zones with variants, items with upgrades, bosses with new mechanics, events that remix old assets.
- Let players create content for each other if the game fits: build slots, outfits, maps, custom servers. Moderation required.
- Watch the late-game: if the top 10 percent of players run out of goals in a week, add a system before another zone.
- Keep a backlog split into "systems" and "content" and pull from both every month.
- Retire content that does not get used, so you do not keep paying maintenance.

## Compare
| Approach | Pros | Cons | Fits |
|---|---|---|---|
| Content treadmill (new zones, items) | Easy to explain, easy to announce | High cost, short life | Story, adventure |
| Systems (prestige, crafting, modifiers) | Long tail, cheap to extend with data | Hard to design, needs balance | Simulator, tycoon, roguelike |
| UGC (build, share, custom rules) | Players supply the content | Moderation, discovery | Sandbox, social |
| Competitive loop (ranked, seasons) | Self-renewing | Needs population | PvP |

## Systems that extend life
- Prestige: reset for a permanent multiplier, with new unlocks per tier (see [[05-Progression/progression-systems]]).
- Crafting and recipes: turn spare drops into goals ([[06-Retention/collection-and-completion]]).
- Modifiers: weekly rule changes ("low gravity week") to remix old levels at almost no art cost.
- Procedural generation: new layouts each run ([[02-Game-Design/procedural-generation]]).
- Roguelike meta-progress: permanent unlocks across runs.
- Ranked seasons: resets give a fresh race.

## When to add content anyway
- Players ask for "more" in the same place (you hit a wall).
- A zone or item is clearly the most loved; extend that style.
- Marketing moments: a new zone for a seasonal event ([[15-LiveOps/event-design]]).
- Early life (first 30 days): small content drops prove the game is alive ([[15-LiveOps/post-launch-first-30-days]]).

## How to build it in Roblox
- Data-driven design: items, zones, recipes in tables or ModuleScripts so a new variant is a row, not a script.
- Use Roblox's free Studio tools and your own assets; never pay for assets.
- Build one reusable "rule modifier" module that other systems can call.
- Track completion of each content piece (reach rate, time spent). Content with under 20 percent reach and low time is a candidate to retire (UNVERIFIED threshold).

## Mistakes
- Releasing a big zone that burns in 2 days and leaves a wasteland.
- Building a system with no content to feed it.
- Over-engineering a system for 10 players.
- Ignoring the casual majority while serving the top 1 percent.

## Decision rule
If regulars quit because they finished everything, add a system (prestige, modifiers). If new players quit before the content, fix onboarding instead ([[04-Onboarding/onboarding-mistakes]]). If both are fine, ship small content to keep the rhythm ([[15-LiveOps/update-cadence]]).

## For our Roblox games
Copy the principle: favor systems that multiply play, use content as a seasoning, keep both data-driven. Never copy another game's zones, items, art, music or logos; make original assets with free tools only.
