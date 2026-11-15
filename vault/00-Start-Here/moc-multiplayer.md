---
description: Map of notes on servers, rounds, lobbies, teams, trading, leaderboards, NPCs, bots, bosses, streaming and social play.
type: reference
status: draft
last_checked: 2026-11-15
keywords: multiplayer, servers, rounds, lobbies, matchmaking, teams, parties, trading, leaderboards, npc, bots, boss, streaming, pvp
sources: vault notes only, read 2026-11-15 (front matter of each linked note)
verify: Built from note descriptions; each linked note carries its own source and verify line.
tags: [start-here, reference]
---
# Multiplayer
Use when more than one player shares a world, or when NPCs fill the world.

## Do this
- Read first, step 1: [[12-Multiplayer-NPCs/multiplayer-patterns]] (server-authoritative basics).
- Read first, step 2: [[12-Multiplayer-NPCs/round-system-pattern]] (the round loop).
- Read first, step 3: [[12-Multiplayer-NPCs/matchmaking-and-lobbies]] (gather and start).
- Then pick the section below that matches your task and read only those notes.

## How to use this map
- Read the three notes under Do this, then open one section below.
- Each note has its own Do this list; act on that, not on this page.
- If a note is marked UNVERIFIED, tune from your own analytics.

## Core patterns
- [[12-Multiplayer-NPCs/multiplayer-patterns]]: shared state and social features.
- [[12-Multiplayer-NPCs/round-system-pattern]]: intermission, map, play, results.
- [[12-Multiplayer-NPCs/matchmaking-and-lobbies]]: lobby size and teleport.
- [[12-Multiplayer-NPCs/teams-and-parties]]: team balance and friend parties.
- [[12-Multiplayer-NPCs/server-size-and-streaming]]: max players and StreamingEnabled.
- [[12-Multiplayer-NPCs/index]]: section index.

## Competition and fairness
- [[12-Multiplayer-NPCs/leaderboards]]: OrderedDataStore boards.
- [[02-Game-Design/pvp-fairness-and-balance]]: matchmaking, caps, counterplay.
- [[04-Onboarding/new-player-protection]]: protect newcomers.
- [[14-QA-Optimization/exploit-testing]]: test like an exploiter.

## Trading and economy
- [[12-Multiplayer-NPCs/trading-system-safety]]: two-step confirm, escrow.
- [[07-Monetization/server-boosts]]: boosts for the whole server.
- [[13-Luau-Engineering/cooldowns-and-rate-limits]]: stop spam.

## NPCs, bots and bosses
- [[12-Multiplayer-NPCs/npc-ai]]: pathfinding and state machines.
- [[12-Multiplayer-NPCs/bots-to-fill-servers]]: keep small servers lively.
- [[12-Multiplayer-NPCs/enemy-waves-and-spawners]]: budgets and alive caps.
- [[12-Multiplayer-NPCs/boss-fight-patterns]]: phases and telegraphs.
- [[12-Multiplayer-NPCs/pets-and-followers]]: followers and pets.
- [[13-Luau-Engineering/state-machines]]: FSM for NPCs and rounds.

## Social design
- [[06-Retention/social-retention]]: friends and guilds raise retention.
- [[03-Game-Psychology/play-with-friends]]: invites and co-op goals.
- [[02-Game-Design/genre-playbooks/social-hangout]]: hub layout and emotes.
- [[09-Animation-VFX/emotes-and-idle-personality]]: group emotes.

## Security and performance
- [[13-Luau-Engineering/remotes-and-security]]: never trust the client.
- [[13-Luau-Engineering/networking-hardening]]: remote hardening.
- [[14-QA-Optimization/lag-sources-ranked]]: replication and server lag.

## For our Roblox games
- Copy the principle: the server decides, clients only ask. Never copy another game's map, modes, names or art.
- Children are present: use Roblox chat filtering, no private contact prompts, safe trading with two-step confirm.
- Test with several clients and a bad network before launch.
