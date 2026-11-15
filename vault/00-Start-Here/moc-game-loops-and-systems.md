---
description: Map of notes on core loops, progression, economy math, quests, difficulty, drops, procedural worlds and other systems that make a game work.
type: reference
status: draft
last_checked: 2026-11-15
keywords: core loop, loop stack, progression, economy, difficulty, quests, drop tables, rebirth, prestige, levels, unlocks, systems
sources: vault notes only, read 2026-11-15 (front matter of each linked note)
verify: Built from note descriptions; each linked note carries its own source and verify line.
tags: [start-here, reference]
---
# Game Loops and Systems
Use when designing how the game plays, grows and balances.

## Do this
- Read first, step 1: [[02-Game-Design/core-loop-template]] (the verb, the reward, the goal).
- Read first, step 2: [[02-Game-Design/loop-stack]] (how loops nest).
- Read first, step 3: [[05-Progression/progression-systems]] (currencies, levels, unlocks).
- Then pick the section below that matches your task and read only those notes.

## How to use this map
- Read the three notes under Do this, then open one section below.
- Each note has its own Do this list; act on that, not on this page.
- If a note is marked UNVERIFIED, tune from your own analytics.

## Loops
- [[02-Game-Design/core-loop-template]]: one-page core loop.
- [[02-Game-Design/loop-stack]]: core, session, meta, social, long-term.
- [[02-Game-Design/game-design-doc-template]]: put the loop in the design doc.
- [[02-Game-Design/index]]: section index.

## Progression
- [[05-Progression/progression-systems]]: currencies, multipliers, rebirth, offline.
- [[05-Progression/progression-curves]]: XP and cost formulas.
- [[05-Progression/unlock-pacing]]: how fast to unlock.
- [[05-Progression/rebirth-and-prestige-design]]: when to offer a reset.
- [[05-Progression/pets-and-gear-tiers]]: tier counts and power spacing.
- [[05-Progression/achievements-and-badges]]: goal sizes and badges.
- [[05-Progression/goal-gradient-and-session-goals]]: always show a next goal.

## Economy and drops
- [[02-Game-Design/economy-math]]: sources, sinks, inflation, offline caps.
- [[02-Game-Design/economy-worksheet]]: prices in one ModuleScript.
- [[07-Monetization/economy-pacing]]: cost curves and buy max.
- [[02-Game-Design/rng-and-drop-tables]]: weights, pity, odds display.

## Difficulty and content
- [[02-Game-Design/difficulty-and-flow]]: flow channel and dynamic difficulty.
- [[05-Progression/difficulty-curve-worksheet]]: sawtooth ramp worksheet.
- [[02-Game-Design/quest-and-mission-design]]: objectives, chains, markers.
- [[02-Game-Design/world-and-level-design]]: landmarks, sightlines, hubs.
- [[02-Game-Design/procedural-generation]]: seeded terrain and rooms.
- [[02-Game-Design/combat-design-basics]]: attack, defense, telegraphs.

## Code for systems
- [[13-Luau-Engineering/state-machines]]: typed FSM for rounds, NPCs, abilities.
- [[12-Multiplayer-NPCs/round-system-pattern]]: round loop.
- [[13-Luau-Engineering/datastore-playbook]]: saving progress.
- [[06-Retention/collection-and-completion]]: collections and codex.

## For our Roblox games
- Copy the principle: a short verb loop that pays out often, nested inside slower goals. Never copy another game's exact numbers, item names or art.
- Mark any number you did not measure as UNVERIFIED and tune from your own analytics.
- Resets and random drops must be optional, clear and show their odds.
