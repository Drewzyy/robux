---
description: One font, one palette, a spacing grid, one button template cloned everywhere, and 4.5:1 contrast on text and key elements.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C3; Veronica Aalin, dev.to 'Designing Clean Roblox GUIs'; ashexplained roblox-game-ux
verify: Community guidance. The 4.5:1 contrast ratio is the WCAG AA text standard; check with any contrast checker on real colours.
---
# Design system

- **One font**, used everywhere. Weights for hierarchy, not new fonts.
- **One palette**: background, surface, text, accent, success, danger. Write them in one shared module so every screen uses the same values.
- **Grid**: build on a grid, not by eye. `UIAspectRatioConstraint` plus `UIGridLayout`, a base cell (example 80x80), every size a multiple of it.
- **Contrast** at least 4.5:1 on text and key elements.
- **One button template** (Frame + TextLabel + UIStroke + UICorner + a small LocalScript), cloned everywhere. Same press feedback on every button.
- **Readable at phone size** first; see [[30-ui-ux/mobile-first-checklist]].
- Before building any non-trivial screen, write a short markdown design brief (screens, what each shows, where on a phone) and get it approved. Verify the built result with a phone playtest.
