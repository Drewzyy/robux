from common import note
S = "10-game-design"
R = "Research report (Nov 9 2026) Part B and C1"

note(f"{S}/genre-playbooks/obby", "Obby playbook: readable stages, checkpoints as mercy, skill vs progression obby, first stage trivially easy, first success within 30 seconds.", "playbook", "draft",
 R + "; rocode.app/guides/roblox-game-genres; Tower of Hell teardown (70-competitors)",
 "Design rules are community and practitioner guidance, not Roblox rules. Check against Creator Analytics after launch: first-play bounce under 60s and D1 vs similar-game benchmarks.", r"""
# Obby playbook

**Core loop:** jump numbered stages, reach the next checkpoint.
**Hard part:** a difficulty curve that does not lose players early.

## Two models, pick one on purpose
| | Skill obby (Tower of Hell) | Progression obby |
|---|---|---|
| Checkpoints | none | every stage or every few |
| Round | short, timed (ToH: 8 minutes, random tower) | open-ended |
| Retention from | mastery, rivalry, spectating, "one more run" | coins, checkpoints, finish line, best times |
| Who finishes | most never do | most can |

A middle model (checkpoints plus timed races, best times, leaderboards) is what the god plan picks for Sky Tower Obby (god plan not in this repo; flagged UNVERIFIED here).

## Rules
- Stage 1 is trivially easy. First success within 30 seconds of spawn.
- Every stage has a readable solution before the player jumps: telegraph hazards, readable silhouettes, contrast against the background.
- Sawtooth difficulty (see [[10-game-design/difficulty-curve-worksheet]]): each block of 5 to 10 stages introduces one new hazard, dips easy, practices, peaks, then resets with the next mechanic.
- One new idea at a time. Never a new mechanic and a new difficulty jump together.
- Death to retry under 2 seconds, one input, no screens in between. Checkpoints are the mercy system: failure costs seconds, not progress.
- Hidden mercy after 3 to 5 deaths at one spot (extra checkpoint, slower hazard), never announced. UNVERIFIED for Roblox specifically; it is a general design pattern (Crash Bandicoot).
- Progression rewards on fixed schedules (coins per stage). Variable only for bonus surprises (secret coins, rare drop).
- Checkpoints are never sold. Skip Stage as a developer product is a judgment call; see [[40-monetization/do-not-sell-list]].

## Motivation coverage
Achievers: stages, leaderboards, collections. Explorers: secret coins, hidden paths, new tower themes. Socializers: co-play, parties, visiting. Killers: timed races and leaderboards; keep PvP optional and friendly for kids.

## What kills an obby
Long walk to stage 1; unreadable stage 3; a shop pop-up on death; lag on join; a new mechanic introduced at a difficulty peak.
""")

note(f"{S}/genre-playbooks/simulator", "Simulator playbook: click, upgrade, rebirth loop; every price in one ModuleScript; build order; no trading in game 1.", "playbook", "draft",
 R + "; rocode.app/guides/roblox-game-genres", "Community guidance. Verify the curve with the economy worksheet before launch and with economy events (AnalyticsService) after.", r"""
# Simulator playbook

**Core loop:** click (or act) for currency, buy upgrades, rebirth for a multiplier.
**Hard part:** numbers that grow for weeks without breaking.

## Rules
- Every price, rate, and multiplier lives in one shared ModuleScript (`src/shared/Data/Economy.luau`, frozen with `table.freeze`), so the whole curve is tuned in one place and the server and client agree.
- Build order: core loop on a baseplate first, then progression, then saving, then UI, then monetization.
- Rebirth resets base progress for a permanent multiplier. Show the multiplier before the player commits.
- Do not build trading in game 1. Trading needs a working economy, moderation, and scam protection.
- Model the curve in [[10-game-design/economy-worksheet]] before coding: time to next upgrade must stay short early and grow slowly.

## Reward schedules
Fixed ratio: currency per action. Fixed: upgrade ladder. Variable: rare drops, eggs (free only in game 1; paid random items need [[40-monetization/paid-random-items-checklist]]).
""")

