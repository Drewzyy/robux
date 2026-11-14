---
description: Six ordered passes that turn a generic, cluttered screen into a polished one: clean and group, one primary action, depth, fold the unimportant, grouped nav with icons, and search.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: ui polish, ai slop, generic ui, hierarchy, grouping, primary button, cta, glass, frosted, fold, collapse, navigation, search, command palette, delight
sources: TikTok video by @buildwithomar on refining an AI-made dashboard step by step (frames and captions read 2026-11-14, audio not heard); applied to the BrewixGames dashboard the same day
verify: The order of passes is from the video's on-screen captions; the numbers (counts, timings, sizes) are this studio's starting rules, UNVERIFIED against player data.
tags: [ui-ux, playbook]
---
# UI polish passes

A first draft of any screen tends to look generic: too many equal boxes, several loud buttons, everything visible at once. Fix it in passes, one change of kind per pass, and look at the result after each.

## Do this
- **Pass 1, clean and group:** remove decoration that carries no information; put related elements in one group; aim for 3 to 5 groups per screen, not 10 equal cards.
- **Pass 2, one primary action:** exactly one filled, bright button per screen, and its label says the outcome with the number in it ("Collect 120 coins", not "Collect"). Everything else is a quiet secondary button.
- **Pass 3, delight on that one button only:** a small looping motion on hover or while waiting (arrow nudge of 4 to 6 px, 0.8 s loop, or a soft pulse). Nowhere else.
- **Pass 4, depth:** let the world show through panels (BackgroundTransparency 0.15 to 0.3 with a dark tint, UIStroke 1 px at 20 to 35% white) so the UI feels part of the game. Text contrast stays at 4.5:1 or better per [[08-UI-UX/accessibility]].
- **Pass 5, fold the unimportant:** anything not needed in the next 10 seconds collapses to one line or moves behind a tab or a "More" row. A new player's HUD shows at most 4 things.
- **Pass 6, navigation:** group menu items under 2 to 4 headings, give each an icon, and keep the menu to 6 items or fewer. On screens with many destinations (inventory, shop), add a search box at the top.
- After every pass, check it on a phone-sized screen per [[08-UI-UX/mobile-first-checklist]] before starting the next.

## How to build it in Roblox
- Groups: one Frame per group with UIListLayout and UIPadding; a section title TextLabel at 60% white.
- Primary button: one colour from the style kit accent; UIGradient optional; TweenService loop on a child arrow ImageLabel, cancelled when the button is hidden.
- Depth: Frame.BackgroundTransparency plus UIStroke and UICorner. Roblox has no real blur behind UI on a ScreenGui; do not fake it with a full-screen BlurEffect while the player is moving.
- Fold: a collapsed row that expands with a 0.15 s size tween; remember the state per player for the session.
- Timings follow [[08-UI-UX/ui-animation]]; sounds follow [[08-UI-UX/ui-sound-pairing]].

## Mistakes
- Two primary buttons on one screen. The second one makes both weaker.
- Motion on everything. Delight works because it is in one place.
- Transparent panels over a busy scene with thin text. Darken the tint before giving up contrast.
- Folding the thing the player needs right now (their currency, the next goal).

## For our Roblox games
Copy the method: passes in this order, one primary action, fold what is not needed now. Never copy another product's layout pixel for pixel, its icons, or its brand colours.
