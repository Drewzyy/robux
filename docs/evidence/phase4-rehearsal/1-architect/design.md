# Dawn Watch: design change "Fast First Minute"
Run: pipe-improve-build-20261109-114826-63b1--1-architect. Date: 2026-11-09.
Task: the player does the core verb within 10 s of spawning and gets the first reward within 30 s.

**Number rule:** every number in this doc (seconds, studs, Glow, counts, percentages) is a **starting value to tune** in playtest. This is stated once here. Nothing is final.

**What I read:** the vault notes in section 11, plus the existing project docs and code: `docs/design.md`, `Config`, `NightPlan`, `Economy`, `Tutorial`, `NightService`, `LampService`, `CreatureService`, `WorldService`, `HudController`, `AimController`. Nothing was run in Studio, so every timing I give for "what exists now" is **UNVERIFIED**. It comes from reading the code.

**Vault status:** the notes this task needs now exist (status `draft`, with `verify:` lines). The old project doc says they were stubs. That is out of date. `02-Game-Design/index` itself is not a stub.

---

## 1. Pitch and genre
**Pitch (unchanged):** Up to 4 friends (or 1 player plus bots) defend a lighthouse through one 9-minute night by tending lamps and aiming the beam at fog creatures on telegraphed lanes. Dawn always arrives.
**Genre:** short-round co-op defense. **Core verb: Tend** (pour oil into a lamp, aim its beam).
**This change:** make the first minute fast and the same for every new player, not only the first player in a server.

## 2. Core loop (unchanged; shown so the change has a place)
All values are starting values.
| Cadence | What happens | Change |
|---|---|---|
| Every 5 s | Tend: carry oil, pour, swing the beam onto a lane. | None. |
| Every 30 s | A creature pops into sparkles and Glow ticks. | The first reward is now a real celebration, and the first three pops pay Glow (see 3 and 5). |
| Every session | One 9:00 night, then a result with stars and Glow. | None. |
| Tomorrow / this week | Daily +10 Glow bonus, weekly 7-night reward, shared Lighthouse Board. | None. Both are still not built (no saving this build). |

## 3. First 60 seconds, second by second
**Clock:** T0 is the moment the player's character spawns on the gallery, not the moment they join the server.

### What exists now (UNVERIFIED, read from code)
- The first player in starts a tutorial night 1 s after joining. That is not tied to their character loading. Anyone who joins later skips all of it.
- The player spawns at (0, 20.5, 8). They must grab oil from a barrel, then **hold 2 s** to light a dark lamp. The first light-up is the slowest step of the minute.
- After the lamp lights, the first Mumble waits for the next 1 s spawn check, then a 3 s telegraph. It spawns 110 studs out, beyond easy sight.
- The beam rests at angle 0 and the first Mumble is on lane 2 (0.25 rad). The beam half-width is 0.1, so the player **must** aim to get any reward. The hint says "Light it up." and nothing tells them how to aim. Touch and gamepad aiming are marked UNVERIFIED in `AimController`.
- If the Mumble is not popped it reaches the lamp, dims it by 30, and nothing pays.
- The first pop pays +5 Glow (`Economy.FIRST_POP`) only as a number in the bottom status line. There is a particle burst and no sound (audio IDs are a TODO).
- The tutorial night compresses the 9-minute plan into 120 s. "Rise 1" starts at about 10 s real time, so up to 3 Mumbles can be alive at once in the first 30 s.
- **A player who joins after the first one has no tutorial.** Lamps may be bot-lit, the hint is blank, and the first reward is the dawn payout up to about 9 minutes later. This is the biggest gap against the task.

### The new first minute (targets; hard limits in brackets)
| Window | What the player sees and does | Time target |
|---|---|---|
| **0-5 s** Spawn and orient | The player spawns **already carrying oil** (hands glow, status "Carrying oil"). A light pillar and a pulsing arrow mark the nearest dark lamp (Harbor, about 13 studs away). The hint reads "Pour it in." No menu and no barrel trip. | Orient by 2 s. Walk to the lamp by 4 s. |
| **5-10 s** First core verb | The player holds the prompt on the lamp for **0.4 s** (was 2 s) and pours. The lamp lights with a warm sound, a brightness surge, and the beam sweeping on. **The beam starts already aimed down lane 2.** The lane glows at once (not after the next 1 s check). | Lamp lit by 6 s [hard limit 10 s]. |
| **10-30 s** First reward | A short telegraph (2 s), then one Mumble appears **60 studs out** (was 110), drifts down the lit lane and into the pre-aimed beam, and pops in 1.5 s. The pop gets: sparkle burst, lamp flash, "+10 Glow" floating up to the Glow readout, a 2 s "First light!" banner, and a giggle/chime (asset needed, see 10). The hint now says "Aim at the glow." for the **second** Mumble (lane 1), which needs a real aim. | First pop at about 11 s [hard limit 30 s]. Second pop at about 20 s. |
| **30-60 s** Open loop | The 2nd and 3rd pops pay +5 Glow each, so the bar toward Lamp Polish reads 25 of 40 and shows "Lamp Polish in 15 Glow" at about 35 s. Only 1 or 2 Mumbles are alive at once. Bots take the other stations at 20 s. At about 49 s the oil drops below 25 and "Refuel!" appears. A highlight on the nearest barrel teaches the barrel step now, just in time. The dawn bar shows 120 s tutorial night. | Open loop visible by 35 s. Refuel taught by 55 s. |

