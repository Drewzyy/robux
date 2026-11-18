---
description: Roblox Studio for a builder: every panel and ribbon tab, building tools, snapping, anchoring, grouping, unions, and every keyboard shortcut, from 6 popular beginner tutorials.
type: reference
status: draft
last_checked: 2026-11-18
keywords: roblox studio, studio ui, explorer, properties, toolbox, ribbon, home tab, model tab, view tab, test tab, shortcuts, ctrl d, ctrl g, snapping, increments, anchor, union, negate, world space, z fighting, play, run, asset manager, material generator
sources: Captions of 6 YouTube tutorials (watched 2026-11-18): SmartyRBX "The ULTIMATE Beginner Guide to Roblox Studio" (p005iduooyw, 5.1M views); RoBuilder "ABSOLUTE Beginner Basics" (-5OOovmkJss); Mikey Vibe Coding "Roblox Studio Full Tutorial for Beginners 2026" (YO4lGQJVUGM); Psuedon "The Absolute Basics Of Roblox Studio" (7uN-RToHvYs); Simple Games "Explorer and Properties" (l5Q5871Kh6o); CodeBro29 "How to make your FIRST Roblox Game" (-QSmtYVMRH4, 2.9M views)
verify: Medium. Taken from what the creators say and show; menu names can move between Studio versions. One video says R is the scale tool, others disagree: UNVERIFIED.
tags: [roblox-fundamentals, studio, reference]
---
# Roblox Studio: the UI and the shortcuts

What every beginner tutorial teaches first. Agents working through the Studio connection use the same names.

## Do this
- Start from New > Baseplate (not Classic Baseplate).
- Keep Explorer, Properties, Toolbox and Output open (View tab). Output shows errors in red with script name and line.
- Set snapping before building: Model tab, move 0.5 or 1 stud, rotate 15 degrees (the default 45 is stiff). Go smaller for detail.
- Anchor everything that should not move ("nine times out of ten"), or the map falls and lags.
- Group (Ctrl G) and name things with no spaces (MagicBlock). Use folders (Ctrl Alt G) for organisation only.
- Never trust Toolbox models blindly: check Explorer for scripts in anything you insert (some spread fire, teleport players away, or lag the game). Prefer our own assets ([[01-Roblox-Fundamentals/ip-and-toolbox-rules]]).

## The windows
- **Viewport**: the 3D world in the middle. Windows around it resize and dock; Studio remembers the layout.
- **Explorer**: the tree of everything, grouped by services. Select here or in the viewport. The plus beside an item inserts a child (Part, Folder, Script, IntValue, ScreenGui, Highlight...). Rename with F2. Drag onto another item to make it a child.
  - Workspace: everything visible and physical. Only things in Workspace show in the game.
  - Lighting: lighting settings and effects.
  - ServerScriptService: server Scripts. StarterGui: on-screen UI (ScreenGui). StarterPack: tools players spawn with. StarterPlayer: player settings.
  - ServerStorage: things players cannot see until moved into Workspace. ReplicatedStorage and ReplicatedFirst: shared with scripts.
  - MaterialService: custom material variants.
- **Properties**: every setting of the selected object; hover a name for help; changes apply at once. Works for services too (select Lighting).
- **Toolbox**: free models, meshes, audio, images, plugins. Tabs for your Inventory and the Creator Store.
- **Output**: errors (red), warnings (yellow), prints.
- **Asset Manager**: images, audio, meshes and extra places of the game; right click to upload. The game must be published first.
- **Terrain Editor**: View tab. See [[10-Art-Environment/map-building-from-tutorials]].
- **Material Generator** (beta, File > Beta Features): type a prompt, get textures, "save and apply variant" (StudsPerTile, Organic).

## Ribbon tabs
- Home: Part (Block, Sphere, Wedge, Corner Wedge, Cylinder), Select, Move, Scale, Rotate, Color, Material, Anchor, Lock.
- Model: Snap to Grid and increments, Union, Negate, Group, pivot editing, effects.
- Avatar: rigs and avatar tools. Test: Play, Run, Team Test, Server and Clients. View: show and hide every window, Team Create. Plugins: the plugin marketplace and installed plugins.
- File: Save to File (.rbxl), Publish to Roblox, Publish to Roblox As, Advanced > Customize Shortcuts, Beta Features.

## Building basics
- Move tool for exact moves (arrows on X, Y, Z), not free dragging. Scale has a handle per face; Rotate has rings.
- Right click a part > Use World Space so arrows follow the world, not the rotated part.
- Lock a part (like the floor) so it cannot be moved by accident.
- Negate (Model tab) a part, then Union it with another to cut holes for doors and windows. Unions are meshes: do not overuse them. If players get stuck in a union's hole, set CollisionFidelity to PreciseConvexDecomposition.
- Overlapping surfaces flicker (Z fighting): offset them slightly.
- Use even sizes (36, half is 18) so parts line up.
- Properties to know: Color (custom) over BrickColor, Material (Plastic, SmoothPlastic, Neon glows, WoodPlanks, Cobblestone, Granite), Transparency 0 to 1, Reflectance, Anchored, CanCollide (off = walk through), CanTouch, CastShadow, CollisionFidelity, PrimaryPart (set it if a model drags oddly).
- Test with Play (your character) and Run (physics only, no character). Team Test and Server and Clients for multiplayer.

## Shortcuts
| Keys | Action |
|---|---|
| W A S D or arrows, Q, E | Move the camera; Q down, E up |
| Right mouse drag, scroll | Look around, zoom |
| F | Focus the camera on the selection (finds lost parts) |
| F2 | Rename |
| Delete or Backspace | Delete |
| Ctrl D | Duplicate |
| Ctrl G / Ctrl Alt G | Group as a Model / as a Folder |
| Ctrl Shift G / Ctrl Shift N | Union / Negate |
| Ctrl R / Ctrl T | Rotate 90 degrees / tilt 90 degrees |
| Ctrl Z / Ctrl Shift Z | Undo / redo (Cmd on Mac) |
| Ctrl S | Save |
| Shift click, drag a box | Select several |
| Hold Shift while dragging | Turn snapping off for that move |
| Ctrl / Shift / Ctrl Shift while scaling | Scale both sides / all sides one way / evenly from the center |

## Handy plugins the creators use
Resize Align and Model Reflect (Stravant), GapFill, Part to Terrain, Archimedes (curved paths), Material Flip. Install only from trusted creators with good reviews: plugins run code.

## Where to look next
[[01-Roblox-Fundamentals/publishing-steps]] · [[10-Art-Environment/map-building-from-tutorials]] · [[10-Art-Environment/lighting-recipes-from-tutorials]] · [[18-Source-Library/video-tutorials-watched]]
