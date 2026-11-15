---
description: When to use an ECS (jecs or Matter) or a server-authoritative character controller (Chickynoid) in a Roblox game, with stars, licenses, short examples and the cases where plain modules win.
type: reference
status: draft
last_checked: 2026-11-15
keywords: ecs, jecs, matter, entity component system, chickynoid, character controller, server authoritative, prediction, lag compensation, system, query, component, archetype, rollback
sources: https://github.com/Ukendio/jecs (README.md, how_to/, read 2026-11-15); https://github.com/matter-ecs/matter (README.md, docs/GettingStarted.md, example/, read 2026-11-15); https://github.com/easy-games/chickynoid (README.md, Index.md, src/, read 2026-11-15); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. The 800,000-entities-at-60fps figure is jecs's own README claim and UNVERIFIED by us. Chickynoid behavior beyond its README and file layout was not tested.
tags: [luau-engineering, reference]
---
# ECS and character controllers: jecs, Matter, Chickynoid

An ECS stores game data as components on entities and runs systems over queries. A custom character controller replaces Roblox's default movement so the server decides positions.

| Repo | Stars | License | Use |
|---|---|---|---|
| Ukendio/jecs | 486 | MIT | Fast ECS, entity relationships, typed API |
| evaera/matter | 142 | MIT | ECS with hooks and a debugger (original repo) |
| matter-ecs/matter | 121 | MIT | Current home of Matter (scope matter-ecs on Wally) |
| nidorx/ecs-lua | 236 | MIT | Older ECS engine |
| easy-games/chickynoid | 266 | MIT | Server-authoritative character controller |

## Do this
- Start with plain services and modules. Add an ECS only when you have hundreds or thousands of similar things (bullets, enemies, crops, particles) that all run the same logic each frame.
- If you do add one, pick jecs for raw speed and relationships, Matter for hooks, hot reload and a built-in debugger. Do not use both.
- Make components plain data (numbers, vectors, ids). Put behavior in systems, which are functions run each frame.
- Keep game rules in server systems. Client systems only render, using replicated components.
- Use Matter's `useEvent` hook to react to Roblox events inside a system, such as CharacterAdded.
- For melee or shooter games where hit timing matters, evaluate Chickynoid: the server simulates movement, the client predicts, and the server can rewind for fair hit checks (Antilag module).
- For obbies, tycoons, simulators and most kid-friendly games, keep the default Humanoid. A custom controller is a big cost.
- Chickynoid expects its own folders (ServerScriptService, StarterPlayerScripts, ReplicatedFirst packages, a GameArea for collision). Read its Index.md before adding it.

## Examples (our style, short)
jecs:
```luau
local jecs = require(Packages.jecs)
local world = jecs.world()
local Position = world:component() :: jecs.Id<vector>
local Velocity = world:component() :: jecs.Id<vector>

local bullet = world:entity()
world:set(bullet, Position, vector.create(0, 5, 0))
world:set(bullet, Velocity, vector.create(0, 0, 80))

local function moveSystem(dt: number)
	for id, pos, vel in world:query(Position, Velocity) do
		world:set(id, Position, pos + vel * dt)
	end
end
```
Matter system shape:
```luau
local function expireBullets(world)
	for id, life in world:query(Components.Lifetime) do
		if os.clock() > life.endsAt then world:despawn(id) end
	end
end
```

## Pitfalls
- An ECS makes small games harder to read. Teaching value is high, shipping value is low below a few hundred entities.
- Component ids and the world must be shared by client and server systems carefully; replication is your job (Matter's example has receiveReplication).
- Do not store Instances you do not clean up; despawn when the model is destroyed.
- Modifying a world during iteration needs care; read the library docs for safe despawn rules.
- Chickynoid replaces physics-based movement. Seats, vehicles, swimming and other Humanoid features need your own code.
- Prediction code multiplies bug surface. Budget extra test time on mobile with high ping. See [[14-QA-Optimization/device-test-matrix]].

## For our Roblox games
- Copy the principle: data in components, logic in systems, server authority over movement and hits.
- Use the libraries as packages, keep MIT notices. Never copy another game's weapon, hit or movement tuning numbers.
- Matter's example game and jecs's how_to files are good teaching material; do not copy their art or assets.
- Related: [[13-Luau-Engineering/framework-choice]], [[13-Luau-Engineering/object-pooling]], [[12-Multiplayer-NPCs/index]].
