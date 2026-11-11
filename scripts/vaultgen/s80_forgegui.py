from common import note
note("11-Audio/forgegui",
 "ForgeGUI: Andrew's chosen tool for sound effects (2026-11-09). What is confirmed, what is not, and how the studio hands it work through sounds.json.",
 "reference", "unverified",
 "Andrew's instruction 2026-11-09; web search results (creati.ai, topai.tools, moge.ai, forgegui.com blog listing); forgegui.com itself could not be reached from the build sandbox",
 "UNVERIFIED: no source found says ForgeGUI makes audio; every listing describes visual assets. Andrew confirms in his ForgeGUI account whether it generates sound effects, its price, and its license for Roblox games, then updates this note.",
r"""
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
""")

note("11-Audio/audio-pipeline",
 "Audio: sounds.json request sheet per game, generated in ForgeGUI by Andrew, uploaded and moderated, wired by Builder only with real IDs; license recorded; silence beats a bad loop.",
 "playbook", "draft",
 "Research report (Nov 9 2026) C8 audio; Andrew's ForgeGUI decision (2026-11-09)",
 "The sounds.json shape is checked by scripts/gates/check_stage.py (librarian). Whether ForgeGUI makes audio is UNVERIFIED (see 11-Audio/forgegui). Each license line is checked when the sound is chosen.",
r"""
# Audio pipeline

## Rules
- 100% copyright-free or properly licensed audio only. Record the source and license for every sound.
- No copyrighted music. No Toolbox audio.
- Uploads are moderated by Roblox.
- Silence is better than a bad loop.
- Fit the mood; loud or jarring sounds break it.
- Every core action gets a short sound ([[09-Animation-VFX/vfx-sfx-library]]).

## Source
Sound effects come from **ForgeGUI** (Andrew's decision, 2026-11-09). Whether ForgeGUI makes audio is UNVERIFIED; see [[11-Audio/forgegui]].

## The request sheet: `projects/<game>/sounds.json`
```json
{
  "source": "forgegui",
  "sounds": [
    {
      "id": "coin_pickup",
      "event": "player touches a coin",
      "prompt": "short bright coin chime, 0.3 seconds, cheerful, no reverb tail",
      "length_s": 0.3,
      "loop": false,
      "volume": 0.6,
      "first_60_seconds": true,
      "asset_id": null,
      "license": null
    }
  ]
}
```
- `id`: lowercase snake case, unique. Code refers to sounds only by this id.
- `prompt`: what to type into the generator. Describe length, character, and what to avoid.
- `asset_id`: `null` until Andrew uploads the sound, then `"rbxassetid://<number>"`.
- `license`: who made it and under what terms, filled with the asset ID.

## Who does what
| Step | Who |
|---|---|
| Write or update the sheet | Librarian (agent) |
| Generate, upload, fill `asset_id` and `license` | Andrew (needs his ForgeGUI and Roblox accounts) |
| Wire sounds by `id` from a shared module mirroring the sheet; skip entries with no ID | Builder (agent) |
| Check every first-60-seconds sound has an ID | Playtest (agent) |
""")
print("ok")
