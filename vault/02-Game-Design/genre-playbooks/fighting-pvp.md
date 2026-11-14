---
description: Fighting and PvP arena playbook for Roblox: match format, move sets, ranked ladder, matchmaking basics, anti-frustration rules, and spectator hooks. Pair with the fairness and combat notes.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: fighting game, pvp, arena, duel, ranked, matchmaking, elo, combos, cooldowns, battlegrounds, 1v1, spectate
sources: Author working knowledge of fighting and arena design (no pages opened 2026-11-14); Roblox Creator Hub docs, MessagingService and TeleportService pages (titles only, not opened 2026-11-14)
verify: General practice. Matchmaking and cooldown numbers are starting points. UNVERIFIED: exact current popularity of battlegrounds games.
tags: [game-design, playbook]
---
# Fighting and PvP playbook

Two shapes dominate: open "battlegrounds" (many fighters, one arena, respawn) and structured duels (1v1 or 2v2 with ranked ladders).

## Do this
- Pick one shape and commit. Battlegrounds suits drop-in play; duels suit ranked progression.
- Keep each fight 60-180 s. A kill or round should resolve before attention drifts.
- Give each character or style 4 to 6 moves: 1 basic combo, 1 ranged or gap closer, 1 guard or dodge, 1 finisher with a long cooldown.
- Counterplay for every move: a dodge window, block, or punish opening. If an attack has no answer, remove or slow it.
- Show cooldowns on screen and play a sound when they refresh. See [[09-Animation-VFX/combat-animation-and-hit-feedback]].
- Respawn in 3-5 s with 2 s spawn protection. Never leave players spectating a long time.
- Ranked: bronze to top tiers with visible points; use a placement of 5-10 matches. See [[02-Game-Design/pvp-fairness-and-balance]].
- Free-to-win cosmetics only for the shop; do not sell raw power.

## Why it works
- Skill expression and bragging rights; clips are social currency. See [[03-Game-Psychology/social-status-and-flex]].
- Short matches fit a 15-30 minute session and encourage "one more".
- Ranks give a long-term goal that gear cannot buy.

## How to build it in Roblox
- Server is authoritative for damage, cooldowns, and hit detection; the client only predicts visuals.
- Hit detection: short-lived hitbox parts or workspace:GetPartBoundsInBox on the server at animation hit frames.
- Matchmaking for duels: queue by rating band (start at plus or minus 100, widen by 50 every 10 s); teleport matched players to a reserved server.
- Rating: Elo with K around 32 for new players and 16 for settled ones.
- Anti-lag: use ping compensation with a small lag budget (under 150 ms) and show ping to players.
- Leaderboards via OrderedDataStore, refreshed on a timer.

## Numbers (playtest starting points)
- Basic combo 3-4 hits, 0.4-0.6 s per hit.
- Dodge 0.5 s with 1.5-3 s cooldown and 0.2 s invulnerability.
- Finisher cooldown 25-45 s; ultimate meter fills over about 60-90 s of fighting.
- Stun chain cap: 1.5 s total before a short immunity so nobody is locked out.
- Queue time target under 30 s; fall back to bots when empty.

## Mistakes
- Stun-locks and infinite combos; add combo scaling or immunity.
- Matching new players with veterans.
- One dominant move; check pick and win rates weekly after launch.
- Rank loss so harsh it ends the session; keep a rank floor per tier.
- Ignoring low-end mobile input: add big buttons and aim assist.

## Spectator and clip hooks
- Add a replay or kill-cam of 3-5 s after each kill so wins are shareable.
- Spectate mode that follows top-ranked matches in a lobby board.
- A weekly tournament bracket with free cosmetic titles as prizes.

## Playtest check
- Run 20 bot-vs-bot matches per move set and compare win rates; anything outside 45-55 percent needs a tweak.
- Have a mobile player finish a ranked match.

## For our Roblox games
Copy structure: short matches, counterplay, authoritative server, rating bands, no pay-to-win. Never copy another game's move names, character looks, anime IP, music, or UI. Build original move sets with their own animations.
