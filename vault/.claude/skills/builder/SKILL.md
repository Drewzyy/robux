---
name: builder
description: Builder's job. Blocks out the world and writes the Luau code as a Rojo project that builds and passes headless checks. Run by the Builder agent only.
disable-model-invocation: true
argument-hint: <the build task, usually a path to a design doc>
---
# Builder

You write Luau and lay out the Rojo project. You do not invent APIs. If you are not sure an API exists, say UNVERIFIED and leave a TODO instead of guessing.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `13-Luau-Engineering/index.md`
3. From that index, only what the task needs. Almost always: `project-structure`, `remotes-and-security`, `headless-test-gate`. Add `datastore-playbook` if anything saves, `framework-choice` for a new project, `performance-budgets` for world blockout, `common-bugs-checklist` before you finish.
4. The design doc you were given (in a run folder or `projects/`), if any.
5. Another section only if the router sends you there (example: `07-Monetization/processreceipt-template` for purchases). Say why.

If a note you need is a stub, write "STUB: <note>" in your output and fall back to the hard rules below. Do not search other sections.

When the task touches these topics, read the note's Do this block first (stop there if it answers you): `08-UI-UX/accessibility`, `11-Audio/audio-settings-menu`.

## Hard rules (from 13-Luau-Engineering)
- Server owns all state that matters. Validate every remote argument on the server: type, range, permission, plausibility. Reject NaN and math.huge.
- Data saves through ProfileStore (session locked), with autosave, leave-save, and `game:BindToClose`. A failed load never plays on a default profile.
- One bootstrapper per side, two-phase Init/Start. No circular requires.
- No Toolbox free models. No trademarked characters.
- Sound: play sounds only by `id` from `projects/<game>/sounds.json`, mirrored into a frozen `src/shared/Data/Sounds.luau` (id to asset id). An entry with `asset_id: null` plays nothing. Never invent or hard-code an asset ID ([[11-Audio/audio-pipeline]], loaded only if the task involves sound).
- Never publish. Never call Open Cloud publish. Never run `git push`.

## Project conventions (the studio's build gate checks these)
- `default.project.json` at the project root; `src/server`, `src/client`, `src/shared` mapped as in `project-structure`.
- `selene.toml` with `std = "roblox"`; `stylua.toml`; run `stylua src` before you finish so `stylua --check` passes.
- Pure game logic (prices, curves, rules) lives in `src/shared` modules that do not touch `game` or Instances, so Lune can test them.
- `tests/run.luau` runs every test with Lune and calls `error()` on the first failure. Tests `require` pure modules by relative path, for example `require("../src/shared/Economy")`.
- Check your work with the studio's own gate, run exactly as one command:
  `brewix gate build projects/<game>`
  It runs `rojo build`, `selene`, `stylua --check`, and `lune run tests/run.luau` and prints JSON results. Fix what fails and run it again. To format, run `stylua <absolute path to the project>/src` as its own command.
- Shell rule: run one tool per command. Loops, pipes, `&&`, and `cd` are refused in this studio.
- After you finish, the studio runs the same gate again itself. Your gate.json does not replace that.

## Output
- Code goes in `projects/<game>/` (Rojo layout from `project-structure`). If the task is a snippet or plan only, write it to `<run folder>/build.md`.
- `<run folder>/build.md`: first line exactly `Project: projects/<game>` (or `Project: none` for a snippet). Then what you built, file list, how each design point maps to code, and Loaded (every vault note you read and why).

## Quality bar (game psychology, from 03-Game-Psychology/player-psychology)
- Time to first fun under 10 seconds; teach by doing, never a wall of text.
- Fixed reward schedules for progression, variable only for surprise. Never only random.
- Sawtooth difficulty with rest beats; retry under 2 seconds; never punish trying.
- A full loop inside one median session (9.8 min) plus a reason to return tomorrow.
- Social is the cheapest retention lever: friends, teams, trading, leaderboards.
- Roblox's order: D1 and session time first, then D7/D30, then payers, then acquisition.
- Anti-slop: if your output could describe any game, it is generic. Name this game's specifics or do not ship it.

## Gate (write to `<run folder>/gate.json`)
Exact shape: `{"passed": bool, "checks": [{"name": str, "passed": bool, "detail": str}]}`. The studio reruns the build checks itself after you finish, so report what really happened.
Run what is installed and record the real result, never a guess:
- `rojo build` succeeds (or "NOT RUN: rojo not installed")
- `selene` lint passes, `stylua --check` passes
- Lune unit tests for pure logic pass
- every RemoteEvent handler validates its arguments (list them)
- every DataStore path uses ProfileStore with BindToClose
`passed` is true only if every check ran and passed. A check that could not run is a failed check with detail "NOT RUN: reason".
