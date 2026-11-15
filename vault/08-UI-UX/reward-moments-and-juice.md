---
description: The four-beat reward reveal (anticipation, reveal, celebration, collection) with timings, sound and particle rules, scaled for small free rewards and big paid ones, plus skip and reduced-motion rules.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: reward reveal, juice, anticipation, celebration, particles, sound, tween, collect animation, chest open, level up, skip button, reduced motion, feedback timing
sources: creator-docs content/en-us/production/game-design/season-pass-design.md (read 2026-11-15); creator-docs content/en-us/production/game-design/monetization-foundations.md (read 2026-11-15); common game-feel practice, paraphrased
verify: Timings are studio starting values from general game-feel practice, UNVERIFIED for our audience; playtest and trim. Docs support showing 3D rewards in person, not 2D icons only.
tags: [ui-ux, playbook]
---
# Reward moments and juice

Every reward, free or paid, gets the same four beats. Only the size changes. Same care for a free daily coin as for a bought pet.

## Do this
- Always run the four beats in order: anticipation, reveal, celebration, collection.
- Scale by rarity: small reward 0.6 to 1 s total, medium 2 to 3 s, rare 4 to 6 s, never more than 6 s.
- Add a Skip or tap-to-fast-forward on anything over 2 s. Repeat rewards shorten automatically (second chest in a row: half length).
- Show the item large and readable. Rewards the player can wear or place should appear in 3D on them, not just as an icon (the docs advise showing 3D rewards in person).
- Pair every beat with a sound (see [[08-UI-UX/ui-sound-pairing]]). Mute respects the player setting.
- Never block movement during a free reward under 1 s. For longer ones, protect the player (no damage during a chest open).
- Respect reduced motion: swap shake and flash for a fade and a short glow (see [[08-UI-UX/accessibility]]). No flashing faster than 3 times a second.

## The four beats
| Beat | Small (free, 0.8 s) | Rare (paid or big, 5 s) | Sound | Particles |
|---|---|---|---|---|
| 1 Anticipation | 0.1 s object wobble | 1.2 s: chest shakes, glow grows, low rise sound | Rising tone | Dust, light motes, low count |
| 2 Reveal | 0.15 s pop with scale 0.6 to 1.1 to 1 | 0.5 s flash (soft, not white strobe), item scales 0 to 1.2 to 1 | Pop or chime, pitch by rarity | One burst, 20 to 40 particles |
| 3 Celebration | 0.2 s sparkle | 1.5 s: name and rarity text, camera nudge, confetti | Short fanfare, 1 to 2 s | Confetti 60 to 100, fades by 1.5 s |
| 4 Collection | 0.3 s fly to counter, counter ticks | 0.8 s item flies to inventory icon, icon bumps, "New" dot | Soft whoosh plus tick | Trail 10 particles |

## How to build it in Roblox
- UI side: `TweenService` with `Enum.EasingStyle.Back` for pop, `Quad` out for fly. Use `UIScale` for scale, `CanvasGroup` for fades.
- 3D side: `ParticleEmitter:Emit(count)` for a burst, so you control the count; keep total particles on screen under 150 on mobile.
- Sound: `SoundService` groups, a `SoundGroup` for UI, volume under music's loudest part, pitch varied by +/- 5 percent so repeats do not grate.
- Server grants first, client plays the animation. Never make the player wait for the animation to get the item.
- Trigger the reveal from a RemoteEvent with the item id, so paid and free rewards use one code path.
- Preload assets with `ContentProvider:PreloadAsync` before the first reveal.

## Counter ticks and collection
- Currency counters count up over 0.4 s with a tick sound every few units; do not jump instantly.
- Items land on the HUD icon with a bump of 1.0 to 1.15 scale over 0.2 s.
- "New" dot stays until viewed. It is a badge, not a popup (see [[07-Monetization/offer-pressure-limits]]).

## Fair vs exploitative
- Fair: celebrate what the player actually got; show the real rarity; same celebration for free and paid items of the same rarity; skip allowed.
- Exploitative (never): slot-machine near-miss reveals that show a rare then swap it; fake "almost won" screens; long unskippable reveals to push paid skips; flashing lights; making free rewards visually sad to push buying.

## Quick test
Mute the game and watch: can you still tell a reward happened? Then turn the screen off and listen: can you still tell? If both work, the beats are clear.

## Mistakes
- Same animation for everything, so nothing feels special.
- Over 6 s with no skip, which turns joy into a chore.
- Loud pops on every coin pickup. Use pitch and low volume for frequent rewards.
- Rewards hidden behind UI that covers the screen on phones. See [[08-UI-UX/mobile-first-checklist]].

## Related
[[08-UI-UX/ui-animation]], [[08-UI-UX/ui-polish-passes]], [[07-Monetization/rewarding-purchases]], [[04-Onboarding/fast-early-rewards]], [[03-Game-Psychology/reward-schedules]]

## For our Roblox games
Copy the principle: anticipation, clear reveal, celebration, then a satisfying collect, with the same care for free rewards. Never copy another game's exact animations, sounds, music, characters or logos, and keep paid reveals as honest as free ones.
