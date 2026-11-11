---
description: Mobile first: Scale not Offset, constraints, CoreUISafeInsets and the 58 px top bar, thumb zones, Activated not mouse events, real-device test.
type: checklist
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C3; ashexplained, tabooharmony roblox-gui, simplified.media, johnygorsky roblox-ui
verify: Community guidance. Mobile share of sessions differs by source (50% to 78%): no single figure is verified. The 44-48 px tap size is Apple/Material guidance, not a Roblox rule. Verify on a real phone.
tags: [ui-ux, checklist]
---
# Mobile-first checklist

Mobile is the majority of Roblox sessions (sources say 50% to 78%; no single number is verified). Design for the phone, then expand.

## Layout
- [ ] Position and size with **Scale**, not Offset. Offset only for thin borders and padding.
- [ ] `UIAspectRatioConstraint` and `UISizeConstraint` (MinSize/MaxSize) lock proportions.
- [ ] Text: `TextScaled` plus `UITextSizeConstraint`. Minimum readable size about 16 px on mobile (one guide).
- [ ] One global `UIScale` knob.
- [ ] Prefer vertical layouts on phones.

## Safe areas
- [ ] Nothing interactive in the top 58 px (Roblox top bar). `ScreenInsets = CoreUISafeInsets` for HUD and menus; `None` only for full-bleed non-interactive backgrounds.
- [ ] Nothing critical in the bottom ~30% (joystick and jump button) or at screen edges and gesture zones.
- [ ] `IgnoreGuiInset = true` only for fullscreen art, never with critical content behind the top bar.

## Input
- [ ] `GuiButton.Activated` for core actions (fires for click and tap). Never mouse-only events.
- [ ] Prefer the Input Action System for cross-platform input; `UserInputService.TouchEnabled` to adapt layout.
- [ ] Tap targets: 44x44 (Apple) to 48x48 (Material) as an accessibility guide. There is no official Roblox tap-size rule.

## Test
- [ ] Studio Device Emulator for layout.
- [ ] A real phone for touch size, thumb reach, and low-end performance. The emulator does not show these.
