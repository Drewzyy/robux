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
| Core loop | the repeated action cycle: verb every 5 s, reward every 30 s | [[02-Game-Design/core-loop-template]] |
| FTUE | first-time user experience; the first session | [[04-Onboarding/first-60-seconds-checklist]] |
| Time to first fun | seconds from spawn to the first enjoyable action; under 10 | [[04-Onboarding/first-60-seconds-checklist]] |
| Open loop | an unfinished, visible goal that pulls the player back (Zeigarnik) | [[03-Game-Psychology/player-psychology]] |
| Sawtooth curve | difficulty rises, rests, rises again | [[05-Progression/difficulty-curve-worksheet]] |
| Fixed / variable schedule | predictable vs unpredictable reward timing | [[03-Game-Psychology/reward-schedules]] |
| Rebirth / prestige | reset progress for a permanent bonus | [[05-Progression/progression-systems]] |
| D1 / D7 / D30 | share of a day's new players who return 1, 7, 30 days later | [[06-Retention/benchmarks]] |
| Cohort | everyone who first played on the same day | [[06-Retention/analytics-reading-guide]] |
| Play-through rate | share who play after seeing the game in Recommended for You | [[06-Retention/discovery-signals]] |
| First-play bounce | share who leave after a short first session | [[06-Retention/discovery-signals]] |
| QPTR | qualified play-through rate; retired June 2026 | [[06-Retention/discovery-signals]] |
| CCU / DAU / MAU | concurrent / daily / monthly active users | [[06-Retention/benchmarks]] |
| ARPPU | average revenue per paying user | [[06-Retention/benchmarks]] |
| Gamepass | one-time permanent purchase | [[07-Monetization/product-catalog-template]] |
| Developer product | repeatable purchase; needs ProcessReceipt | [[07-Monetization/processreceipt-template]] |
| ProcessReceipt | server callback that grants developer products | [[07-Monetization/processreceipt-template]] |
| Paid random items | anything random bought directly or indirectly with Robux | [[07-Monetization/paid-random-items-checklist]] |
| DevEx | Developer Exchange: earned Robux to cash | [[07-Monetization/money-math]] |
| Roblox Wallet | daily USD payouts for eligible US creators from Dec 2026 | [[07-Monetization/roblox-wallet]] |
| Creator Rewards | Daily Engagement and Audience Expansion payouts | [[07-Monetization/money-math]] |
| Earned Robux | Robux from sales or rewards; the only Robux that cash out | [[07-Monetization/money-math]] |
| Rojo | syncs files on disk into a Roblox place; builds place files | [[13-Luau-Engineering/toolchain]] |
| Lune | runs Luau outside Roblox; used for tests | [[13-Luau-Engineering/headless-test-gate]] |
| Selene / StyLua | Luau linter / formatter | [[13-Luau-Engineering/toolchain]] |
| ProfileStore | session-locked player data library; successor to ProfileService | [[13-Luau-Engineering/datastore-playbook]] |
| BindToClose | callback that runs when a server shuts down; last chance to save | [[13-Luau-Engineering/datastore-playbook]] |
| Remote | RemoteEvent / RemoteFunction; client-server messages, attacker-controlled | [[13-Luau-Engineering/remotes-and-security]] |
| Studio MCP | Roblox Studio's built-in server that lets agents read and act in an open place | [[10-Art-Environment/tool-setup]] |
| Safe area | screen zones UI must avoid: 58 px top bar, bottom thumb controls | [[08-UI-UX/mobile-first-checklist]] |
| Scale vs Offset | UDim2 sizing relative to screen vs fixed pixels | [[08-UI-UX/mobile-first-checklist]] |
| CTR | click-through rate; on Roblox the closest signal is play-through rate | [[10-Art-Environment/thumbnail-mrbeast-logic]] |
| Curiosity gap | a thumbnail shows the setup, not the payoff, so the viewer clicks to find out | [[10-Art-Environment/thumbnail-mrbeast-logic]] |
| Gate | the checks an output must pass before review; gate.json plus studio checks | [[17-Development-Templates/agent-output-format]] |
| Review queue | where Andrew approves or rejects; nothing ships without it | [[17-Development-Templates/launch-checklist]] |
| UNVERIFIED | a claim not checked against a primary source | [[00-Start-Here/how-to-use-this-vault]] |
