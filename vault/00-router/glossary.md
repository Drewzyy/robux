---
description: Shared words for game design, Roblox, Luau, and this studio, each pointing to the note that defines it.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (all parts)
verify: Each term links to the note that defines it; scripts/check_vault.py checks the links resolve.
---
# Glossary

| Term | Meaning | See |
|---|---|---|
| Core loop | the repeated action cycle: verb every 5 s, reward every 30 s | [[10-game-design/core-loop-template]] |
| FTUE | first-time user experience; the first session | [[10-game-design/first-60-seconds-checklist]] |
| Time to first fun | seconds from spawn to the first enjoyable action; under 10 | [[10-game-design/first-60-seconds-checklist]] |
| Open loop | an unfinished, visible goal that pulls the player back (Zeigarnik) | [[10-game-design/player-psychology]] |
| Sawtooth curve | difficulty rises, rests, rises again | [[10-game-design/difficulty-curve-worksheet]] |
| Fixed / variable schedule | predictable vs unpredictable reward timing | [[10-game-design/reward-schedules]] |
| Rebirth / prestige | reset progress for a permanent bonus | [[10-game-design/progression-systems]] |
| D1 / D7 / D30 | share of a day's new players who return 1, 7, 30 days later | [[50-retention-analytics/benchmarks]] |
| Cohort | everyone who first played on the same day | [[50-retention-analytics/analytics-reading-guide]] |
| Play-through rate | share who play after seeing the game in Recommended for You | [[50-retention-analytics/discovery-signals]] |
| First-play bounce | share who leave after a short first session | [[50-retention-analytics/discovery-signals]] |
| QPTR | qualified play-through rate; retired June 2026 | [[50-retention-analytics/discovery-signals]] |
| CCU / DAU / MAU | concurrent / daily / monthly active users | [[50-retention-analytics/benchmarks]] |
| ARPPU | average revenue per paying user | [[50-retention-analytics/benchmarks]] |
| Gamepass | one-time permanent purchase | [[40-monetization/product-catalog-template]] |
| Developer product | repeatable purchase; needs ProcessReceipt | [[40-monetization/processreceipt-template]] |
| ProcessReceipt | server callback that grants developer products | [[40-monetization/processreceipt-template]] |
| Paid random items | anything random bought directly or indirectly with Robux | [[40-monetization/paid-random-items-checklist]] |
| DevEx | Developer Exchange: earned Robux to cash | [[40-monetization/money-math]] |
| Roblox Wallet | daily USD payouts for eligible US creators from Dec 2026 | [[40-monetization/roblox-wallet]] |
| Creator Rewards | Daily Engagement and Audience Expansion payouts | [[40-monetization/money-math]] |
| Earned Robux | Robux from sales or rewards; the only Robux that cash out | [[40-monetization/money-math]] |
| Rojo | syncs files on disk into a Roblox place; builds place files | [[20-luau-rojo/toolchain]] |
| Lune | runs Luau outside Roblox; used for tests | [[20-luau-rojo/headless-test-gate]] |
| Selene / StyLua | Luau linter / formatter | [[20-luau-rojo/toolchain]] |
| ProfileStore | session-locked player data library; successor to ProfileService | [[20-luau-rojo/datastore-playbook]] |
| BindToClose | callback that runs when a server shuts down; last chance to save | [[20-luau-rojo/datastore-playbook]] |
| Remote | RemoteEvent / RemoteFunction; client-server messages, attacker-controlled | [[20-luau-rojo/remotes-and-security]] |
| Studio MCP | Roblox Studio's built-in server that lets agents read and act in an open place | [[80-assets/tool-setup]] |
| Safe area | screen zones UI must avoid: 58 px top bar, bottom thumb controls | [[30-ui-ux/mobile-first-checklist]] |
| Scale vs Offset | UDim2 sizing relative to screen vs fixed pixels | [[30-ui-ux/mobile-first-checklist]] |
| CTR | click-through rate; on Roblox the closest signal is play-through rate | [[80-assets/thumbnail-mrbeast-logic]] |
| Curiosity gap | a thumbnail shows the setup, not the payoff, so the viewer clicks to find out | [[80-assets/thumbnail-mrbeast-logic]] |
| Gate | the checks an output must pass before review; gate.json plus studio checks | [[90-templates-checklists/agent-output-format]] |
| Review queue | where Andrew approves or rejects; nothing ships without it | [[90-templates-checklists/launch-checklist]] |
| UNVERIFIED | a claim not checked against a primary source | [[00-router/how-to-use-this-vault]] |
