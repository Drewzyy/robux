# Playtest report: Dawn Watch "Fast First Minute"
Run: pipe-improve-build-20261109-114826-63b1--3-playtest. Date: 2026-11-09.

## How tested
**Code walk only. Nothing was played live.** Studio MCP was not used. I could not run Lune, `rojo build`, selene or stylua because Bash was denied in this run. The builder's gate result ("rojo, selene, stylua, Lune passed") is therefore not re-verified by me.
I read: `NightService`, `LampService`, `CreatureService`, `GuideService`, `WorldService`, `RemotesService`, `HudController`, `AimController`, and the shared `FirstRun`, `Tutorial`, `Economy`, `Config`, `Beam`.
I did not read the specs, `Aim`, `NightPlan` or `Main.server`. Every timing below is a calculation from constants, not a stopwatch. Per the design, every number is a starting value.

## Verdict: FAIL
There are 0 blockers and 5 majors. Fix the majors, then do the 5-run stopwatch test per device.
The core timeline is sound on paper. The player spawns with oil, a 0.4 s first light, a first Mumble 60 studs out after a 2 s telegraph, and a +10, +5, +5 Glow schedule. The first pop lands at about 9 to 11 s in the code.
Two things stop it from holding up. The "first pop needs no aiming" promise breaks on PC and phone. The guidance points at the wrong lamp at two moments.

