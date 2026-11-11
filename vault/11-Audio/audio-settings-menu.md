---
description: Player audio settings: Master, Music, Effects, UI sliders with SoundGroups, saving them, and mute music.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: audio settings, volume, soundgroup, music slider, sfx slider, mute, settings menu, sound options
sources: https://create.roblox.com/docs/sound/groups (search summary); https://devforum.roblox.com/t/support-for-nested-soundgroups/1669856
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
---
# Audio settings menu

## Do this
1. In SoundService make SoundGroups: Master, with Music, Effects, UI, and Ambience under it.
2. Set every Sound's SoundGroup property. A sound with no group cannot be controlled.
3. Settings menu: one slider (0 to 1) per group, bound to that group's Volume. A Mute Music toggle sets Music to 0 and remembers the old value.
4. Save the four values with the player's data (on change, debounced 2 s); apply them on join before music starts.
5. Defaults: Music 0.5, Effects 0.8, UI 0.7, Ambience 0.6.
6. Roblox's own Escape-menu volume still applies on top; ours is a personal mix.

## Personalisation extras (cheap)
Music track picker for unlocked songs, a "calm mode" with softer impact sounds, and captions ([[08-UI-UX/accessibility]]).
