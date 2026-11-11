---
description: Feedback sounds, music, mixing with SoundGroups, and using only free, licensed audio.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/performance-optimization
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/performance-optimization
---
# Audio design rules

## What needs a sound
Every player action that matters: button press, pickup, reward, level up, hit, error, purchase prompt. Silence on a reward feels like a bug.

## Mixing
- Put sounds in `SoundGroup`s (Music, SFX, UI) so volumes can be balanced and players can mute music alone.
- Music lower than SFX. Rewards a little louder than ambient.
- Positional sounds: parent the Sound to a part so it fades with distance; UI sounds: parent to SoundService or PlayerGui.

## Licensing (free only)
- Use sounds Andrew generates and uploads (see [[11-Audio/forgegui]] and [[11-Audio/audio-pipeline]]) or Roblox-provided free audio in the Creator Store.
- Never use copyrighted music. Never buy audio.
- Agents list needed sounds in `sounds.json`; Andrew fills the asset IDs.
