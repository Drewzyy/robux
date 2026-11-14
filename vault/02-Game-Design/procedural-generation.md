---
description: Procedural generation for Roblox: seeded noise terrain, room and obby stitching, seed handling, server-side generation and the engine limits to watch (streaming, part counts, yielding).
type: playbook
status: draft
last_checked: 2026-11-14
keywords: procedural generation, noise, math.noise, perlin, fractal, terrain, seed, obby generator, room stitching, chunks, StreamingEnabled, FillBlock, WriteVoxels, Random.new, wave function
sources: DevForum, What is math.noise and how to use it, https://devforum.roblox.com/t/what-is-mathnoise-and-how-to-use-it/2336003 (seen in search 2026-11-14, not opened); DevForum, How does one set the seed of Roblox's 3d noise generator, https://devforum.roblox.com/t/how-does-one-set-the-seed-of-robloxs-3d-noise-generator/1406059 (search 2026-11-14); DevForum, Creating Procedural Mountains: A Fractal Noise Tutorial, https://devforum.roblox.com/t/creating-procedural-mountains-a-fractal-noise-tutorial/1494362 (search 2026-11-14); Roblox Creator Docs, Random datatype (search summary 2026-11-14)
verify: math.noise range is disputed in community sources (about -0.5 to 0.5 versus possibly outside -1 to 1): measure it yourself. No official numeric limits for streaming or part counts were found; every limit below is UNVERIFIED and must be profiled in Studio on a low-end phone.
tags: [game-design, playbook]
---
# Procedural generation

Generate levels, terrain or obbies from a seed so content is endless but reproducible. Related: [[02-Game-Design/rng-and-drop-tables]] (seeded Random), [[02-Game-Design/difficulty-and-flow]] (pacing).

## Do this
- Generate on the server from a single integer seed; send only the seed (or the chunk list) to clients. Store the seed with the round so you can debug or replay.
- Prefer hand-made modules stitched by rules over pure noise for gameplay (obbies, dungeons): noise terrain is scenery, stitching is level design.
- Validate every generated layout with a solver check: path exists, gaps within jump limit with margin, no overlap. Regenerate with seed+1 on failure (cap tries at about 10).
- Generate in chunks and yield: `task.wait()` every 100-500 parts or voxel writes so the server heartbeat is not blocked (UNVERIFIED budget; profile).
- Bound the world: fixed number of chunks live; destroy chunks behind the player; pool parts.
- Seed noise by offsetting coordinates, since `math.noise` has no seed argument (community source).
- Test with 10 fixed seeds in CI/Studio and keep a "known bad seeds" list.
- Keep difficulty on rails: stitch from a pool tagged easy/medium/hard and pick by position in the run, not pure random.

## Noise terrain
- `math.noise(x, y, z)` is Perlin-like and returns 0 at integer coordinates, so scale inputs: use `x * 0.05`, not `x`.
- Fractal (fBm): sum octaves, each with double frequency and half amplitude. Normalize by the total amplitude.
```lua
--!strict
local function fbm(x: number, z: number, seed: number, octaves: number): number
	local amp, freq, sum, norm = 1, 0.01, 0, 0
	for _ = 1, octaves do
		sum += amp * math.noise(x * freq + seed, z * freq + seed * 0.5, seed * 0.1)
		norm += amp
		amp *= 0.5
		freq *= 2
	end
	return sum / norm -- roughly -0.5..0.5 per community sources; measure it
end
```
- Height = `baseHeight + fbm * heightScale`. Fill with `workspace.Terrain:FillBlock(cframe, size, material)` per column or `WriteVoxels` for bulk (voxel resolution is 4 studs; confirm in docs).
- Biome: sample a second, lower-frequency noise for moisture and map ranges to materials (grass, sand, snow).
- Cave and island: threshold 3D noise (`n > 0.15` is solid) or multiply height by a radial falloff for islands.
- Large terrain writes are slow: batch per chunk of about 64 by 64 studs and yield between chunks (UNVERIFIED).

## Room and obby stitching
- Each module is a Model with an `Entry` and `Exit` attachment/Part and a bounding box. Place next module so its Entry CFrame equals the previous Exit CFrame.
```lua
--!strict
local function buildRun(seed: number, pool: {Model}, count: number, origin: CFrame): {Model}
	local rng = Random.new(seed)
	local placed: {Model} = {}
	local cursor = origin
	for i = 1, count do
		local src = pool[rng:NextInteger(1, #pool)]
		local m = src:Clone()
		local entry = m:FindFirstChild("Entry") :: BasePart
		local exit = m:FindFirstChild("Exit") :: BasePart
		m:PivotTo(cursor * entry.CFrame:Inverse() * m:GetPivot())
		m.Parent = workspace
		cursor = exit.CFrame
		table.insert(placed, m)
	end
	return placed
end
```
Verify pivot math in Studio with one pair of modules; `exit.CFrame` is read after the model is positioned.
- Avoid repeating the same module back to back; keep a short history and re-roll.
- Wave function collapse or grammar-based generation is only worth it past about 30 module types; start with a weighted pool by difficulty.

## Seeds
- Integer seeds only (Random truncates to an integer, per docs). Derive sub-seeds: `chunkSeed = seed * 73856093 + cx * 19349663 + cz * 83492791` (hash style), so a chunk generates the same regardless of order.
- Show the seed in the UI for daily or shared runs; same seed means same level for a speedrun leaderboard.

## Roblox-specific limits and watch-outs (all UNVERIFIED, profile)
- StreamingEnabled: generated content streams to clients by radius; set StreamingMinRadius and TargetRadius deliberately, and generate gameplay-critical parts on the server only.
- Part count and mobile memory: dozens of thousands of unions or meshes hurt low-end phones; prefer instancing the same mesh and few materials (see the roblox-performance skill).
- Server script time: a long loop without yielding triggers script timeout and hurts heartbeat.
- Anchor everything; unanchored generated parts cost physics.
- Collision fidelity and CanQuery off on decoration.

## Mistakes
- Pure random layouts with an impossible jump.
- Generating on every client with different seeds (desync).
- Forgetting to destroy old chunks (memory leak).

## For our Roblox games
Copy: seed + validation + stitching approach, our own modules and noise. Never copy another game's maps, module art, or generator names; build our own pool of modules.
