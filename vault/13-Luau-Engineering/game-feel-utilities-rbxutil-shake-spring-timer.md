---
description: RbxUtil's Shake, Spring, Timer, TaskQueue, WaitFor, Component and TableUtil modules (MIT) for game feel and plumbing, with settings to start from, pitfalls, and a safe-motion rule for kids.
type: reference
status: draft
last_checked: 2026-11-15
keywords: rbxutil, shake, camera shake, spring, smoothdamp, timer, taskqueue, waitfor, component, collectionservice, tableutil, input, game feel, sleitnick
sources: https://github.com/Sleitnick/RbxUtil (modules/shake, modules/spring, modules/timer, modules/task-queue, modules/wait-for, modules/component, modules/table-util, read 2026-11-15); https://github.com/Sleitnick/RbxCameraShaker (search listing only); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15 (RbxUtil 477 MIT, RbxCameraShaker 128 MIT). Shake property names read from source. Good values for shake strength are our starting guesses, UNVERIFIED by playtests.
tags: [luau-engineering, reference]
---
# Game-feel and plumbing utilities from RbxUtil

Sleitnick/RbxUtil (477 stars, MIT) is a collection of small modules published separately on Wally. Most games need five or six of them.

## Do this
- Use Shake for hits, explosions and landings. Create one per effect, set Amplitude, Frequency, FadeInTime, FadeOutTime and PositionInfluence or RotationInfluence, call Start, then bind with BindToRenderStep. It stops and cleans up by itself.
- Start values to tune from (our guess): Amplitude 1 to 3, Frequency 0.1, FadeInTime 0, FadeOutTime 0.3, RotationInfluence Vector3.new(0.05, 0.05, 0.05). Test on a small phone screen.
- Offer a "Reduce camera motion" setting that scales Amplitude to 0 or 0.25. See [[08-UI-UX/accessibility]].
- Use Spring (a critically damped spring over TweenService:SmoothDamp) to follow a target smoothly: camera offsets, UI bars, floating labels. Call Update(dt) every frame, even when the target is unchanged.
- Use Timer for repeating server work (spawn waves, passive income) instead of `while true do task.wait()`. It exposes a Tick signal and can be destroyed.
- Use TaskQueue to batch sends that happen many times per frame; it flushes once at the end of the step with task.defer.
- Use WaitFor.Child(parent, name, timeout) promises when waiting for instances on the client. Default timeout is 60 seconds; pass a shorter one.
- Use Component to bind behavior to CollectionService tags (doors, pickups, spawners). Each tagged instance gets one component object that is cleaned when the tag or instance goes away.
- Use TableUtil for copy, map, filter, reduce and shuffle instead of writing them again.

## Example (our style)
```luau
--!strict
local Shake = require(Packages.Shake)
local Spring = require(Packages.Spring)

local function hitShake(strength: number)
	local shake = Shake.new()
	shake.Amplitude = strength * settings.motionScale
	shake.Frequency = 0.1
	shake.FadeInTime = 0
	shake.FadeOutTime = 0.3
	shake.RotationInfluence = Vector3.new(0.05, 0.05, 0.05)
	shake:Start()
	shake:BindToRenderStep(Shake.NextRenderName(), Enum.RenderPriority.Last.Value, function(pos, rot)
		camera.CFrame *= CFrame.new(pos) * CFrame.Angles(rot.X, rot.Y, rot.Z)
	end)
end

local barSpring = Spring.new(0, 0.15)
RunService.RenderStepped:Connect(function(dt)
	barSpring.Target = health / maxHealth
	bar.Size = UDim2.fromScale(barSpring:Update(dt), 1)
end)
```

## Pitfalls
- The Shake docs warn about camera drift when you do not control the camera's base CFrame; read its "Fixing drift" section when using Roblox's default camera scripts.
- Spring supports number, Vector2, Vector3 and CFrame only. A Color3 needs conversion.
- Timer ticks do not catch up if the server lags; do not use them as the source of truth for time-based rewards (use os.time against saved timestamps).
- Component classes need `Tag` set, and extending lifecycle methods wrongly can leak. Destroy components on tag removal.
- Each RbxUtil module is its own Wally package with its own version; do not assume a latest-version API from old tutorials.
- Strong shake, flashes or fast zoom can bother some players. Keep effects short and optional.

## For our Roblox games
- Copy the principle: tiny single-purpose modules, effects driven by settings, a motion-reduction switch.
- Use the packages with MIT notices kept; do not copy another game's effect tuning tables or sounds.
- Related: [[09-Animation-VFX/index]], [[13-Luau-Engineering/tween-and-animation-cleanup]], [[13-Luau-Engineering/state-and-signals-promise-trove-signal]].