## Problems
| id | platform | where | what happens | what should happen | severity |
|---|---|---|---|---|---|
| P-01 | PC, phone | `AimController.luau` 42-50, 74-82; `RemotesService.luau` 59; `LampService.luau` 196-203 | `PreAim` snaps the beam to lane 2 (0.25 rad) when the player pours. The pourer becomes keeper in the same call, so `Station` is set. Within 0.05 s the client's Heartbeat sends its stored mouse or touch angle. `lastSent` is nil, so it always sends. The server sets `targetAim` and the beam turns away at 3 rad/s. Harbor's half-width is 0.1, so the beam must stay within 0.1 rad of 0.25. A mouse resting anywhere else on the sea plane loses the first pop. | The first Mumble pops with no aiming, as the design promises. Either hold the pre-aim until the player first deliberately aims, or ignore client aim for about 3 s after a pre-aim. | major |
| P-02 | phone | `AimController.luau` 50-51 | Any `Touch` `InputChanged` sets the aim angle, with no `gameProcessed` check. The thumb dragging the movement joystick on the walk to the lamp therefore sets the beam angle from the joystick's screen position. This feeds P-01. A second finger is needed to aim at all, and nothing teaches that. | Ignore touches that start on the joystick or on the UI. Show a drag-to-aim cue at the second Mumble. UNVERIFIED on a device. | major |
| P-03 | PC, phone, gamepad | `WorldService.luau` 78 and 85-107; `Config.luau` 41-69; `GuideService.luau` 102 | The spawn is (0, 21, 8). The Crown Lens (0, 26, 0) is about 9 to 10 studs away, inside the 12-stud prompt range. Harbor (10, 23, 0) is about 13 studs away, outside it. The barrel prompt "Grab oil" is 3 studs away. At spawn the first prompts the player sees are Crown's "Tend" and the barrel. The pillar and arrow point at Harbor, the farthest of the three. The design assumed Harbor is the obvious first lamp. | The one prompt in reach at spawn should be the lamp the pillar marks. Point the pillar at the nearest dark lamp, or move the spawn or Crown. Mute the barrel prompt until "Refuel!". | major |
| P-04 | all | `GuideService.luau` 82-110; `LampService.luau` 206-214 | `needPour` is true while any player has 0 pops. After the first lamp is lit, `FirstDarkPosition` returns the next dark lamp (Stormside, then Crown). The pillar and arrow jump to a second lamp for the 3 to 5 s until the first pop. The player has no oil, since they poured their only one. The hint says "Light it up." The screen shows a lit lamp, a coming creature, and an arrow to another lamp. This breaks "one mechanic, one goal". | Hide the pillar once any lamp is lit by that player. A joiner's pillar should mark a lamp they can use. | major |
| P-05 | all (joiner) | `Tutorial.luau` 18-25; `GuideService.luau` 102-110; `NightService.luau` 105-120 | A player who joins a running night while bots have lit lamps sees `anyLit` true. With 0 pops they get the hint "Light it up." while holding oil. The pillar marks only a dark lamp (`fuel <= 0`). If all three lamps are lit, there is no pillar at all. Nothing says "pour into this lamp to become its keeper". The welcome Mumble only comes after they pour, and nothing guides them to pour. The design promises the same fast minute for joiners. | For a 0-pop player holding oil, show "Pour it in." and mark the lamp they should take, bot-lit or not. Pillar logic should key on the player's own state. | major |
| P-06 | gamepad | `AimController.luau` 52-65; `Tutorial.luau` 27-28 | The hint "Aim at glow." never says how to aim. Gamepad aiming uses `Thumbstick2`, which is also the default camera-rotate stick. Aim is derived from the camera look vector, so turning the camera changes the aim. The builder and the design both flag gamepad aim UNVERIFIED. The second pop (lane 1) cannot be done by luck on gamepad, because there is no pre-aim. | A tested aim input on gamepad. Either a glyph or cue for the aim input, or disable camera rotation from the right stick while keeper. | major |
| P-07 | all | `Tutorial.luau` 24-26 | After the lamp is lit and while the Mumble is telegraphed and drifting (about 6 to 8 s), the hint says "Light it up." The lamp is already lit. | Say what is happening, such as "Hold the beam." or nothing. | minor |
| P-08 | all | `Tutorial.luau` 27-32 | While `popped == 1` the hint is "Aim at glow." and masks "Refuel!" even if the oil is low. If bots or luck skip the lesson, the hint is wrong. | "Refuel!" should win when the lamp is low and the player has no oil. | minor |
| P-09 | all | `Config.luau` 22; `LampService.luau` 313-323; `NightService.luau` 210-216 | Bots start at 20 s in the tutorial. The ringed second Mumble spawns at about 13.5 s on a 110-stud lane and takes about 12 s to arrive. Bot lamps (Crown/Stormside, refuel after 6 s) can pop it at about 26 s or later. All keepers get the +5 and `Pops` becomes 2, so the aim hint disappears though the player never aimed. UNVERIFIED. | Delay bots in the tutorial until the player's second pop. | minor |
| P-10 | all | `NightService.luau` 78-82, 85-89 | Starter oil is granted on every respawn while `Pops == 0`, even mid-night. It gives free oil without a barrel trip and weakens the "Refuel!" lesson. | Grant it once per night, not per respawn. | minor |
| P-11 | all | `CreatureService.luau` 148 | No sound on first pop or lamp light (no approved asset IDs). The first-60-seconds checklist asks for a visual and sound celebration at 10 to 30 s. The visual (burst, flash, "+10 Glow", "First light!") is present. This needs Andrew's assets and is not the builder's fault. | Audio before launch. | minor |
| P-12 | phone | `HudController.luau` 51, 57, 93-98 | The status line, Glow bar and the "+N Glow" float end are at the bottom, in the joystick and jump zone. The status line is 18 px, not wrapped, in 60% of the width. "Carrying oil | Glow 20 | Next: Lamp Polish in 20 Glow" is about 55 characters and probably spills past its label on narrow phones. The thumbs cover the open-loop cue. UNVERIFIED on a device. | Move the Glow readout and bar up. Wrap or scale the text. Check on the Device Emulator and a real phone. | minor |
| P-13 | phone | `NightService.luau` 93-99 | The night starts 1 s after the server-side character spawn, not after the client has loaded. On a slow phone, tutorial time and the 30 s lamp grace burn during the loading screen. `HudController` also misses `HudUpdate` events fired before it connects, until the next 0.25 s update. UNVERIFIED. | Start the clock when the client reports it is ready, or accept it and measure. | minor |
| P-14 | all | `design.md` sections 3 and 5 | The design contradicts itself. Section 3 says "25 of 40, Lamp Polish in 15 Glow at about 35 s". Section 5 says 20 of 40. The code pays 10 + 5 + 5 = 20, so "in 20 Glow" is correct after the 3rd pop. After the 2nd pop it shows 15 of 40. | Fix the doc, or tune the amounts. | minor |

