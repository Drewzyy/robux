---
description: Cross-game game feel rules with millisecond timings: screen shake, hit-stop, squash and stretch, particles, number popups, sound layering, for Roblox combat, collect and UI feedback.
type: reference
status: draft
last_checked: 2026-11-14
keywords: game feel, juice, screen shake, hit-stop, hit pause, squash and stretch, particles, damage numbers, sound layering, easing, feedback, vlambeer
sources: "The Art of Screenshake" talk by Jan Willem Nijman, Vlambeer, 2013 (referenced in search results read 2026-11-14, talk video not watched); "Juice it or lose it" by Martin Jonasson and Petri Purho, 2012 (title only); https://gamedeveloper.com/design/vlambeer-co-founder-shares-advice-on-building-better-action-games (snippet only)
verify: UNVERIFIED: all millisecond values are common starting points from practice and memory, not from a single source. Tune by playtest; less is better on mobile.
tags: [animation-vfx, reference]
---
# Game feel and juice
Juice is feedback that makes actions feel heavy, certain and rewarding. Layer small effects; each alone is subtle.

## Do this
- Respond to input within 100 ms. Show or play something in the first frame (a sound, a flash, a move).
- Hit-stop: freeze or slow the attacker and target to about 5 percent speed for 50 to 80 ms on normal hits, 100 to 150 ms on heavy hits or kills. Never more than 200 ms.
- Screen shake: amplitude 0.1 to 0.4 studs (small), 0.5 to 1 (big), duration 100 to 250 ms, decaying fast. Add a setting to turn it off, and never shake during UI text.
- Squash and stretch: stretch 1.2 on the motion axis for 80 ms on jump/launch, squash 0.85 for 100 ms on land, back to 1.0 in 150 ms. Keep volume roughly constant (stretch one axis, shrink the others).
- Particles: 6 to 15 on a normal hit, 25 to 40 on a kill, lifetime 0.2 to 0.6 s. Reuse emitters via :Emit(n), do not create new ones per hit.
- Number popups: rise 1 to 2 studs over 600 to 800 ms, fade the last 200 ms, scale 1.3 to 1.0 in the first 100 ms. Bigger font for crits; color by type; pool the labels, show at most 20 at once.
- Sound layering: 3 layers per hit: a sharp transient (click), a body (thump), a tail or sweetener (sparkle on crit). Random pitch 0.9 to 1.1 so repeats vary. See [[11-Audio/satisfying-sfx]].

## Why it works
- Feedback confirms cause and effect, so players trust controls.
- Weight comes from time: stopping for a few frames makes impacts read as heavy.
- Exaggeration makes cartoon games read clearly on small phone screens.
- Many tiny cues together feel rich; one big cue feels like a gimmick. See [[03-Game-Psychology/player-psychology]].

## Timing table
| Effect | Duration | Notes |
|---|---|---|
| Button press squash | 80 ms | scale 0.92 |
| UI pop in | 200 ms | Back Out, overshoot 1.1 |
| Hit flash (white) | 60 to 100 ms | Highlight or material swap |
| Hit-stop light | 50 ms | |
| Hit-stop heavy | 100 to 150 ms | |
| Screen shake small | 120 ms | decay |
| Screen shake big | 250 ms | |
| Coin collect fly to HUD | 400 to 600 ms | curved path, ease in |
| HUD counter tick | 250 ms | count up with scale pulse |
| Level-up banner | 1.5 s | sound and burst |
| Damage number life | 700 ms | |

## Pattern: a sword hit, in order
1. Frame 0: sound transient plays; attacker anticipation already done earlier.
2. Frame 0: target flashes white 80 ms and is knocked back 1 to 2 studs.
3. Frame 0 to 3: hit-stop 60 ms.
4. Particles burst, number pops, camera nudges 0.2 studs.
5. After 150 ms: body sound tail, enemy recovers or dies with a longer effect.

## How to build it in Roblox
- Camera shake: offset Humanoid.CameraOffset or a CFrame offset in a RenderStepped bound after the camera, with exponential decay. See roblox-camera and roblox-animation-vfx.
- Hit-stop: set AnimationTrack:AdjustSpeed(0.05) on attacker and target, then restore after the delay with task.delay. Keep server hit logic separate so lag is not added to damage.
- Squash and stretch: tween Size or a Motor6D scale on the visual part only, never the collision root.
- Number popups: BillboardGui pooled; tween Position and TextTransparency.
- Mobile: halve particles and skip shake by default on low-end devices; see roblox-performance.
- Client-side feedback instantly, server confirms later; roll back visual if rejected.

## Mistakes
- Shake plus blur plus flash all at once on every hit: nausea and noise.
- Hit-stop on the whole game, which freezes other players on the server view.
- Long number animations that cover the enemy.
- Effects that hide hit readability (see [[16-Market-Research/teardown-elden-ring]]).
- No accessibility option for motion.

## For our Roblox games
Copy the principles and the timing ranges. Never copy: another game's particle art, sounds, damage font or branded effects. Use free or self-made assets only. Related: [[09-Animation-VFX/animation-basics]], [[08-UI-UX/ui-animation]], [[08-UI-UX/nintendo-ui-and-feel]], [[16-Market-Research/teardown-vampire-survivors-and-roguelikes]].
