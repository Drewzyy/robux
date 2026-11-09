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
1. `00-router/index.md`
2. `20-luau-rojo/index.md`
3. From that index, only what the task needs. Almost always: `project-structure`, `remotes-and-security`, `headless-test-gate`. Add `datastore-playbook` if anything saves, `framework-choice` for a new project, `performance-budgets` for world blockout, `common-bugs-checklist` before you finish.
4. The design doc you were given (in a run folder or `projects/`), if any.
5. Another section only if the router sends you there (example: `40-monetization/processreceipt-template` for purchases). Say why.

If a note you need is a stub, write "STUB: <note>" in your output and fall back to the hard rules below. Do not search other sections.

## Hard rules (from 20-luau-rojo)
- Server owns all state that matters. Validate every remote argument on the server: type, range, permission, plausibility. Reject NaN and math.huge.
- Data saves through ProfileStore (session locked), with autosave, leave-save, and `game:BindToClose`. A failed load never plays on a default profile.
- One bootstrapper per side, two-phase Init/Start. No circular requires.
- No Toolbox free models. No trademarked characters.
- Never publish. Never call Open Cloud publish. Never run `git push`.

## Output
- Code goes in `projects/<game>/` (Rojo layout from `project-structure`). If the task is a snippet or plan only, write it to `<run folder>/build.md`.
- `<run folder>/build.md`: what you built, file list, how each design point maps to code, and Loaded (every vault note you read and why).

## Gate (write to `<run folder>/gate.json`, same shape as other agents)
Run what is installed and record the real result, never a guess:
- `rojo build` succeeds (or "NOT RUN: rojo not installed")
- `selene` lint passes, `stylua --check` passes
- Lune unit tests for pure logic pass
- every RemoteEvent handler validates its arguments (list them)
- every DataStore path uses ProfileStore with BindToClose
`passed` is true only if every check ran and passed. A check that could not run is a failed check with detail "NOT RUN: reason".
