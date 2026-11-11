---
description: Attacks that feel strong: anticipation, active, recovery frames, hitstop, knockback, camera kick, and fair telegraphs.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: attack animation, combat, hitstop, impact, anticipation, recovery, melee, sword, punch, hit feedback, telegraph
sources: https://www.gamedeveloper.com/game-platforms/anatomy-of-an-enemy-attack-in-dark-souls-3 ; https://aztez.com/blog/2012/05/24/the-amalur-problem/ ; https://www.slynyrd.com/blog/2018/9/8/pixelblog-9-melee-attacks
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [animation-vfx, playbook]
---
# Combat animation

## Do this
- Three phases per attack: anticipation (wind-up), active (damage window), recovery (follow-through).
- Player attacks: anticipation short (about 0.05 to 0.12 s) so input feels instant; put weight into the follow-through instead. Enemy attacks: long, readable anticipation (0.4 to 1 s) so players can react (telegraph).
- Hitstop on connect: freeze attacker and target 0.03 to 0.07 s (light) to 0.1 s (heavy). Too much feels laggy.
- Every hit, within 0.1 s: hit sound (pitch varied), small particle burst at contact, target flash (Highlight or brief white), small knockback, damage number pop.
- Camera kick: 0.05 to 0.15 s small offset, scaled by hit strength; off when reduced motion is on.
- Combos: allow the next input during the last part of recovery (buffer about 0.2 s).

## Roblox wiring
- Animation markers (GetMarkerReachedSignal) mark the active frame; the client plays effects there, the server checks the hit (range, cooldown, angle) and applies damage.
- Hitstop: AnimationTrack:AdjustSpeed(0) then back to 1 after the pause, on the client.
- Never trust client damage numbers ([[14-QA-Optimization/exploit-testing]]).