note(f"{S}/genre-playbooks/tycoon", "Tycoon playbook: droppers and buttons, pacing prices so each purchase feels close, rebirth for permanent multipliers.", "playbook", "draft",
 R + "; rocode.app/guides/roblox-game-genres; newgame.co.jp Gun Armory Tycoon guide; tbod-tycoon-but-one-dropper.wiki rebirth guide",
 "Community guidance. Verify pacing with the economy worksheet (time-to-next-purchase column) and with playtest timing of the first 5 purchases.", r"""
# Tycoon playbook

**Core loop:** droppers make money, buttons buy more droppers.
**Hard part:** pacing prices so each purchase feels close.

## Classic loop
Collect cash, buy buttons, unlock areas, (in combat tycoons) arm up and fight, repeat.

## Rules
- The next button is always visible with its price. The player should almost always be within reach of a purchase in the first session.
- Rebirth: after buying every button, reset base progress for permanent multipliers, rebirth tokens, or advanced tiers. Delaying a reset too long gives diminishing returns, so show the player when rebirthing pays off.
- All prices in one ModuleScript; model them in [[10-game-design/economy-worksheet]].
- First purchase inside 30 seconds of spawn (see [[10-game-design/first-60-seconds-checklist]]).
- Social: visiting friends' bases is the cheapest social hook.
""")

note(f"{S}/genre-playbooks/rng", "RNG playbook: roll, collect, index boosts; free rolls fine everywhere; paid rolls need the Paid Random Items rules and region gating.", "playbook", "draft",
 R + "; rocode.app/guides/roblox-game-genres; Paid Random Items reporting (see 40-monetization)",
 "Odds rules are Roblox policy as reported in 2026 secondary sources. Re-read the Paid Random Items policy in Creator Hub before building any paid roll.", r"""
# RNG playbook

**Core loop:** roll for rare results, collect, roll again.
**Hard part:** odds design and the Paid Random Items rules.

## Rules
- Free rolls are allowed everywhere.
- Paid rolls are Paid Random Items: numeric odds for every outcome shown before purchase, summing to 100%; every outcome has value; region gating with PolicyService; fail closed. Full list: [[40-monetization/paid-random-items-checklist]]. Luck boosts, pity systems, rerolls, and keys bought with Robux all count as paid random items.
- Not in game 1 (studio rule, see [[40-monetization/do-not-sell-list]]).
- Example structure (Tycoon RNG, from RoCode): odds displayed like 1/94.5B, an index that unlocks permanent boosts as you collect, rebirth unlocking new tiers of rarer machines.
- Progress must not depend only on luck. Pair the variable reward with a fixed track (index completion, levels) so competence is visible. See [[10-game-design/reward-schedules]].
- Audience is mostly minors. Do not build the core retention loop on paid RNG.
""")

note(f"{S}/genre-playbooks/tower-defense", "Tower defense playbook: loadout, waves, coins and gems, prestige, new towers or maps every 2 to 3 weeks.", "playbook", "draft",
 R + "; bloxplan.app/examples/tower-defense (Tower Defense Simulator practice)", "Community guidance from one example source. Verify wave balance by playtest (quit-rate per wave) before launch.", r"""
# Tower defense playbook

**Core loop:** place towers, survive waves, earn cash, upgrade.
**Hard part:** targeting and wave balance.

## Proven structure
1. Pick a loadout before the match.
2. Defend escalating waves. Coins per kill, gems per boss.
3. Upgrade mid-match with coins.
4. Between matches spend gems on permanent unlocks.
5. Prestige at level cap for stars, exclusive skins, and a permanent damage bonus.

## Cadence
New towers or maps every 2 to 3 weeks (Tower Defense Simulator practice).

## Monetization in this genre
Sold on power and progression, not only cosmetics: extra loadout slots, an exclusive tower, double XP, VIP with a daily gem drip. Check every one against [[40-monetization/do-not-sell-list]]; never sell the win.

## Difficulty
Sawtooth by wave block. A wave that kills most new players is a quit point; log wave reached on loss (funnel event).
""")

note(f"{S}/genre-playbooks/farming", "Farming (Grow a Garden style): plant, wait, harvest; offline growth from os.time; 5-minute clock restocks; four reward schedules at once.", "playbook", "draft",
 R + "; rocode.app/guides/how-to-make-a-grow-a-garden-game; geeiq.com/resources/blog/grow-a-garden; tabooharmony roblox-brain player psychology",
 "Mechanics are from community breakdowns of Grow a Garden, not its source code. Verify the offline growth code with a Lune test (growth from planted time) before shipping.", r"""
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
Model both in [[10-game-design/economy-worksheet]].

## Notes
Exclusive items claimable only while online and pets from loot boxes exist in Grow a Garden; for game 1 the studio does not sell paid random items ([[40-monetization/do-not-sell-list]]).
""")

