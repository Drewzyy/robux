---
description: Server-authoritative multiplayer: shared state, social features that raise retention, and fair play.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/scripting/security/client-server-boundary
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/scripting/security/client-server-boundary
---
# Multiplayer patterns

## The server owns the truth
State that matters (scores, inventory, round timer, who won) lives on the server. Clients show it and ask to change it through remotes; the server checks and decides.

## Social features worth building
- **Play together**: party join (friends land in the same server), co-op goals, trading only with server checks.
- **See each other**: leaderboards, visible cosmetics, emotes. Being seen is a core reason players return.
- **Low-pressure contact**: high fives, gifts with no Robux, shared world events.

## Fairness
- Rate-limit every remote. Reject impossible values (speed, distance, cooldown).
- Never let one player's lag or exploit affect another's saved data.

## Small servers
Design so 1 player can still have fun (bots or solo goals) and 10 players feel lively. Empty servers kill retention.
