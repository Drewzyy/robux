---
description: Minigame party playbook for Roblox: round format, voting, short games, scoring, elimination versus points, lobby flow, and how to add new minigames cheaply without breaking the lobby loop.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: minigame, party, rounds, lobby, voting, elimination, points, intermission, party game, last man standing, team, rotation
sources: Author working knowledge of party-game design (no pages opened 2026-11-14); Roblox Creator Hub docs, TeleportService and Teams pages (titles only, not opened 2026-11-14)
verify: General practice. UNVERIFIED: genre rankings. Timings are playtest starting points.
tags: [game-design, playbook]
---
# Minigame party playbook

A lobby cycles through short games. Each minigame teaches in 5 s, plays in 60-120 s, and crowns a winner. The wrapper is the product; minigames are content.

## Do this
- Round length: 45-120 s each; a full match of 4-6 minigames takes 8-12 min.
- Explain every minigame with a 3-5 s title card: one icon, one verb, one line of text.
- Intermission under 20 s with a visible countdown and a mini lobby activity so waiting is never dead.
- Mix elimination games and points games so eliminated players come back next round, not the whole match.
- Use a score board with total points over the match; a catch-up element (last place gets a small boost) keeps everyone in.
- Let players vote between 2-3 minigames; the winner is chosen by majority, ties by random.
- Reward participation: coins for every player, a bigger prize for the match winner.
- Add spectator tools: free camera or follow-cam for eliminated players, plus emotes and cheer buttons.

## Why it works
- Variety prevents boredom; short games lower the cost of losing.
- Easy for friends to join mid-match; a new minigame is a fresh reason to return.
- Winners are easy to celebrate and clip. See [[03-Game-Psychology/social-status-and-flex]].

## How to build it in Roblox
- Round controller: a state machine (Lobby, Vote, Intro, Play, Results) on the server, one ModuleScript per minigame with Start, End, and Cleanup.
- Teleport players to an arena folder cloned from ServerStorage; destroy after each round.
- Use the same interface for every minigame so adding one is 1 to 3 days of work.
- Player count handling: minimum 4 to start; fill with bots or extend the countdown when below 6.
- Join-in-progress: place late joiners in spectate and add them next round.
- Cleanup: always clear connections and parts after a round to prevent leaks.

## Numbers (playtest starting points)
- Min players 4, max 16 per server.
- Vote 10 s, intro 5 s, play 60-120 s, results 8 s.
- Elimination share per round 25-50 percent.
- Library of 10 minigames at launch, add 1-2 per update.
- Match winner bonus 3 times the participation reward.

## Mistakes
- Minigames needing long explanation.
- Early eliminations stuck watching for over 90 s.
- One minigame dominating the vote; weight picks by recency.
- Memory leaks from poor cleanup, causing lag late in a session.
- Lobby with nothing to do while waiting.

## Minigame ideas that explain in one line
- Last platform standing: floor tiles vanish after being stepped on.
- Coin dash: collect the most coins in 60 s.
- Color match: stand on the called color before the timer ends.
- Obstacle sprint: first to the finish line through spinning arms. See [[02-Game-Design/genre-playbooks/obby]].

## Playtest check
- Time how long a first-time player needs to understand each minigame; over 10 s means redesign the card.
- Run 30 rounds back to back and watch memory use in the developer console.

## For our Roblox games
Copy the wrapper: vote, intro, play, results, fair catch-up, shared interface. Never copy another game's minigame names, maps, characters, music, or mascot. Invent originals and keep the card text your own.
