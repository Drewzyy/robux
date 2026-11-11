---
description: ForgeGUI: Andrew's chosen tool for sound effects (2026-11-09). What is confirmed, what is not, and how the studio hands it work through sounds.json.
type: reference
status: unverified
last_checked: 2026-11-09
sources: Andrew's instruction 2026-11-09; web search results (creati.ai, topai.tools, moge.ai, forgegui.com blog listing); forgegui.com itself could not be reached from the build sandbox
verify: UNVERIFIED: no source found says ForgeGUI makes audio; every listing describes visual assets. Andrew confirms in his ForgeGUI account whether it generates sound effects, its price, and its license for Roblox games, then updates this note.
---
# ForgeGUI

**Decision (Andrew, 2026-11-09):** use ForgeGUI for sound effects.

## What is confirmed
Nothing about audio. Third-party listings and ForgeGUI's own blog title describe it as an AI game-asset tool for **visual** assets: UI sets, HUD elements, icons, thumbnails, GFX, and 3D assets, exported as PNG or ZIP or pushed to Roblox Studio through the ForgeGUI plugin.

## UNVERIFIED (check in Andrew's account)
- Does it generate sound effects at all?
- Price and free tier. The studio's build rule is $0; if sound generation is paid, that is Andrew's call.
- License for using its output in a monetized Roblox game.
- Whether it has an API. Agents have no access to it either way: Andrew runs it.

## How the studio uses it (works with any sound tool)
1. Librarian writes the sound request sheet into `projects/<game>/sounds.json` ([[11-Audio/audio-pipeline]]): every sound the game needs, the event that plays it, a ready-to-paste prompt, length, loop, volume.
2. Andrew generates each sound in ForgeGUI, uploads it to Roblox (Creator Hub or Studio; audio is moderated), and pastes the `rbxassetid://` into that entry, with the license line.
3. Builder wires only entries that have a real asset ID. An entry without one plays nothing: silence beats a bad loop.
4. Playtest checks that every core action in the first 60 seconds has a sound with an ID.

If ForgeGUI turns out not to make audio, change `source` in the sheet to the tool used instead; nothing else changes. Never Toolbox audio ([[10-Art-Environment/no-toolbox-rule]]).

## Also worth trying
Every listing says ForgeGUI makes thumbnails and icons. Splash's comps ([[10-Art-Environment/thumbnail-mrbeast-logic]]) could be finished in ForgeGUI from the ranked layouts, using real game captures. UNVERIFIED until Andrew tries it.
