---
description: NPCs that feel alive: PathfindingService, simple state machines, and keeping many NPCs cheap.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/performance-optimization
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/performance-optimization
---
# NPC AI

## Movement
`PathfindingService:CreatePath(params)` then `path:ComputeAsync(from, to)` and walk the waypoints with `Humanoid:MoveTo`. Recompute when blocked (`path.Blocked`) or every few seconds, not every frame.

## Behaviour as a state machine
Idle -> Wander -> Notice player -> Chase or Help -> Return. One module, one state at a time, clear transitions. Easy to test and to tune.

## Cheap at scale
- Run NPC logic on the server at a low rate (for example 5 to 10 times a second), not every Heartbeat.
- Many NPCs: use simple anchored models with tweened movement instead of full Humanoids.
- Sleep NPCs far from every player.

## Bots as players
Companions or rivals fill empty servers. Make it clear they are bots.
