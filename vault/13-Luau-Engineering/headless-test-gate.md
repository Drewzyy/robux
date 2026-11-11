---
description: Builder's build gate: rojo build, Selene, StyLua check, and Lune tests via scripts/gates/build_gate.py. What passing means and does not mean.
type: reference
status: verified
last_checked: 2026-11-09
sources: scripts/gates/build_gate.py; Phase 2 verification (LOG.md)
verify: Verified 2026-11-09: gate passes a known-good project and fails a project with a lint error, bad formatting, and a failing test (LOG.md Phase 2).
tags: [luau-engineering, reference]
---
# Headless test gate

Run it yourself before finishing:
```
brewix gate build projects/<game>
```
The studio runs the same script again after you finish; your gate.json does not replace it.

## Checks
| Check | Passes when |
|---|---|
| `rojo build` | `default.project.json` builds a place file |
| `selene lint` | no errors or warnings in `src/` |
| `stylua --check` | `src/` is formatted (run `stylua <abs path>/src` to fix) |
| `lune unit tests` | `tests/run.luau` exits cleanly |

A tool that is missing is a failed check ("NOT RUN"), never a pass.

## Writing tests Lune can run
- Pure logic lives in `src/shared` modules that do not touch `game`, Instances, or services.
- `tests/run.luau` requires them by relative path: `require("../src/shared/Economy")`.
- Use `assert(cond, "message")`; the first failure stops the run with a non-zero exit.

## What passing does NOT mean
It does not mean the game works in Roblox. Rojo builds, lint, format, and pure-logic tests say nothing about remotes, physics, UI, or feel. Selene offline cannot check API member names. Live behaviour needs a Studio playtest (Studio MCP) or Andrew.
