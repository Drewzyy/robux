---
description: Catalog of 44 named retention and engagement techniques from hit games, each with the Roblox build, the fair version we ship and the exploitative version we never build.
type: reference
status: draft
last_checked: 2026-11-15
keywords: retention tricks, engagement techniques, streaks, daily rewards, variable rewards, near miss, collection, comeback bonus, social proof, timers, session hooks, fair design, dark patterns
sources: creator-docs content/en-us/production/game-design/liveops-essentials.md (read 2026-11-15); creator-docs content/en-us/production/game-design/introduction-to-quest-design.md (read 2026-11-15); creator-docs content/en-us/production/game-design/season-pass-design.md (read 2026-11-15); creator-docs content/en-us/production/promotion/experience-notifications.md (read 2026-11-15); general game design practice, paraphrased
verify: Technique names are common industry vocabulary. Docs support quests, seasons, limited-time events and notifications. Effect sizes are not given here: UNVERIFIED, measure each in our own analytics.
tags: [game-psychology, reference]
---
# Retention tricks catalog

44 techniques grouped by what they do. Each row: what it is, the Roblox build, the fair version (we build this) and the exploitative version (we never build this). Audience includes children, so anything that relies on fear, fake scarcity, or hidden odds is out.

## Do this
- Pick 6 to 8 techniques for a game, not all 44. Cover at least one from each group: hook, loop, goal, social, return.
- Every technique we ship must have its fair version written down in the design doc before building.
- Show every timer, odds and cost honestly. If you cannot show it, do not build it.
- Ship the free version first; paid shortcuts attach to it later (see [[07-Monetization/rewarding-purchases]]).
- Measure D1, D7 and session length before and after each one; remove the one that hurts trust (reviews, quit rate).
- Never stack three pressure techniques on one screen (timer plus streak plus limited offer).

## A. Session hooks (why start now)
| # | Name | What it is | Roblox build | Fair (ship) | Exploitative (never) |
|---|---|---|---|---|---|
| 1 | Daily reward | Gift for logging in each day | DataStore `lastClaim` day number, claim button | Small, claimable late, no loss | Huge loss if one day missed |
| 2 | Login streak | Day count grows reward | Streak int, UTC day compare | Grace day, partial keep after a miss | Full wipe plus pay-to-restore |
| 3 | Daily quests | 3 small goals per day | Quest table reset at UTC midnight | 3 quests, 5 min each, skippable | Quests that need 4 hours |
| 4 | Timed chest | Chest opens after N minutes | `os.time()` stored, UI countdown | Real timer, shown, free | Pay-to-skip as the only way |
| 5 | Offline earnings | Progress while away | Calc from saved timestamp, cap 8 hours | Show "welcome back, you earned X" | Cap hidden, pay to raise |
| 6 | Event calendar | Known upcoming events | Config table, banner in lobby | Dates visible a week ahead | Surprise events that punish absence |
| 7 | Weekly reset goals | Fresh weekly target | Weekly key in DataStore | Carry over half of unfinished | Hard wipe to force grind |
| 8 | Notification hook | Told when something is ready | ExperienceNotificationService, opt-in 13+ | 1 per day max, event-based | Nagging, guilt copy |
| 9 | Pending claim badge | Red dot on unclaimed reward | UI dot bound to claim state | Dot only for real rewards | Fake dots to drive taps |
| 10 | Free spin per day | One free random reward | Weighted table, odds in UI | Odds shown, all outcomes useful | Paid spins without odds |

## B. Loop and reward (why keep playing)
| # | Name | What it is | Roblox build | Fair (ship) | Exploitative (never) |
|---|---|---|---|---|---|
| 11 | Variable reward | Drop quality varies | Weighted random, server side | Shown odds, floor value | Hidden odds, near-miss tricks |
| 12 | Pity counter | Guaranteed rare after N tries | Counter in DataStore | Counter visible to player | Counter reset on purchase |
| 13 | Surprise drop | Unannounced bonus | Random server event, global toast | Rare, always free | Drops that appear only after spending |
| 14 | Combo multiplier | Chain actions for more | Timer plus counter | Generous window, no punish | Combo lost in 0.2 s |
| 15 | Level-up burst | Big feedback on level | VFX, sound, stat popup | Short, skippable | Blocks play for 10 s |
| 16 | Lucky crit | Occasional x2 hit | Server random 5 percent | Shown, feels fair | Fake luck tied to payment |
| 17 | Jackpot moment | Rare huge payout | Rare table row, server announce | Free play path to it | Only reachable with Robux |
| 18 | Bonus round | Short mode after streak | Mini state on server | Clear entry, free | Entry costs Robux each time |
| 19 | Loot choice | Pick 1 of 3 rewards | UI of three cards | Player agency | Fake choice, same item |
| 20 | Double-or-nothing | Optional risk on reward | Server coin flip | Never lose base reward | Loses what player earned |