note(f"{S}/genre-playbooks/roleplay-sandbox", "Roleplay sandbox (Brookhaven style): no goals, props for social play, retention from friends and constant updates. Hardest to copy with code alone.", "playbook", "draft",
 R + "; pvpbank.com roblox-2026-hot-games-guide", "Community analysis of one game. No numeric claims to verify.", r"""
# Roleplay sandbox playbook

No goals at all. A social sandbox: houses, vehicles, and public spaces are props for improvised role-play, not a victory condition.

- Longevity comes from social play plus constant updates that add new secrets and scenes.
- This is the strongest retention model on Roblox and the hardest to copy with code alone: the players make the content.
- Not a fit for game 1 of a solo studio. Useful lesson for every genre: friends make the same game different every session.
- If roleplay chat or private spaces are central, check the maturity rules in [[60-policy/content-maturity-guide]] (social hangouts and private spaces push toward Restricted).
""")

note(f"{S}/core-loop-template", "One-page core loop: the verb every 5 seconds, the reward every 30 seconds, the goal every session, the goal every week.", "template", "draft",
 R + " (C1, B0)", "Template. Verify a filled copy by playtest: time the first verb, first reward, and first open loop with a stopwatch.", r"""
# Core loop template

Fill one per game. If a line is blank, the game is not designed yet.

| Timescale | Question | Answer |
|---|---|---|
| Every 5 seconds | What is the one verb the player does? | |
| Every 30 seconds | What reward lands, and how does it look and sound? | |
| Every session (median 9.8 min) | What goal can be finished in one session? | |
| Tomorrow | What reason is there to come back tomorrow (timer, daily reward, friend)? | |
| This week | What longer goal (collection, event, leaderboard)? | |
| This month | What is the identity or long goal (season, mastery)? | |

## Checks
- The 5-second verb is fun on its own with clear feedback (Koster: fun is learning patterns).
- The 30-second reward is fixed-schedule; surprises are extra, never the only path.
- The session goal ends on a peak (level up, rare drop, celebration) and shows the next goal (peak-end rule, Zeigarnik effect).
- At least one social reason to return ([[10-game-design/player-psychology]]).
""")

note(f"{S}/game-design-doc-template", "One-page game design doc the Architect fills for every pitch: 11 sections from pitch to what we will not do.", "template", "draft",
 "Architect skill (vault/.claude/skills/architect); " + R, "Checked by scripts/gates/check_stage.py (architect): core loop, first 60 seconds windows, and 'will not do' sections must be present.", r"""
# Game design doc template (one page)

1. **Pitch and genre.** One sentence. Name the genre and the one core verb.
2. **Core loop.** From [[10-game-design/core-loop-template]].
3. **First 60 seconds**, second by second: 0-5 spawn and orient, 5-10 first core verb, 10-30 first reward, 30-60 open loop. See [[10-game-design/first-60-seconds-checklist]].
4. **Mechanics.** For each: what the player does, what it teaches, what can go wrong.
5. **Progression.** Fixed schedules for progression, variable only for surprise ([[10-game-design/reward-schedules]]).
6. **Difficulty or economy curve.** Sawtooth; retry under 2 seconds ([[10-game-design/difficulty-curve-worksheet]] or [[10-game-design/economy-worksheet]]).
7. **Social hooks**, cheapest first: join friends, invite, shared goals, friend leaderboard.
8. **Lobby and layout.** What the player sees from spawn.
9. **What we will not do.** Pay-to-win, gated finish, punishing free path, paid random items in game 1 ([[40-monetization/do-not-sell-list]]).
10. **Open questions and UNVERIFIED claims.**
11. **Loaded.** Every vault note read, and why.

Every number is a starting value to tune, and says so.
""")

