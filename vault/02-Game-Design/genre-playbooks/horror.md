---
description: Horror playbook for Roblox: tension curve, safe rooms, jump scare budget, lighting and sound rules, round structure for co-op or asymmetric horror, and the fairness rules that stop rage quits.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: horror, roblox horror, jump scare, tension, safe room, asymmetric, co-op horror, flashlight, chase, fear, pacing, audio
sources: Author working knowledge of Roblox horror hits and general horror design (no pages opened 2026-11-14); Roblox Creator Hub docs (create.roblox.com/docs, title only, not opened 2026-11-14)
verify: Principles are common design practice. All timings and counts below are starting points to playtest. No player-count or revenue figures used. UNVERIFIED: any claim about what currently top-charts.
tags: [game-design, playbook]
---
# Horror playbook

Horror on Roblox is short rounds, shared fear, and clips friends send each other. The core is a tension loop: calm, dread, scare, relief, repeat.

## Do this
- Build a tension curve per round: 60-90 s calm, 20-40 s rising dread (audio cue first), 5-15 s peak, 30-60 s relief. Never peak twice in a row.
- Cap hard jump scares at 1 per 2-3 minutes. Scares you can predict by sound beat scares you cannot predict by silence only once.
- Give a safe room or safe zone with no threat and a visible exit. Players need relief to feel fear again.
- Make the monster fair: it telegraphs (sound, light flicker, 1-2 s warning) before it can hurt. Death must feel like the player's mistake.
- Limit vision, not control: dark map plus a flashlight with a battery that lasts 90-180 s and recharges at pickups.
- Co-op rounds: 2-6 players, one clear objective (find 3 keys, reach the exit), with a downed and revive state instead of instant elimination.
- Respawn spectators into a useful role (watch cameras, ping the monster) so waiting costs under 60 s of boredom.
- Test with a 12-year-old and a 16-year-old: if the youngest quits crying, lower scare intensity, add a "calm mode" toggle.

## Why it works
- Fear shared with friends is social content: players stream, clip and invite.
- Short rounds give many small stories per session, which fits [[02-Game-Design/loop-stack]] and the short Roblox session.
- Uncertainty drives dread more than visuals. See [[03-Game-Psychology/curiosity-and-mystery]].

## How to build it in Roblox
- Lighting: set Lighting.Brightness low, ClockTime near 0 or fog (Atmosphere) dense, add a PointLight to the player flashlight only. See [[10-Art-Environment/lighting-and-mood]].
- Monster AI: PathfindingService with a patrol list plus a hearing radius (for example 40 studs, doubled when the player sprints).
- Use server authority for kills; the client only plays the scare effect.
- Audio: layered ambience loop, a heartbeat that rises as distance falls, silence before a scare. Use only free or self-made audio.
- Use StreamingEnabled-friendly small rooms so loading never reveals the monster early.
- Add a settings menu: reduce flashing, mute screams, camera shake off.

## Numbers (playtest starting points)
- Round length 8-12 min for co-op, 3-5 min for hide and seek.
- Monster speed 85-95 percent of player sprint so a chase is escapable with a good route.
- Flashlight battery 120 s, pickups 3-5 per map.
- Safe room count: 1 per 3-4 rooms.
- Revive time 4-6 s with a visible progress ring.

## Mistakes
- Constant noise: players go numb in under 2 minutes.
- Unfair instant death from out of sight.
- Scares that require voice chat to land.
- No way to quit a round gracefully, so players leave the game entirely.
- Gore for shock value; it hurts discovery and invites moderation trouble. Keep it implied.

## Metrics to watch
- Round completion rate (target above 60 percent). Below that, the monster is too lethal or the objective too vague.
- Early-leave rate in the first 90 s. High means the opening is either too scary or too empty.
- Revive count per round: zero means revives are too hard to reach, many means the monster is too easy to evade into.

## Playtest check
- Watch a stranger play the first 3 minutes silently; note where they stop moving. That is where fear turned into confusion.
- Count scares per round against the cap above.

## For our Roblox games
Copy the principles: tension curve, safe rooms, telegraphed threats, short rounds, accessibility toggles. Never copy another game's monster, map, names, jingle, jump-scare audio, or logo. Design an original creature with its own silhouette and sound set.
