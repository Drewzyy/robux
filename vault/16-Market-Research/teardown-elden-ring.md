---
description: Why Elden Ring got huge and keeps players, with its combat animation feel (anticipation, commitment, hit-stop, tells, i-frames) and minimal UI, translated into Roblox combat and onboarding rules.
type: teardown
status: draft
last_checked: 2026-11-14
keywords: elden ring, fromsoftware, souls, combat feel, anticipation, commitment, i-frames, hit-stop, enemy tells, minimal ui, open world, difficulty, boss
sources: https://gamedeveloper.com/design/vlambeer-co-founder-shares-advice-on-building-better-action-games (search result read 2026-11-14, snippet only); general knowledge of FromSoftware design write-ups (no single page re-read)
verify: UNVERIFIED: frame counts are community-measured typical values, not official. Sales figures not cited here on purpose. Test every timing in Studio.
tags: [market-research, teardown]
---
# Teardown: Elden Ring
A hard open-world action RPG. Lessons here are about combat readability, player-driven goals and a near-empty screen.

## Do this
- Give every enemy attack three beats: wind-up 300 to 600 ms (readable pose or sound), a short active window 100 to 200 ms, then a recovery 400 to 800 ms where the player may punish.
- Make player attacks commit: once started, the swing cannot be cancelled for most of its length. Commitment is what makes timing a skill and makes enemies fair.
- Add a dodge with i-frames: invulnerable for about 200 to 350 ms from the start, then a recovery with no control. Server checks hits against a timestamp; do not let the client grant invulnerability.
- Add hit-stop of 40 to 100 ms on a landed hit (freeze the attacker animation speed to near zero), bigger for heavy weapons or kills. See [[09-Animation-VFX/game-feel-juice]].
- Keep the HUD to health, stamina, and the equipped item. Hide everything else until needed; teach by the world (a glowing tree, a boss door, a ghost message), not by pop-ups.
- Let players choose the order: show two or three goals at once in the first 5 minutes, never one forced line.
- Death is cheap: respawn near the fight within 10 seconds and keep progress. Hard but fast to retry beats easy but slow.

## Why it got huge
- Open world with no quest marker spam: players choose where to go, so each story of discovery is their own and gets retold and shared.
- Hard bosses create stories and clips. Hard is a feature when failure is understood ("I rolled too late").
- Co-op summons and messages left by other players gave social help without a lobby.
- Players could change difficulty by changing build: summons, spells, levelling in an easy area. Several ways to win means fewer walls.

## Why people play
- Mastery: you see your own skill improve against the same boss.
- Exploration: reward behind every odd corner (item, shortcut, secret boss).
- Build identity: many weapons and spells, so a build is a statement.

## Why they stay and come back
- Long power curve with many branches; a second character feels different.
- New Game Plus, bosses to re-fight, and community challenges (no-hit runs).
- DLC and patches re-open the game for returning players. UNVERIFIED which of these drives the most return play.

## Animation feel (the part to copy)
- Anticipation: enemy raises weapon, shifts weight, or lights a body part before striking. Big attacks get longer wind-up and a sound cue. Players learn the tell once and then trust it.
- Commitment: attacks have fixed start and recovery. Mashing is punished because recovery cannot be cancelled. This is the opposite of floaty combat.
- Hit-stop: a few frames of freeze on contact make hits feel heavy. Pair with a small camera nudge (see [[08-UI-UX/ui-animation]] for easing on UI, and [[09-Animation-VFX/animation-basics]]).
- Readable tells: each enemy family has a distinct silhouette and a distinct sound for each attack. A delayed attack (hold then swing) is the classic trick to beat panic dodging; use it sparingly.
- I-frames and stamina: a dodge costs stamina, so every dodge is a choice. Stamina refills only when not acting, which creates rhythm.
- Camera: lock-on keeps the enemy centered so tells stay in view.

## Minimal UI
- Bars at top left, small, not animated unless hit. Damage numbers are absent.
- Prompts only when something is interactable, in the world, one button.
- Maps and menus are opened on purpose; nothing nags.
- Roblox mobile caveat: the screen is small, so keep three thumb buttons max (attack, dodge, item) and put a visible stamina ring around the dodge button. See [[08-UI-UX/hud-template]].

## Mistakes to avoid
- Copying the difficulty without the fairness: if the tell is unreadable, players call it cheating.
- Lag: with network latency over 150 ms, tight 300 ms tells fail. Lengthen wind-ups for Roblox (400 to 700 ms) and resolve hits on the server with a small grace window.
- Punishing death with lost progress on a kid audience.

## For our Roblox games
Copy: telegraphed attacks, committed swings, dodge with stamina, hit-stop, empty HUD, player-chosen goal order, fast retry. Never copy: FromSoftware names, bosses, items, lore, music, or art. Link with [[03-Game-Psychology/player-psychology]] for mastery motives and [[03-Game-Psychology/motivation-coverage-checklist]].

## Timings table (our starting points, UNVERIFIED against the real game)
| Item | Roblox start value |
|---|---|
| Light enemy wind-up | 350 ms |
| Heavy enemy wind-up | 600 to 800 ms |
| Delayed attack extra hold | 300 ms |
| Dodge i-frames | 250 ms |
| Dodge stamina cost | 20 percent of bar |
| Hit-stop light / heavy | 50 / 100 ms |
| Respawn to fight | under 10 s |
