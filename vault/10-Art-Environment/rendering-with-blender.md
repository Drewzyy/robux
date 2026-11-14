---
description: Using the computer's graphics card to render high-quality images in Blender (Cycles) for thumbnails, icons, shop previews, and model checks.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: render, gpu, cycles, eevee, blender, thumbnail render, icon render, shop preview, graphics card, cuda, optix, hip
sources: Blender manual (Cycles render devices); 10-Art-Environment/thumbnail-pipeline
verify: UNVERIFIED: check device names in Blender Preferences > System on the actual PC.
tags: [art-environment, playbook]
---
# Rendering with Blender

Roblox draws the game itself on each player's device, so a stronger PC does not make the live game look better. Where the PC's graphics card does help: rendering images in Blender.

## Do this
- Use Cycles with the GPU for final images: Preferences > System > Cycles Render Devices (OptiX or CUDA for NVIDIA, HIP for AMD, Metal on Mac), then Render Properties > Device: GPU Compute.
- Use EEVEE for fast previews of models and effects.
- Good uses: game icon (512x512), thumbnails (1920x1080) from the real map and characters, shop item previews (transparent PNG), turntables of models for review.
- Settings that look good fast: 128 to 256 samples with denoising on, a three-point light (key, fill, rim), the game's palette in the background, Filmic or AgX view.
- Thumbnails still follow [[08-UI-UX/thumbnail-icon-spec]] and [[10-Art-Environment/thumbnail-mrbeast-logic]].
