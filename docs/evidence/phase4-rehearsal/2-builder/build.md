Project: projects/dawn-watch

# Dawn Watch: "Fast First Minute" build
Design: `runs/pipe-improve-build-20261109-114826-63b1--1-architect/design.md`. Existing structure kept. One new service (`GuideService`) and one new shared module (`FirstRun`). Every number is a starting value to tune (design rule). Nothing was run in Studio.

## Gate (real result)
`python3 ../scripts/gates/build_gate.py projects/dawn-watch`: rojo build, selene, stylua --check, and Lune all passed (specs: Validate, Aim, Beam, Fog, NightPlan, Economy, Tutorial, Config, FirstRun). This says nothing about feel, remotes in a live server, or UI.

## Files
New:
- `src/shared/FirstRun.luau`: pure `holdFor`, `aliveCap`, `needsWelcome`. Takes Config as an argument (no sibling requires).
- `src/server/Services/GuideService.luau`: light pillar, pulsing arrow, barrel highlight.
- `tests/FirstRun.spec.luau`.

Changed: `Config`, `Economy`, `Tutorial` (shared); `NightService`, `LampService`, `CreatureService`, `WorldService`, `Main.server` (server); `HudController` (client); `tests/run.luau`, `Economy.spec`, `Tutorial.spec`, `Config.spec`.

## Design point to code
| Design point | Where |
|---|---|
| Starter oil on spawn (first-run only) | `NightService:_giveStarterOil`, called from `_onSpawn` (CharacterAdded) and again after `Lamps:Reset()` in `StartNight`. Gated on the player having 0 pops. |
| T0 is character spawn; night starts 1 s after a first-run player spawns in Lobby; no lantern press | `NightService:_onSpawn` (replaces the old "first player, 1 s after join" delay). Start lantern kept. |
| Joiner during Dawn waits up to 8 s | `waiting` set; `_tick` starts the night when the Dawn result ends. Known exception, as designed. |
| 0.4 s hold for a never-lit lamp; 2 s only for relight | `FirstRun.holdFor`, `Config.FIRST_BEAM_HOLD`, `LampService` `everLit` flag (reset each night). |
| Pre-aimed first beam on lane 2 | `LampService:PreAim`, `Config.FIRST_LANE` (Harbor 2, Stormside 4, Crown 2), applied only if the pourer is the keeper. |
| First lane at lamp-lit, not at next 1 s check | `NightService:OnPoured`, called by `LampService:_onTend` after a pour. Lane glows at once (pending lane). |
| First Mumble 60 studs out, 2 s telegraph | `CreatureService:Schedule(..., {fast=true})`, `Config.FIRST_SPAWN_DIST`, `FIRST_TELEGRAPH`. |
| Welcome Mumble for joiners | `OnPoured` + `FirstRun.needsWelcome` (no pops, nothing alive or telegraphed, once per player per night). |
| Second Mumble on lane 1 with a ring | `_maybeSpawn` (`guidedStep` 1 to 2, `Config.SECOND_LANE`), `Highlight` ring in `CreatureService:_spawn`. |
| Tutorial alive-cap 1 then 2 until 60 s | `FirstRun.aliveCap` used in `_maybeSpawn`. |
| Pop pays 10, 5, 5 per player | `Economy.popReward`, `NightService:_onPop` (per-player `Pops`; replaces global `firstPopPaid`). `FIRST_POP` is now 10. |
| Lamp flash on light-up and on pop | `LampService` `flash` timer and brightness surge; `FlashAll` from `CreatureService:_pop`. |
| "+N Glow" float, "First light!" banner, Glow bar | `HudController` (`_onPop`, `glowFill`), driven by player attributes `Pops` and `LastPopGlow`. No new remote. |
| Hints "Pour it in.", "Refuel!", aim hint | `Tutorial.hint`; HUD now passes the player's own pops and `FirstRun`. |
| Pillar + arrow over the first dark lamp; barrel highlight at "Refuel!" | `GuideService`; `WorldService.barrels` exposed. |

## Deviations (read these)
1. Aim hint is **"Aim at glow."**, not "Aim at the glow.". The design string is 4 words, and both the design and the existing `Tutorial.spec` cap hints at 3 words.
2. Guidance is **global, not per player**: the pillar, arrow, and barrel highlight are server-made parts, shown while any player has 0 pops (pillar) or any first-run keeper is low on oil (barrels). A per-player version needs a client controller. Not done.
3. The arrow is a text "v" in a BillboardGui, not a model.
4. Not built: the audio (no approved asset IDs; TODO stays in `CreatureService:_pop`), and the 4-timestamp instrumentation (event names undecided, Producer's call).
5. `FirstRun` attribute and `Pops` are in memory only (no saving this build), so a returning player gets the guided start again, as the design says.

## Security and data
- No new remotes. The one client-to-server remote (`AimBeam`) is unchanged: type, NaN/huge, keeper permission, rate limit. All new state (oil, pops, Glow, pre-aim, spawns) is server-owned; the client only reads attributes.
- No DataStore in this project (grep found none), so there is no ProfileStore or BindToClose to add. Saving is still unbuilt.

## UNVERIFIED
- All timings (first light by ~6 s, first pop by ~11 s) are design targets, not measured. The 5-run stopwatch playtest is the check.
- `Highlight` properties and `Highlight.DepthMode` are used from memory; selene offline does not check API members. Check in Studio.
- That attribute updates (`LastPopGlow`, `Pops`, `Glow`) arrive in order on the client. The HUD reads `LastPopGlow` when `Pops` changes.
- 0.4 s hold on phones; the pillar/arrow on phones; touch and gamepad aiming (pre-aim means the first pop needs no aim, but the second does).
- A player who pours into a bot-lit lamp in the tutorial night before 20 s is not handled specially beyond the same first-light flow.

## Open questions
1. Audio IDs for the lamp-light sound and the first-pop giggle/chime (Scenic or Librarian).
2. Accept the up-to-8 s Dawn wait for joiners, or shorten the result for them?
3. Make guidance per-player with a client controller? Wanted before launch if the global pillar confuses a second player.
4. Is a 4-word aim hint worth raising the 3-word cap?

## Loaded
- `00-Start-Here/index.md`: required first read.
- `13-Luau-Engineering/index.md`: required section index.
- `13-Luau-Engineering/remotes-and-security.md`: confirmed server-owned state and that no new remote is added.
- `13-Luau-Engineering/headless-test-gate.md`: how to write Lune-testable pure logic and run the gate.
- Design doc (architect run) and the project source files listed above.
- Not loaded: `project-structure` (layout unchanged, only added files in existing folders), `datastore-playbook` (nothing saves), `common-bugs-checklist` (not read; I reviewed manually instead, so that check is not claimed).