## C. Goals and progress (why one more)
| # | Name | What it is | Roblox build | Fair (ship) | Exploitative (never) |
|---|---|---|---|---|---|
| 21 | Near-goal bar | Bar close to full | Progress frame, tween | Honest percent | Bar jumps to 95 percent falsely |
| 22 | Goal gradient | Faster at the end | Bonus last 20 percent | Real extra progress | Bar stalls near end |
| 23 | Next unlock preview | Greyed future item | Locked card with silhouette | Show real requirement | Impossible requirement |
| 24 | Milestone chain | Chain of small goals | Linked quest table | Each step 1 to 3 min | Steps grow without warning |
| 25 | Collection book | Fill a set | Index of item ids | Show missing, hint where | Missing item only for sale |
| 26 | Completion bonus | Reward for full set | Check set on grant | Grant free item | Set needs paid items |
| 27 | Rank ladder | Named ranks | Rank thresholds | Visible, reachable | Rank decay with fear copy |
| 28 | Prestige reset | Restart for permanent boost | Reset with kept stats | Clear gain, optional | Surprise reset |
| 29 | Season pass | Tiered season rewards | Tier XP, free and premium row | Free row real value | Pass needed to stay competitive |
| 30 | Skill mastery | Tracks stats | Counters, badges | Pride, no cost | Locks gameplay behind stats |

## D. Social (why with others)
| # | Name | What it is | Roblox build | Fair (ship) | Exploitative (never) |
|---|---|---|---|---|---|
| 31 | Social proof | See others playing | Show player count, recent wins | True counts | Fake players or fake winners |
| 32 | Leaderboard | Rank players | OrderedDataStore | Daily and friends boards | Boards ranked by spending |
| 33 | Flex items | Visible status | Trails, titles, auras | Earnable ones exist | Only purchasable status |
| 34 | Co-op bonus | More reward with friends | Party buff by group size | Solo still fine | Solo penalised hard |
| 35 | Invite reward | Reward for bringing friends | Roblox referral system | Small cosmetic | Pressure, spam |
| 36 | Gifting | Give items | Trade or gift UI, safe limits | Cap, parent-safe | Unmoderated trade for Robux |
| 37 | Shared world goal | Server-wide target | Server counter, bar | All join, all rewarded | Only payers rewarded |

## E. Return and comeback (why come back)
| # | Name | What it is | Roblox build | Fair (ship) | Exploitative (never) |
|---|---|---|---|---|---|
| 38 | Comeback bonus | Gift after absence | Compare last seen, 3 days+ | Welcome back pack | Guilt, "you lost X" |
| 39 | Progress carryover | Keep progress across updates | Versioned save data | Never lose items | Resets that force buy |
| 40 | Limited event | Time-boxed content | Event config, ends date | Item returns later | Fake countdown that resets |
| 41 | Seasonal theme | Holiday look and goals | Theme assets, lighting | Free rewards inside | FOMO only paid |
| 42 | Cliffhanger | Unfinished story or quest | Saved step, "continue" | Easy resume | Hidden reset |
| 43 | Rotating shop | Daily item rotation | Seeded rotation by date | Earnable with coins | Rotation tuned to pressure |
| 44 | Update teaser | Hint at next content | Locked door, sign | Real date | Teaser never delivered |

## Why these work
They tap curiosity, goal pursuit, belonging and habit (see [[03-Game-Psychology/why-players-return]], [[03-Game-Psychology/reward-schedules]], [[03-Game-Psychology/streaks-and-daily-rewards]]). The same hooks become exploitative when they use fear of loss, hidden odds or fake scarcity (see [[03-Game-Psychology/loss-aversion-used-fairly]]).

## Related
[[05-Progression/goal-gradient-and-session-goals]], [[06-Retention/collection-and-completion]], [[04-Onboarding/returning-player-welcome-back]], [[06-Retention/notifications-and-reengagement]], [[06-Retention/favorite-and-follow-prompts]], [[03-Game-Psychology/social-status-and-flex]]

## For our Roblox games
Copy the principles: a reason to open, a loop that rewards, a goal that is close, friends, and a warm return. Never copy another game's names, characters, art, music or logos, and never build the exploitative column.
