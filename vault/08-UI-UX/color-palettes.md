---
description: UI colour rules: 60-30-10, one accent, contrast, colour meaning, and colourblind-safe pairs.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: color, colour, palette, ui colors, contrast, accent, colorblind, okabe-ito, theme
sources: https://www.datawrapper.de/blog/colorblindness-part2 ; Okabe-Ito palette (conceptviz.app reference)
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
---
# Colour palettes for UI

## Do this
- 60-30-10: 60% one calm background tone, 30% a secondary surface tone, 10% one accent for things to press.
- One accent colour means "press me". Use it only on primary buttons and the next goal.
- Never carry meaning with colour alone: add an icon, shape, or word (rarity = colour + label + border style).
- Text contrast: light text on dark panels or dark text on light panels; add UIStroke (1 to 2 px, dark, 0.3 to 0.5 transparency) on text over the 3D world.
- Keep a palette module (`src/shared/Theme.luau`) with named colours; never hard-code Color3 in screens.

## Meaning that players already know
| Meaning | Colour | Always pair with |
|---|---|---|
| Buy, confirm, go | green or the game accent | label "Buy" / check icon |
| Cancel, danger, damage | red | X icon, shake |
| Currency, rewards | gold / yellow | coin icon |
| Rare tiers | grey < green < blue < purple < orange/gold | tier name + border |
| Info, links | blue | icon |

## Colourblind-safe set (Okabe-Ito)
Orange #E69F00, sky blue #56B4E9, bluish green #009E73, yellow #F0E442, blue #0072B2, vermillion #D55E00, reddish purple #CC79A7, black #000000. Use these for anything players must tell apart (teams, rarities, map markers). Avoid red vs green as the only difference.

## Palettes that work together (starting points)
- Cozy: cream #FFF4E0, warm brown #6B4F3A, sage #8DB596, accent coral #FF7A59.
- Night / spooky: navy #0E1A2B, slate #23344D, fog blue #5C7A99, accent amber #FFB347.
- Bright simulator: sky #4FC3F7, white #FFFFFF, grass #7CD86B, accent gold #FFC83D.
- Sci-fi: near-black #0B0F14, steel #1F2A36, cyan #2DE2E6, accent magenta #FF3CAC.
Check each against the colourblind set above when used for meaning.
