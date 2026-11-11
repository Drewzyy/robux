---
description: Lighting starting points per mood (cozy, spooky, bright, sci-fi), bloom and colour correction cautions, and phone cost.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: lighting, atmosphere, bloom, colorcorrection, mood, future lighting, shadows, day night, preset
sources: https://devforum.roblox.com/t/improve-lighting-step-by-step/1387033 ; https://devforum.roblox.com/t/bloom-vs-color-correction/494805
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
---
# Lighting presets

## Do this
- Put values in a Rojo module (`src/shared/Configs/Lighting.luau`) applied at start. Tune in Studio, copy values back.
- Future lighting looks best but costs the most; keep local lights with shadows to a handful. Test on a phone.
- Bloom: keep Intensity low (about 0.3 to 0.6) and Threshold around 1.5 to 2.5, or bright parts wash out. Bloom Size also changes how Neon glows.
- ColorCorrection: small values only (Contrast and Saturation 0.05 to 0.2). Big contrast makes dark scenes unreadable.
- Hazards and the next goal must stay readable in every preset.

## Starting points (tune by eye)
| Mood | ClockTime | Ambient / OutdoorAmbient | Atmosphere | ColorCorrection |
|---|---|---|---|---|
| Cozy afternoon | 15.5 | warm grey | Density 0.3, Haze 1.5, warm Color | TintColor slight warm, Saturation 0.1 |
| Spooky night | 0 to 2 | dark blue | Density 0.4 to 0.5, Haze 2, blue-grey | Saturation -0.1, Contrast 0.1 |
| Bright simulator | 13 | light | Density 0.2, Haze 0.5 | Saturation 0.15 |
| Sci-fi interior | any (indoor) | dark | low | TintColor cool, Contrast 0.1 |
Community starting point for outdoor scenes: Atmosphere Density 0.3, Haze 2, ColorCorrection Contrast 0.1, Saturation 0.1.

Day-night and flicker animation: [[09-Animation-VFX/building-and-world-animation]].
