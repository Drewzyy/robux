---
description: Original SVG charts for hit-stop, shake and squash timing, the core loop, retention loop stack, first-60-seconds onboarding timeline and a sound feedback map.
type: reference
status: draft
last_checked: 2026-11-15
keywords: game feel, juice, hit-stop, screen shake, squash, core loop, retention, onboarding, sound feedback, timeline
sources: Original SVG diagrams drawn for this vault, 2026-11-15; rules from the linked vault notes, read 2026-11-15
verify: Diagrams are our own illustrations of the vault rules. Numbers come from the linked notes; check any UNVERIFIED mark there. No real game art used.
tags: [development-templates, reference]
---
# Example gallery: game feel and loops
Original diagrams drawn for this vault so agents and the owner can see what the rules look like. Nothing here is copied from a real game.

## Do this
- Respond to input in the first frame and within 100 ms.
- Hit-stop 50 ms light, 100 to 150 ms heavy, never above 200 ms; shake with an off setting.
- Squash 0.85 and stretch 1.2 with volume held constant.
- Keep one core-loop lap to 20 to 60 s with a visible number change at every step.
- Build the stack bottom up: core loop, session, daily, weekly, long term.
- First 60 seconds: play first, first reward by 20 s, first purchase by 60 s, no shop popups.
- Give each event 1 to 3 sound layers with random pitch 0.9 to 1.1.

## Rule notes these pictures follow
- [[09-Animation-VFX/game-feel-juice]]
- [[02-Game-Design/core-loop-template]]
- [[02-Game-Design/loop-stack]]
- [[06-Retention/day-1-day-7-day-30-playbook]]
- [[06-Retention/session-length-levers]]
- [[11-Audio/sound-feedback-map]]
- [[11-Audio/satisfying-sfx]]

## Game feel timing chart
![[attachments/examples/game-feel-timing-chart.svg]]
*Five lanes over 0 to 300 ms: input, flash, hit-stop, shake, squash.*

What it teaches:
- Layers overlap; none is large alone.
- Numbers match the timing table in the rule note.
- Reduced motion swaps shake for a fade.

## Core loop
![[attachments/examples/core-loop-diagram.svg]]
*Do, earn, spend, grow, unlock in a ring.*

What it teaches:
- Each lap is short and shows a bigger number.
- Unlock opens a goal outside the loop.
- Merge or cut any boring step.

## Retention loop stack
![[attachments/examples/retention-loop-stack.svg]]
*Five nested loops from seconds to months.*

What it teaches:
- A weak core loop cannot be saved by daily rewards.
- All rewards free, honest and kind.
- Add layers only after the lower one works.

## First 60 seconds
![[attachments/examples/onboarding-first-60-seconds.svg]]
*Timeline with six milestones and three phases.*

What it teaches:
- Learn by moving, then loop clicks, then hook.
- No popups, shop or favorite asks in minute one.
- Test with a new player and a stopwatch.

## Sound feedback map
![[attachments/examples/sound-feedback-map-chart.svg]]
*Event rows with transient, body and tail layers.*

What it teaches:
- Bigger moments add layers and length.
- Duck music under stingers.
- Mute and volume always work.

## How to use these pictures
- Open the note, read the badges on each picture, then compare your own screenshot side by side.
- Keep the SVG files as the reference; redraw in Figma or Studio with your own art.
- If a picture and a rule note disagree, the rule note wins; fix the picture.

## For our Roblox games
- Copy: the principle shown (layout zones, timings, counts, order of steps).
- Never copy: names, art, logos, characters or music from real games. Every picture here uses our own shapes.
- Zero spend: all of this is built with free tools; no paid assets or ads.
- Children play: no pressure timers, no shame copy, odds always shown, purchases only from a clear player tap.