note(f"{S}/progression-systems", "Currencies, multipliers, rebirth/prestige, collections, levels, offline progress, scarcity: what each does and when to use it.", "reference", "draft",
 R + " (B2, B5, C1)", "Design guidance; no numeric claims. Verify per game through D7 retention (low D7 usually means weak progression, per Roblox guidance as summarized in the report).", r"""
# Progression systems

| System | What it gives the player | Use when | Watch out |
|---|---|---|---|
| Soft currency | visible progress every action | almost always | one currency in game 1; more currencies means more confusion |
| Levels / XP bar | competence feedback (fixed ratio) | any game with repeat play | low early thresholds so the first level-up lands in session 1 |
| Multipliers | numbers going up faster | simulators, tycoons | must live in one tuning module |
| Rebirth / prestige | a fresh start with a permanent bonus | after content runs out (tycoon, TD, simulator) | show the payoff before the reset |
| Collections / index | long goals, completion drive | pets, RNG, farming | completion should give something (boost, title) |
| Offline progress / timers | a reason to come back tomorrow | farming, idle | compute from saved timestamps, never server countdowns |
| Scarcity (restocks, limited items) | urgency, talk | farming, events | free scarcity is fine; paid FOMO on kids is not |
| Best times / leaderboards | mastery and rivalry | obby, racing | friends leaderboards beat global ones |

## Rules
- Fixed schedules carry progression; variable schedules are for surprise ([[10-game-design/reward-schedules]]).
- Endowed progress: start bars partly filled, never at zero (goal gradient effect).
- Low D7 usually means a weak progression system; low D30 means missing end-game goals ([[50-retention-analytics/decision-tree]]).
""")

note(f"{S}/difficulty-curve-worksheet", "Stage-by-stage difficulty worksheet: sawtooth ramp, one new idea per block, quit-rate column, retune rule, retry under 2 seconds.", "template", "draft",
 R + " (B6); gamedesign.gg difficulty-curves; Celeste and Super Meat Boy conventions as cited", "Template. A filled copy is verified by playtest plus a funnel event per stage (stage reached) after launch.", r"""
# Difficulty curve worksheet

Two curves: **challenge** (what the game demands) and **capability** (what the player brings). Only the gap is felt. Raise difficulty just after the player shows they have the pattern.

## Shape: sawtooth
Introduce a mechanic, dip easy while the player is a novice again, practice, peak, then introduce the next mechanic and dip again. Linear ramps feel too hard early and too easy late; exponential ramps hit a wall.

## Worksheet
| Stage | New idea (one at most) | Difficulty 1-10 | Rest beat? | Retry cost (seconds) | Players reaching (%) | Quit here (%) | Action |
|---|---|---|---|---|---|---|---|
| 1 | core verb only | 1 | | <2 | | | |
| 2 | | | | | | | |

## Rules
- Stage 1 is nearly unfailable and pays out a win.
- Teach, repeat with one twist, combine. Re-test old skills while teaching new ones.
- Retry under 2 seconds, one input, no confirmation dialogs or death screens between player and retry.
- Failure must be legible: the player can say what went wrong and sees a path.
- Raise difficulty on mastery events, never on a clock.

## Retune rule
If a stage's quit rate is far above its neighbours, first make the solution more readable, then add a checkpoint or rest beat before it, and only then lower difficulty. Change one thing at a time and wait 72 hours before judging ([[50-retention-analytics/analytics-reading-guide]]).
""")

note(f"{S}/economy-worksheet", "Economy worksheet: sources, sinks, prices in one ModuleScript, time-to-next-goal column, and the failure modes that break games.", "template", "draft",
 R + " (C1, C5)", "Template. Verify a filled copy with a Lune test that simulates the first hour of income and prints time-to-next-goal per purchase.", r"""
# Economy worksheet

## Sources and sinks
| Source (earned) | Amount | Per | Sink (spent) | Amount | Notes |
|---|---|---|---|---|---|
| | | | | | |

A missing sink is the classic hidden inflation bug. Log both with AnalyticsService economy events, with balance after each transaction ([[50-retention-analytics/event-taxonomy]]).

## Price ladder
| Item | Price | Income per minute at that point | Time to afford (min) | Feels close? |
|---|---|---|---|---|
| | | | | |

Keep early time-to-afford short (under a minute in session 1); let it grow slowly.

## Rules
- All prices and rates in one frozen shared ModuleScript, read by server and client.
- Server does all currency math; client only displays.
- Pure functions (price for level, income for upgrades) live in `src/shared` so Lune can test them.

## Failure modes
- Rare items available too often: no reason to wait or return.
- Sell prices too high: players finish the content in a weekend.
- Prices that jump: a wall that reads as bait and switch.
""")

