Project: projects/dawn-watch

# Dawn Watch: first build (core loop + first 60 seconds)

**Nothing was executed.** Bash was denied in this run, so `rojo build`, `selene`, `stylua`, and `lune` were NOT RUN. Every file was written by hand and has never been parsed by any tool. Expect small errors on the first real run. The studio's own gate run is the first real check.

## What I built
One playable night, start to finish: spawn on the gallery, grab oil, light the Harbor lamp, aim the beam at a telegraphed lane, a Mumble pops, more creatures arrive on the sawtooth curve, dawn, 1 to 3 stars, Glow paid. Then the lantern button starts the next night (hold time 0).

Cut from this build (not in the core loop or the first 60 s): saving, upgrade purchasing and upgrade effects, daily and weekly rewards, friend join and invites, leaderboards, audio, the ping marker, monetization.

## File list
- `default.project.json`, `selene.toml`, `stylua.toml`, `.gitignore`
- `src/shared` (pure, no `game`): `Config`, `Validate`, `Aim`, `Beam`, `Fog`, `NightPlan`, `Economy`, `Tutorial`
- `src/server`: `Main.server.luau` (bootstrapper, Init then Start), `Services/` RemotesService, WorldService, LampService, CreatureService, NightService
- `src/client`: `Main.client.luau`, `Controllers/` HudController, AimController
- `tests/run.luau` plus a spec per pure module (Validate, Aim, Beam, Fog, NightPlan, Economy, Tutorial, Config)

## Design point to code
| Design point | Where |
|---|---|
| Lamp lit by second 10; barrel 3 studs from spawn; 3-word hints | `WorldService` (spawn z=8, barrel z=11), `Tutorial.hint`, prompt hold 0.3 s / 0.4 s |
| Tutorial night: Mumbles only, lamps can't run dry for 30 s, first Mumble only after a lamp is lit, first pop +5 Glow | `NightPlan.pickCreature` (tutorial flag), `Config.TUTORIAL_GRACE`, `NightService._maybeSpawn` / `_onPop` |
| 9:00 night, dawn always arrives, no elimination | `NightPlan.LENGTH`, `NightService._endNight` |
| 3 stations (Harbor narrow/long, Stormside wide/short/slow, Crown fastest burn) | `Config.STATIONS`, `LampService` |
| 3 creatures (Mumble, Flicker zigzag, Hushhog needs 2 beams) | `Config.CREATURES`, `Beam`, `CreatureService` |
| Telegraphed lanes, 3 s | `CreatureService:Schedule`, lane glow in `Tick` |
| Sawtooth curve with rest beats, +5% per night capped at 10, human/bot scaling | `NightPlan.BEATS`, `target`, `equivalent` |
| Bots at 70% speed, hand back instantly | `LampService:Tick` (bot turn speed x0.7, slow refuel); keeper takes over by tapping or pouring |
| Relight 2 s | prompt `HoldDuration` is 2 s when a lamp is out |
| Fog event at 4:30, 1 of 3, the only random element | `Fog`, `NightService._tick` (not in tutorial) |
| Fixed rewards: 20 + 10 per star, 1 star minimum | `Economy` |
| Retry under 2 s, no game-over | Start lantern, `HoldDuration = 0` |
| Nothing for sale, no random purchases | no purchase code exists |

## Security (remotes-and-security note is missing; I used the hard rules)
- `AimBeam` (client to server) is the only remote the server listens to. Order: station id is a known string, angle is finite and under 1000 in magnitude (NaN and math.huge rejected), the sender is the lamp's keeper, 25 msg/s per-player limit. The server then caps the beam turn rate itself (`Aim.step`), so an exploiter cannot snap a beam.
- Oil pickup and pouring are server `ProximityPrompt.Triggered` events, not remotes. Fuel, creatures, the timer, and Glow are all server state.
- `HudUpdate` is server to client only.

## Persistence
No DataStore or ProfileStore code. Glow lives in player attributes and is lost when the player leaves. Daily bonus, weekly reward, and the upgrade board need ProfileStore (session locked, autosave, `BindToClose`, failed load kicks). That is a next step and needs Andrew's OK, because Lead's plan limits storage to the upgrade board.

## UNVERIFIED
- All of it compiles and runs: no tool was run.
- Touch and gamepad aiming (`AimController`), including `ScreenPointToRay` for touch and the right-stick mapping.
- That `Part.Shape = Cylinder` with the `CFrame.Angles(0,0,pi/2)` rotation stands upright and looks right.
- `ProximityPrompt` hold times dynamically changed each frame, and that a prompt with `HoldDuration = 0` works for retry.
- Balance numbers: all are starting values from the design doc.
- Max 4 players: a place setting, not enforced by code (`Players.MaxPlayers` is not set here).
- Friend join and invite prompt: not built, API not confirmed.
- Audio: no sound ids were invented; the pop has particles only (TODO in `CreatureService`).
- `selene.toml` needs the Roblox std definition file that `selene generate-roblox-std` creates; I did not generate one.
- Sizing and lane geometry: lanes are only east and west (4 lanes). Crown has no dedicated lane. Creatures only dim Harbor or Stormside.

## Open questions for Andrew
1. May Dawn Watch save data (ProfileStore) in the next build? Plan limits storage to the upgrade board.
2. Solo scaling: I mapped design "solo" to 1 human + 2 bots (equivalent 2.4) and "4 players" to 4 humans. Confirm.
3. The tutorial flag is per server (night 1 of the server), not per player, because there is no saving. Fine for now?
4. Does Rojo/Lune/Selene/StyLua exist in the studio environment? I could not check.

## Loaded
- `00-Start-Here/index.md`: required first read; routed Luau/Rojo to `13-Luau-Engineering`.
- `13-Luau-Engineering/index.md`: section index. **STUB.** Every note it lists is "planned" and does not exist on disk (Glob of that folder showed only `index.md`): **STUB: project-structure, remotes-and-security, headless-test-gate, datastore-playbook, framework-choice, performance-budgets, common-bugs-checklist.** I fell back to the hard rules in the builder skill. I did not run the common-bugs checklist; I checked by hand only that remotes are validated and nothing publishes.
- The design doc (`...--3-architect/design.md`) and Lead's plan (`...--2-lead/plan.md`): the task inputs.
- No second section loaded.
