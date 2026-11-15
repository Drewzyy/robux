---
description: Map of notes on Roblox interface work: mobile-first layout, HUD, shop screens, fonts, colour, accessibility, animation and Figma handoff.
type: reference
status: draft
last_checked: 2026-11-15
keywords: ui, ux, hud, shop ui, mobile, safe area, fonts, colour, accessibility, buttons, menus, figma, layout, thumbnails
sources: vault notes only, read 2026-11-15 (front matter of each linked note)
verify: Built from note descriptions; each linked note carries its own source and verify line.
tags: [start-here, reference]
---
# UI
Use for any screen, button, menu or HUD question.

## Do this
- Read first, step 1: [[08-UI-UX/mobile-first-checklist]] (Scale not Offset and safe area).
- Read first, step 2: [[08-UI-UX/hud-template]] (what the HUD must show).
- Read first, step 3: [[08-UI-UX/design-system]] (one font, palette, button).
- Then pick the section below that matches your task and read only those notes.

## How to use this map
- Read the three notes under Do this, then open one section below.
- Each note has its own Do this list; act on that, not on this page.
- If a note is marked UNVERIFIED, tune from your own analytics.

## Layout and structure
- [[08-UI-UX/mobile-first-checklist]]: Scale, constraints, 58 px top bar.
- [[08-UI-UX/roblox-ui-production]]: lifecycle, safe insets, respawns.
- [[08-UI-UX/hud-template]]: objective, currency, progress.
- [[08-UI-UX/shop-ui]]: one overlay per feature, 3-tap rule.
- [[08-UI-UX/index]]: section index.

## Look
- [[08-UI-UX/design-system]]: one font, one palette, one button template.
- [[08-UI-UX/color-palettes]]: 60-30-10 and colourblind safety.
- [[08-UI-UX/fonts-and-text]]: fonts and sizes that read on phones.
- [[08-UI-UX/ui-polish-passes]]: six ordered passes.
- [[08-UI-UX/accessibility]]: text size, reduced motion, contrast.

## Patterns and inspiration
- [[08-UI-UX/game-ui-patterns-from-hits]]: patterns from hit games.
- [[08-UI-UX/nintendo-ui-and-feel]]: Nintendo feel principles.
- [[08-UI-UX/reward-moments-and-juice]]: four-beat reveal.

## Motion and sound
- [[08-UI-UX/ui-animation]]: durations, easing, press feedback.
- [[08-UI-UX/ui-sound-pairing]]: sounds per UI action.
- [[13-Luau-Engineering/tween-and-animation-cleanup]]: clean tweens.

## Handoff and packaging
- [[08-UI-UX/figma-handoff]]: SVG mockups to Figma to Studio.
- [[08-UI-UX/thumbnail-icon-spec]]: thumbnail and icon sizes.
- [[17-Development-Templates/style-kit-template]]: style kit for every game.

## UI that supports play
- [[04-Onboarding/onboarding-flow]]: onboarding UI order.
- [[07-Monetization/offer-pressure-limits]]: how many prompts before players leave.
- [[07-Monetization/currency-satisfaction]]: currency display feel.
- [[14-QA-Optimization/device-test-matrix]]: test screen sizes.

## For our Roblox games
- Copy the principle: one HUD, one panel at a time, big touch targets, Scale not Offset. Never copy another game's UI art, icons or layouts pixel for pixel.
- Purchase prompts open only from a player tap; no pop-up pressure.
- Test on the smallest phone first.
