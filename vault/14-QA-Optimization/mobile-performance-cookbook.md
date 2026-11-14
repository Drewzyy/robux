---
description: Concrete steps to make a Roblox game run on low-end phones: what to cut first, UI and effect limits, streaming, memory headroom, and how to verify with Device Simulator and the performance dashboard.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: mobile performance, low-end phone, Device Simulator, StreamingEnabled, out of memory, crash rate, particles, shadows, part count, UI, frame rate, draw calls, texture size
sources: creator-docs content/en-us/production/game-design/design-for-roblox.md (most users play on mobile, read 2026-11-14); creator-docs content/en-us/studio/device-simulator.md (read 2026-11-14); creator-docs content/en-us/production/analytics/performance.md (client memory and crash charts, read 2026-11-14); creator-docs content/en-us/studio/optimization/memory-usage.md (read 2026-11-14)
verify: The mobile-first fact and the dashboard charts are in local docs. All numeric limits (parts, particles, textures) are community rules of thumb, UNVERIFIED as limits: confirm on a real low-end phone. Device Simulator is a beta in the docs read.
tags: [qa-optimization, playbook]
---
# Mobile performance cookbook

The docs state that most Roblox users play on mobile and are sensitive to friction and load time. Treat a weak phone as the main target, not an edge case.

## Do this
- Pick a reference device: an old, low-RAM Android phone. If you can only test one, test this one. Make 30 fps stable there before you polish anything for desktop.
- Set a memory goal: client memory under about 60% of the device's available memory (the performance dashboard shows the memory percentage used per platform; an Android line near 100% means crashes). Watch the out-of-memory chart after each update.
- Turn on `StreamingEnabled` for any map bigger than a small arena (see [[12-Multiplayer-NPCs/server-size-and-streaming]]).
- Cut in this order: (1) particles and beams, (2) dynamic lights and shadows on small objects, (3) transparent and neon overlap, (4) high-res textures, (5) MeshPart triangle count, (6) part count in view, (7) per-frame scripts.
- Starting caps for phones (UNVERIFIED): 100 active particles per effect, 5 to 10 emitters on screen at once, textures 512 px or less for props, character-sized meshes under 5k triangles, active lights 4 or fewer.
- Keep GUIs flat: fewer nested Frames, fewer `UIGradient` and `UIStroke` stacked on many elements, no per-frame text updates. Use `UIListLayout` sparingly in long lists; page or virtualize lists over 50 rows.
- Touch targets at least 44 px (about 9 mm) and keep important controls off the thumb-blocked corners.
- Run client scripts at lower rates: AI display, minimaps and counters at 10 to 20 Hz, not every frame.
- Keep per-frame client work under 4 ms on the reference phone (see [[14-QA-Optimization/performance-budgets]] for the full frame budget).
- Provide a graphics quality option and auto-lower effects when frame time stays high for 5 s.

## Detect and adapt

```lua
--!strict
local RunService = game:GetService("RunService")

local SAMPLE_SECONDS = 5
local LOW_FPS = 35

local accumulated = 0
local frames = 0

local function applyLowQuality()
	-- disable particles, reduce view distance, hide decorative parts
	for _, emitter in workspace:GetDescendants() do
		if emitter:IsA("ParticleEmitter") then
			emitter.Enabled = false
		end
	end
end

local connection: RBXScriptConnection
connection = RunService.Heartbeat:Connect(function(dt: number)
	accumulated += dt
	frames += 1
	if accumulated >= SAMPLE_SECONDS then
		local fps = frames / accumulated
		if fps < LOW_FPS then
			applyLowQuality()
			connection:Disconnect()
		end
		accumulated = 0
		frames = 0
	end
end)
```

Run on the client only. Prefer tagging decorative effects with `CollectionService` instead of walking all of Workspace.

## Verify
1. Studio Device Simulator (beta per the docs): pick a phone profile to check screen size, pixel density and on-screen keyboard. It does not simulate GPU or CPU speed, so it does not replace a real device.
2. Network Simulator: use the Standard Mobile preset to test latency (see [[14-QA-Optimization/device-test-matrix]]).
3. After release, watch the Performance dashboard: client crash rate, out-of-memory exits and memory usage by platform; break down by place version to catch regressions. Needs at least 100 daily active users for the charts.
4. MicroProfiler on a real phone for the heaviest scene (see [[14-QA-Optimization/microprofiler-and-stats-guide]]).

## Common mobile killers
- Dozens of `Beam`s or `Trail`s attached to every player.
- Large `Neon` or transparent parts overlapping (overdraw).
- `SurfaceGui` on many parts with live updates.
- Huge terrain water or `Decal` stacks.
- Streaming off with a big map (memory spike, loading crash).
- Loading all sounds and animations at start.
- Many humanoid NPCs with full accessories.

## Mistakes
- Testing only in Studio on a desktop.
- Cutting visuals randomly instead of measuring.
- One heavy lobby scene that crashes before the game starts.
- Raising quality settings when the first slow frame appears (flapping); require 5 s of data.

## Related
[[14-QA-Optimization/optimization-checklist]], [[14-QA-Optimization/lag-sources-ranked]], [[14-QA-Optimization/performance-budgets]], [[12-Multiplayer-NPCs/pets-and-followers]]

## For our Roblox games
Copy the principle: a weak reference device, measured cuts, adaptive quality, streaming on. No names, art or assets are involved. Zero spend: built-in Studio tools and a free dashboard.
