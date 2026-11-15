---
description: How to mix Roblox audio with SoundGroups: mix tree, volume multipliers (0 to 10), nesting, CompressorSoundEffect ducking settings, and the legacy warning that audio objects are the newer path.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: soundgroup, mixing, ducking, compressor, sidechain, volume, nested soundgroups, soundservice, equalizer, reverb, audio mix, master volume, rolloff
sources: creator-docs content/en-us/sound/groups.md (read 2026-11-15); creator-docs content/en-us/sound/objects.md (read 2026-11-15); creator-docs content/en-us/sound/dynamic-effects.md (read 2026-11-15); creator-docs content/en-us/resources/the-mystery-of-duvall-drive/design-dark-soundscapes.md (read 2026-11-15); creator-docs content/en-us/resources/beyond-the-dark/sound-design.md (read 2026-11-15)
verify: All properties and ranges read in the docs 2026-11-15. SoundGroup is marked legacy in favor of audio objects (AudioPlayer, AudioFader and friends, docs page not in the clone). Our own starting values are suggestions: UNVERIFIED, tune by ear.
tags: [audio, playbook]
---
# Sound groups and mixing

A SoundGroup is a mixer bus for Sound objects. The docs flag it as legacy, with newer audio objects offering more control; SoundGroups still work and are simple to start with. Settings menu wiring is in [[11-Audio/audio-settings-menu]].

## Do this
- Create all SoundGroups under SoundService in one place and name them by role: Master, Music, Effects, UI, Ambience, Voice or Dialogue.
- Assign each Sound by setting its SoundGroup property. Parent-child location does not assign it, because a Sound's location only sets where it emits from.
- Nest groups into a mix tree (Master > Music, Effects, UI, Ambience). Raise or lower one branch to change everything under it.
- Use SoundGroup.Volume as a multiplier from 0 to 10 and keep it near 1. Example from the docs: a Sound at 0.5 under a group at 0.5 plays at 0.25.
- Duck low-priority audio with CompressorSoundEffect and its SideChain, so important sounds stay clear.
- Wire player volume sliders to the group volume, not to individual sounds ([[11-Audio/audio-settings-menu]]).
- Test the whole mix on phone speakers, headphones and a loud scene (combat plus music plus UI).

## Ducking recipe (CompressorSoundEffect)
1. Insert a CompressorSoundEffect inside the SoundGroup you want to duck (for example Music or Ambience).
2. Set its SideChain property to the high priority Sound or SoundGroup (for example UI notifications or dialogue).
3. Set Threshold (-80 full compression to 0 full volume), Attack and Release (0.001 to 1 second each) and Ratio (1 none to 50).
4. Playtest. The docs say values depend on the source audio; no fixed numbers. Start with a gentle ratio and a release near a quarter second (UNVERIFIED starting point).

## Priority tree idea
| Priority | Group | Ducked by |
|---|---|---|
| High | UI alerts, dialogue, key gameplay cues | nothing |
| Medium | Effects, footsteps | high |
| Low | Music, generic ambience | high and medium |

The Duvall Drive sample prioritized puzzle and key object sounds over decoration. The Beyond the Dark sample ducked a generic ambience bed when a local ambience (a waterfall) came close, so each area felt fresh.

## Effects you can add on a group or sound
- EqualizerSoundEffect: muffle everything underwater.
- ReverbSoundEffect: caves and big rooms.
- CompressorSoundEffect: even loudness and ducking.
- One effect on a shared group is cheaper than the same effect on many sounds (the docs mention lower CPU cost for common effects on nested groups).

## Placement basics from the Sound docs
- Child of a block, sphere or cylinder part: volumetric, emits from the whole surface; enable SoundService.VolumetricAudio.
- Child of an Attachment or MeshPart: point source.
- In SoundService or Workspace: global, same volume everywhere (music, UI).
- RollOffMinDistance and RollOffMaxDistance set where fading starts and ends; RollOffMode has Inverse, Linear, InverseTapered, LinearSquare.
- Set Looped for background beds so silence never appears.

## Mistakes
- Pushing volume to 10 to be heard. Fix the mix instead.
- Putting a Sound in a group but leaving the group's volume at 0.
- Many always-playing sounds each with their own effect.
- No mute path for music, which blocks players with sensory needs.

## For our Roblox games
Copy the principle: three or four buses, one priority rule, and sliders on buses. Keep UI and key gameplay cues always readable.

Never copy: another game's mix presets or audio assets. Use only our own or licensed sounds and no paid audio plugins.
