---
description: How to build music for a Roblox game: seamless loops, layering by intensity, ducking under key sounds, free CC0 and Roblox-library sources, and starting volume values in dB and Roblox Volume.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: music, loop, adaptive audio, layers, stems, ducking, crossfade, volume, decibel, SoundGroup, AudioFader, CC0, intensity, mix levels
sources: create.roblox.com/docs/sound/groups and class SoundGroup (search summary, read 2026-11-14: group volume multiplies sound volume, range 0-10, SoundGroup discouraged in favor of audio objects); general game audio practice (loudness, ducking, stems), model knowledge
verify: Roblox docs pages could not be opened; only search summaries. dB to Volume mapping is a rule of thumb (Volume 1.0 treated as 0 dB reference); UNVERIFIED that Roblox Volume is linear gain. Check the Audio API objects (AudioPlayer, AudioFader) in Studio. Free-source licences must be rechecked per file.
tags: [audio, playbook]
---
# Music and adaptive audio

Music sets pace and mood and must never fight sound effects. Effects live in [[11-Audio/sound-feedback-map]] and [[11-Audio/satisfying-sfx]]; rules in [[11-Audio/audio-design-rules]]; sources in [[11-Audio/free-sound-sources]].

## Do this
- Music bus at about -18 to -14 dB relative to peak effects; start with Roblox Volume 0.3-0.5 on music, 0.6-1.0 on effects, then adjust by ear on a phone speaker and headphones.
- Loop length 60-120 s for gameplay, 8-30 s for menus and shops. Loop points on a bar line, no silence at either end, no reverb tail cut. Test the loop 5 times in a row.
- Layer by intensity: 3 stems (base pad or drone, rhythm, lead or tension), same tempo and key, same length, starting together.
- Fade layers over 1.0-2.0 s (rise) and 2.0-4.0 s (fall). Never hard-cut music except for a deliberate hit.
- Duck music by 6-10 dB (Volume x0.5 to x0.3) for 0.3 s before a rare reveal and hold 1-2 s; recover over 0.5-1.0 s.
- Give players a music slider and an SFX slider, saved per player. See [[11-Audio/audio-settings-menu]].
- Use only CC0 music, Roblox audio library items marked free, or music we generate. Record the licence and URL for each track.
- Keep file count low: 1 menu loop, 2-3 zone loops, 1 boss loop, 3-5 stings per game.

## Layering by intensity
| State | Layers on | Trigger |
|---|---|---|
| Menu or lobby | base only, soft | In lobby |
| Explore | base + rhythm | Normal play |
| Tension | base + rhythm + tension | Timer under 30 s, low health, enemy near |
| Climax | all + lead | Boss, final stretch, event |
| Victory | sting then base | Win |
| Defeat | low sting, drop layers | Lose |
Switch by changing the Volume of each layer (tween), not by stopping and starting tracks, so they stay in sync. Start all layers at the same time with TimePosition aligned.

## Ducking
- Rule: a high-priority sound lowers lower-priority buses while it plays.
- Order of priority: reward and warning stings, then voice or prompts, then SFX, then ambience, then music.
- Implementation options in Roblox (UNVERIFIED which is best): tween the music SoundGroup or layer Volume to 30-50% on trigger and back; or use the audio objects (fader or compressor with sidechain) if available in the API.
- Duck length equals the sound length plus 0.2 s. Do not duck for frequent sounds such as coin pickups; only for sounds longer than 0.5 s that matter.

## Volume values (starting points)
| Item | dB vs peak | Roblox Volume |
|---|---|---|
| Music, gameplay | -18 to -14 | 0.3-0.5 |
| Music, menu | -20 | 0.25-0.4 |
| Ambience | -24 to -20 | 0.2-0.35 |
| Footsteps, small UI | -20 to -14 | 0.3-0.5 |
| Frequent pickups | -12 to -9 | 0.5-0.7 |
| Rare rewards, level up | -6 to -3 | 0.8-1.0 |
| Warnings and alarms | -6 | 0.8-1.0 |
Conversion: gain = 10^(dB/20), so -6 dB = 0.5, -12 dB = 0.25, -18 dB = 0.125. Volume 1 as reference only if the file itself is normalised near -3 dB peak. Roblox allows Volume above 1 on groups (range 0-10); avoid it, it can clip.

## Free sources
- CC0 libraries: OpenGameArt (filter CC0), Kenney audio packs (CC0), freesound.org (filter CC0 only, check each file), Incompetech is CC-BY (needs credit, so skip unless we credit).
- Roblox Creator Store audio marked free to use; check the licence per asset and the moderation status.
- Generated music: our own tracks from free tools in [[11-Audio/audio-pipeline]].
- Never use commercial songs, game rips or YouTube music.

## Making a seamless loop
1. Pick a tempo and write in whole bars (for example 16 bars at 120 BPM is 32 s).
2. Export so the tail of the last bar overlaps the start (or render with the reverb tail copied to the start).
3. Trim to an exact bar boundary, zero-crossing cuts.
4. Normalise to about -3 dB peak, loudness about -16 to -14 LUFS for music (rule of thumb).
5. Set Looped = true, test with headphones and phone speaker.

## Mistakes
- Music louder than effects, masking the reward sound.
- Loops with an audible gap or click.
- Many long tracks that bloat memory and take long to load; stream or keep loops short.
- Same loop for hours with no variation; add a second track or a layer that changes by zone.
- Ducking on every sound so music pumps.

## For our Roblox games
Copy: the structure (stems, intensity states, ducking order, level table). Never copy: any game's soundtrack, melodies, stings or recognisable chord hooks, and never use music from commercial games or artists; credit CC-BY sources if used at all.
