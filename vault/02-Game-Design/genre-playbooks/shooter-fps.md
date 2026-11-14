---
description: Shooter and FPS playbook for Roblox: weapon feel, TTK targets, map flow, spawns, loadout progression, aim assist for mobile, and server-authoritative hit detection to resist exploiters.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: shooter, fps, tps, weapons, ttk, spawn, loadout, hitscan, aim assist, map flow, recoil, team deathmatch, exploit, hit detection
sources: Author working knowledge of shooter design (no pages opened 2026-11-14); Roblox Creator Hub docs, Raycasting and RemoteEvent security pages (titles only, not opened 2026-11-14)
verify: TTK and recoil numbers are starting points; UNVERIFIED as any standard. Exploiter risk is real on shooters; test with two clients.
tags: [game-design, playbook]
---
# Shooter and FPS playbook

Fast matches, readable gunplay, fair spawns. Cover input for phones: a large share of Roblox players use mobile (UNVERIFIED share). See [[02-Game-Design/combat-design-basics]].

## Do this
- Set a time to kill (TTK) of 0.4-0.9 s for most weapons so skill matters but one mistake is not instant death.
- Give each weapon a role: automatic, burst, shotgun, sniper, with a clear tradeoff of range, rate, and handling.
- Starter loadout is free and decent; unlocks are sideways options, not strictly stronger. See [[02-Game-Design/pvp-fairness-and-balance]].
- Use maps with 3 lanes, cover every 8-15 studs, and no long dead sightlines. See [[02-Game-Design/world-and-level-design]].
- Spawn away from enemies and out of sightline; give 2 s spawn protection that ends when firing.
- Add hit markers, damage numbers, and headshot sounds. See [[09-Animation-VFX/combat-animation-and-hit-feedback]].
- Offer aim assist and auto-fire for mobile; keep it light so PC players do not feel disadvantaged.
- Matches 5-8 min (team deathmatch to 50 kills, or best of 3 rounds); show scoreboard and MVP.

## Why it works
- Instant skill feedback and clip-worthy moments.
- Loadout unlocks give a long-term goal. See [[05-Progression/unlock-pacing]].
- Teams make friends play together.

## How to build it in Roblox
- Hitscan: the client sends a ray origin and direction; the server re-casts with a tolerance and checks distance, fire rate, and line of sight.
- Never accept damage values from the client.
- Recoil and spread are server-reproducible patterns with a seed.
- Use a weapon config ModuleScript (damage, rate, range, spread, recoil); balance from one table.
- Replicate shots to others as effects only; keep tracers cosmetic.
- Anti-cheat basics: speed checks, rate limits on fire remote, and kick on repeated impossible shots.

## Numbers (playtest starting points)
- Player health 100; rifle 22-30 damage, 9-10 rounds per second equals TTK about 0.4-0.5 s.
- Sniper one-shot to head only, 1.2 s cycle.
- Shotgun 8 pellets, effective under 25 studs.
- Reload 1.6-2.5 s; sprint speed 1.25 times walk.
- Respawn 3 s; match size 6v6 to 8v8 on a map of about 200 by 200 studs.

## Mistakes
- Client-trusted hits; exploiters will aimbot and one-shot everyone.
- Spawn kills from poor spawn logic.
- Weapon that beats all others at all ranges.
- No sensitivity or FOV settings.
- Gore or realistic real-world weapon branding; keep it stylized and age appropriate.

## Balance review loop
- After each update compare kills per weapon, pick rate, and win rate.
- Nerf the outlier by changing one number at a time and re-test.
- Keep a changelog players can read in the lobby.

## Playtest check
- Run a 6v6 with one player on a phone; ask if they could hit anything.
- Replay spawns 20 times and count spawns in an enemy line of sight; target zero.
- Test the fire remote with 100 shots per second from a script to confirm the server drops it.

## For our Roblox games
Copy the principles: authoritative hits, readable TTK, role-based weapons, sideways unlocks, safe spawns. Never copy real gun brands, existing game weapon names, maps, or sound packs. Design stylized original gear.
