---
description: Particles, beams, trails, and highlights: what each is for and how to keep effects cheap on phones.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/performance-optimization
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/performance-optimization
---
# VFX guide

| Tool | Use it for | Watch out |
|---|---|---|
| ParticleEmitter | Sparks, dust, pickups, hits | Rate x Lifetime = particles alive. Keep bursts short; use `:Emit(n)` for one-off hits instead of high Rate. |
| Beam | Lasers, light shafts, connections between two points | Cheap; texture scrolling adds motion for free. |
| Trail | Swings, dashes, fast movers | Short Lifetime; disable when idle. |
| Highlight | Outlining a selected or important object | Only a limited number render at once; reuse one and move it. |
| PointLight / SpotLight | Local glow | Shadows on many lights are expensive; most do not need them. |

## Rules
1. Every reward and hit gets feedback within 0.1 s: a sound plus a small effect. See [[03-Game-Psychology/reward-schedules]].
2. Spawn effects on the client that sees them; the server only says "this happened".
3. Clean up: `Debris:AddItem(part, seconds)` or destroy emitters when done.
4. Test on the lowest graphics level; effects must still read.
