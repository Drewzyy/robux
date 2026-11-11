# Sky Tower Obby: first 60 seconds (working title)

**Convention:** every number is a STARTING VALUE to tune by stopwatch playtest (tagged "sv" where it first appears in a section; the rule applies to all).

## 1. Pitch and genre
Climb a tower of floating islands, one checkpoint beacon at a time, in a short progression obby (middle model: checkpoints plus best times; obby playbook). **Core verb: jump.**

## 2. Core loop
| Timescale | Answer |
|---|---|
| Every 5 s | Jump to the next island (sv: 4-6 studs gap early) |
| Every 30 s | Reach a beacon: chime, light burst, +10 coins, floor counter ticks (sv) |
| Every session (median 9.8 min, core-loop-template) | Reach Cloud Gate 1: floors 1-10 (sv) |
| Tomorrow | Daily first-climb coin bonus, fixed amount (sv); friends' heights shown |
| This week | Reach the next Cloud Gate; beat your best time to Gate 1 |

## 3. First 60 seconds
- **0-5 s, spawn and orient:** Spawn on a wide, safe Base island. The tower is visible above. A glowing beam points at Island 1, 6 studs away (sv). No logo, text wall or shop. One prompt, "Jump!" (button icon), appears only when the player stands still for 3 s (sv).
- **5-10 s, first core verb:** Jump to Islands 1 and 2 (gap 3-4 studs, sv). Cannot fail meaningfully: a low safety cloud under the first gap catches a fall and bounces the player back.
- **10-30 s, first reward:** Island 3 holds Beacon 1 (about 15 s in, sv). Touching it fires chime, burst, +10 coins, and "Floor 1" on a slim height bar. Respawn point set. Islands 4-5 stay easy as a rest beat.
- **30-60 s, open loop:** Updraft Pad launches the player up to Island 6, a visible thrill (one new idea). The height bar shows "Floor 1 of 10" with Beacon 2 and the glowing Cloud Gate in view, bar started about 20% full (sv, goal gradient). First crumbling island (Island 7) is the only mild challenge. A fall respawns at Beacon 1 in under 2 s (sv).

## 4. Mechanics
| Mechanic | Player does | Teaches | Can go wrong |
|---|---|---|---|
| Stable islands + beam | Jump gap to gap | Jump timing, goal direction | Gap too wide for mobile: keep early gaps at most 4 studs (sv) |
| Beacon checkpoints | Touch to bank progress | Failure costs seconds, not progress | Beacons too far apart: space 15-30 s apart (sv) |
| Updraft Pad | Step on to be launched upward | Vertical climb, tower fantasy | Landing unreadable: landing island lit and in view before launch |
| Crumbling island | Cross before it falls (1.5 s delay, sv) | Keep moving | Unfair if no tell: shake and dust first |
| Safety cloud (stage 1 only) | Falls bounce back | Nothing can punish learning | Seen as a bug if left in later; remove after Beacon 1 |

## 5. Progression
Fixed: +10 coins per beacon (sv), floor counter, Cloud Gate every 10 floors (sv). Variable (surprise only): a hidden gold star on an optional side island, small bonus coins (sv). Coins cosmetic only. Source: reward-schedules not loaded; the fixed/variable rule comes from the obby playbook.

## 6. Difficulty curve (sawtooth)
| Stage | New idea | Diff 1-10 (sv) | Rest beat | Retry |
|---|---|---|---|---|
| 1-2 | jump only | 1 | n/a | under 2 s |
| 3-5 | none, practice | 1-2 | yes | under 2 s |
| 6 | Updraft Pad | 2 | no | under 2 s |
| 7-8 | crumbling | 3-4 | no | under 2 s |
| 9 | rest beat, Beacon 2 | 2 | yes | under 2 s |
Then: next block introduces one mechanic (moving islands), dips, peaks. Never a new mechanic at a peak. After 3-5 deaths at one spot, quietly add mercy (extra beacon, slower hazard); UNVERIFIED for Roblox.

## 7. Social hooks (cheapest first)
1. Friends join anywhere and spawn at their own beacon. 2. Invite prompt only after Beacon 2, never before the first reward. 3. Shared goal: party's combined floors fill a Cloud Gate meter. 4. Friend-only height leaderboard on the base island.

## 8. Lobby and layout
No separate lobby. Spawn is the Base island: tower overhead, one beam, one first island, low-key HUD (height bar only). Leaderboard and shop stay out of sight until after Beacon 2.

## 9. What we will not do
No pay-to-win, no gated finish, no punishing free path, no paid random items in game 1, no sale of checkpoints, no shop before 5 minutes, no unskippable intro, no like/follow ask before the first reward. Nothing in this page is for sale; the do-not-sell-list was not loaded because nothing is proposed.

## 10. Open questions and UNVERIFIED
- UNVERIFIED: all seconds and stud values; need stopwatch playtest and first-play bounce data.
- UNVERIFIED: hidden mercy after 3-5 deaths works on Roblox (general pattern per playbook).
- UNVERIFIED: the god plan for "Sky Tower Obby" is not in this repo; its decisions may override this page.
- Q: Is the safety cloud acceptable, or should Island 1-2 simply be wider?
- Q: Does Andrew want timed races in version 1, or only best times?
- Q: Mobile jump range and camera on small screens are untested.

## 11. Loaded
- 00-Start-Here/index: required first read, routes to 02-Game-Design.
- 02-Game-Design/index: section index.
- genre-playbooks/obby: matching genre rules.
- first-60-seconds-checklist: timeline windows and Day 1 killers.
- core-loop-template: loop timescales, 9.8 min median.
- difficulty-curve-worksheet: sawtooth table, retry rule.
- game-design-doc-template: the 11 sections.
No other section loaded; none needed.
