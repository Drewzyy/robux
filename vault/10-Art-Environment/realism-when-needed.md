---
description: When to go realistic and how: PBR with SurfaceAppearance, reuse and tint textures, trim sheets, and mobile cost.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: realism, realistic, pbr, surfaceappearance, textures, materials, trim sheet, mobile performance
sources: https://devforum.roblox.com/t/surfaceappearance-studio-beta-released/663449 ; https://create.roblox.com/docs/resources/the-mystery-of-duvall-drive/materialize-the-world (search summary)
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
---
# Realism when needed

## Do this
- Default to stylised (clean shapes, flat or simple materials). Most top Roblox games are stylised; it runs on every phone and ages well.
- Use realism on purpose: horror, driving, or one hero object players stare at.
- PBR only works on MeshParts via SurfaceAppearance (colour, normal, roughness, metalness maps).
- Reuse one texture and tint it (SurfaceAppearance colour tint) instead of many unique textures; use trim sheets for walls and floors.
- Keep textures modest (1024 for props); thousands of unique textures sink low-end phones.
- Test on a live server and a low-end phone: mobile may stream lower-quality textures.
- Free CC0 materials: Poly Haven (through the Modeler agent).

## Cheap realism tricks that are not textures
Correct real-world scale, soft lighting with Atmosphere, a little fog, sound (footsteps per material, room echo), and small details at eye height.