### Same minute for a player who joins later
1. If the night is **running**, the joiner spawns carrying oil and the dark or bot-kept lamp (Harbor first, then Stormside) is highlighted.
2. When they pour and become keeper, and **no creature is alive or telegraphed**, the server schedules one free "welcome" Mumble on that lamp's lane with the pre-aimed beam. That is a one-off, so it can be one point above the normal intensity target. If creatures are already coming, nothing extra is added.
3. Their first pop pays +10 Glow **to them** (a per-player flag, no longer the global tutorial flag).
4. If the server is in the **Lobby** phase, the night starts 1 s after their character spawns. No lantern press for a first-run player.
5. If the server is in the **Dawn** result (8 s), they wait up to 8 s. This is a known exception to the 10 s target (see 10).
6. The per-player first-run flag lives in memory this build, so a returning player gets the guided start again. That is harmless. Saving it is a later decision.

## 4. Mechanics
Existing mechanics are unchanged. These are the new or changed ones, each specific to this pitch.
| Mechanic | Player does | Teaches | What can go wrong |
|---|---|---|---|
| **Starter oil** (new, first-run only) | Spawns holding one pour of oil. | "Oil goes into lamps" with no barrel detour. | The player never learns the barrel. Fix: the "Refuel!" barrel highlight at about 49 s. |
| **Fast first light** (changed) | A lamp that has **never been lit this night** takes a 0.4 s hold (was 2 s). The 2 s relight hold only applies to a lamp that was lit and went out. | Lighting is quick and cheap. A mistake later still costs seconds. | Lamps stay easy to light while dark, so the bots' and the players' relight costs differ slightly. Acceptable. |
| **Pre-aimed first beam** (new) | Nothing; the first beam lands on lane 2. | That the beam is the thing that kills fog. It gives the reward before aiming is learned. | The player gets the pop without aiming. The second Mumble (lane 1) needs a real aim, with the hint "Aim at the glow." and a ring on the creature. |
| **Welcome Mumble** (new, joiners) | Pours and waits about 4 s. | The same lesson as the first-night player gets. | A joiner who leaves the lamp before the pop gets no reward. Acceptable. |
| **First-light celebration** (changed) | Watches and hears it. | That popping is the goal. Peak moment in minute one. | Sound is missing until Andrew supplies approved audio. Visuals ship first. |
| **Tutorial intensity cap** (changed) | Nothing. | One mechanic at a time. | Too easy. Tune: target 1 alive until the first pop, 2 until 60 s real. |

Teach by doing: the only text is the existing 3-word hint (Tutorial.luau). The new hints are "Pour it in.", "Aim at the glow.", "Refuel!". No text walls, no tutorial that blocks movement.

## 5. Progression
**Fixed schedule (the player knows the next reward and when):**
- First pop +10 Glow (was +5). The 2nd and 3rd pops +5 each, in the tutorial night and for a joiner's first three pops. A fixed, known, small schedule.
- Night payout unchanged: 20 + 10 per star (30 to 50).
- Lamp Polish still costs 40. At about 35 s the player is at 20 of 40 Glow, so the bar is half full at minute one (goal gradient; vault checklist says start bars partly full). Upgrades are still not spendable in this build.
- Daily +10 and weekly 7-night reward: unchanged and still unbuilt.

**Variable schedule (surprise only):** unchanged. The only random element is still the 4:30 fog event. **Nothing in the first minute is random.** The first reward is fixed, which also keeps it guaranteed.

## 6. Difficulty curve
Unchanged for the full 9-minute night. For the tutorial night only (starting values):
| Real time | Alive-at-once cap | Beat |
|---|---|---|
| 0 s to first pop | 1 | Warm-up (rest-like: the player is learning) |
| first pop to 60 s | 2 | Rise |
| 60 s to 120 s | normal compressed plan | Rise, then rest beat |

- Retry stays under 2 s: relight is 2 s at most, a failed night still pays 20 Glow and 1 star, and there is no game-over screen.
- Failure in minute one costs nothing: lamps have a 30 s no-burn grace, and a missed first Mumble just gets the next one. Because the welcome and first Mumbles are pre-aimed, a first-time miss is not possible on the first creature.

## 7. Social hooks
Cheapest first. Only what this change touches:
1. **Joiners get the fast start** and are assigned a lamp that needs a keeper, so a friend arriving mid-night is useful in under 10 s. Friends are never made to wait for the next night.
2. The first-pop Glow goes to every player holding a station, as now, so friends playing together both see "+10".
3. Invite prompt and friend leaderboard are unchanged and remain UNVERIFIED (see 10). Not added to the first minute (an ask before the first reward hurts Day 1, per the vault checklist).

