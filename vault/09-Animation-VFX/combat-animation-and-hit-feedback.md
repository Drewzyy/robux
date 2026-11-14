---
description: Hit feedback recipe for Roblox combat: windup, active and recovery frames, hit stop, camera shake, sound, particles and damage numbers, with durations and a layering checklist.
type: reference
status: draft
last_checked: 2026-11-14
keywords: hit feedback, hit stop, hitstop, camera shake, animation frames, windup, recovery, damage numbers, impact vfx, juice, combat feel, markers
sources: Author working knowledge of animation and game feel (no pages opened 2026-11-14); Roblox Creator Hub docs, Animation Editor and AnimationTrack pages (titles only, not opened 2026-11-14); Swink, Game Feel (2008, from memory, not re-read)
verify: Durations are common starting points. UNVERIFIED as standards. Tune with players on phone and PC.
tags: [animation-vfx, reference]
---
# Combat animation and hit feedback

A hit should be seen, heard, and felt within 100 ms. Complements [[09-Animation-VFX/combat-animation]] and [[09-Animation-VFX/game-feel-juice]]; design rules live in [[02-Game-Design/combat-design-basics]].

## Do this
- Split every attack into windup, active, recovery. Example light attack: windup 0.12 s, active 0.08 s, recovery 0.25 s.
- Use animation markers (KeyframeMarkers) to trigger the hit and sounds at the active frame.
- Add hit stop: freeze or slow the attacker and victim animation 0.04-0.08 s on a hit; 0.10-0.15 s on a heavy finisher.
- Layer feedback at hit time: sound, particle spark, flash on the victim (white for 0.05 s), damage number, small camera nudge.
- Make heavy attacks bigger: longer windup, bigger effect, stronger shake (0.15 s), louder sound.
- Show whiffs too: a different swing sound and no spark so players read a miss.
- Use knockback of 3-8 studs for light hits, 12-20 for heavy ones with a quick ease-out.
- Keep effects short and readable; clutter hides telegraphs. See [[09-Animation-VFX/vfx-guide]].

## Why it works
- Impact feedback tells the brain the hit mattered, which raises satisfaction without changing balance.
- Windup readability makes combat fair. See [[02-Game-Design/pvp-fairness-and-balance]].
- Consistent timing lets skilled players learn rhythm.

## Feedback stack (apply in this order)
1. Animation hit frame reached (marker).
2. Sound plays within 1 frame.
3. Particle burst at contact point, 0.2-0.4 s.
4. Victim flash and hit-react animation.
5. Hit stop 0.05 s.
6. Camera shake, small amplitude, 0.1-0.2 s.
7. Damage number floats up for 0.6-0.8 s.

## How to build it in Roblox
- AnimationTrack:GetMarkerReachedSignal for the hit frame; the server validates the hit.
- Hit stop: AnimationTrack:AdjustSpeed(0) for 0.05 s then restore.
- Camera shake via a Humanoid.CameraOffset or a camera module with decay.
- Hit effect: pooled ParticleEmitters emitting 6-12 particles.
- Highlight object for flash, destroyed after 0.05-0.1 s.
- Damage numbers: BillboardGui tweened up and faded; cap 10 on screen.
- Client plays prediction effects instantly; server confirms damage.

## Numbers (starting points)
- Input to windup start: under 50 ms.
- Total light attack 0.45 s; heavy 0.8-1.2 s.
- Hit stop 0.05 s light, 0.12 s heavy.
- Shake amplitude 0.1-0.3 studs light, 0.5 heavy.
- Particle count under 30 per hit on mobile.

## Mistakes
- Long recovery that makes combos feel sluggish.
- Hit stop on everything, which feels laggy.
- Camera shake that makes phone players dizzy; add a toggle.
- No sound on hit.
- Effects that cover the telegraph of the next attack.

## For our Roblox games
Copy the stack: marker-triggered hit, short hit stop, layered feedback, heavy versus light contrast. Never copy another game's exact animations, effect art, sound clips, or move names. Make your own animations with free tools.
