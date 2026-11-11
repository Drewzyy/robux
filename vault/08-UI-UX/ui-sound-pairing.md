---
description: Which sound goes with which UI action, how loud, and how to keep clicks satisfying instead of annoying.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: ui sound, click sound, button sound, satisfying, sfx, feedback, pitch
sources: https://devforum.roblox.com/t/the-audio-balance-in-most-roblox-games-is-terrible-but-it-doesnt-have-to-be/729433
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [ui-ux, playbook]
---
# UI sound pairing

## Do this
| Action | Sound | Notes |
|---|---|---|
| Button press | short soft click or pop (under 0.1 s) | random pitch 0.95 to 1.05 every play |
| Open panel | soft whoosh up | quieter than clicks |
| Close panel | whoosh down | |
| Purchase success | bright chime + coin jingle | the most satisfying sound in the game |
| Not enough currency | low muted thud | never harsh; show what is missing |
| Reward / level up | rising arpeggio, 0.5 to 1 s | pair with reward pop animation |
| Error | short low buzz | rare; do not punish |
- All UI sounds in a UI SoundGroup ([[11-Audio/audio-settings-menu]]).
- UI sounds start within 1 frame of the press; play them on the client.
- Same action = same sound everywhere. Players learn the language.
- Cap repeats: if the same sound plays more than 4 times a second, skip extras.
