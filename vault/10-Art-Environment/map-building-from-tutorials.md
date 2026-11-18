---
description: How Roblox map builders actually make maps, start to finish: blockout, spawn and POI layout, paths, borders, art styles, low-poly kits, Terrain Editor tools and realistic mesh terrain.
type: playbook
status: draft
last_checked: 2026-11-18
keywords: roblox map, map building, simulator map, blockout, spawn, points of interest, POI, paths, borders, low poly, realistic, art style, terrain editor, generate, sculpt, sea level, heightmap, foliage, PBR, SurfaceAppearance, palette
sources: Captions of 7 YouTube tutorials (watched 2026-11-18): DeHapy "How to Build PRO Roblox Maps!" (I-AB6pJ-y5g); RoBuilder "How To Make Good Simulator Maps" (Y3eUDs3FFGQ); DatBoiEle "I just figured out how ALL ROBLOX games make their MAPS" (wmB8BW4y5EA); VainOath "Building The 3 Best Roblox Art Styles" (3KBgLxPV-8E) and "How To Make Any Low-Poly Map With Ease" (21gSbDsLk5s); Scroof "How to make Low Poly Terrain" (gI1Oe7lSkSE); HollyHollers "How to Make Realistic Terrain" (Ye1zLeYa7qE). Terrain Editor tools: create.roblox.com/docs/studio/terrain-editor and /docs/parts/terrain (read 2026-11-18).
verify: Medium. Process and layout rules are the creators' own practice. Terrain Editor tool names are from the official docs.
tags: [art-environment, maps, playbook]
---
# Map building, the way Roblox builders do it

## Do this
- Gather references and pick one colour palette before building (the ground colour sets it).
- Set lighting early: colours look different under the final lighting ([[10-Art-Environment/lighting-recipes-from-tutorials]]).
- Block out first with placeholder parts (a cylinder for spawn, pink blocks for points of interest), then replace them.
- Big things first (borders, mountains, big trees, rocks), small details last.
- Never a flat square box with straight borders: add height, curve or break the edges, and vary repeated pieces in size, colour and rotation.
- Nothing floats: sink rocks, trees and grass into the ground.
- Detail where the camera looks; finish with a playtest, then look again the next day with fresh eyes.

## Start to finish
1. References and palette.
2. Ground: one big part. Simulator maps about 250x250 to 300x300 studs; 200x200 "scales to the player much better" than 100x100.
3. Blockout: decide spawn (corner or centre) and the points of interest (POIs).
4. Replace placeholders with styled platforms; give the spawn a shape, not a plain circle.
5. Paths: one big loop around the map plus inner paths to each POI, and a grander main path (for example to portals). Keep path pieces at the same angle so the style stays clean (the Archimedes plugin makes curves).
6. Borders once the space is clear: a row of about five pieces, copied around, then a second layer for fullness.
7. Props big to small: large trees at the borders, big rocks, grass, smaller inner trees.
8. Height: hills "every once in a while"; fill big green gaps with terrain chunks, rocks at their base as a transition.
9. A colour accent (a waterfall) to break up too much green and brown.
10. Gameplay POIs last, then polish: lighting, folders, playtest.

Pros make a simulator map in about an hour; a classic map takes about 2 hours, a realistic scene about 3.

## Layout rules (simulators and beyond)
- Spawn in a corner pulls players across the map to discover things; spawn in the centre with paths out each way is the common simulator layout.
- Near spawn: leaderboards (very close), wheel spin, shop, starter eggs, rare pet showcase.
- Eggs a little away from spawn so players have a reason to move; the shop on the opposite side (two sides to buy, two sides to play).
- Put extras (King of the Hill, dungeons, secret egg, VIP portal) in other corners so players explore. Never lock players in one spot.
- Use buildings to frame what players see from spawn (eggs one way, shops the other).
- Paths that make players wonder "what is down there?" make a small map feel big.
- Borders: never straight. Push pieces back, vary them, cut or curve corners, or use water or a dense tree line.

## Three art styles
- **Classic**: 1-stud snapping, 90-degree rotations, no scaled models, studs on surfaces (Resurface plugin), transparent water. Beginner friendly.
- **Low poly**: snapping down to 0.25 studs, any rotation and scale, a small custom kit made in Blender, bright saturated cartoon lighting. Intermediate. It is common now: add your own twist.
- **Realistic**: mesh assets with PBR (SurfaceAppearance ColorMap, NormalMap, RoughnessMap, MetalnessMap), more trees to hide seams, Realistic lighting slightly gloomy and a little desaturated. Advanced; heavier on phones.

## Low-poly kit method
1. Make a kit of 9 to 11 pieces: rock, mountain rock, oak, pine, mushroom, tall grass, fence, sign, lamp post, bridge, waterfall.
2. Flat base for the zone; mountain pieces around the edge, bigger at the back for depth.
3. Scatter trees, grass, rocks and props within the palette, alternating dark and light, tall and short.
4. New zone = copy the zone, recolour it and swap one asset (oak to pine). Themes: light forest, dark forest, desert (hardest: lighting shifts sand colours), snow, volcano (black, red, orange, yellow).

## Terrain
Terrain Editor (View tab), from the official docs:
- **Create tab**: Import (a heightmap and optional colormap onto a region), Generate (biomes such as Mountains, Hills, Plains, Dunes, Canyons, Water, Arctic, Marsh, Lavascape), Clear.
- **Edit tab**: Select, Transform, Fill, Sea Level (one water level or remove water in a region), Draw (Ctrl to subtract), Sculpt, Smooth (hold Shift while drawing), Paint (material), Flatten (Erode to Flat, Grow to Flat, Flatten All).
- Brush size 1 to 64; hold B and drag to change base size, Ctrl B for strength; Alt-click picks a material.
- From the tutorials: Grow beats Add for natural shapes; turn Plane Lock off for Paint; raise base size if materials poke through; Terrain > Material Colors recolours a material.
- Low-poly terrain: place nodes and connect them into triangles with a node plugin, using normal terrain as a guide, then delete the guide.
- Realistic terrain: many creators use meshes ("PBR Nature" packs) instead of terrain tools; it looks better and costs more on phones. Mix several plant types in clusters by hand, add leaf litter and pebbles as gradients, vary textures (nothing repeats in nature).

## Mistakes
Square map with straight borders; flat ground; big empty green space; floating props; the same asset repeated at the same size and colour; everything crammed at spawn; only raising sun rays and calling it realistic; skipping references.

## For our Roblox games
Scenic and Builder follow this order and these layout rules. For phones, prefer low poly or classic, keep unions few, and check [[14-QA-Optimization/performance-budgets]].