## 8. Lobby and layout
What the player sees from spawn (changed parts only):
- The gallery ring, the tower, the dark Harbor lamp about 13 studs away, and a **light pillar and pulsing arrow** over it. These are visible whichever way the camera faces.
- The Start lantern at (-7, 21.5, 9) stays but is not needed by a first-run player.
- Highlights fade after the player's first pop (mastery), and again only if a lamp goes dark.
- Barrels at z = +11 and z = -11 stay. They are highlighted only at the "Refuel!" moment.
- No new objects in the lobby. No shop, no pop-up, no ask to like or follow in the first 5 minutes.

## 9. What we will not do
- No pay-to-win. No gamepass or product touches beam, oil, Glow, or the first minute.
- No gated finish and no punishing of the free path. Pre-aiming the first beam is a gift to every player, not a paid boost.
- No paid random items in game 1. No random reward in the first minute.
- No cutscene, logo, unskippable intro, long tutorial, or text wall. No shop in the first 30 s. No "like the game" ask before the first reward.
- No change to the 9-minute night, the station and creature set, or the Board.
- **Nothing here is for sale.** I did not load `07-Monetization/do-not-sell-list` because nothing is proposed for sale.

## 10. Open questions and UNVERIFIED claims
**Open questions for Andrew / Lead:**
1. Audio: the giggle/chime for the first pop and the lamp-light sound need asset IDs from Andrew's approved assets. I invented none. Until then the celebration is visual only. (`10-Art-Environment` not loaded; Scenic or Librarian to supply.)
2. Joiners who arrive during the 8 s Dawn result wait up to 8 s. Shorten the Dawn result for them, or accept the exception?
3. Should "first run" be saved per player once saving exists (ProfileStore, Builder to decide), so highlights fade permanently after mastery?
4. Is it fine for the first beam to be pre-aimed, so the first reward does not depend on aiming? It is a deliberate choice: the aiming input on touch and gamepad is untested.
5. Instrumentation: log the 4 timestamps (T0, first oil poured, first pop, 3rd pop) per player so Playtest and Producer can read the funnel. Event names are not decided (`06-Retention` not loaded because this design does not need it; Producer to define).

**UNVERIFIED:**
- All timings under "What exists now". They are read from code, not run. The stopwatch playtest is the check (5 fresh runs, median verb under 10 s, median reward under 30 s, per device type).
- Touch and gamepad aiming (`AimController` says so itself).
- That 0.4 s is a comfortable hold on phones, and that a pillar and arrow render clearly there.
- Whether the first pop at about 11 s feels earned or too quick; the 60-stud distance and 2 s telegraph are guesses.
- The 9.8 minute median session and the Cutting et al. 2023 study: they come from the skill text. The `core-loop-template` note also gives 9.8 min with a research-report source. I did not load `player-psychology`, so the study is not checked against a vault `verify:` line.
- The seconds-level targets themselves: the vault says Roblox gives none, and the windows are community guidance.
- Any API for friend join, invite, or leaderboards.

**Changes for Builder (design level, no code from me):**
- `NightService`: start the night on the first player's character spawn; auto-start the Lobby night for a first-run player; trigger the first lane at lamp-lit; per-player first-pop flag and the +10/+5/+5 schedule; the tutorial alive-cap.
- `LampService`: 0.4 s hold for never-lit lamps; pre-aim on first light; highlight on the target lamp; flash on lit.
- `CreatureService`: first Mumble at 60 studs with a 2 s telegraph; the welcome Mumble; ring on the second Mumble.
- `WorldService`: starter oil on spawn; pillar and arrow parts.
- `Tutorial` and `HudController`: new hints; floating "+Glow"; "First light!" banner; Glow bar.
- `Economy` and `Config`: new constants (`FIRST_POP` 10, `NEXT_POPS` 5, `FIRST_BEAM_HOLD` 0.4, `FIRST_SPAWN_DIST` 60, `FIRST_TELEGRAPH` 2).
- Update `Tutorial.spec` and `Economy.spec` for the changed rules.

## 11. Loaded
- `00-Start-Here/index.md`: required first read; routed design to `02-Game-Design`.
- `02-Game-Design/index.md`: required section index; showed which notes exist.
- `04-Onboarding/first-60-seconds-checklist.md`: the four time windows, teach-by-doing rules, "kills Day 1" list, and the note that seconds targets are community guidance.
- `02-Game-Design/core-loop-template.md`: loop cadence, fixed-schedule check, and the 9.8 minute median.
- Project files (not vault): `projects/dawn-watch/docs/design.md` and the source files listed above, to describe the change against what exists.
- Not loaded, on purpose: `game-design-doc-template`, `difficulty-curve-worksheet`, `economy-worksheet`, `player-psychology` (a change doc, not a new game; numbers kept to the existing curve), `07-Monetization` (nothing for sale), `06-Retention`, `01-Roblox-Fundamentals`, `10-Art-Environment`.
