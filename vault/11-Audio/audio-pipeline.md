---
description: Audio: sounds.json request sheet per game, generated in ForgeGUI by Andrew, uploaded and moderated, wired by Builder only with real IDs; license recorded; silence beats a bad loop.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C8 audio; Andrew's ForgeGUI decision (2026-11-09)
verify: The sounds.json shape is checked by scripts/gates/check_stage.py (librarian). Whether ForgeGUI makes audio is UNVERIFIED (see 11-Audio/forgegui). Each license line is checked when the sound is chosen.
tags: [audio, playbook]
---
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
