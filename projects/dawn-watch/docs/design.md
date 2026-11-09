# Dawn Watch: design doc
Run: pipe-pitch-to-build-20261109-105542-ca9c--3-architect. Date: 2026-11-09.

**Number rule:** every number in this doc (seconds, counts, prices, percentages) is a **starting value to tune** in playtest. I say so once here and tag each table. Nothing is final.

**Vault status:** `10-game-design/index` is `status: stub`. Every note it lists (templates, playbooks, worksheets, player-psychology) is marked "planned". I did not look for them elsewhere. **STUB: 10-game-design/index. STUB/NOT READ (listed as planned): game-design-doc-template, core-loop-template, first-60-seconds-checklist, difficulty-curve-worksheet, player-psychology.** There is no co-op or survival genre playbook in the vault. I designed from the architect skill rules, Pulse's pitch and Lead's plan.

---

## 1. Pitch and genre
**Pitch:** Up to 4 friends (or 1 player plus bots) defend a lighthouse through one 9-minute night by tending lamps and aiming the beam at fog creatures that drift in on glowing, visible lanes; dawn always arrives.
**Genre:** short-round co-op defense / survive-the-night. **Core verb: Tend** (pour oil into a lamp, aim its beam).

## 2. Core loop (all numbers are starting values to tune)
| Cadence | What happens |
|---|---|
| Every 5 s | A verb: grab oil from a barrel, pour it into a lamp, or swing a beam onto a lane that has a creature coming. |
| Every 30 s | A reward: a creature "pops" into glow-sparkles when the beam holds on it for 1.5 s. A wave clears with a chime and a Glow (soft currency) tick. |
| Every session | One full night (9:00). Dawn, a result screen with 1 to 3 stars, Glow paid out, and the board shows progress toward the next upgrade. |
| Every week | The shared Lighthouse Board: the group's lighthouse tier (see 5). A weekly target of 7 nights completed by the player, with a fixed reward. |

## 3. First 60 seconds (starting values)
| Window | What the player sees and does |
|---|---|
| **0-5 s** Spawn and orient | Player spawns on the lighthouse gallery facing a dark lamp. A glowing oil barrel sits 3 studs away. One arrow, 3 words: "Fuel the lamp." No menu, no text wall. |
| **5-10 s** First core verb | Player walks to the barrel and holds interact to pick up oil, then holds interact at the lamp to pour. **The lamp is lit by second 10 at the latest.** The beam sweeps on with a warm sound. |
| **10-30 s** First reward | A lane on the water glows. One Mumble (the gentlest creature) drifts in slowly. The player drags the beam onto it (the arrow now says "Light it up"). It giggles and pops into sparkles. The first Glow drops (+5, starting value) and the lamp visibly brightens. |
| **30-60 s** Open loop | A second lane lights up and a second Mumble arrives, now with the oil gauge ticking down so the player has to refuel. A second station glows and a bot or a friend is standing at it. The dawn bar at the top shows the whole night. The Board in the lobby shows "1 of 3 nights to Lamp Polish" (a fixed next reward). |

Night 1 for a new player is a shortened tutorial night. It has only Mumbles, and lamps cannot run dry for the first 30 s (starting values).

## 4. Mechanics
All three stations and all three creatures are MVP. There are exactly 3 of each.

**Stations (one per lighthouse side; one player or bot each)**
| Station | Player does | Teaches | What can go wrong |
|---|---|---|---|
| **Harbor Lamp** (east) | Carries oil from the barrel, pours in, aims a narrow, long beam. | Resource upkeep, timing. | Oil runs out and the lamp dims. Relight takes 2 s and a small amount of oil (starting values). |
| **Stormside Lamp** (west) | Same upkeep, wide short beam that covers two lanes. | Prioritizing lanes. | Wide beam works slowly, so fast creatures slip through. |
| **Crown Lens** (top) | Aims the big rotating beam and calls out lanes to the others. Burns oil fastest. | Calling out and team awareness. | Over-aiming leaves the side lamps without cover. |

