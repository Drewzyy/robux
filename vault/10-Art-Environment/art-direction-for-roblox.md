---
description: Art direction rules for Roblox games: silhouettes, shape language, colour value hierarchy, stylized versus realistic, and consistency kits, with examples from hit Roblox games.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: art direction, silhouette, shape language, value hierarchy, stylized, low poly, consistency, style guide, readability, mobile, color value, roblox hits
sources: General game art practice (silhouette test, value grouping, shape psychology), model knowledge; Roblox hit games Adopt Me, Blox Fruits, Brookhaven, Grow a Garden, Doors as examples from general knowledge (not re-viewed 2026-11-14)
verify: UNVERIFIED: written from established art practice and model knowledge; no sources opened. Examples of hit games are recalled, not re-checked. Test every rule on a phone screenshot.
tags: [art-environment, playbook]
---
# Art direction for Roblox

Goal: every object reads at a glance on a 6-inch phone, and the whole game looks like one world. Realism rules are in [[10-Art-Environment/realism-when-needed]]; lighting in [[10-Art-Environment/lighting-and-mood]]; colour sets in [[08-UI-UX/color-palettes]].

## Do this
- Silhouette test: fill the asset solid black at 128 px. If you cannot name it, change the outline (bigger head, longer tail, distinct hat), not the texture.
- Give each role one shape family: friendly or collectible = circles and soft curves; hazard or enemy = triangles, spikes, sharp angles; solid or safe structure = squares and rectangles.
- Three value levels per scene: dark background, mid-value world, light or saturated focus. The thing the player must act on is the brightest or most saturated in frame.
- Limit a scene to 1 dominant hue, 1 support hue, 1 accent (60/30/10). Accent is for goals, rewards and hazards only.
- Pick ONE style and write it down as a kit: palette (6-10 hex codes), outline rule, corner radius, material list, texture scale, prop density, lighting preset. Reuse it for every asset.
- Prefer stylized (flat colour, simple materials, chunky proportions) for most games; use realism only when the genre needs it (horror, simulators of real places) and keep mobile cost in mind.
- Characters and key objects 15-25% larger than "realistic" so they read on a phone; heads and hands exaggerated.
- Keep detail where the player looks (goal, tools, pets), and flat calm areas elsewhere.

## Silhouette and shape language
| Role | Shape | Colour | Size cue |
|---|---|---|---|
| Player or pet | Round, soft, big head | Warm, saturated | Largest on screen |
| Collectible | Circle, star, gem | Gold, cyan, bright | Small but glowing |
| Hazard | Triangle, spikes | Red or high contrast | Same height as player or taller |
| Platform or path | Rectangle, flat top | Mid value, low saturation | Wide, clear edge |
| Goal | Tall, symmetrical | Brightest value | Taller than surroundings |
| NPC or shop | Distinct hat or prop | Accent colour | Standing alone, clear space around |

## Colour value hierarchy
1. Convert a screenshot to greyscale. Goal and hazard must still stand out; if not, adjust value before hue.
2. Background: darkest or lowest saturation. World: middle. Focus: lightest or most saturated.
3. Never put two similar values next to each other when one is the path and one is the pit.
4. Use colour for meaning consistently: red danger, green safe or go, gold reward, blue neutral or water. Add shape or icon for colour-blind players.
5. Zone identity by hue shift: each zone owns one hue, with the same value structure.

## Stylized versus realistic
- Stylized: cheaper (fewer textures, fewer triangles), reads at small size, ages well, easier to keep consistent. Good for simulators, obbies, tycoons, pet games.
- Realistic: needs lighting, materials and detail density to feel right, costs more on phones, hard to keep consistent with generated assets.
- Mixed styles in one scene is the most common failure. Choose per game, not per asset.

## Examples from hit games (recalled, UNVERIFIED)
- Adopt Me style: rounded, pastel, big-eyed pets; shape and colour make each pet distinct; props share one soft material look.
- Blox Fruits style: anime character silhouettes and bright effect colours over a more muted world; abilities read through effect colour.
- Brookhaven: simple, familiar low-detail houses; clear colours and large door and roof shapes make places easy to find.
- Grow a Garden style: bright flat greens and chunky crop shapes; the ripe crop is the highest saturation object.
- Doors style: dark, narrow rooms with a single warm light source; the door and the threat are the only high-contrast shapes.

## Consistency kit (copy into each project)
- Palette file with hex codes and roles.
- Outline rule: none, or one dark colour at fixed thickness.
- Bevel or corner radius: one value (for example 0.2 studs on parts).
- Material list: 3-5 Roblox materials only, with fixed colours.
- Texture scale and density: one value.
- Lighting preset: one per zone, same exposure.
- Reference sheet: 4-6 hero assets every new asset is compared against before merge.

## Mistakes
- Detail everywhere, so nothing stands out.
- Hazard and platform the same value.
- Mixing realistic textures with toy-like shapes.
- Dark scenes that hide the goal on a phone in sunlight.
- Palette drift across zones or updates.

## For our Roblox games
Copy: the principles (silhouette, shape roles, value hierarchy, one style kit). Never copy: any game's characters, pets, props, maps, icons or palettes beyond generic ideas; build original shapes and write our own kit. Related: [[10-Art-Environment/roblox-icon-and-thumbnail-patterns]], [[08-UI-UX/game-ui-patterns-from-hits]].
