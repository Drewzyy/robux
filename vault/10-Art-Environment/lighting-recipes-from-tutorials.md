---
description: Lighting from popular tutorials, checked against the docs: LightingStyle, what each property does, the exact cartoon simulator preset, and realistic, cozy and night recipes.
type: playbook
status: draft
last_checked: 2026-11-18
keywords: lighting, lightingstyle, realistic, soft, technology deprecated, ambient, outdoorambient, colorshift, environmentdiffusescale, atmosphere, bloom, colorcorrection, sunrays, depthoffield, sky, cartoon lighting, simulator lighting, fog, clouds
sources: Captions of 7 YouTube tutorials (watched 2026-11-18): Rileybytes "Give me 2 minutes and I'll fix your lighting FOREVER" (06KmD8aNJ5o); Gamer M8 "How to make CARTOON LIGHTING" (YIyir52FiYo); Develuper "Make Your Roblox Game Look 10x Better With Lighting" (vkEXLmRAQgs); Ice Guy "How to Make Realistic Lighting" (adgCWET34rU); RingWorld Studio "How I Do Lighting & Atmosphere" (UlvSrWHESwA); HollyHollers "How to Make Realistic Lighting" (mrPm-nzhaV0); BONES "Unified lighting Explained" (6zSe8cuBB_I). Checked against create.roblox.com/docs/environment/lighting and the Technology enum page (read 2026-11-18).
verify: The cartoon preset numbers are exact (two videos agree). Other recipes are the creators' steps without full numbers; tune in Studio.
tags: [art-environment, lighting, playbook]
---
# Lighting recipes

## Do this
- Set `Lighting.LightingStyle`: **Soft** for stylised, low-poly and phone-first games (cheaper, flatter); **Realistic** for detailed scenes or many point lights. The old `Technology` setting (Future, ShadowMap, Voxel) is deprecated (docs, 2026).
- Decide the mood first, then pick the Sky before anything else (Atmosphere colours follow it).
- Line the sun up with the skybox's sun using ClockTime and GeographicLatitude.
- Small effect values only; copying numbers blindly fails, every map differs.
- Phone-first games: Soft style, few shadowed lights, and test on a phone.

## What the properties do
- Ambient: light colour in shadow areas. OutdoorAmbient: the same, outdoors only.
- Brightness: sun strength. ColorShift_Top: tint on sun-facing surfaces (shapes the look a lot). ColorShift_Bottom: tint on the shadow side (usually black).
- EnvironmentDiffuseScale: sky colour bouncing onto objects; lower it if the sun looks over-saturated. EnvironmentSpecularScale: reflections on shiny parts.
- GlobalShadows on. ShadowSoftness 0 (hard) to 1 (soft); only works with Realistic.
- ClockTime: time of day. GeographicLatitude: where the sun and moon sit.
- PrioritizeLightingQuality (docs): when quality drops, true keeps shadows and shaders near the player, false keeps view distance.

## Effects (insert into Lighting)
- Sky first. Atmosphere: Density 0.3 to 0.4 and Offset 0 for depth without fog; Glare only works with Haze; heavy density hides the world and balloons the sun.
- Bloom: Threshold decides what glows; a low threshold makes everything glow (players too). Keep it subtle.
- SunRays: a little, mostly for afternoons and sunsets.
- DepthOfField: blurs only far things; never overdo. Avoid Blur (blurs everything).
- ColorCorrection: the final look; small contrast up, slight saturation change, very subtle tint.
- Clouds (on Terrain): Cover and Density darken the sky.

## Cartoon simulator preset (exact)
Two popular videos use the same values:
| Property | Value |
|---|---|
| Ambient | 200, 160, 225 |
| OutdoorAmbient | 125, 100, 150 |
| ColorShift_Top | 215, 190, 135 |
| ColorShift_Bottom | 0, 0, 0 |
| Brightness | 2 to 2.5 |
| EnvironmentDiffuseScale | 0.3 to 0.4 |
| EnvironmentSpecularScale | 0.25 to 0.5 |
| ClockTime / GeographicLatitude | 14 / 40 |
| Fog | FogColor 200, 170, 250; FogStart 0; FogEnd 2500 |
| ColorCorrection | Brightness 0.05, Contrast 0.1, Saturation 0.15, TintColor white |
Remove Atmosphere, Bloom, DepthOfField and SunRays for this look (old fog needs no Atmosphere), and use a bright blue skybox. The videos set it with ShadowMap; with today's LightingStyle use Soft and tune Brightness by eye.

## Other recipes
- **Realistic day**: Realistic style, sky first, Brightness up (one creator uses 4.5), blue-tinted Ambient, warm ColorShift_Top, EnvironmentDiffuseScale a bit lower, low-density Atmosphere, subtle Bloom (low intensity and size), light DepthOfField, ColorCorrection with a little contrast, slight desaturation and a faint warm tint.
- **Cozy sunset**: sunset sky, Realistic, cool Ambient against a warm sun, creamy ColorShift_Top, terracotta ColorShift_Bottom, slight desaturation so greens are not harsh, light mist, a few sun rays.
- **Dark rainy night**: ClockTime at night, very dark ambients, low Brightness, no Atmosphere glare, blue tint, two point lights (one near, one far) as eye-catchers, heavy Clouds, rain effects and night sounds.
- Sound sells the mood: a looped ambient track in SoundService.

## For our Roblox games
Scenic writes these as a Rojo config (see [[10-Art-Environment/lighting-presets]]); start from the cartoon preset for simulators and kid games.
