---
description: Accessibility for Roblox games: text size, reduced motion, transparency, colourblind modes, captions, controls, and audio cues.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: accessibility, disability, colorblind, colourblind, captions, subtitles, reduced motion, text size, deaf, hearing, controls
sources: https://create.roblox.com/docs/production/publishing/accessibility (search summary); https://www.datawrapper.de/blog/colorblindness-part2
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
---
# Accessibility

## Do this (all cheap, all expected)
1. Respect Roblox's player settings (read-only on GuiService): PreferredTextSize (use AutomaticSize, not TextScaled), ReducedMotionEnabled (no shakes, flashes, or big scale bounces), PreferredTransparency (make panel backgrounds more solid when it is lower). Listen with GetPropertyChangedSignal.
2. Colour: never colour alone. Icons, shapes, labels. Offer a colourblind mode that swaps team and rarity colours to the Okabe-Ito set ([[08-UI-UX/color-palettes]]).
3. Hearing: every important sound has a visual twin (hit marker, screen-edge arrow for off-screen danger, icon for "boss is coming"). Optional captions for story or NPC lines in a TextLabel at the bottom.
4. Volume sliders per group (master, music, effects, UI) ([[11-Audio/audio-settings-menu]]).
5. No flashing faster than 3 times a second; keep full-screen flashes off when reduced motion is on.
6. Controls: big touch targets (at least 44 px), no required fast double-taps, hold-to-repeat instead of rapid tapping, gamepad support for every button.
7. Reading: short sentences, icons next to words, no time limit on reading prompts.

## Settings menu checklist
Text size note (uses Roblox setting), colourblind mode toggle, reduce motion toggle (game-side, defaults to Roblox's), captions toggle, volume sliders, camera shake toggle. Save to the player's data.
