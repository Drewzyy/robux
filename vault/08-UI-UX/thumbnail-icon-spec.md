---
description: Thumbnail 1920x1080 under 3 MB (up to 10), icon 512x512 as a separate crop, game pass 150x150, badge 512 circle; packaging rules.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C3; thumbblox.com roblox-thumbnail-size; umatechnology upload guide; obby.fun icon/thumbnail maker
verify: Sizes from secondary guides. Confirm in the Creator Dashboard upload screen before first upload. The custom-thumbnail requirement was verified in creator-docs discovery.md on 2026-11-09.
tags: [ui-ux, reference]
---
# Thumbnail and icon spec

| Asset | Size | Notes |
|---|---|---|
| Experience thumbnail | 1920x1080, 16:9 | under 3 MB each, up to 10 images or videos; smaller uploads are stretched |
| Game icon | 512x512, 1:1 | a separate crop, not a resized thumbnail; readable at about 150x150 |
| Game pass icon | 150x150 | |
| Badge icon | 512x512 | circular crop |
| YouTube cover / trailer | 1280x720 | |

Formats: .png, .jpg, .gif, .tga, .bmp. Avoid .webp (community reports of silent failures).

## Packaging rules
- Avatar clearly visible; one focal point.
- Text under 4 words, readable on a phone; high contrast.
- Show something that exists in the game. The thumbnail is a promise the first 10 seconds must keep.
- No platform logos, catalog items, trademarks, or recognisable characters. Every image is moderated.
- A custom thumbnail is **required** to receive impressions from homepage recommendations (verified in Roblox creator-docs discovery.md, 2026-11-09).

Capture workflow: [[10-Art-Environment/thumbnail-pipeline]]. What makes a thumbnail get clicked: [[10-Art-Environment/thumbnail-mrbeast-logic]].
