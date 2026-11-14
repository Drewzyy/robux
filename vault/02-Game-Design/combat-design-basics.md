---
description: Combat design fundamentals for Roblox: attack, defense, telegraphs, health and damage ranges, enemy roles, boss patterns, and server-side rules that make fights readable on all devices.
type: reference
status: draft
last_checked: 2026-11-14
keywords: combat, damage, health, telegraph, enemy types, boss, cooldown, dodge, block, hitbox, aggro, TTK, readable combat
sources: Author working knowledge of game design practice (no pages opened 2026-11-14); Roblox Creator Hub docs (create.roblox.com/docs, titles only, not opened 2026-11-14)
verify: Numbers are starting points. UNVERIFIED as a standard; tune with playtests on phone and PC.
tags: [game-design, reference]
---
# Combat design basics

Good combat is readable (I know what is happening), fair (I can respond), and satisfying (hits feel good). See [[02-Game-Design/difficulty-and-flow]] and [[09-Animation-VFX/combat-animation-and-hit-feedback]].

## Do this
- Give the player 3 verbs at first: attack, dodge or block, one special. Add a 4th only after the first fights.
- Telegraph every enemy attack: windup 0.4-0.8 s with a sound and a visible cue (red flash, glow, ground marker).
- Make basic enemies die in 2-4 hits; elite enemies in 8-12; bosses in 40-100 hits, spread over phases.
- Keep enemy damage readable: player 100 health, common enemy hit 8-12, elite 15-20, boss 25-35.
- Use enemy roles: charger (runs, can be dodged), ranged (strafe), swarm (many weak), tank (slow, armored), healer (priority target).
- Give players a safe reset: after death respawn at a checkpoint within 5 s.
- Add a gentle aggro system: enemies chase to a leash distance of 40-60 studs, then return and heal.
- Scale rewards with risk, so a harder room pays more.

## Why it works
- Telegraphs turn losses into learning, not unfairness.
- Roles create decisions in groups of enemies.
- Short fights fit the Roblox session.

## Boss pattern template
1. Phase 1 (100-70 percent): two attacks, 1 s gaps, teaches the patterns.
2. Phase 2 (70-30 percent): add a third attack and a summon.
3. Phase 3 (30-0 percent): faster tempo, 1 new attack, short vulnerability window after each big move.
- Always show a health bar and the next hint (name of the attack).

## How to build it in Roblox
- Server holds enemy state (health, target, attack timers); clients only play effects.
- Hit detection at animation marker events with short hitboxes; see [[02-Game-Design/pvp-fairness-and-balance]] for PvP rules.
- Use a damage module: base, crit chance, armor, resist; one formula for all.
- Enemy AI: simple state machine (Idle, Chase, Windup, Attack, Recover).
- Pathfinding with throttle (one path update per 0.3 s) to save frames.
- Cap simultaneous attackers at 3 per player so crowds stay fair.

## Numbers (playtest starting points)
- Player attack cooldown 0.4-0.6 s; dodge 0.5 s with 2 s cooldown.
- Enemy recover window 0.6-1.0 s after a big attack.
- Crit chance 5-10 percent, 1.5-2 times damage.
- Enemy count per encounter 3-8; elite 1 plus 3 adds.

## Mistakes
- Enemies that attack through the player with no warning.
- Damage sponge bosses with no pattern.
- Camera that hides attackers; keep a mild lock-on.
- Combat with no sound.
- Mobile controls that cannot dodge reliably.

## Playtest check
- Ask a new player what killed them; if they say they did not see it, add a telegraph.
- Count deaths per encounter; more than 2 average means retune damage or health.
- Test a boss with 1 player and with 4.

## For our Roblox games
Copy the principles: telegraphs, roles, phased bosses, server authority. Never copy another game's enemy designs, attack names, or boss art. Design original creatures and moves.
