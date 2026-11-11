---
description: Tiered performance budgets: 16.67 ms frame at 60 fps, memory by device tier, part counts, particles, streaming; measure on the weakest device.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2; brockmartin performance-optimization; tabooharmony roblox-performance; el4cteo roblox-performance; andrian-syh device-performance
verify: Frame budget is Roblox guidance. Memory, part, and particle numbers differ between community sources: UNVERIFIED as limits, use as tiered targets and measure with MicroProfiler on a low-end phone.
tags: [qa-optimization, reference]
---
# Performance budgets

## Frame
60 fps = 16.67 ms per frame (Roblox guidance); 30 fps = 33.3 ms. Scripts should use a few ms at most. One Heartbeat callback taking 8 ms is half a 60 fps frame.
Targets: 60 fps desktop, 45 to 60 mobile, never under 30.

## Memory (sources disagree; tiered targets)
| Tier | Target | Source notes |
|---|---|---|
| Low-end phone | 400 to 500 MB | one reference also gives 1,000 draw calls, 1,000,000 triangles |
| Mobile | under 600 to 800 MB | 1.2 GB called a crash zone in one reference |
| Desktop | about 1.5 GB | |

## World
- About 500 parts max per model; visible scene under 10,000 parts desktop, 5,000 to 7,000 mobile.
- Anchor every static part (unanchored parts enter the physics solver).
- Per model (studio rule of thumb, UNVERIFIED against Roblox docs): small props under 2,000 triangles, hero props under 10,000. Roblox caps one imported mesh at about 20,000 triangles (UNVERIFIED). Reuse one mesh many times instead of many unique meshes.
- MeshParts sharing a MeshId are instanced (cheap). Unions can be expensive: check triangles in Scene Analysis.
- Particles: about 100/s mobile, 400 desktop in one reference; cap around 50 active emitters in another. Limit shadow-casting lights, beams, trails.
- Load to playable under 10 s, 20 s at most.

## Streaming
On by default for new places. Start `StreamingTargetRadius` at 256 studs, `StreamingMinRadius` about 64, `StreamOutBehavior = Opportunistic`. Set in Studio on Workspace; not settable by script.

## Measure
On the weakest device, not the dev machine. The Studio device emulator is not a memory test (server and client share one process). Tools: MicroProfiler (Ctrl+F6), Script Profiler, Developer Console (F9), Performance Stats, Creator Hub Performance dashboard. Roblox's worked example test set: Infinix Smart 9, Motorola Moto G05, Oppo A18, Amazon Fire HD 10, Samsung Galaxy S22 Ultra.
