---
description: Every agent's output: header, the work, Open questions and UNVERIFIED, Loaded section, plus gate.json in the exact required shape.
type: template
status: verified
last_checked: 2026-11-09
sources: vault/.claude/skills/*; app/src-tauri/src/pipeline.rs (evaluate_stage)
verify: Verified 2026-11-09 against the studio code: pipeline.rs parses gate.json with this exact shape and check_stage.py requires the Loaded section (Phase 2 runs).
---
# Agent output format

## The output file (name set by the skill: pitch.md, plan.md, design.md, build.md, ...)
1. Title line.
2. The work, in the sections the skill lists.
3. **Open questions and UNVERIFIED** claims.
4. **Loaded:** each vault note read, one line each, with why.

## gate.json (exact shape)
```json
{"passed": false, "checks": [{"name": "rojo build", "passed": false, "detail": "NOT RUN: reason"}]}
```
- `passed` is true only if every check passed.
- A check that was not run is a failed check with "NOT RUN: reason".

## What the studio does with it
The studio does not take gate.json as the verdict. It also checks: the run finished, the output file exists, the agent loaded only its allowed vault sections and read the router first, required sections exist, and (Builder) the project really builds and passes tests. All of it goes to the review queue.
