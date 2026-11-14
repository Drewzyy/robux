---
description: What to give a new player in session 1 and when: an early win in 30 s, a guaranteed upgrade by minute 3, a visible goal for next time, and no gating behind purchase. Zero-spend rewards only.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: first session, reward, early win, starter pack, free item, unlock, goal gradient, return hook, day 2, onboarding, pet, upgrade
sources: vault notes 03-Game-Psychology/reward-schedules, 05-Progression/goal-gradient-and-session-goals, consulted 2026-11-14; general game-design practice (model knowledge)
verify: Timings are heuristics, UNVERIFIED against Roblox data. Validate with funnel events (reward_claimed time and D1 by reward cohort).
tags: [onboarding, playbook]
---
# First-session rewards

A new player decides in minutes. Rewards are proof the game pays back effort. This note covers what to give and when; schedules and psychology live in [[03-Game-Psychology/reward-schedules]].

## Do this
- Deliver a first reward within 30 s of the first input (a number going up, a coin, a pet hatch, a visible unlock). Target under 10 s for the first feedback tick.
- Guarantee a meaningful upgrade by minute 2 to 3: a new item, a stronger tool, a bigger zone. It must be felt, not a +1 percent stat.
- Make the second reward bigger than the first and the third bigger again. Early curve is steep, then flatten (see [[05-Progression/progression-curves]]).
- Show the next unlock as a progress bar that is 60 to 80 percent full by the end of the tutorial (goal gradient, see [[05-Progression/goal-gradient-and-session-goals]]).
- End session 1 with an open loop: a half-built goal, an egg that hatches in a few hours, a daily reward that unlocks tomorrow. This is the day-2 hook (see [[06-Retention/day-1-day-7-day-30-playbook]]).
- Give one rare-feeling free item in session 1 (an RNG roll with a lucky first result is allowed, but never promise odds you do not deliver). Never hide it behind Robux.
- Make every reward claimable by tapping once. No multi-screen claim flow.
- Save the reward state in DataStore the moment it is granted, so a quick disconnect does not lose the first win.

## Reward ladder for session 1 (template)
| Time | Reward | Purpose |
|---|---|---|
| 0:10 | first currency tick plus sound | proves the verb works |
| 0:30 | first collectible or hatch | first dopamine |
| 1:30 | first upgrade purchase, almost free | teaches the economy loop |
| 3:00 | zone or tool unlock | sense of progress |
| 5:00 | named goal with bar at 70 percent | return hook |
| session end | "come back in X" timer or daily reward preview | day-2 pull |

## Rules of thumb
- Newbie economy faster than the mid game: first purchases cost 1 to 3 taps worth of income (UNVERIFIED number).
- Never gate session-1 progress on a timer. Timers belong after the player is invested.
- Cosmetic first wins (a hat, a trail) feel personal and cost no balance. Good zero-spend reward.
- Do not show the shop in the first minute. First sale prompt is after the first upgrade, never during the tutorial (see [[08-UI-UX/shop-ui]]).

## How to build it in Roblox
- Reward grants happen on the server (RemoteEvent request, server validates, then grants). Client plays sound and particles.
- Store `firstSessionStep` and `rewardsClaimed` in the profile. Log reward_claimed with seconds since join.
- Use TweenService for number counting up and a ParticleEmitter burst on grant; pair with a short sound (see [[08-UI-UX/ui-sound-pairing]]).

## Measure
- Time from join to first reward (median) and the share who never get one.
- D1 for players who claimed all 3 early rewards versus those who did not. A big gap means the ladder works, or that engaged players claim more; confirm with an A/B on reward timing.

## Common mistakes
- Reward is a pop-up the player must read and dismiss before playing again. Auto-claim and show a toast instead.
- First upgrade is too expensive, so the player grinds with no feeling of progress.
- Reward numbers are huge and mean nothing (1,000,000 coins with no scale). Use small numbers early so each tick is felt.
- All rewards come in the first minute and then nothing for 10 minutes. Space them so the player always sees the next one within 60 to 90 s.
- Rewards tied to a paid pass in session 1. Zero-spend studio and new players both lose.

## Example reward copy (original, short)
- "+1 Coin" with a rising pitch sound, "New tool unlocked", "Zone 2 is open", "Your egg hatches in 20 minutes". Keep each under 5 words.

## Handoff to retention
The last reward of session 1 should point to something that exists tomorrow. See [[06-Retention/day-1-day-7-day-30-playbook]] and [[03-Game-Psychology/streaks-and-daily-rewards]].

## For our Roblox games
Copy the principle: early guaranteed wins, steep first curve, open loop at session end. Never copy another game's reward names, pets or characters, art, sounds or logos, and never lock early rewards behind paid items.
