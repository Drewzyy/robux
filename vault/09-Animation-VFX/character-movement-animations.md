---
description: Idle, walk, run, jump, fall, and land animations: replacing defaults, speed matching, priorities, and free sources.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: walk animation, run animation, idle, jump, movement animation, animate script, mixamo, moon animator, r15
sources: https://devforum.roblox.com/t/plugin-romixamo-use-mixamo-animations-in-roblox-easily-as-one-click/3655115 ; creation.dev animation plugins
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [animation-vfx, playbook]
---
# Character movement animations

## Do this
- Keep Roblox's default movement unless the game's identity needs a custom feel. Replacing is a polish step, not MVP.
- Replace by swapping the Animation IDs in the character's Animate script (idle, walk, run, jump, fall, climb, swim) on spawn. Andrew makes or uploads the animations; agents only wire IDs he provides.
- Match playback speed to movement: walk/run track speed = Humanoid speed / authored speed, so feet do not slide.
- Priorities: movement = Movement, idle = Idle, attacks and emotes = Action. Layer upper-body actions over running.
- Transition times: 0.1 to 0.2 s fade between movement tracks.

## Feel by genre
| Feel | Walk | Run | Jump |
|---|---|---|---|
| Cozy | slow bob, arms loose | light skip | floaty, longer hang |
| Action | tight, forward lean | strong lean, arm pump | quick crouch anticipation |
| Spooky | hunched | stumbling | short |

## Free sources (zero spend)
- Roblox Animation Editor (built in, free).
- Moon Animator 2 (listed as free by creation.dev; check the Creator Store before using).
- Mixamo (free, Adobe account): import FBX without skin into the Animation Editor; some clips do not fit Roblox rigs, test first. Paid one-click plugins exist; not needed.
