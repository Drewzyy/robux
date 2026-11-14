---
description: Why Among Us got huge (social deduction, short rounds, streamers, free and cheap) and how to borrow the deception loop and low-cost design for a Roblox party game.
type: teardown
status: draft
last_checked: 2026-11-14
keywords: among us, social deduction, impostor, crewmate, party game, short rounds, voting, tasks, streamers, proximity chat, low cost
sources: general knowledge of public coverage of Among Us (InnerSloth, 2020 surge); no page re-read 2026-11-14
verify: UNVERIFIED: no player or revenue numbers cited. Round lengths and task counts are typical values from memory.
tags: [market-research, teardown]
---
# Teardown: Among Us
A tiny-scope party game where the fun is people lying to each other. Lessons: social games need little content, and the best content is the other players.

## Do this
- Build rounds of 8 to 12 minutes with 4 to 10 players, and a lobby that fills fast (target start within 60 seconds of joining).
- Give innocent players simple tasks (30 to 60 seconds each, 4 to 8 per round) so everyone has something to do while watching.
- Give the hidden role one power that creates doubt (sabotage, vent) and one cost (a cooldown of 20 to 30 seconds).
- Add a discussion and vote phase of 60 to 90 seconds with quick chat buttons and a clear "who is suspicious" list. Use Roblox filtered chat only.
- Make roles and the win condition understandable in one screen: crew finish tasks or find the impostor; impostor reduces crew.
- Let eliminated players keep doing something (ghost tasks, watching) so no one is sent away. See [[03-Game-Psychology/play-with-friends]].
- Add skins and hats as the only purchase; no gameplay advantage.

## Why it got huge
- Free or cheap, runs on any phone, tiny download.
- A round makes a story worth retelling ("I saw you vent").
- Streamers and short clips spread it; watching it is fun because the lying is visible.
- Players brought friends because the game needs a group.

## Why people play
- Social connection and play-acting: lying and detecting is a safe, funny contest.
- Low skill barrier: anyone can bluff or accuse.
- Short, replayable, different every time because people differ.

## Why they stay
- Mastery in reading people, not mechanics.
- Cosmetics and map variety.
- Regular group: friends play nightly.

## Why they come back
- The same friend group returns; each round is a new plot.
- New maps or roles every few months add variety. UNVERIFIED how much updates drove the later retention.

## Roblox build notes
- Server authoritative roles: assign on the server and never send the impostor list to clients except to the impostors. See roblox-security.
- Anti-grief: votes need a majority; report buttons are rate limited (one per 2 seconds).
- Tasks as small proximity prompts with a progress bar; avoid anything requiring typing.
- Low cost: reuse one small map with color-coded rooms; 60 parts per room is plenty.
- Bots to fill empty slots in a quiet server so a round can start (see roblox-npc-ai); bots should act simply and never fool real players deeply.

## Mistakes
- Waiting for a full lobby: lose players. Start with bots at 60 seconds.
- Chat abuse and bullying in accusation phases: use preset accusations and report buttons.
- First-time confusion: add a 20 second role card and a practice round.

## For our Roblox games
Copy: hidden role plus simple tasks, short discussion/vote, ghost play, cosmetics-only. Never copy: crewmate/impostor character designs, names, sound effects, map art. Related: [[03-Game-Psychology/player-psychology]], [[03-Game-Psychology/motivation-coverage-checklist]].

## Round timeline (suggested)
| Time | Beat |
|---|---|
| 0 s | role card for 8 seconds, then spawn |
| 0 to 4 min | tasks and first sabotage |
| 4 min | first report or meeting |
| 4 to 6 min | discussion and vote, 75 s |
| 6 to 10 min | second phase, fewer players, higher tension |
| end | win screen shows who the impostor was and a replay of key moments |
