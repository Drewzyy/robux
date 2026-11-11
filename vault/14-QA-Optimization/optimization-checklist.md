---
description: Keep frame rate and memory inside budget on low-end phones: what to measure and the usual fixes.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/performance-optimization
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/performance-optimization
---
# Optimization checklist

## Measure first
- Developer Console (F9): memory, network, script activity.
- MicroProfiler (Ctrl+F6 / Ctrl+Alt+F6): where frame time goes.
- Test on a phone or the lowest graphics setting, not only a fast PC.

## Usual fixes
- Fewer parts: merge static scenery, use meshes, set `CanCollide`, `CanTouch`, `CanQuery` off where not needed.
- Turn on Streaming so far parts are not loaded.
- No per-frame loops that do work every frame for every object; use events.
- Fewer shadows and lights; short-lived particles.
- Destroy what you create; disconnect events you no longer need.

Budgets for this studio: [[14-QA-Optimization/performance-budgets]].
