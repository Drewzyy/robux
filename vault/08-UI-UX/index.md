---
description: Interface rules: mobile-first, Scale not Offset, 58px safe area, HUD layout, onboarding flow, shop UI, thumbnail and icon specs.
type: section-index
status: verified
last_checked: 2026-11-09
sources: Research report C3
verify: scripts/check_vault.py --full checks every link below resolves and every status matches the note.
tags: [ui-ux, section-index]
---
# 08-UI-UX

Interface rules: mobile-first, Scale not Offset, 58px safe area, HUD layout, onboarding flow, shop UI, thumbnail and icon specs.

Load only the note your task needs. Do not read every note in this section.

| Note | What it holds | Status |
|---|---|---|
| [[08-UI-UX/ui-kit]] | START HERE: the UI kit in every game (Screen, Window, Button, Bar, Counter, Grid, Card, Toast). Build every screen from it. | draft |
| [[08-UI-UX/design-system]] | One font, one palette, spacing grid, one button template, 4.5:1 contrast. | draft |
| [[08-UI-UX/hud-template]] | What the HUD must show during the loop and where it goes on a phone. | draft |
| [[08-UI-UX/shop-ui]] | One HUD, one overlay per feature, 3-tap rule, no pop-ups in the first 5 minutes or on death. | draft |
| [[08-UI-UX/mobile-first-checklist]] | Scale not Offset, constraints, safe area (CoreUISafeInsets, 58px top bar), thumb zones, real-device test. | draft |
| [[08-UI-UX/thumbnail-icon-spec]] | Thumbnail 1920x1080 under 3 MB, icon 512x512 separate crop, gamepass 150x150, packaging rules. | draft |
| [[08-UI-UX/color-palettes]] | 60-30-10, one accent, meaning by colour plus icon, Okabe-Ito colourblind set, four palettes. | draft |
| [[08-UI-UX/fonts-and-text]] | Two fonts max, display vs body, phone sizes, avoid Gotham/Arial, prefer AutomaticSize over TextScaled. | draft |
| [[08-UI-UX/ui-animation]] | Duration and easing table, UIScale not Size, under 0.3 s, reduced motion, premium touches. | draft |
| [[08-UI-UX/ui-sound-pairing]] | Sound per UI action, pitch variation, consistency, rate cap. | draft |
| [[08-UI-UX/accessibility]] | Roblox's 3 accessibility settings, colourblind mode, visual twins for sounds, captions, controls. | draft |
| [[08-UI-UX/roblox-ui-production]] | Named-Instance shell, ResetOnSpawn, safe insets, responsive layout, Activated, focus, restrained motion, tests. | draft |
| [[08-UI-UX/game-ui-patterns-from-hits]] | Concrete UI patterns from hit Roblox and other games for menus, HUD, shop, reward reveals and notifications, with sizes in px and timings in seconds. | draft |
| [[08-UI-UX/nintendo-ui-and-feel]] | Nintendo UI and feel principles (Animal Crossing, Mario Kart, Zelda BOTW/TOTK, Splatoon) as rules for Roblox menus: sound on every input, bouncy motion, big icons, teaching without text. | draft |
| [[08-UI-UX/figma-handoff]] | How UI moves between BrewixGames, Figma and Roblox Studio: SVG mockups with named layers, editing in Figma, and free plugins that turn a Figma frame into Roblox UI. | draft |
| [[08-UI-UX/ui-polish-passes]] | Six ordered passes that turn a generic, cluttered screen into a polished one: clean and group, one primary action, depth, fold the unimportant, grouped nav with icons, and search. | draft |
| [[08-UI-UX/reward-moments-and-juice]] | The four-beat reward reveal (anticipation, reveal, celebration, collection) with timings, sound and particle rules, scaled for small free rewards and big paid ones, plus skip and reduced-motion rules. | draft |
| [[08-UI-UX/ui-frameworks-fusion-react-lua-vide]] | Compare Fusion, React Lua, Vide and Roact for building Roblox UI in code, with stars, licenses, a tiny example each, and which one an agent should pick. | draft |
