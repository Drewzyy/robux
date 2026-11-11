---
description: Building Roblox UI that survives phones, consoles, and respawns: lifecycle, safe insets, responsive layout, input, focus, and quality gates.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: roblox ui, screengui, resetonspawn, safe insets, responsive, udim2, gamepad, focus, preferredinput, activated, layout, canvasgroup
sources: https://github.com/gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0)
verify: UNVERIFIED: condensed from the roblox-ui skill (Apache-2.0) on 2026-11-11, which cites Roblox's UI docs.
tags: [ui-ux, playbook]
---
# Roblox UI for production

Condensed from the roblox-ui skill in gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0).

## Do this
- Author the UI shell as named Instances (ScreenGui > Root > Header, Content, Footer) with a small controller LocalScript; generate only repeated cells. One giant script that builds everything is unreadable and looks generated.
- Decide ResetOnSpawn per ScreenGui (usually false for HUD and menus); never hold references to a destroyed copy.
- Keep interactive UI inside CoreUISafeInsets (ScreenGui.ScreenInsets) and away from the touch move and jump corners.
- Responsive: UDim2 scale for placement plus bounded offsets, AnchorPoint, UIListLayout or UIGridLayout, AutomaticCanvasSize for scrolling, UISizeConstraint and UIAspectRatioConstraint. Make a real narrow layout instead of shrinking the desktop one.
- Input: use GuiButton.Activated (one path for mouse, touch, gamepad). Choose hints from UserInputService.PreferredInput, not TouchEnabled. On gamepad, set GuiService.SelectedObject when a panel opens; modals trap focus and restore it on close.
- Motion: animate a panel once (CanvasGroup GroupTransparency fade about 0.14 s), not every child; respect ReducedMotionEnabled; cancel tweens and disconnect events when a screen closes.
- Quality: hierarchy from size, weight, contrast, and spacing before decoration. No card-in-card shells, pills everywhere, random gradients, neon outlines, or huge empty panels. Buttons of different importance look different.
- Test in Device Simulator (phone portrait and landscape, tablet, console) and with the Controller Emulator, then respawn and test again.
