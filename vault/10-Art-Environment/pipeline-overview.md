---
description: Asset pipelines at a glance: Blender to Studio, Studio capture to thumbnail, audio to SoundService, VFX and SFX, and where each lands in the Rojo project.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C8
verify: Overview; each step is detailed and sourced in its own note.
tags: [art-environment, reference]
---
# Pipeline overview

| Asset | Path | Note |
|---|---|---|
| 3D models | primitives in Studio, or Blender (optionally driven through Blender MCP) to FBX/OBJ, import to Studio | [[10-Art-Environment/3d-model-checklist]] |
| Creator Store models | search and insert through the Studio MCP server (Andrew's Studio must be open), license recorded | [[10-Art-Environment/asset-sources-and-licenses]] |
| Thumbnails and icon | Studio Free Camera capture, then layout | [[10-Art-Environment/thumbnail-pipeline]] |
| Audio | copyright-free, licensed, moderated on upload | [[11-Audio/audio-pipeline]] |
| VFX and SFX | small feedback on every action, within particle caps | [[09-Animation-VFX/vfx-sfx-library]] |
| Lighting | Lighting and Atmosphere values in a Rojo config module | [[10-Art-Environment/lighting-and-mood]] |

## Two lanes (god plan model, as summarized in the report)
- **Lane A, headless:** Rojo, Lune, Open Cloud. Code and data live in the repo. Lasting changes go here.
- **Lane B, live:** Studio MCP server in Andrew's open Studio. Scene reads, inserts, playtests, screenshots. MCP edits to code are not persistent; copy lasting changes back through Lane A.
A cloud session cannot touch Studio or Blender; the repo is the handoff.
