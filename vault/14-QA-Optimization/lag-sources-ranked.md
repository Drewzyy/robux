---
description: Ranked list of the usual causes of Roblox lag (server heartbeat, replication, physics, scripts, rendering, memory) with how to spot each one and the first fix to try.
type: reference
status: draft
last_checked: 2026-11-14
keywords: lag, server lag, heartbeat, replication, physics, unanchored parts, remote spam, GetDescendants, particles, memory, ping, network ownership, constraints, frame time
sources: creator-docs content/en-us/production/analytics/performance.md (server heartbeat, memory, read 2026-11-14); creator-docs content/en-us/studio/optimization/scriptprofiler.md (read 2026-11-14); creator-docs content/en-us/studio/optimization/memory-usage.md (read 2026-11-14); creator-docs content/en-us/studio/network-simulator.md (read 2026-11-14)
verify: Ranking is an experience-based heuristic from model knowledge, UNVERIFIED as universal. Always confirm with a profile before fixing (see the MicroProfiler guide). Thresholds are starting points.
tags: [qa-optimization, reference]
---
# Lag sources ranked

First decide which lag: server lag (everyone stutters, other players rubber-band), client lag (only one player, low FPS), or network lag (high ping). Then use this list from top to bottom.

## Do this
- Identify the lag type before touching code: server heartbeat chart or Dev Console for server, local frame time for client, ping and Network Simulator for network.
- Work the list in order. The first items fix most games.
- Profile, change one thing, profile again (see [[14-QA-Optimization/microprofiler-and-stats-guide]]).
- Keep the server at a steady 60 heartbeat. If P10 server heartbeat on the dashboard drops to 40 or below, 10% of servers are suffering (per the dashboard doc).

## The ranked list

**1. Heavy per-frame scripts (server or client).** Loops with `GetDescendants`, `GetChildren`, `FindFirstChild` over thousands of parts, string building, or work per player per frame. Spot: Script Profiler, Heartbeat node wide. Fix: move to events, run on a timer (10 Hz), cache lists, tag with `CollectionService`.

**2. Too many unanchored, simulated parts.** Debris, ragdolls, destructible maps, hundreds of tools. Spot: MicroProfiler physics scopes, `PhysicsParts` memory. Fix: anchor what you can, `CanCollide`/`CanTouch`/`CanQuery` off for decor, delete debris after 3 to 5 s (see [[13-Luau-Engineering/object-pooling]]).

**3. Remote and property spam (replication).** Firing remotes every frame, updating properties on many instances every frame. Spot: network capture in the MicroProfiler, bandwidth. Fix: send on change, batch, lower the rate, use attributes sparingly, `UnreliableRemoteEvent` for cosmetic data (see [[13-Luau-Engineering/networking-hardening]], [[13-Luau-Engineering/cooldowns-and-rate-limits]]).

**4. Too many NPCs with full AI.** Pathfinding per NPC per frame, raycasts every step, Humanoids with many accessories. Spot: server heartbeat falls as NPC count rises. Fix: shared brain loop at 5 to 10 Hz, path recompute throttled, cull distant NPCs (see [[12-Multiplayer-NPCs/enemy-waves-and-spawners]], [[12-Multiplayer-NPCs/npc-ai]]).

**5. Physics ownership and constraints.** Many constraints, network ownership flipping between clients, vehicles without owner. Spot: jittery parts. Fix: set ownership deliberately (`SetNetworkOwner(nil)` for server NPCs), reduce constraints, simplify collision geometry.

**6. Particles, lights, transparency (client rendering).** Spot: low FPS on phones even with a quiet script profile. Fix: see [[14-QA-Optimization/mobile-performance-cookbook]].

**7. Memory growth and leaks.** Spot: Memory tab grows over time; out-of-memory exits on dashboard; server memory above about 3 GB or 50% of total (per the dashboard doc). Fix: [[13-Luau-Engineering/memory-leak-checklist]].

**8. Huge map without streaming.** Spot: long load, crash on phones. Fix: [[12-Multiplayer-NPCs/server-size-and-streaming]].

**9. DataStore calls on hot paths.** Spot: yields, queue warnings in output. Fix: cache in memory, save on leave and on a slow timer (see [[13-Luau-Engineering/datastore-playbook]]).

**10. Network latency itself.** Spot: ping high for far-away players. Fix: client prediction for feel, server confirmation, avoid round trips for every step; test with Network Simulator presets (see [[14-QA-Optimization/device-test-matrix]]).

## Quick diagnosis table
| Symptom | Likely cause |
|---|---|
| Everyone rubber-bands at once | server heartbeat low: 1, 2, 4 |
| One player low FPS, others fine | client: 6, 7, 8 |
| Lag grows over an hour | 7 (leak) |
| Lag when a boss or wave starts | 2, 3, 4 |
| Lag only for far-away players | 10 |
| Hitch every few seconds | GC churn: Clone/Destroy spam (see object pooling), or a periodic heavy script |

## Mistakes
- Fixing rendering when the problem is the server.
- Adding `task.wait()` everywhere without finding the heavy loop.
- Ignoring the dashboard while chasing one reported device.
- Optimizing without a before and after number.

## For our Roblox games
Copy the principle: classify the lag, then work a ranked list with measurements. No names, art or assets are involved. Zero spend: free tools only.
