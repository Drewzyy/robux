---
description: Table of 45 game actions mapped to sound character, length in ms, pitch variation and when to skip sound, for Roblox gameplay, UI and rewards.
type: reference
status: draft
last_checked: 2026-11-14
keywords: sound map, sfx, feedback, length ms, pitch variation, playbackspeed, ui sounds, reward sound, footsteps, notification, silence, audio fatigue
sources: General game audio practice (UI sounds short and soft, pitch randomisation 5-10%, rising combo pitch), model knowledge; existing vault notes satisfying-sfx and audio-design-rules (read 2026-11-14)
verify: UNVERIFIED: all lengths and ranges are starting values from common practice, not measured. Tune in Studio. PlaybackSpeed ratio for a semitone is 2^(1/12) = 1.0595.
tags: [audio, reference]
---
# Sound feedback map

Starting values for what to play on each action. Method and layering in [[11-Audio/satisfying-sfx]]; levels in [[11-Audio/music-and-adaptive-audio]]; pairing with UI motion in [[08-UI-UX/ui-sound-pairing]].

## Do this
- Give every action that changes state or money a sound; skip sounds for things the player does over 5 times per second.
- Keep UI sounds 40-150 ms, gameplay impacts 80-300 ms, big rewards 600-2000 ms.
- Pitch variation: random PlaybackSpeed 0.92-1.08 for repeated sounds (about +/-1.4 semitones); none for stings that must stay identical.
- Use rising pitch for streaks: step x1.0595 per step, up to 8 steps, reset after 1.5 s idle.
- Play each sound at the frame of the action. If latency is over 100 ms, play it locally on the client and let the server confirm.
- Limit overlap: max 4 instances of the same sound at once; stop the oldest.
- Never spam: min 60 ms between plays of the same pickup sound.
- Every error gets a short low sound (not a buzzer longer than 200 ms).

## Map (character, length, pitch, skip when)
| # | Action | Sound character | Length ms | Pitch variation | Skip when |
|---|---|---|---|---|---|
| 1 | Button hover (desktop) | Faint tick | 30-50 | +/-5% | On touch devices |
| 2 | Button press | Soft click | 50-90 | +/-5% | Never |
| 3 | Back or close | Lower soft click | 60-100 | +/-3% | - |
| 4 | Tab switch | Light tick-swoosh | 60-100 | +/-5% | Rapid switching under 100 ms |
| 5 | Menu open | Short rising whoosh | 150-250 | none | On join before first interaction |
| 6 | Menu close | Short falling whoosh | 100-180 | none | - |
| 7 | Toggle on | Higher click | 60-100 | none | - |
| 8 | Toggle off | Lower click | 60-100 | none | - |
| 9 | Slider drag | Tiny tick per step | 20-40 | step-based | Continuous drags: tick every 10% |
| 10 | Text input key | Soft key tick | 20-40 | +/-8% | Mobile keyboards |
| 11 | Error or denied | Low dull thud-buzz | 120-200 | none | Repeated within 500 ms |
| 12 | Purchase success | Bright register chime | 400-800 | none | - |
| 13 | Purchase fail | Soft low double tone | 200-300 | none | - |
| 14 | Coin pickup | Bright short ping | 80-150 | rising per streak | Over 5 per second: group into one |
| 15 | Gem or rare pickup | Crystal chime with tail | 300-600 | +/-3% | - |
| 16 | Currency tick up | Tiny tick | 20-40 | rising | Counters over 1 s long: sample every 100 ms |
| 17 | Item equip | Cloth or metal clip | 100-200 | +/-6% | - |
| 18 | Item unequip | Softer clip | 80-150 | +/-6% | - |
| 19 | Inventory full | Dull thud | 150-250 | none | Repeated within 1 s |
| 20 | Crafting or upgrade success | Hammer plus chime | 400-900 | none | - |
| 21 | Level up | Rising arpeggio sting | 1000-1800 | none | - |
| 22 | XP gain | Quiet tick | 30-60 | +/-5% | Frequent micro gains: batch |
| 23 | Quest complete | Positive sting | 700-1200 | none | - |
| 24 | Achievement | Fanfare short | 1200-2000 | none | Several at once: queue |
| 25 | Daily reward claim | Warm chime then coins | 800-1500 | none | - |
| 26 | Egg shake | Low rumble pulses | 100-200 per pulse | rising per pulse | - |
| 27 | Egg crack | Crack plus riser | 300-500 | none | - |
| 28 | Common reveal | Short soft pop | 300-500 | +/-5% | - |
| 29 | Rare reveal | Swell then chime | 1200-2500 | none | After 20 opens in a row, shorten |
| 30 | Legendary reveal | Big riser, hit, choir pad | 2500-4000 | none | Auto-open mode |
| 31 | Jump | Light puff | 80-150 | +/-8% | Obby with constant jumps: lower volume 30% |
| 32 | Land | Soft thud, surface-based | 80-160 | +/-8% | Short falls under 2 studs |
| 33 | Footstep | Surface tap | 50-120 | +/-10% | Distance over 60 studs |
| 34 | Dash or sprint | Quick whoosh | 150-300 | +/-5% | - |
| 35 | Melee swing | Air swish | 120-250 | +/-6% | - |
| 36 | Melee hit | Thump plus click | 100-250 | +/-8% | - |
| 37 | Ranged shot | Short pop or zap | 80-250 | +/-5% | Automatic fire over 10 per second: loop |
| 38 | Reload or switch | Mechanical click | 150-400 | +/-3% | - |
| 39 | Take damage (self) | Low thud plus short tone | 100-250 | +/-5% | Damage over time ticks: play every 500 ms |
| 40 | Death or fail | Falling tone, soft | 500-1200 | none | - |
| 41 | Respawn | Rising shimmer | 300-600 | none | - |
| 42 | Checkpoint | Light bell | 300-500 | none | Obby fast sections |
| 43 | Timer warning (last 10 s) | Tick per second, higher each | 60-100 | rising | Outside last 10 s |
| 44 | Round start | Short horn or beep | 300-700 | none | - |
| 45 | Round win | Fanfare | 1500-3000 | none | - |
| 46 | Notification toast | Soft ping | 150-300 | none | During cutscene or reveal |
| 47 | Friend joined | Gentle pop | 200-400 | none | In combat |
| 48 | Chat message | Faint tick | 30-60 | none | Default off; busy servers |
| 49 | Door or gate open | Mechanical creak or whoosh | 300-800 | +/-4% | - |
| 50 | Ambient loop | Quiet bed | loop | n/a | Indoors vs outdoors switch by zone |

## When to skip sound (rules)
- Spam events: more than 5 per second, replace with one aggregated sound.
- Distance: past hearing range, do not play and do not even create the Sound.
- Overlap with a bigger sound: skip small ones for 300 ms around a reveal or sting.
- Passive background changes the player did not cause (server ticks, leaderboard updates).
- Menus opening on join, before the player has done anything.
- Repeated errors: first plays, the next ones within 500 ms stay silent.
- Players with sounds muted: respect settings; do not play through a bypass.

## Mistakes
- Same sample every time, no variation.
- Late sounds (over 100 ms after action).
- Rewards quieter than ambience.
- Loud sounds for common actions.
- Unbounded sound instances for many players; cap and pool Sound objects.

## For our Roblox games
Copy: the structure (short, varied, rising on streaks, skipped when spammy) and the table's starting values. Never copy: any game's actual sounds, stings, jingles or recorded voices; use only CC0, Roblox free library or self-made audio ([[11-Audio/free-sound-sources]]).
