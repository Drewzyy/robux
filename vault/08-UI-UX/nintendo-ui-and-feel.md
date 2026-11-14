---
description: Nintendo UI and feel principles (Animal Crossing, Mario Kart, Zelda BOTW/TOTK, Splatoon) as rules for Roblox menus: sound on every input, bouncy motion, big icons, teaching without text.
type: reference
status: draft
last_checked: 2026-11-14
keywords: nintendo, ui, animal crossing, mario kart, zelda, breath of the wild, splatoon, menu, sound feedback, bounce, icons, tutorial, onboarding
sources: GDC 2017 talk "Breaking Conventions with The Legend of Zelda: Breath of the Wild" (title only, not re-read); Nintendo public interviews (general knowledge, not re-read); Animal Crossing and Splatoon UI observed from public footage
verify: UNVERIFIED: no primary source re-read on 2026-11-14. Millisecond values are our suggested starting points, not Nintendo numbers.
tags: [ui-ux, reference]
---
# Nintendo UI and feel
Nintendo menus feel like toys: every touch answers with sound and motion, icons are huge, and text is a last resort.

## Do this
- Every input gets feedback within 50 ms: a sound on hover/focus, a different one on press, another on confirm and back. No silent button in the game. See [[11-Audio/satisfying-sfx]].
- Animate with a small overshoot: scale 1.0 to 1.1 to 1.0 in about 180 to 250 ms (Back or Elastic easing) when something appears or is selected.
- Use big, rounded icons (at least 48 px on a 1080p layout, 44 dp minimum touch target) with a clear single color per meaning. Pair each icon with one or two words, not sentences.
- Teach by doing: first action available in 10 seconds, shown by a pointing arrow, a highlighted object, or a character doing it. Text appears only as a short label after the player acts.
- Keep menus shallow: root screen max 6 to 8 tiles, back is always the same button, state is remembered when reopening.
- Give the world a voice: a character reacts (nod, hop) when you finish a menu action. Reactions replace plain confirmation dialogs.
- Use one consistent shape language (rounded rectangles, thick outlines) so the whole UI looks like one object.

## Examples by game
- Animal Crossing: a pocket device with large icons; every tap has a soft pop and a short animated reaction; the music and menu change with time and season; talking uses a light voice blip per letter, which makes text feel alive.
- Mario Kart: huge position number, item box roulette that spins and clicks (anticipation before reward), countdown with beeps, rubber-band feel so many racers stay in view.
- Zelda BOTW/TOTK: almost no HUD in the field; hearts and stamina wheel appear when used; the map is a tool you open on purpose; the sheikah-style sounds confirm every selection; tutorial is a place (the plateau) not a text wall.
- Splatoon: menus are a plaza where players walk to a shop or a battle door; squid-ink color is both style and rule (your ink is your territory), so the UI explains the game.

## Why it works
- Feedback on every input proves the game heard you, which lowers hesitation for young players.
- Bounce and overshoot signal "friendly" and draw the eye to what changed.
- Big icons and few words cut reading load, helping ages 7 and up and non-English players.
- Learning by doing keeps the first minute active, which links to [[03-Game-Psychology/player-psychology]].

## How to build it in Roblox
- A shared `UIFeedback` module: `onHover(btn)`, `onPress(btn)`, `onConfirm(btn)` that play a Sound and tween scale with `TweenService` (Back easing). Reuse across all buttons so none are forgotten.
- Pitch variation: randomize PlaybackSpeed 0.95 to 1.05 so repeated clicks do not fatigue.
- Item roulette: spin icons, slow with Quad easing over 1.5 to 2.5 seconds, click on each pass, a longer pause before landing.
- Sound budget: 4 to 6 UI sounds total (hover, press, confirm, back, error, reward). More becomes noise.
- Controller and touch: focus highlight must be as visible as a mouse hover. See [[08-UI-UX/ui-animation]] and [[08-UI-UX/hud-template]].

## Mistakes
- Bounce on everything, constantly: reserve it for state changes.
- Tiny icons with long captions on phones.
- Silent errors: a failed action needs a gentle low sound and a shake of about 8 px over 200 ms.
- Copying look rather than principle.

## For our Roblox games
Copy the principles: sound plus motion on every input, overshoot easing, large icons, learn-by-doing, a world that reacts. Never copy: Nintendo characters, item boxes, ink art, UI sound files, fonts, logos or music; use free or self-made sounds only. Related: [[03-Game-Psychology/lessons-from-classic-games]].

## Quick checklist before shipping a menu
- Does every clickable thing make a sound and move?
- Can a 7 year old find the main button in 2 seconds?
- Is there any paragraph of text? Cut it to a label or an icon.
- Does the back button sit in the same place on every screen?
- Do the first three actions teach themselves without a pop-up?
- Does the UI still work with a thumb on a 6 inch phone and with a gamepad?

## Timings table (our starting points, tune in Studio)
| Moment | Duration | Easing |
|---|---|---|
| Hover or focus grow | 120 ms | Quad Out |
| Press squash to 0.92 | 80 ms | Quad Out |
| Release overshoot to 1.08 then 1.0 | 200 ms | Back Out |
| Panel slide in | 250 ms | Back Out |
| Panel slide out | 150 ms | Quad In |
| Error shake | 200 ms | Sine |
