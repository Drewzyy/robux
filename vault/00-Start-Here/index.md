---
description: The router. Every agent reads this first. Maps a task to the one section index to load next. Pointers only, no content.
type: router
status: verified
last_checked: 2026-11-09
verify: scripts/check_vault.py checks every path in this file exists and that this file stays under 80 lines.
tags: [start-here, router]
---
# Router

## Rules for every agent, every task
1. Never load the whole vault. Never Glob or Grep the vault root. Read this file, then ONE section index, then only the notes that index points to for your task.
2. Load a second section only when a row below sends your task there. Say why in your output.
3. If a note you need is a stub or missing, say so in your output. Do not guess and do not hunt in other sections.
4. Never edit vault notes. Write output only to your run folder.
5. Any claim you use must come from a note with a `verify:` line, or be marked UNVERIFIED in your output.

## Task to section
| If the task is about | Load next |
|---|---|
| how Roblox works, rules, maturity, gambling line, AI content, IP, policy checks | [[01-Roblox-Fundamentals/index]] |
| game idea, genre, core loop, economy, design doc | [[02-Game-Design/index]] |
| motivation, reward schedules, why players stay | [[03-Game-Psychology/index]] |
| first session, first 60 seconds, tutorial | [[04-Onboarding/index]] |
| levels, unlocks, difficulty curve | [[05-Progression/index]] |
| analytics, retention numbers, benchmarks, events | [[06-Retention/index]] |
| prices, gamepasses, developer products, paid random items | [[07-Monetization/index]] |
| UI, HUD, shop screen, menus, mobile layout | [[08-UI-UX/index]] |
| animations, particles, tweens, juice | [[09-Animation-VFX/index]] |
| 3D models, lighting, thumbnails, tool setup | [[10-Art-Environment/index]] |
| sounds, music, the sound sheet | [[11-Audio/index]] |
| multiplayer, social play, NPCs, bots | [[12-Multiplayer-NPCs/index]] |
| Luau code, Rojo, remotes, DataStores, toolchain | [[13-Luau-Engineering/index]] |
| bugs, exploits, performance, playtest checklist | [[14-QA-Optimization/index]] |
| updates after launch, events, weekly review | [[15-LiveOps/index]] |
| other games, teardowns, trends | [[16-Market-Research/index]] |
| templates, launch and update checklists, MVP order | [[17-Development-Templates/index]] |
| where a fact comes from | [[18-Source-Library/index]] |
| what was already decided on a game | [[19-Project-Memory/index]] |
| a word you do not know | [[00-Start-Here/glossary]] |

## Home section per agent
| Agent (role) | Home | Agent (role) | Home |
|---|---|---|---|
| Lead (Executive Producer) | 17 | Pulse (Market Research) | 16 |
| Architect (Game Designer) | 02 | Simulator (Psychology and Retention) | 03 |
| Merchant (Monetization) | 07 | Frame (UX/UI) | 08 |
| Scenic (World and Environment) | 10 | Motion (Animation and VFX) | 09 |
| Librarian (Audio and assets) | 11 | Builder (Luau Engineering) | 13 |
| Crowd (NPC and Multiplayer) | 12 | Playtest (QA and Exploits) | 14 |
| Tuner (Optimization) | 14 | Director (Creative Director) | 02 |
| Release (Release Manager) | 17 | Splash, Liveops, Producer | 10, 15, 15 |

## Other notes in this folder
- [[00-Start-Here/how-to-use-this-vault]]: progressive disclosure rules and the note format, for humans and agents.
- [[00-Start-Here/glossary]]: shared words.
- [[00-Start-Here/policy-change-log]]: dated policy changes and what they changed in the vault.
| [[00-Start-Here/recommended-tools]] | Free tool per job, costs, cautions about plugins. | draft |
