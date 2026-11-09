# Plan: Dawn Watch (pipe-pitch-to-build-20261109-105542-ca9c--2-lead)

## Goal
Turn Pulse's top-ranked pitch, Dawn Watch (4-player co-op, one 9-minute lighthouse night), into a design doc and then a first playable Rojo + Luau build, and put both in Andrew's review queue.

Pulse's list has three pitches: 1 Dawn Watch, 2 Nest Watch, 3 Tide Table. I take #1 as instructed. Pulse's own table scored it highest (20 of 25), and it has the lowest policy risk (1).

## Plan
| Stage | Agent | Input | Output | Gate |
|---|---|---|---|---|
| 1 | Architect | `runs/pipe-pitch-to-build-20261109-105542-ca9c--1-pulse/pitch.md`, pitch #1 only. Architect loads `10-game-design` (index is a stub, so it must say what was missing). | `runs/<architect run>/design.md` and `gate.json` | See Stage 1 gate below. |
| 2 | Builder | Stage 1 `design.md`. Builder loads `20-luau-rojo`. | Rojo project in `projects/dawn-watch/` (`default.project.json`, `src/`), `runs/<builder run>/build-notes.md`, `gate.json` | See Stage 2 gate below. |
| 3 (added by Lead, not in Andrew's two-stage pipeline) | Playtest | Stage 2 project and `build-notes.md`. | `runs/<playtest run>/playtest-report.md` and `gate.json` | Report covers spawn, the first 10 seconds, all 3 stations, a failed night, a solo run with bots, and phone, PC and gamepad input. Anything not run is marked NOT RUN. Problems are listed with severity. |
| 4 | Lead | Outputs of stages 1 to 3 | Review summary in the run folder | Every item in the Review section below is linked, and every UNVERIFIED claim is carried forward. |

### Stage 1 gate (Architect)
- The first lamp is lit within 10 seconds of spawn, and the player learns by doing, with no text wall.
- One night lasts 9 minutes. Retry takes under 2 seconds. Dawn always arrives, so there is no hard wipe.
- Exactly 3 stations and 3 creature types in MVP. Creatures are cute and strange, not horror or gore.
- Fixed rewards (lamp upgrades) come every night. The only variable element is the mid-night fog event.
- The shared lighthouse upgrade board is specified as the reason to return tomorrow.
- Solo and low-population play is specified (bots or scaling), because co-op needs other players online.
- A difficulty curve with rest beats is given as numbers, not adjectives.
- It names specifics of this game (lamps, fog, stations). It must not read as a doc for any game (anti-slop).
- Every claim from the vault has a `verify:` line, or is marked UNVERIFIED.
- No paid random items.

### Stage 2 gate (Builder)
- `rojo build` succeeds. If Rojo is not installed, record that as NOT RUN and fail the check. Do not claim a pass.
- Server owns the night timer, lamp fuel and creature state. The client only sends intent through remotes, and the server validates each remote.
- Only Roblox APIs that Builder confirmed in the vault or in official docs are used. The rest are marked UNVERIFIED.
- The MVP loop works start to finish: spawn, light lamp, tend, creatures arrive, dawn, reward.
- No publish, upload or place-save calls anywhere in the code or scripts.
- DataStore or ProfileStore use, if any, follows `20-luau-rojo` and is limited to the upgrade board.

## Review (what lands in Andrew's review queue)
1. `design.md` from Architect, with its `gate.json`.
2. The `projects/dawn-watch/` Rojo project and `build-notes.md` from Builder, with its `gate.json`.
3. The Playtest report, if stage 3 runs.
4. This plan and its `gate.json`.

Nothing is published, uploaded or posted. Andrew must approve each exact item before it goes anywhere further. Approving the design does not approve the build, and approving the build does not approve a launch. A launch also needs the policy check and the launch checklist, which nobody has run.

## Blocked
Nothing blocks Stage 1 or Stage 2. These need Andrew and are not worked around:
1. **Live demand numbers.** Every number in Pulse's pitch is UNVERIFIED and some contradict each other (986K vs 1.6M CCU for the same game). Someone with chart access (Rolimons, RoVitals, BLOXG, RoWatcher) must check them. Dawn Watch relies on the "co-op survive-the-night" thread, which rests on one source (S4, forgegui, Jun 2026).
2. **Co-op decision.** Pulse asked whether 4-player co-op is acceptable. I told Architect to design for 1 to 4 players with bots filling empty stations, so solo works. Andrew should confirm or change this at review.
3. **Policy check.** `60-policy` was not loaded by Pulse or by me. A pre-launch policy check, including the age rating for fog creatures, is needed before any publish. Nobody has run it.
4. **Missing vault notes.** `70-competitors` notes (teardown-template, tracker-watchlist, copy-vs-avoid) do not exist. `10-game-design/index` and `90-templates-checklists/index` are stubs. Andrew decides whether to write them. Agents carry on without them and mark what they could not check.

## Open questions
- Should Playtest (stage 3) run before Andrew's first review, or only after he approves the design?
- Is Rojo installed in the Builder environment? I did not check, and I did not check the toolchain.

## Loaded
- `00-router/index.md`: required first read. It routed me to `90-templates-checklists`.
- `90-templates-checklists/index.md`: my home section. It is a stub, and every note it lists is marked "planned".
- Not read: `agent-output-format`, `mvp-build-order`, `anti-slop-checklist`. They are only "planned", and I did not need them to write the plan. The anti-slop bar comes from the lead skill.
- `runs/pipe-pitch-to-build-20261109-105542-ca9c--1-pulse/pitch.md`: the input named in Andrew's instruction.
