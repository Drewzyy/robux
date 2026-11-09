from common import note
note("00-router/glossary", "Shared words for game design, Roblox, Luau, and this studio, each pointing to the note that defines it.", "reference", "draft",
 "Research report (all parts)", "Each term links to the note that defines it; scripts/check_vault.py checks the links resolve.", r"""
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
| Gate | the checks an output must pass before review; gate.json plus studio checks | [[90-templates-checklists/agent-output-format]] |
| Review queue | where Andrew approves or rejects; nothing ships without it | [[90-templates-checklists/launch-checklist]] |
| UNVERIFIED | a claim not checked against a primary source | [[00-router/how-to-use-this-vault]] |
""")

note("00-router/policy-change-log", "Dated Roblox policy and platform changes, newest first, with the source and what each changed in this vault.", "reference", "draft",
 "Research report Part D and C4 to C6", "Each entry names its source; the authoritative dated table lives in 60-policy/policy-change-watch. Re-check sources before launch.", r"""
# Policy change log

Newest first. Add an entry whenever a change alters a vault note, with the source.

| Date | Change | Vault notes changed | Source |
|---|---|---|---|
| 2026-11-09 | Vault created. God plan corrections recorded: Roblox Wallet added; QPTR and 7-day window replaced; cross-game pass sales off; ProfileStore over ProfileService; benchmarks re-labeled | 40-monetization/roblox-wallet, money-math; 50-retention-analytics/discovery-signals, benchmarks; 20-luau-rojo/datastore-playbook | Research report Part D |
| Dec 2026 (announced) | Roblox Wallet opt-in (US 18+ sole proprietors); DevEx US transition mid-2027 | 40-monetization/roblox-wallet | gamertagmythras.com, livecards.net, en.creatorshub.info |
| 2026-06 | Discovery: 28-day window; QPTR retired | 50-retention-analytics/discovery-signals | Roblox announcement as summarized; creator-docs discovery.md |
| 2026-05-30 | Cross-game pass sales disabled | 40-monetization/product-catalog-template | moonetix.com |
| 2026-05-26 | Paid Random Items policy live | 40-monetization/paid-random-items-checklist | zehn-studio26.com and others |

Full dated list: [[60-policy/policy-change-watch]].
""")

note("00-router/sources", "Every source behind the vault, grouped by topic, with the date checked. Most are secondary; the primary ones are marked.", "reference", "draft",
 "Research report source list (all URLs checked live Nov 9, 2026 by the research)",
 "URLs copied from the research report, which says each was live on 2026-11-09. Primary sources are marked; spot checks are logged in LOG.md Phase 3.", r"""
# Sources

All checked live on 2026-11-09 by the research report unless noted. **P** = primary (Roblox or the tool's own docs). Others are secondary or community.

## Roblox (primary)
- P: Creator Docs, discovery: https://github.com/Roblox/creator-docs/blob/main/content/en-us/discovery.md
- P: Creator Docs, content maturity: https://github.com/Roblox/creator-docs/blob/main/content/en-us/production/promotion/content-maturity.md
- P: Onboarding: https://create.roblox.com/docs/production/game-design/onboarding and https://create.roblox.com/docs/production/game-design/onboarding-techniques
- P: LiveOps essentials: https://create.roblox.com/docs/production/game-design/liveops-essentials
- P: Terms of Use: https://en.help.roblox.com/hc/en-us/articles/115004647846

## Benchmarks
- GameAnalytics 2026 Roblox report: https://www.gameanalytics.com/reports/2026-roblox-report
- BLOXG retention guide: https://bloxg.com/guides/roblox-player-retention
- Yeno, RoWatcher (two), Craftane, Obby.fun analytics guides (URLs in the research report)

## Money and policy (secondary)
- Moonetix: https://moonetix.com/academy/monetization/how-roblox-developers-make-money
- Obby.fun Creator Rewards: https://www.obby.fun/blog/roblox-creator-rewards-guide
- GamerTagMythras DevEx guide and RDC 2026: https://gamertagmythras.com/blog/roblox/roblox-devex-monetization-guide , https://gamertagmythras.com/blog/roblox/roblox-rdc-2026-announcements
- LiveCards Wallet: https://livecards.net/en/all-blogs/roblox-wallet-explained
- Creators Hub Wallet: https://en.creatorshub.info/roblox-wallet-daily-creator-payouts-airwallex/
- Zehn Studio odds disclosure: https://zehn-studio26.com/news/paid-random-items-odds-disclosure/
- TechTimes Korea loot boxes: https://www.techtimes.com/articles/319148/20260626/koreas-loot-box-rules-push-roblox-disclose-item-odds-worldwide.htm
- AntiMedia gambling: https://antimedia.co/news/roblox-gambling-2026
- We Are Family: https://we-are-family.com/uk/blog-insights/roblox-at-20-growing-up-in-public/

## Luau, Rojo, security, data (community)
pinheadsk8r, ivar-anon, brockmartin, tabooharmony (via igosha), justozy, chguo999, mattqdev, kanzerog, carmodyofficial-ops, quanticsoul4772, justinclevenger, el4cteo, andrian-syh (GitHub URLs in the research report).
- Verified locally (2026-11-09): Rojo 7.7.1, Lune 0.11.1, Selene 0.31.0, StyLua 2.6.0 from crates.io.

## Game design and psychology
- gamedesign.gg (flow, difficulty, Roblox design), gdad.wiki, falcon.so, gamedev-canon, Yu-kai Chou (Octalysis), Quantic Foundry reference PDF, guul.games, adriancrook.com, helpshift, wizuslabs.
- Papers as cited: Ryan, Rigby, Przybylski 2006 (DOI 10.1007/s11031-006-9051-8); Cutting et al. 2023 (Royal Society Open Science); Fong et al. 2015 (Journal of Positive Psychology); Wojtasinski et al. 2025.

## Genres and competitors
rocode.app (genres, Grow a Garden), geeiq.com, bloxplan.app, pvpbank.com, studio23, studiokrew, newgame.co.jp, tbod wiki.

## UI
ashexplained, dev.to (Veronica Aalin), simplified.media, johnygorsky, wilfredy, thumbblox, umatechnology, obby.fun icon maker.

## Not in this repo (internal)
- Roblox god plan `business-plans/roblox-god-plan.md`, `PIPELINE.md`, `roblox-games/sky-tower-obby/`, `game-idea-template.md`: on Andrew's Mac.
- Research report `agent-tool/vault-research.md`: pasted into the build prompt; not yet committed.
""")
print("00 done")
