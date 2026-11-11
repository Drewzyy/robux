---
description: What the HUD must show during the loop (objective, currency, progress, health or energy, next reward) and where it goes on a phone.
type: template
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C3; ashexplained roblox-game-ux; tabooharmony roblox-gui
verify: Template. Verify by a phone playtest screenshot: every HUD element readable and outside the top 58 px and the bottom thumb controls.
tags: [ui-ux, template]
---
# HUD template

## Must be visible during the core loop
| Element | Why | Where on a phone |
|---|---|---|
| Current objective | the player always knows what to do | top third, below the 58 px top bar |
| Currency | feedback on every reward | top corner, inside safe insets |
| Progress (bar, stage, collection meter) | goal gradient: near goals pull | top third |
| Health / energy (if any) | failure is legible | near the objective |
| Next reward | open loop | close to progress |

## Rules
- Anchor critical HUD to the top 40% of the screen: the virtual joystick and jump button take roughly the bottom 30% on mobile.
- Nothing interactive in the top 58 px (Roblox top bar). Use `ScreenGui.ScreenInsets = Enum.ScreenInsets.CoreUISafeInsets`.
- Minimal HUD in the first minute; add action and meta HUD as the player meets them (progressive disclosure).
- `ResetOnSpawn = false` for persistent HUD.
