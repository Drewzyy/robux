---
description: Map of notes on animations, particles, tweens, hit feedback, emotes and reward moments, with cheap-on-phones rules.
type: reference
status: draft
last_checked: 2026-11-15
keywords: animation, vfx, particles, beams, trails, tweens, hit feedback, emotes, juice, combat animation, effects, polish
sources: vault notes only, read 2026-11-15 (front matter of each linked note)
verify: Built from note descriptions; each linked note carries its own source and verify line.
tags: [start-here, reference]
---
# Animation and Effects
Use when something must move, flash, shake or celebrate.

## Do this
- Read first, step 1: [[09-Animation-VFX/game-feel-juice]] (timings for shake, hit-stop, squash).
- Read first, step 2: [[09-Animation-VFX/vfx-guide]] (what each effect is for).
- Read first, step 3: [[09-Animation-VFX/animation-basics]] (how animation playback works).
- Then pick the section below that matches your task and read only those notes.

## How to use this map
- Read the three notes under Do this, then open one section below.
- Each note has its own Do this list; act on that, not on this page.
- If a note is marked UNVERIFIED, tune from your own analytics.

## Basics
- [[09-Animation-VFX/animation-basics]]: Animation Editor, Animator, priorities.
- [[09-Animation-VFX/free-animation-library]]: the 17 official animation packs.
- [[09-Animation-VFX/character-movement-animations]]: idle, walk, run, jump, land.
- [[09-Animation-VFX/index]]: section index.

## Effects
- [[09-Animation-VFX/vfx-guide]]: particles, beams, trails, highlights.
- [[09-Animation-VFX/vfx-sfx-library]]: effect and sound pairs per action.
- [[09-Animation-VFX/character-effects-library]]: every player effect by purpose.
- [[09-Animation-VFX/building-and-world-animation]]: build, place, spawn, collect.

## Combat feedback
- [[09-Animation-VFX/combat-animation]]: anticipation, active, recovery, hitstop.
- [[09-Animation-VFX/combat-animation-and-hit-feedback]]: hit feedback recipe.
- [[02-Game-Design/combat-design-basics]]: attack, defense, telegraphs.
- [[16-Market-Research/teardown-elden-ring]]: combat animation feel lessons.

## Personality and social
- [[09-Animation-VFX/emotes-and-idle-personality]]: emote wheel, group emotes.
- [[12-Multiplayer-NPCs/pets-and-followers]]: follower smoothing.

## UI and reward moments
- [[08-UI-UX/ui-animation]]: durations, easing, press feedback.
- [[08-UI-UX/reward-moments-and-juice]]: anticipation, reveal, celebration, collection.
- [[07-Monetization/currency-satisfaction]]: count-up currency animations.
- [[07-Monetization/rewarding-purchases]]: make a purchase feel great.

## Pair with sound and access
- [[08-UI-UX/ui-sound-pairing]]: which sound goes with which UI action.
- [[11-Audio/satisfying-sfx]]: why some sounds feel good.
- [[08-UI-UX/accessibility]]: reduced motion and transparency options.

## Code for motion
- [[13-Luau-Engineering/tween-and-animation-cleanup]]: cancel old tweens, clean up.
- [[13-Luau-Engineering/object-pooling]]: reuse effect instances.
- [[14-QA-Optimization/optimization-checklist]]: budget for effects.

## For our Roblox games
- Copy the rule: every action gets feedback in under 100 ms and effects stay cheap. Never copy another game's animation assets or effect art.
- Offer reduced motion (see accessibility notes) and avoid flashing effects.
- Use only the free official animation packs or your own.
