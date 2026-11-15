---
description: Season pass design from Roblox's own guide: goals, 1 month seasons, 10 tiers, free and premium tracks, daily missions, claim buttons, catch-up XP, plus kid-safe limits on pressure and FOMO.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: season pass, battle pass, tiers, missions, xp, premium pass, free pass, final reward, season length, daily missions, catch-up, liveops, progression, claim
sources: creator-docs content/en-us/production/game-design/season-pass-design.md (read 2026-11-15); creator-docs content/en-us/resources/feature-packages/season-passes.md (read 2026-11-15); creator-docs content/en-us/production/game-design/liveops-essentials.md (read 2026-11-15)
verify: Numbers (one month season, ten tiers, one week rest, double XP final week) come from the Roblox guide and are starting points, not proven targets. Pricing, XP per tier and conversion are not in the docs: UNVERIFIED. Our anti-FOMO limits are studio policy, not Roblox policy.
tags: [progression, playbook]
---
# Battle pass and season design

A season pass is a time-bound, quest-based progression system: players finish missions, earn pass XP, climb tiers and collect rewards. It is different from a one-time game pass that sells a bonus. This note follows the Roblox guide and adds limits for a game with children.

## Do this
- Define goals first: align with the core loop, motivate, be understandable at a glance, and make players look forward to the next season.
- Start with a 1 month season and at least a 1 week rest before the next, which protects players from burnout and gives the team time. (Roblox guide)
- Start with 10 tiers. Players should see progress often but not face a wall at the start. (Roblox guide)
- Enroll everyone automatically in the free track each season, with no opt-in to remember.
- Make missions cover every system: exploration, collection, combat, social, customization, survival. Aim for at least three categories.
- Unlock new missions daily. Mix easy, medium and hard. Never require spending currency to complete a mission.
- Make claiming a button press in the pass UI, so players see what they got and are brought back into the screen. Auto-award anything unclaimed at season end.
- Show remaining time, tier progress and rewards clearly. Add a badge on the button when something is ready to claim.
- Add a catch-up: double XP in the final week, so behind players can finish. (Roblox guide)

## Structure
| Part | Guideline |
|---|---|
| Season length | About 1 month, 1 week gap |
| Tiers | About 10 to start |
| Tier rewards | Wanted but not too valuable: consumables, small currency, short buffs, small cosmetics |
| Final reward | A new, exciting 3D item or cosmetic shown in the world, as a capstone |
| Free track | Daily missions, own rewards, no payment |
| Premium track | Extra missions and bigger rewards for a paid pass; price should match reward value |
| Upgrade mid season | Award all premium rewards for tiers already done, so waiting is never punished |

## Mission design rules (docs)
- New missions every day; missions reset daily (finish or lose that day's progress).
- Vary category and difficulty; rarely repeat; have many unique missions.
- Free players get daily missions; premium players get extra bonus missions that are visible to all but only completable by premium. That is the main selling point.
- Do not make daily missions too hard. If many players fail, interest drops. Leave time to play freely.
- Weekly missions with bigger XP serve high achievers; cooperative missions add a social reason.
- Difficulty comes from count (eat 5 vs 25) and from task type (combat harder than driving).

## Planning and production
- Asset request: list evergreen assets (season UI, missions screen, mission icons) and new-each-season assets (final reward, themed marketing).
- Tech request: data hooks for each mission, seasons UI, progress tracking.
- Roblox ships a Season Passes feature package (beta) that needs Core and Missions packages and DataStore API access; it needs configuration and balance testing. Check licenses and read the code before use ([[10-Art-Environment/no-toolbox-rule]]).
- The guide links a tier balance calculator spreadsheet and UI wireframes (PDFs).

## Limits we add for children
- The free track must be a complete, enjoyable path. Premium adds extras, never power that decides fights.
- Show price in Robux, rewards, and what is included before purchase; no countdown pressure, and no repeated pop-ups ([[07-Monetization/offer-pressure-limits]]).
- Purchase prompts only from a player tap on the pass screen.
- Do not use limited "never returns" rewards as a pressure tool. The Roblox guide suggests retiring final rewards for exclusivity; we prefer returning them later in a shop or next season (our policy).
- No loss of paid progress at season end. Do not erase paid tier rewards or make players feel they wasted money.
- No missions that require daily login on pain of loss, or that require talking to others. See [[03-Game-Psychology/streaks-and-daily-rewards]].
- Keep quest and mission language calm and clear ([[02-Game-Design/quest-and-mission-design]]).

## Measure
- Share of players who open the pass screen, claim a tier, finish 5 and 10 tiers, and return next day. Use funnel events (ftue-analytics-funnel).
- Watch complaint volume and abuse reports after launch.
- Pair with event cadence ([[15-LiveOps/event-design]]) and progression pacing ([[05-Progression/unlock-pacing]]).

## For our Roblox games
Copy the principle: short seasons with a free track everyone finishes, varied daily missions, a claim button, and a catch-up week.

Never copy: another game's pass name, art, reward items, UI layout or mission text (the guide's Jailbreak and Dragon Adventures examples are for learning). No paid asset kits, no countdown pressure, no hidden odds in pass rewards.