**Creatures (cute and strange, no horror or gore)**
| Creature | Behavior | Counter |
|---|---|---|
| **Mumble** | Slow, round fog blob. Dims a lamp it touches. | Hold the beam on it for 1.5 s. |
| **Flicker** | Small and fast. Zigzags but stays on its lane. | Wide beam, or two quick sweeps. |
| **Hushhog** | Big and slow, wears a sleepy face. Needs two beams at once. | Two stations aim together (this is the co-op moment). |

**Other mechanics unique to this game**
- **Telegraphed lanes.** Each creature's path glows on the water 3 s before it spawns (starting value). A death from a surprise should not be possible.
- **Dawn bar.** A visible 9:00 sun-rise track. Dawn always arrives; nobody is eliminated and no night is a hard wipe. Poor defense means a dim lighthouse and fewer stars, not a restart.
- **Mid-night fog event** at 4:30 (see 5). It is the only random element.
- **Bots.** Empty stations are filled by lamp-keeper bots that work at 70% of human speed (starting value). They hand a station back instantly when a player steps up, so solo and low-population play work.
- **Relight.** If a lamp goes out the player can relight it in 2 s (starting value), so a mistake costs seconds, not the run.

## 5. Progression
**Fixed schedule (the player always knows the next reward and when):**
- Each completed night pays Glow: 20 base + 10 per star (so 30 to 50; starting values).
- The Lighthouse Board is **shared by the whole server group and persists per player's contribution**: the first upgrades are Lamp Polish (40 Glow), Longer Wick (80), Wide Lens (140), Sea Chime (220) (starting values). Each upgrade has a visible effect (more beam reach, slower oil burn, a cosmetic chime).
- Daily: the first night each day pays +10 Glow (starting value). That is the reason to come back tomorrow.
- Weekly: finish 7 nights in the week for a lamp color cosmetic. It is fixed and known in advance.
- Time to the first upgrade: about 2 nights (about 19 min). Time to the first board tier: about 1 week of casual play (starting values; to be checked by the economy worksheet, which is not in the vault).

**Variable schedule (surprise only):**
- The fog event at 4:30 is one of 3, picked at random: **Thick Fog** (beam reach -30% for 45 s), **Gust** (beams drift slowly, aim must be corrected for 45 s), **Lantern Rain** (free oil drops for 20 s; a gift). Starting values. None of these gate progress or Glow.
- No variable rewards of any kind, and nothing random can be bought.

## 6. Difficulty curve (sawtooth; starting values)
Intensity = creatures alive at once (Mumble 1, Flicker 1, Hushhog 3 points).
| Time | Beat | Intensity (solo / 4 players) |
|---|---|---|
| 0:00-0:45 | Warm-up | 1 / 2 |
| 0:45-2:15 | **Rise 1** | 3 / 5 |
| 2:15-2:45 | **Rest** (oil barrels refill, no creatures) | 0 |
| 2:45-4:15 | **Rise 2** (first Hushhog) | 4 / 7 |
| 4:15-4:30 | Rest beat | 0 |
| 4:30-5:15 | **Fog event** | 3 / 5 |
| 5:15-5:45 | **Rest** | 0 |
| 5:45-7:30 | **Rise 3** | 5 / 9 |
| 7:30-8:00 | **Rest** | 0 |
| 8:00-9:00 | **Final push** to dawn, then the sun rises with a celebration | 6 / 10 |

- **Per-night growth:** +5% intensity per night, capped at night 10 (starting values).
- **Solo scaling:** intensity is scaled to humans plus bots, with bots counting as 0.7 players.
- **Retry:** a dimmed lamp is relit in under 2 s. There is no "game over" screen. A new night starts from the lobby in under 2 s via one button.
- A failed night (lamps out for more than 40% of the time; starting value) gives 1 star and 20 Glow, so trying is never punished.
- There is no quit-rate data yet. Tune from playtest (the difficulty worksheet is not in the vault).

