---
description: Clean tween and animation lifecycle in Luau: cancel old tweens, use Completed correctly, stop AnimationTracks, disconnect on respawn, and avoid tween fights and leaks.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: TweenService, tween cancel, tween completed, AnimationTrack, Stop, Destroy, Animator, respawn, connection leak, tween conflict, PlaybackState, Enum.TweenStatus
sources: creator-docs content/en-us/reference/engine/enums/TweenStatus.yaml (Canceled, Completed, read 2026-11-14); creator-docs content/en-us/reference/engine/enums/DeveloperMemoryTag.yaml (Signals, Animation, read 2026-11-14); creator-docs content/en-us/studio/optimization/memory-usage.md (read 2026-11-14)
verify: TweenStatus values verified in local docs. Tween:Cancel, AnimationTrack:Stop and Animator methods are from model knowledge: UNVERIFIED locally, class pages absent from the clone.
tags: [luau-engineering, checklist]
---
# Tween and animation cleanup

Tweens and animation tracks are cheap until you stack them. Two tweens on the same property fight. Tracks left playing keep running. Connections to them leak.

## Do this
- Keep one active tween per `(instance, property-set)`. Store it in a table; call `:Cancel()` on the old one before creating the new one.
- Use the `Completed` signal with `:Once()` so the connection frees itself. It passes an `Enum.TweenStatus` (`Completed` or `Canceled`): check it before acting.
- Do not rely on `Completed` when the instance can be destroyed first. Guard with `if not part.Parent then return end`.
- Stop looped `AnimationTrack`s explicitly (`track:Stop(fadeTime)`) when the state ends. Looped tracks never end on their own.
- Load animations once per character via `Animator:LoadAnimation`, cache the track, and reuse; do not load per use.
- On respawn the old character dies and its tracks go with it. Re-load tracks on `CharacterAdded`, and disconnect handlers tied to the old character.
- For UI, cancel tweens on `Destroying` of the frame or when the screen closes.
- Prefer `TweenService` or springs for UI and parts; avoid per-frame `Heartbeat` loops that hand-move many objects.
- Never tween properties the server also sets every frame (physics-driven parts). Tween anchored or client-only visuals.

## Safe tween helper

```lua
--!strict
local TweenService = game:GetService("TweenService")

local active: { [Instance]: Tween } = {}

local function play(instance: Instance, info: TweenInfo, goal: { [string]: any }, onDone: (() -> ())?)
	local old = active[instance]
	if old then
		old:Cancel()
	end
	local tween = TweenService:Create(instance, info, goal)
	active[instance] = tween
	tween.Completed:Once(function(status: Enum.PlaybackState)
		if active[instance] == tween then
			active[instance] = nil
		end
		if status == Enum.PlaybackState.Completed and onDone then
			onDone()
		end
	end)
	instance.Destroying:Once(function()
		tween:Cancel()
		active[instance] = nil
	end)
	tween:Play()
end
```

Note: `Tween.Completed` passes an `Enum.PlaybackState` (`Completed` or `Cancelled`). The docs enum `TweenStatus` (`Canceled`, `Completed`) is the older documented enum, so check both names against the live reference before relying on spelling. UNVERIFIED which one the current engine passes.

## Animation track lifecycle

```lua
--!strict
local function loadTrack(humanoid: Humanoid, animationId: string): AnimationTrack?
	local animator = humanoid:FindFirstChildOfClass("Animator")
	if not animator then
		return nil
	end
	local animation = Instance.new("Animation")
	animation.AnimationId = animationId
	local track = animator:LoadAnimation(animation)
	animation:Destroy()
	return track
end
```

Call this once per character, keep the track, `Play` and `Stop` as needed. Do not destroy the Animation instance until the track is loaded.

## Cleanup checklist
- [ ] Old tween canceled before new one on same target
- [ ] Every `Completed` use is `Once` or disconnected
- [ ] Looped tracks stopped on state exit
- [ ] Tracks cached per character, reloaded on respawn
- [ ] Handlers on dead characters disconnected (use a Maid or connection list)
- [ ] UI tweens canceled on screen close
- [ ] Animation events (`GetMarkerReachedSignal`) disconnected with the track

## Mistakes
- Creating a new tween per frame in a loop without canceling (hundreds alive).
- Using `task.wait(tweenTime)` instead of `Completed` (drifts, runs after destroy).
- Re-loading the same animation on every attack.
- `Heartbeat` connection that moves a part forever after the part is gone.

## Related
[[13-Luau-Engineering/memory-leak-checklist]], [[13-Luau-Engineering/state-machines]], [[12-Multiplayer-NPCs/pets-and-followers]], [[13-Luau-Engineering/common-bugs-checklist]]

## For our Roblox games
Copy the principle: one tween per target, `Once` connections, explicit stops, per-character caching. No names, art or assets are involved. Zero spend: plain Luau, free animation tools.
