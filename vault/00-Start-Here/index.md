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

## Shortcuts
Not sure where to look? [[00-Start-Here/question-router]] maps 90+ common questions to the 1-3 notes that answer them. Building a whole game? Follow [[00-Start-Here/moc-make-a-game-start-to-finish]].

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
| [[00-Start-Here/moc-animation-and-effects]] | Map of notes on animations, particles, tweens, hit feedback, emotes and reward moments, with cheap-on-phones rules. | verified |
| [[00-Start-Here/moc-code-and-performance]] | Map of notes on Luau engineering, project setup, security, data saving, testing, bugs, profiling, and low-end phone performance. | verified |
| [[00-Start-Here/moc-game-loops-and-systems]] | Map of notes on core loops, progression, economy math, quests, difficulty, drops, procedural worlds and other systems that make a game work. | verified |
| [[00-Start-Here/moc-genres]] | Every genre playbook, teardown and player-review note grouped by genre, plus cross-genre pattern notes. | verified |
| [[00-Start-Here/moc-graphics-and-feel]] | Map of notes on how a game looks and feels: art direction, lighting, models, textures, thumbnails, UI look, and game feel rules. | verified |
| [[00-Start-Here/moc-keep-players-playing]] | Map of notes on why players stay or leave: psychology, onboarding, retention levers, analytics, churn, events, and returning-player care. | verified |
| [[00-Start-Here/moc-make-a-game-start-to-finish]] | Ordered path from idea to launch and the first 30 days: each step lists the 2 to 5 vault notes to read, in order. | verified |
| [[00-Start-Here/moc-money]] | Map of notes on earning Robux fairly: shop design, pricing, gamepasses, developer products, receipts, payouts, DevEx, and what never to sell. | verified |
| [[00-Start-Here/moc-multiplayer]] | Map of notes on servers, rounds, lobbies, teams, trading, leaderboards, NPCs, bots, bosses, streaming and social play. | verified |
| [[00-Start-Here/moc-research-and-trends]] | Map of notes on market research: hit patterns, teardowns, competitor tracking, discovery signals, benchmarks, policy watch, and sources. | verified |
| [[00-Start-Here/moc-sound]] | Map of notes on game audio: feedback sounds, music, mixing, settings menu, free licensed sources, and the ForgeGUI pipeline. | verified |
| [[00-Start-Here/moc-ui]] | Map of notes on Roblox interface work: mobile-first layout, HUD, shop screens, fonts, colour, accessibility, animation and Figma handoff. | verified |
| [[00-Start-Here/question-router]] | 70+ common questions (daily reward, quit at minute 2, shop prices, exploits, lag) each mapped to the 1 to 3 vault notes that answer it. | verified |
| [[00-Start-Here/top-roblox-repos]] | Ranked table of 30 popular Roblox and Luau GitHub repos with stars, licenses, why each matters for building games, and which vault note covers it; skips spam and exploit repos. | draft |