## 7. Social hooks (cheapest first)
1. **Join friends:** the lobby and the join flow let friends land on the same lighthouse. Plan for friend-join on day 1. Which Roblox API to use is UNVERIFIED; Builder to confirm in `20-luau-rojo`.
2. **Roles that need each other:** three stations plus the Hushhog requiring two beams makes the group matter. Players can ping a lane with a simple on-screen marker (no chat needed).
3. **Shared goal:** the Lighthouse Board that everyone funds.
4. **Friend leaderboard:** friends' nights completed this week. UNVERIFIED whether it is feasible; it needs a data store, and the plan limits storage to the upgrade board, so this is a Lead decision.
5. Invite prompt after a finished night. API UNVERIFIED.

## 8. Lobby and layout
- **From spawn the player sees:** the lighthouse as a tall landmark ahead, the Board at its foot (next upgrade and bar), a big "Start Night" lantern button, and a table showing station slots (3 icons with names or bot icons).
- **Lobby:** small round cove. Oil barrels and a practice lamp (an optional free tutorial that makes the first lamp lit in under 10 s).
- **Night map:** the gallery ring with 3 stations spaced evenly, barrels between them, 3 to 5 lanes radiating over the water, a dawn bar at the top of the screen, and an oil gauge on each lamp.
- **Palette:** warm lamp light against deep blue fog. Cute, round creatures.
- **Input:** phone, PC and gamepad all supported (one big interact button and one aim stick/drag). The UI specifics are for Frame (`30-ui-ux`, not loaded).

## 9. What we will not do
- No pay-to-win. No gamepass or product raises beam strength, oil, or Glow.
- No gated finish: every upgrade, star and the weekly reward is earnable for free.
- No punishing the free path: nobody is eliminated, and dawn always arrives.
- No paid random items in game 1. The only random element is the free fog event.
- No horror, gore, or jump scares (age rating risk; Pulse noted this).
- **Nothing in this design is for sale.** Monetization is a later, separate step (Merchant). I did not load `40-monetization/do-not-sell-list` because I am not proposing anything for sale.

## 10. Open questions and UNVERIFIED
**Open questions for Andrew:**
1. Confirm 1 to 4 players with bots (Lead's decision, Pulse asked). 
2. Should friend leaderboard storage be allowed? Lead's plan limits data storage to the upgrade board.
3. Is the "shared" board shared per server, per player group, or global? I assumed per-player progress with the lobby showing a group tier. Tune with Builder.
4. Is a 4-player cap OK for the stations? Three stations plus a fourth player is "floating helper / oil runner" (my suggestion, not tested).

**UNVERIFIED:**
- The skill rule that the median session is 9.8 min and the Cutting et al. 2023 study (n=311) come from the architect skill text. The vault's `player-psychology` note is only planned, so I could not check them against a `verify:` line. The 9-minute night leans on the 9.8-minute figure.
- Pulse's demand numbers, as Pulse itself flagged. "Co-op survive-the-night" rests on one source (S4).
- Any Roblox API for friend join, invites, and leaderboards.
- All balance numbers (all are starting values).
- Policy and age rating for fog creatures; `60-policy` was not loaded and no one has done a policy check.
- Whether 3 stations and 3 creatures fit a solo build budget.

## 11. Loaded
- `00-router/index.md`: required first read; routed design to `10-game-design`.
- `10-game-design/index.md`: required section index. It is a stub, and all its notes are "planned", so I read none of them.
- `runs/...--1-pulse/pitch.md`: the pitch (pitch #1, Dawn Watch).
- `runs/...--2-lead/plan.md`: Lead's plan and Stage 1 gate.
- Not loaded: `40-monetization` (nothing proposed for sale), `60-policy`, `30-ui-ux`, `20-luau-rojo`.
