---
description: Scenic's note: lighting goals (readability first, then mood) and where values live. Specific property values are UNVERIFIED until tested in Studio.
type: reference
status: unverified
last_checked: 2026-11-09
sources: Research report has no lighting section; goals derived from readability rules in C1 and performance rules in C2
verify: UNVERIFIED: the research does not cover lighting. Property names must be checked against the Lighting and Atmosphere Engine API pages before use, and values tested in Studio on a phone.
---
# Lighting and mood

The research behind this vault has no lighting section. This note holds goals only.

## Goals (from verified rules elsewhere)
1. **Readability first.** Hazards and the next goal stand out from the background in every zone ([[10-game-design/genre-playbooks/obby]]).
2. **Phone performance.** Limit shadow-casting lights ([[20-luau-rojo/performance-budgets]]).
3. **Mood second.** Match the design doc's mood words.

## Where values live
A Rojo module (example `src/shared/Configs/Lighting.luau`) applied by the server at start, so the look is in the repo, not only in Studio.

## Before writing values
UNVERIFIED: which properties exist on `Lighting` and `Atmosphere` and their ranges. Read the Engine API pages, then test in Studio on a phone. Record what worked here, after a successful run.