note(f"{S}/first-60-seconds-checklist", "FTUE checklist: 0-5s orient, 5-10s core verb, 10-30s first reward, 30-60s open loop. Teach by doing; time to first fun under 10 seconds.", "checklist", "draft",
 R + " (B4); create.roblox.com/docs/production/game-design/onboarding; wizuslabs FTUE; tabooharmony roblox-brain",
 "Verify by stopwatch in a playtest (all four windows hit) and after launch by first-play bounce rate (under-60s segment) in Creator Analytics.", r"""
# First 60 seconds checklist

## Timeline
- [ ] **0-5 s:** spawn inside the world (no logo, cutscene, or dialogue wall). Player sees the goal, the reward, and the input at a glance. Roblox 3-second rule: what do I do, what do I get, how do I do it.
- [ ] **5-10 s:** the player does the core verb.
- [ ] **10-30 s:** first reward lands, with a visual and sound celebration.
- [ ] **30-60 s:** an open loop: the next milestone is shown and looks close (an incomplete goal, a bar at 80%).
- [ ] Time to first fun under 10 seconds.

## Teaching
- [ ] Teach by doing. The first level is the tutorial, in a space that cannot punish not knowing.
- [ ] One mechanic, one button, one goal in the first minute. Never two new ideas at once.
- [ ] Just-in-time prompts (when the action is first needed), arrows and highlights, fading after mastery. No text walls.
- [ ] Minimal HUD first, then action HUD, then meta HUD (progressive disclosure).

## Kills Day 1
- [ ] No unskippable intro or long tutorial.
- [ ] No shop in the first 30 seconds (studio rule: no shop pop-up in the first 5 minutes).
- [ ] No long walk to the fun.
- [ ] No lag on join; players do not diagnose performance, they leave.
- [ ] No "like the game" ask before the first reward.

## Supporting effects
Zeigarnik (unfinished tasks pull players back), goal gradient (effort speeds up near a goal; start bars partly full), peak-end (put the best effect in minute one, end sessions on a celebration).

## Caution
Forced tutorials can hurt retention. Teach functional literacy fast; leave strategy to discovery. Instrument the first minute as a funnel and A/B test changes ([[50-retention-analytics/event-taxonomy]]).
""")

note(f"{S}/reward-schedules", "Fixed ratio, variable ratio, fixed interval, variable interval: where each belongs in a Roblox game, and the ethics line for a young audience.", "reference", "draft",
 R + " (B2); Skinner 1938; Ferster and Skinner 1957; Yu-kai Chou on operant conditioning",
 "Theory from operant conditioning literature as summarized in the report. Roblox paid-random rules re-checked in 40-monetization before use.", r"""
# Reward schedules

| Schedule | Rule | Behaviour it produces | Use for |
|---|---|---|---|
| Continuous | every action rewarded | fast learning, fast extinction | onboarding only |
| Fixed ratio | every Nth action | steady work, pause after reward | coins per stage, XP per kill |
| Variable ratio | unpredictable count, average rate | highest rate, most persistent | free random drops, rare pets, mutations |
| Fixed interval | after a set time | low activity then a rush near the time | daily reward, crops ready |
| Variable interval | after unpredictable time | steady checking | weather, surprise events |

## Rules
- Fixed schedules for progression: the player knows what the next reward is and when it comes (competence).
- Variable schedules for surprise. Never make the whole game variable: if progress is only random, competence feedback disappears. If only fixed, it becomes a grind.
- Reward timing matters more than reward size.

## Ethics line (kids and teens)
Variable ratio is also the most compulsive schedule. Free random rewards are fine. Paid random items follow [[40-monetization/paid-random-items-checklist]] and are not in game 1. Do not build paid RNG as the core retention loop for minors.
""")

