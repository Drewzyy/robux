---
description: A reward every 10 to 30 seconds in the first 5 minutes, then a cadence curve that stretches to hours, with a minute-by-minute schedule, reward types and tuning rules.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: early rewards, first five minutes, reward cadence, onboarding, first session, quick wins, reward curve, new player, starter pack, tutorial rewards, pacing
sources: creator-docs content/en-us/production/game-design/monetization-foundations.md (read 2026-11-15, starter pack definition); creator-docs content/en-us/production/game-design/introduction-to-quest-design.md (read 2026-11-15); creator-docs content/en-us/production/game-design/season-pass-design.md (read 2026-11-15); general onboarding practice, paraphrased
verify: Docs support early starter offers and frequent progress. The 10 to 30 second cadence and the curve numbers are studio targets, UNVERIFIED; check against our own D1 and first-session funnel.
tags: [onboarding, playbook]
---
# Fast early rewards

New players decide in the first minutes. Give something small and clear very often at the start, then slowly widen the gaps so the game does not run out of reward.

## Do this
- Plan a reward every 10 to 30 seconds from second 0 to minute 5. A reward is anything the player notices: coins, XP tick, item, unlock, sound plus popup.
- Make the first reward land by second 10 and be tied to the first action ("You picked it up!").
- Include one real unlock by minute 2 and one named item by minute 4 (a pet, tool, or cosmetic they keep).
- Rewards must come from play, not from waiting or paying. No offer popup in the first 2 minutes (see [[07-Monetization/offer-pressure-limits]]).
- Teach the next step with each reward ("Now try the shop").
- Use the four-beat reveal, scaled small (see [[08-UI-UX/reward-moments-and-juice]]).
- Never fake it: no bars that jump, no free rewards that are taken back later.
- After minute 5, stretch gaps along the curve below, never drop to nothing.

## First 5 minutes schedule
| Time | Reward | Type | Size |
|---|---|---|---|
| 0:10 | First pickup or hit | Coin or XP tick plus sound | Tiny |
| 0:30 | Complete first mini-goal | XP bar fills, small chime | Small |
| 0:50 | Level 2 | Level-up burst, new ability glimpse | Medium |
| 1:15 | First currency milestone | Counter tick, "You can buy X soon" | Small |
| 1:40 | First unlock | New tool or area opens | Medium |
| 2:10 | First quest done | Item plus quest line | Medium |
| 2:40 | Collection slot filled | Index opens, "1 of 12" | Small |
| 3:10 | Level 3 | Burst plus new ability | Medium |
| 3:40 | Free chest or gift | Weighted, odds shown if random | Medium |
| 4:10 | Named item (pet or cosmetic) | Visible on player | Large |
| 4:40 | Next goal shown | Bar at 20 percent already | Small |
| 5:00 | "Come back tomorrow" | Daily reward preview | Small |

## Cadence curve after minute 5
| Window | Reward gap | Notes |
|---|---|---|
| 0 to 5 min | 10 to 30 s | Tiny and small mixed, one medium per minute |
| 5 to 15 min | 30 to 60 s | One medium per 2 min, one large by minute 12 |
| 15 to 30 min | 1 to 2 min | Goals 3 to 5 min long, visible bars |
| 30 to 60 min | 2 to 5 min | Mid goals, collection, first prestige hint |
| 1 hour plus | 5 to 15 min | Long goals with small rewards between |
| Day 2 plus | Daily reward, quests | Return hook, see [[03-Game-Psychology/streaks-and-daily-rewards]] |

## How to build it in Roblox
- A `RewardSchedule` ModuleScript with entries `{time, id, type, amount}` so designers tune without code. Trigger by event (first pickup) rather than raw time where possible.
- Track `firstSessionStep` in the player's data so a rejoin continues, not restarts.
- Send rewards through one server `GrantReward(player, id)` function; the client only displays.
- Log each reward as an event (`reward_granted`, `step`, `seconds_since_join`) (see [[06-Retention/event-taxonomy]]).
- Skip reward steps for returning players who already passed them.

## Tuning with data
- Find the step where players quit (funnel). Add a reward just before it.
- If players earn too fast and stall at minute 10, thicken the 5 to 15 minute band, do not thin the start.
- If rewards feel ignored, check that sound and visual are clear on a phone.
- Target: more than 60 percent reach minute 5 (UNVERIFIED goal, compare to [[06-Retention/benchmarks]]).

## Fair vs exploitative
- Fair: visible rewards, honest progress, a free starter item, daily preview.
- Exploitative (never): early gambling, hidden odds, pressure to buy before the player knows the game, countdowns that rush children.

## Mistakes
- Rewards only after a long tutorial. Put them inside it (see [[04-Onboarding/tutorial-patterns]]).
- Same reward size for 5 minutes (flat feels dull).
- Spending the best reward too early with nothing to look forward to.

## Related
[[04-Onboarding/first-60-seconds-checklist]], [[04-Onboarding/first-session-rewards]], [[04-Onboarding/onboarding-flow]], [[05-Progression/unlock-pacing]], [[05-Progression/goal-gradient-and-session-goals]], [[03-Game-Psychology/retention-tricks-catalog]]

## For our Roblox games
Copy the principle: frequent, clear, earned wins at the start, with gaps that grow slowly. Never copy another game's tutorial text, characters, art, sounds or logos.
