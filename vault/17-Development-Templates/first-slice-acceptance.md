---
description: Observable acceptance criteria for a first vertical slice: playable loop, visual direction, map layout, mobile UI, feedback, and build checks versus played in Studio.
type: checklist
status: draft
last_checked: 2026-11-18
sources: Studio build gate (brewix gate build); 10-Art-Environment/lighting-recipes-from-tutorials; 10-Art-Environment/map-building-from-tutorials; 08-UI-UX/accessibility
verify: Each criterion below is something a person can check by playing the slice or reading the project. The time and size numbers are studio targets, not Roblox rules.
tags: [development-templates, checklist, vertical-slice]
---
# First slice acceptance criteria

A first slice is a small piece of the game that is complete enough to judge. The Builder uses this as its bar. Each line is checkable; if you cannot check it, it is not met.

## Criteria
- [ ] **One core loop, start to finish.** A new player can do the whole loop (act, get a result, get a reward or consequence, go again) in under 5 minutes, with the first fun action inside 10 seconds.
- [ ] **A distinctive visual direction.** The style kit names a palette (hex values, at most 5 main colours) and a lighting preset, and the game uses them. See [[10-Art-Environment/lighting-recipes-from-tutorials]]. A screenshot should not look like the default baseplate.
- [ ] **Deliberate environment composition.** Marked spawn point, at least 3 points of interest visible from the spawn or on the way, borders that are not straight lines (cliffs, water, props, curves) and no way out of the map. See [[10-Art-Environment/map-building-from-tutorials]].
- [ ] **Readable mobile-first UI.** Tap targets at least 44x44 px, body text at least 16 px, nothing under the top bar or notch (respects the safe area), checked at a phone-sized viewport.
- [ ] **Purposeful feedback.** Every player action gets a visual or sound response within about 0.1 seconds. No silent or invisible actions.
- [ ] **Satisfying core interaction.** The main action feels good repeated 20 times: clear input, clear result, and a short effect (hit stop, squash, number pop, sound) that fits it.

## What the words mean
- **Build checks passed:** `brewix gate build` ran clean (rojo build, selene, stylua, Lune tests). This says the code compiles and its pure logic tests pass. It says nothing about how the game plays or looks.
- **Played in Studio:** a person (or a Studio playtest with logs) actually ran the slice and checked the criteria above. Only this lets you say a criterion is met for feel, visuals, mobile UI and feedback.
- A report must say which of the two it is. Write "build checks passed; not yet played in Studio" when that is the state.

## Wording rule
A code-only or unplayed build must never be called AAA, finished, polished or ready to publish. It can be described by its target ("aiming for...") or by its real state.
