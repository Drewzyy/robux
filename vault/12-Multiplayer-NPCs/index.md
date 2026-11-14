---
description: Server authority, social play, matchmaking basics, and NPCs with pathfinding and simple state machines.
type: section-index
status: verified
last_checked: 2026-11-11
verify: scripts/check_vault.py --full checks every link below resolves and every status matches the note.
tags: [multiplayer-npcs, section-index]
---
# 12-Multiplayer-NPCs

Server authority, social play, matchmaking basics, and NPCs with pathfinding and simple state machines.

Load only the note your task needs. Do not read every note in this section.

| Note | What it holds | Status |
|---|---|---|
| [[12-Multiplayer-NPCs/multiplayer-patterns]] | Server-owned state, social features that bring players back, fairness, works with 1 player. | draft |
| [[12-Multiplayer-NPCs/npc-ai]] | Pathfinding, state machines, low tick rate, cheap crowds, bots for empty servers. | draft |
| [[12-Multiplayer-NPCs/boss-fight-patterns]] | Design and script boss fights for co-op Roblox games: phases, telegraphed attacks, health scaling by player count, aggro rules, a typed boss state loop and loot fairness. | draft |
| [[12-Multiplayer-NPCs/bots-to-fill-servers]] | Fill empty or small servers with bots so solo players still get a lively match: when to add and remove bots, honest labelling, difficulty scaling, and server-side bot cost limits. | draft |
| [[12-Multiplayer-NPCs/enemy-waves-and-spawners]] | Wave-based enemy spawning with budgets, spawn point rules, alive caps, difficulty curves and cleanup; includes a typed wave runner and scaling for solo versus full servers. | draft |
| [[12-Multiplayer-NPCs/leaderboards]] | Build global and per-server leaderboards with OrderedDataStore, throttled writes, cached reads, in-server boards and anti-cheat; includes update intervals and request budgets. | draft |
| [[12-Multiplayer-NPCs/matchmaking-and-lobbies]] | How to gather players in a lobby, start a match at the right size, and teleport them to a reserved server; covers queue timers, fill rules, teleport failure handling and a ready-check. | draft |
| [[12-Multiplayer-NPCs/pets-and-followers]] | Pets and follower NPCs that trail the owner: server-owned data, client-side smooth following, a typed follow loop, spacing offsets, and performance limits for many pets per server. | draft |
| [[12-Multiplayer-NPCs/round-system-pattern]] | Server-side round loop (intermission, map load, play, results, cleanup) with exact timings, win conditions, late-join rules and a typed Luau skeleton that cannot get stuck. | draft |
| [[12-Multiplayer-NPCs/server-size-and-streaming]] | Choosing max players per server and configuring StreamingEnabled: enum options, atomic vs persistent models, pause behaviour, and how server size interacts with NPC and replication cost. | draft |
| [[12-Multiplayer-NPCs/teams-and-parties]] | Implement team assignment, team balance and friend parties: Teams service, auto-balance rules, friendly-fire checks, party invites and keeping friends in the same server. | draft |
| [[12-Multiplayer-NPCs/trading-system-safety]] | Safe player-to-player trading: two-step confirm, server-held escrow, atomic swaps, anti-scam delays, logging and rate limits. Prevents dupes, bait-and-switch and rollback exploits. | draft |
