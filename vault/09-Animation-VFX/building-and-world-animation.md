---
description: Animations for building, placing, spawning, collecting, and the living world: wind, water, day-night, and doors.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: building animation, placement, spawn animation, pop in, environment animation, wind, water, day night, lighting animation, tween
sources: Roblox TweenService and Lighting (model knowledge); community lighting guides (devforum)
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [animation-vfx, playbook]
---
# Building and world animation

## Do this
- Place or build: object scales 0 to 1.1 to 1 over 0.25 s (Back Out) + dust puff + thunk sound. Removing: scale to 0 over 0.15 s.
- Ghost preview while placing: 0.5 transparency, green if valid, red plus X icon if not.
- Spawn or unlock: short rise from the ground (0.3 s) and a sparkle.
- Collect: item flies to the HUD counter (0.4 to 0.6 s arc), counter pops when it lands.
- Run world animation on the client so it costs the server nothing.

## Living world, cheap
| Effect | How | Cost |
|---|---|---|
| Swaying trees and grass | Workspace.GlobalWind for grass and particles; slow looping CFrame tween (2 to 4 s) on a few leaf meshes | low |
| Water | scrolling Texture offset or a Beam; gentle bob tween on floating props | low |
| Day-night | tween Lighting.ClockTime over minutes; tween Atmosphere and ColorCorrection with it | low |
| Lights flicker | random small changes to Brightness every 0.05 to 0.2 s | low; only a few lights |
| Doors and props | TweenService on CFrame; sound at start and end | low |
- Stop animating things nobody can see (distance check every second).
- Lighting presets: [[10-Art-Environment/lighting-presets]].
