---
description: Playtest QA: how it was tested, spawn, first 60 seconds, every stage readable, checkpoints, coins, shop, no early pop-ups, PC, phone, gamepad, performance.
type: checklist
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C9 (C9); 02-Game-Design and 08-UI-UX checklists
verify: Checklist used by the Playtest agent; its report is checked by scripts/gates/check_stage.py (method stated, verdict, severities).
---
# Playtest QA checklist

## State the method first
Live in Studio (Studio MCP or Andrew), headless (Lune tests, rojo build), or code walk only. Never imply live play that did not happen.

## First 60 seconds (per platform)
- [ ] Spawn faces the objective; no wall of text.
- [ ] Core verb within 5-10 s.
- [ ] First reward within 10-30 s with feedback.
- [ ] Next goal visible by 30-60 s.

## Core systems
- [ ] Every stage or level readable before the jump or action.
- [ ] Checkpoints save and respawn under 2 seconds.
- [ ] Currency earned matches the design numbers.
- [ ] Shop opens, shows Robux prices, never pops up on join, on death, or in the first 5 minutes.
- [ ] Progress saves and loads (rejoin test).

## Platforms
- [ ] PC: keyboard and mouse.
- [ ] Phone: touch targets reachable, nothing under the joystick or top bar.
- [ ] Gamepad: every action reachable.

## Performance
- [ ] Loads to playable under 10 s; no stutter on a low-end phone profile.

## Report each problem as
id, platform, where (file and line or place), what happens, what should happen, severity (blocker, major, minor). Verdict PASS only with no blockers or majors. A generic pass is a failed report.
