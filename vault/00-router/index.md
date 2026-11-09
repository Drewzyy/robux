---
description: The router. Every agent reads this first. Maps a task to the one section index to load next. Pointers only, no content.
type: router
status: verified
last_checked: 2026-11-09
verify: scripts/check_vault.py checks every path in this file exists and that this file stays under 80 lines.
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
| game idea, genre, core loop, progression, economy, difficulty curve, first 60 seconds, player psychology | [[10-game-design/index]] |
| Luau code, Rojo project, framework, remotes, security, DataStore/ProfileStore, toolchain, performance | [[20-luau-rojo/index]] |
| UI, HUD, shop screen, menus, mobile layout, thumbnail or icon specs | [[30-ui-ux/index]] |
| prices, gamepasses, developer products, DevEx, Roblox Wallet, Creator Rewards, paid random items | [[40-monetization/index]] |
| analytics, retention, benchmarks, events, live ops calendar | [[50-retention-analytics/index]] |
| Roblox rules, content maturity, gambling line, AI content, IP, pre-launch policy check | [[60-policy/index]] |
| other games, teardowns, trends, market data | [[70-competitors/index]] |
| 3D models, audio, VFX/SFX, thumbnail pipeline, tool setup | [[80-assets/index]] |
| launch, update, playtest, anti-slop checklists; templates; MVP build order | [[90-templates-checklists/index]] |
| a word you do not know | [[00-router/glossary]] |

## Home section per agent
Start here unless the table above sends the task elsewhere.
| Agent | Home section |
|---|---|
| Pulse | 70-competitors |
| Lead | 90-templates-checklists |
| Architect | 10-game-design |
| Builder | 20-luau-rojo |
| Scenic | 80-assets |
| Librarian | 80-assets |
| Frame | 30-ui-ux |
| Playtest | 90-templates-checklists |
| Splash | 30-ui-ux |
| Merchant | 40-monetization |
| Liveops | 50-retention-analytics |
| Producer | 50-retention-analytics |

## Other notes in this folder
- [[00-router/how-to-use-this-vault]]: progressive disclosure rules and the note format, for humans and agents.
- [[00-router/glossary]]: shared words.
- [[00-router/policy-change-log]]: dated policy changes and what they changed in the vault.
- [[00-router/sources]]: every source, with the date checked.