note(f"{S}/player-psychology", "Flow (contested), SDT, Bartle, Quantic Foundry, Octalysis, Hook, Fogg, sessions, difficulty, and social design, condensed with sources and caveats.", "reference", "draft",
 R + " (Part B, all sources listed there)",
 "Each claim carries its source below. Research claims are as summarized in the report, not re-read from primary papers; treat effect sizes as UNVERIFIED until checked against the paper.", r"""
# Player psychology (condensed)

## The stack
Moment to moment, the core loop is fun with clear feedback (Koster: fun is learning patterns; a mastered pattern stops being fun, noise frustrates). Session to session, challenge tracks rising skill, rewards are well timed, and the player leaves with a visible next goal. Day to day, habits, daily reasons, progression, and friends. Long term, identity, ownership, community, mastery. Social ties are the cheapest strong lever.

## Flow (Csikszentmihalyi, 1990)
Clear goals, immediate feedback, challenge matched to skill; the flow channel sits between anxiety and boredom and moves as skill grows. Raise difficulty in waves with rest beats (Schell); flow beyond 15 to 20 minutes is tiring.
**Contested:** Cutting et al., Royal Society Open Science, 2023, preregistered, n=311, found no significant effect of covertly held challenge-skill balance on enjoyment. A meta-analysis finds balance correlates with flow around r=.56 (Fong et al., 2015, 28 studies). Another finds flow higher when perceived skill exceeds challenge (Wojtasinski et al., 2025). Flow is a lens, not a law.

## Self-Determination Theory (Deci and Ryan)
Autonomy (meaningful choice), competence (growth, feedback), relatedness (others matter). Ryan, Rigby, Przybylski 2006 (DOI 10.1007/s11031-006-9051-8): autonomy and competence in play predicted enjoyment; relatedness predicted future play in multiplayer. Too much extrinsic reward risks the overjustification effect.

## Bartle (1996)
Achievers, Explorers, Socializers, Killers. A coverage check, not a measurement ([[10-game-design/motivation-coverage-checklist]]).

## Quantic Foundry (Yee and Ducheneaut)
12 motivations in 6 clusters: Action (Destruction, Excitement), Social (Competition, Community), Mastery (Challenge, Strategy), Achievement (Completion, Power), Immersion (Fantasy, Story), Creativity (Design, Discovery). Continuous scores. Claims that profiles predict churn and spend are Quantic Foundry's own: UNVERIFIED.

## Octalysis (Yu-kai Chou)
8 core drives: Epic Meaning, Accomplishment, Creativity and Feedback, Ownership, Social Influence, Scarcity, Unpredictability, Loss Avoidance. White Hat (1-3) empowering but low urgency; Black Hat (6-8) urgent but draining. Balance them; do not build a kids game mainly on Black Hat drives. Practitioner framework, not peer-reviewed; adoption numbers on his site are UNVERIFIED.

## Hook Model (Eyal, 2014)
Trigger, action, variable reward (tribe, hunt, self), investment. Investment (streaks, collections, bases) loads tomorrow's trigger.

## Fogg Behavior Model
Behavior = Motivation x Ability x Prompt at the same moment. Low motivation: make the action tiny. No prompt, no behavior.

## Sessions
GameAnalytics 2026 (500+ titles at 1M+ MAU): median session 9.8 minutes per game; median player 3 sessions and 22.4 minutes a day across Roblox. Design a full loop inside one session plus a reason to return. Session length is a diagnostic, not a target; padding does not cause retention. The "66 days to form a habit" figure varies widely in the underlying research; do not treat it as a rule.

## Social
Belonging is a basic need (Baumeister and Leary). Social sunk cost and the social calendar bring players back. Cheapest: gifting, visiting, friend leaderboards. Mid: co-op, trading, parties, private servers. Highest stickiness and cost: guilds. Roblox ranks intentional co-play days as an important discovery signal. Numbers like "3x more likely to return" from marketing blogs are UNVERIFIED.

## Roblox order
Fix D1 retention and session time first, then D7 and D30, then payer conversion, then acquisition.
""")

note(f"{S}/motivation-coverage-checklist", "Does the game give Achievers, Explorers, Socializers, and Killers something specific to do? Fill one row each, naming the mechanic.", "checklist", "draft",
 R + " (B3b, B9)", "Checklist. Verified per game by having each row name a real mechanic that exists in the build.", r"""
# Motivation coverage checklist

| Type | Wants | This game gives them (name the mechanic) | Exists in build? |
|---|---|---|---|
| Achiever | goals, points, levels, completion | | |
| Explorer | secrets, lore, systems | | |
| Socializer | doing things with others | | |
| Killer | competition, dominance | | |

## Rules
- A blank row is a gap, not a failure; decide on purpose.
- For a young audience, serve Killers with races and leaderboards, not forced PvP.
- Quantic Foundry's 12 motivations are a finer check when a row is ambiguous ([[10-game-design/player-psychology]]).

## Sky Tower Obby reading (from the report, B9)
Achievers: stages, leaderboards, collections. Explorers: secret coins, hidden paths, new tower themes. Socializers: co-play, parties, visiting. Killers: timed races and leaderboards, PvP optional.
""")
print("10 done")
