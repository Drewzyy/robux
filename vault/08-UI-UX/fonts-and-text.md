---
description: Which Roblox fonts to use where, sizes that read on phones, TextScaled pitfalls, and text-size accessibility.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: fonts, text, typography, mobile, phone, font, luckiest guy, fredoka, gotham, source sans, readability, textscaled
sources: https://create.roblox.com/docs/production/publishing/accessibility (search summary); devforum.roblox.com/t/new-fonts-are-live/821617
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [ui-ux, playbook]
---
# Fonts and text

## Do this
- Two fonts per game at most: one display font for big titles and numbers, one clean font for body text and settings.
- Display: a heavy rounded face (for example Luckiest Guy or Fredoka One) with UIStroke for titles, rewards, and currency counts. Not for long text.
- Body and settings: a plain sans (for example Source Sans Pro or Builder Sans if offered in the font picker) for menus and descriptions.
- Do not build new UI on Gotham or Arial: Roblox has said they will be removed (search summary of the Font docs).
- Phone minimum: about 18 px body, 28 px+ for numbers players must read fast. Test on the smallest phone emulator.
- Prefer AutomaticSize and fixed TextSize over TextScaled: TextScaled ignores the player's preferred text size setting (GuiService.PreferredTextSize).

## Rules for numbers
- Abbreviate big numbers (1.2K, 3.4M) and keep a fixed-width feel by right-aligning counters.
- Animate increases (see [[07-Monetization/currency-satisfaction]]).

## Localisation-safe
Display fonts like Luckiest Guy have glyph issues outside English (reported on the DevForum); fall back to the body font for translated text.