## First 60 seconds walk (calculated from code; the order is tight)
Assumes a fresh server and the intended path. P-01 to P-05 change it.
| Time | PC | What happens |
|---|---|---|
| 0 s | all | `CharacterAdded` fires `_onSpawn`. `Oil` is set to 1 ("Carrying oil"). The Lobby timer starts: `StartNight` runs after 1 s. |
| 1 s | all | Night 1 starts (tutorial). Lamps reset and oil is re-granted. The hint is "Pour it in." The pillar marks Harbor (P-03). |
| 1 to 5 s | all | The player walks. The Crown prompt is in range at spawn and Harbor is not (P-03). |
| about 5 s | PC | Hold E for 0.4 s on the lamp. Fuel 40. `OnPoured` calls `PreAim` and schedules a fast Mumble. |
| about 5 to 6 s | PC | The beam shows. The aim is overridden within 0.05 s by the stored mouse angle (P-01). The lane 2 glow is on at once. |
| about 7 s | all | The Mumble spawns 60 studs out on lane 2. It takes 6 s to reach touch distance and needs 1.5 s of beam to pop. |
| about 8.5 to 11 s | all | If the beam is on lane 2: it pops with burst, flash, "+10 Glow" float and a 2 s "First light!" banner. No sound (P-11). If the beam is off lane 2 (P-01/P-02): no pop, and the Mumble reaches the lamp at about 13 s and dims it by 30. |
| about 11 to 17 s | all | The second Mumble goes to lane 1 with a ring, at a 3 s telegraph and 110 studs. It needs a real aim ("Aim at glow."). |
| about 20 s | all | Bots take the other stations (P-09). |
| about 49 s | all | The lamp falls below 25 and "Refuel!" shows, with a barrel highlight. This matches the design, because the 30 s lamp grace plus 40 fuel at 0.8 per second ends at about 49 s. |
| 30 to 60 s | all | The third pop makes the Glow bar 20 of 40 (P-14). |

## Platform notes
- **PC:** P-01 and P-03. The 0.4 s hold on E is fine. No other issue found in the lamp, pop or HUD logic for keyboard and mouse once P-01 is fixed.
- **Phone:** P-02, P-12, P-13. The 0.4 s tap-and-hold on a ProximityPrompt and the pillar rendering are UNVERIFIED. `HudController` positions the bar at y = 12 px and the title at y = 28 px, inside the 58 px top bar zone. They are display-only, so no tap is blocked.
- **Gamepad:** P-06. The prompt hold is the default gamepad button. For the first pop gamepad is the only platform where pre-aim survives, because the stick is idle until moved. The second pop is where it fails.
- **Security:** `AimBeam` still checks type, keeper and rate (`RemotesService.luau` 43-59). All new state is server-set and the client only reads attributes. No issue found.
- **Not checked at all:** performance and low-end phone load time, rejoin and save (nothing saves this build), the shop (none exists), and the specs and `selene`/`stylua`/`rojo build` results.

## Open questions
1. Should the first pop depend on aiming at all? If pre-aim is kept, the client must be stopped from overriding it (P-01, P-02).
2. Which lamp is the first lamp: Crown (nearest) or Harbor (farthest)? Move the spawn, or make the pillar follow the nearest.
3. Who supplies the audio IDs (P-11)?

## Loaded
- `00-router/index.md`: required first read; sent the playtest task to `90-templates-checklists`.
- `90-templates-checklists/index.md`: section index.
- `90-templates-checklists/playtest-qa-checklist.md`: method, first-60-seconds items, platforms, problem format.
- `90-templates-checklists/anti-slop-checklist.md`: specific-not-generic and UNVERIFIED rules.
- `10-game-design/first-60-seconds-checklist.md`: the four time windows and the "kills Day 1" list.
- `30-ui-ux/mobile-first-checklist.md`: bottom 30% and top 58 px zones.
- Not loaded: the genre playbook. The skill lists it, but the router has no playbook for this genre and I did not search other sections. This is a gap, not a stub.
- Project and run files read (not vault): the design doc, the build doc, and the source files listed under How tested.
