---
description: How to lay out Roblox maps and hubs: landmarks, sightlines, path width, hub spokes, density, and wayfinding so players always know where to go, with studs-based numbers to test.
type: reference
status: draft
last_checked: 2026-11-14
keywords: level design, world design, landmarks, sightlines, hub, wayfinding, spawn, map layout, path width, density, streaming, navigation
sources: Author working knowledge of game design practice (no pages opened 2026-11-14); Roblox Creator Hub docs (create.roblox.com/docs, titles only, not opened 2026-11-14)
verify: Numbers are playtest starting points from common practice. UNVERIFIED as measured Roblox standards; confirm with timed walks on your own map.
tags: [game-design, reference]
---
# World and level design

A map is a set of decisions about where players look, walk, and stop. Good layout teaches itself with no text.

## Do this
- Spawn in a hub with 3-5 spokes, each leading to one activity and each visible from the spawn (a tall landmark per spoke).
- Place one unique landmark per zone, visible from 100+ studs: a tower, a colored tree, a giant prop. Players navigate by it.
- Keep first interactive object within 30 studs of spawn and the first goal within 15 s of walking.
- Path width: main routes 16-24 studs, side paths 8-12 studs, so groups can move without blocking.
- Use sightlines to promise: show the next goal across a gap before the player can reach it.
- Light the path: brighter, warmer, or more saturated along the intended route. See [[10-Art-Environment/lighting-and-mood]].
- Loops over dead ends: every area has two ways out, so players are never forced to backtrack.
- Cap walk time between key points at 20-40 s, and add a teleport pad or sprint for longer routes.

## Why it works
- Landmarks create mental maps; confused players leave.
- Visible goals pull the player forward (curiosity). See [[03-Game-Psychology/curiosity-and-mystery]].
- Compact maps feel alive even with 10 players.

## Hub layout patterns
- Radial hub: center plaza with spokes. Best for multi-activity games.
- Linear strip: a main street with side rooms. Best for story games.
- Ring road: a loop around a central feature, good for racing and social.
- Vertical hub: levels connected by lifts. Use only with a clear map or beacon.

## How to build it in Roblox
- Block out first in gray parts; test by walking before any art. See [[10-Art-Environment/art-direction-for-roblox]].
- Use PathfindingService test NPCs to verify paths are connected and wide enough.
- Mark intended routes with color, decals, or lights; hide shortcuts for explorers as rewards.
- Enable StreamingEnabled for maps above a few thousand parts; keep landmarks in persistent models.
- Add a minimap or compass for maps wider than 500 studs.

## Numbers (playtest starting points)
- Hub radius 60-100 studs; spoke length 80-150 studs.
- Part count per zone 2000-5000 for mobile-friendly play; UNVERIFIED limit, test on a low-end phone.
- Doorways 8 studs wide, ceilings 14-18 studs for a roomy feel.
- Jump heights: keep required jumps within 7 studs vertical for default characters.
- One interactive thing per 20-40 studs on the main route.

## Mistakes
- Huge empty terrain between points.
- All zones that look alike; players cannot tell where they are.
- Dead-end rooms that block groups.
- Landmarks hidden by fog or trees.
- Spawn facing a wall.

## Playtest check
- Time a first-time player from spawn to first goal; over 30 s means a missing landmark.
- Ask testers to point at the spawn without looking at a map; if they cannot, landmarks are weak.
- Walk the map on a low-end phone and note stutters.

## For our Roblox games
Copy the principles: landmarks, spokes, loops, sightlines, and block-out-first. Never copy another game's map shape, named locations, or building models. Draw your own hub and name your own zones.
