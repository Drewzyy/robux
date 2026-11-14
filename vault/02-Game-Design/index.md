---
description: What makes a game worth playing: genre playbooks, core loop, progression, difficulty, economy, first 60 seconds, player psychology.
type: section-index
status: verified
last_checked: 2026-11-09
sources: Research report Part B and C1
verify: scripts/check_vault.py --full checks every link below resolves and every status matches the note.
tags: [game-design, section-index]
---
# 02-Game-Design

What makes a game worth playing: genre playbooks, core loop, progression, difficulty, economy, first 60 seconds, player psychology.

Load only the note your task needs. Do not read every note in this section.

| Note | What it holds | Status |
|---|---|---|
| [[02-Game-Design/genre-playbooks/obby]] | Obby playbook: stage readability, checkpoint mercy, skill vs progression obby, first stage trivially easy. | draft |
| [[02-Game-Design/genre-playbooks/simulator]] | Simulator playbook: click, upgrade, rebirth loop; one price ModuleScript; build order; no trading in game 1. | draft |
| [[02-Game-Design/genre-playbooks/tycoon]] | Tycoon playbook: droppers and buttons, pacing purchases so each feels close, rebirth multipliers. | draft |
| [[02-Game-Design/genre-playbooks/rng]] | RNG playbook: roll, collect, index boosts; free rolls fine everywhere; paid rolls need the Paid Random Items rules. | draft |
| [[02-Game-Design/genre-playbooks/tower-defense]] | Tower defense playbook: loadout, waves, coins and gems, prestige, 2 to 3 week content cadence. | draft |
| [[02-Game-Design/genre-playbooks/farming]] | Farming (Grow a Garden style) playbook: plant, wait, harvest; offline growth from os.time; clock restocks; four reward schedules. | draft |
| [[02-Game-Design/genre-playbooks/roleplay-sandbox]] | Roleplay sandbox playbook: no goals, props for social play, retention from friends and constant updates. | draft |
| [[02-Game-Design/core-loop-template]] | One-page core loop: verb every 5 seconds, reward every 30 seconds, goal every session, goal every week. | draft |
| [[02-Game-Design/game-design-doc-template]] | One-page game design doc template the Architect fills for every pitch. | draft |
| [[02-Game-Design/economy-worksheet]] | Economy worksheet: sources, sinks, prices in one ModuleScript, time-to-next-goal column, failure modes. | draft |
| [[02-Game-Design/difficulty-and-flow]] | Flow channel model, dynamic difficulty adjustment options, obby difficulty pacing, and checkpoint spacing rules with numbers for Roblox obbies and action games. | draft |
| [[02-Game-Design/economy-math]] | Sources and sinks balance, inflation control, idle and offline earnings caps, and additive versus multiplicative multiplier stacking, with a server-side Luau offline-earnings function. | draft |
| [[02-Game-Design/loop-stack]] | Five nested loops (core, session, meta, social, long-term), how each hands off to the next, hit-game examples, and a 60-second test to find which loop is broken. | draft |
| [[02-Game-Design/procedural-generation]] | Procedural generation for Roblox: seeded noise terrain, room and obby stitching, seed handling, server-side generation and the engine limits to watch (streaming, part counts, yielding). | draft |
| [[02-Game-Design/rng-and-drop-tables]] | Weighted random drop tables, pity timers and bad-luck protection, odds display rules for paid random items on Roblox, and seeded RNG, with typed server-side Luau. | draft |
| [[02-Game-Design/combat-design-basics]] | Combat design fundamentals for Roblox: attack, defense, telegraphs, health and damage ranges, enemy roles, boss patterns, and server-side rules that make fights readable on all devices. | draft |
| [[02-Game-Design/genre-playbooks/dress-up-fashion]] | Dress-up and fashion playbook for Roblox: outfit creation, runway and rating loops, wardrobe collection, creative tools, and moderation-safe design aimed at a young, social audience. | draft |
| [[02-Game-Design/genre-playbooks/fighting-pvp]] | Fighting and PvP arena playbook for Roblox: match format, move sets, ranked ladder, matchmaking basics, anti-frustration rules, and spectator hooks. Pair with the fairness and combat notes. | draft |
| [[02-Game-Design/genre-playbooks/horror]] | Horror playbook for Roblox: tension curve, safe rooms, jump scare budget, lighting and sound rules, round structure for co-op or asymmetric horror, and the fairness rules that stop rage quits. | draft |
| [[02-Game-Design/genre-playbooks/incremental-clicker]] | Incremental and clicker playbook for Roblox: click, buy generators, multiply, prestige. Covers cost growth, offline gains, first 10 minutes pacing, and how to avoid a blank spreadsheet feel. | draft |
| [[02-Game-Design/genre-playbooks/minigame-party]] | Minigame party playbook for Roblox: round format, voting, short games, scoring, elimination versus points, lobby flow, and how to add new minigames cheaply without breaking the lobby loop. | draft |
| [[02-Game-Design/genre-playbooks/pet-collecting]] | Pet-collecting playbook for Roblox: hatching, rarity, evolution and trading loops, pet power to economy link, with fairness rules for odds disclosure and trade safety. | draft |
| [[02-Game-Design/genre-playbooks/racing]] | Racing playbook for Roblox: vehicle handling feel, track layout, rubber banding, lap structure, unlock pacing for cars, and multiplayer race flow for short sessions. | draft |
| [[02-Game-Design/genre-playbooks/shooter-fps]] | Shooter and FPS playbook for Roblox: weapon feel, TTK targets, map flow, spawns, loadout progression, aim assist for mobile, and server-authoritative hit detection to resist exploiters. | draft |
| [[02-Game-Design/genre-playbooks/social-hangout]] | Social hangout playbook for Roblox: hub layout, activity stations, emotes, music and photo spots, and low-pressure goals that make people stay and invite friends. No combat, no grind. | draft |
| [[02-Game-Design/genre-playbooks/steal-and-defend]] | Steal-and-defend (base raiding) playbook for Roblox: collect, defend your base, steal from others, with fairness rules, protection timers, and anti-grief design so raiding stays fun for victims. | draft |
| [[02-Game-Design/genre-playbooks/story-adventure]] | Story and adventure playbook for Roblox: chapter structure, quest pacing, cutscene and dialogue length, replay value through choices, and how to ship an episodic game with a small team. | draft |
| [[02-Game-Design/genre-playbooks/survival]] | Survival playbook for Roblox: needs meters, gather-craft-build loop, danger curve across day and night, base building limits, and server-persistence rules for a fair, low-grind survival game. | draft |
| [[02-Game-Design/pvp-fairness-and-balance]] | How to keep Roblox PvP fair: matchmaking, power caps, counterplay, spawn rules, anti-grief, pay-to-win limits, and a balance review loop with win-rate targets. | draft |
| [[02-Game-Design/quest-and-mission-design]] | Quest and mission design for Roblox: objective types, quest chains, markers, rewards scaling, daily quests, and tracking rules that keep new players guided without long text. | draft |
| [[02-Game-Design/world-and-level-design]] | How to lay out Roblox maps and hubs: landmarks, sightlines, path width, hub spokes, density, and wayfinding so players always know where to go, with studs-based numbers to test. | draft |
