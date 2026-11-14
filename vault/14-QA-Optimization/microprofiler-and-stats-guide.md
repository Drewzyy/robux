---
description: How to find slow frames with the MicroProfiler, Script Profiler, Developer Console Memory and Luau heap tools, and the Creator Hub Performance dashboard; what to capture and how to read it.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: MicroProfiler, Script Profiler, Developer Console, Memory, Luau heap, Heartbeat, Stepped, performance dashboard, P90, flame graph, frame time, server profile, Stats
sources: creator-docs content/en-us/studio/optimization/scriptprofiler.md (read 2026-11-14); creator-docs content/en-us/studio/optimization/memory-usage.md (read 2026-11-14); creator-docs content/en-us/production/analytics/performance.md (read 2026-11-14); creator-docs content/en-us/studio/network-simulator.md (MicroProfiler network capture, read 2026-11-14)
verify: Script Profiler, Memory tool and dashboard facts are from local docs. The MicroProfiler page (performance-optimization/microprofiler) is linked by the docs but not in the local clone, so MicroProfiler hotkeys and menu names below are UNVERIFIED model knowledge.
tags: [qa-optimization, playbook]
---
# MicroProfiler and stats guide

Pick the tool by the question. Which script is slow: Script Profiler. Which engine task is slow: MicroProfiler. Is memory growing: Developer Console Memory and Luau heap. Is it slow for real players: Performance dashboard.

## Do this
- Always profile the slow moment, not an idle scene. Reproduce the lag (full server, boss fight, many NPCs), then capture.
- Start with the frame budget: 16.67 ms for 60 fps, 33.3 ms for 30 fps (see [[14-QA-Optimization/performance-budgets]]). Any frame above budget is a candidate.
- Use the Script Profiler first for Luau cost. In the Callgraph view, tasks like `Heartbeat` and `Stepped` appear as nodes with your functions beneath; sort by cost and open the heaviest. File and line show on hover.
- Use the MicroProfiler to see engine work per frame: scripts, physics, rendering, replication. Look for the widest bar in a slow frame and read its label (UNVERIFIED hotkey: Ctrl+F6 in the client, check the docs).
- Take a server profile as well as a client one. A server heartbeat under 60 fps hurts every player (see [[14-QA-Optimization/lag-sources-ranked]]).
- Use the Developer Console Memory tab for categories: `Instances`, `Signals`, `LuaHeap`, `PhysicsParts`, `GraphicsParticles`, `GraphicsTexture`, `Gui`, `Animation`. Pick Client or Server in the dropdown.
- Use Luau heap snapshots to compare before and after a suspected leak (see [[13-Luau-Engineering/memory-leak-checklist]]).
- Change one thing, measure again, record both numbers in the bug or PR.

## Reading a frame
1. Find a spike frame (tall bar over the budget line).
2. Zoom to it and read the top-level scopes. Rule of thumb: the single widest child is where to look next.
3. If it is a script scope, switch to the Script Profiler to find the function.
4. If it is physics, count unanchored parts and constraints near players.
5. If it is rendering, cut particles, lights, overdraw, mesh detail.
6. If it is replication, look at remote firing rates and property changes on many instances.
7. Repeat on a second frame to confirm it is not a one-time load spike.

## Script Profiler workflow
- Open the profiler, start recording, reproduce, stop. Switch views: Callgraph (task tree), and the other views the docs list for sorting by function.
- Look for functions that run every frame and do table scans, `GetDescendants`, `GetChildren` on big folders, or string building.
- Export the data if you need to share (the doc covers exporting); attach to the bug.
- Move heavy loops from every-frame to a timer or event.

## Performance dashboard (post-release)
- Creator Hub, Creations, your game, Monitoring, Performance. Needs 100 daily active users for the charts.
- Use P50 for typical, P10 or P90 for the worst tail. A P10 server heartbeat of 40 means 10% of servers run at 40 or less.
- Break down by place version to see if the last update caused a regression.
- Client charts: crash rate, out-of-memory, memory use by platform. Server charts: heartbeat, memory (keep under 50% of total, about 3 GB guideline in the docs), network.
- The dashboard names CPU task groups that match MicroProfiler timers, so you can jump from chart to profile.

## Metrics to log in tests
| Metric | Where | Target start |
|---|---|---|
| Client frame time | MicroProfiler | under 16.7 ms (60 fps) or 33.3 ms (30 fps on low-end) |
| Server heartbeat | Dashboard, Dev Console | 60 fps steady |
| Server memory | Dev Console, dashboard | under 50% of total |
| Client memory | Dev Console, dashboard | no growth over 10 min |
| Ping | Stats, Network Simulator | playable at 150 ms |

UNVERIFIED: the "starting targets" are guidance, not hard limits.

## Mistakes
- Profiling an empty Studio place and calling the game fast.
- Profiling with Studio open and many other windows (distorts results).
- Optimizing a function that is 0.1 ms because it was the first one seen.
- Skipping the server; most lag reports are server-side.
- Comparing runs with different player counts.

## Related
[[14-QA-Optimization/optimization-checklist]], [[14-QA-Optimization/mobile-performance-cookbook]], [[14-QA-Optimization/bug-triage-and-repro]], [[13-Luau-Engineering/object-pooling]]

## For our Roblox games
Copy the principle: profile the slow moment, server and client, change one thing, record numbers. No names, art or assets are involved. Zero spend: all tools are free in Studio and Creator Hub.
