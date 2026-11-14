---
description: Choosing max players per server and configuring StreamingEnabled: enum options, atomic vs persistent models, pause behaviour, and how server size interacts with NPC and replication cost.
type: reference
status: draft
last_checked: 2026-11-14
keywords: server size, max players, StreamingEnabled, streaming, ModelStreamingMode, Persistent, Atomic, StreamingIntegrityMode, StreamingPauseMode, replication, low-end mobile, instance streaming
sources: creator-docs content/en-us/reference/engine/enums/ModelStreamingMode.yaml (read 2026-11-14); creator-docs content/en-us/reference/engine/enums/StreamingIntegrityMode.yaml (read 2026-11-14); creator-docs content/en-us/reference/engine/enums/StreamingPauseMode.yaml (read 2026-11-14); creator-docs content/en-us/resources/the-mystery-of-duvall-drive/stream-in-immersion.md (read 2026-11-14); creator-docs content/en-us/resources/modules/friends-locator.md (read 2026-11-14)
verify: Enum member names verified in local docs. Recommended player counts and streaming radii are rules of thumb from model knowledge: UNVERIFIED. Property names on Workspace (StreamingEnabled, StreamingTargetRadius, StreamingMinRadius) are UNVERIFIED locally.
tags: [multiplayer-npcs, reference]
---
# Server size and streaming

Server size sets how many players share the world. Streaming decides how much of that world each client holds. Together they set memory, bandwidth and frame cost.

## Do this
- Start with a small max (8 to 16) for action or round games, 30 to 50 for social hubs, and raise only after profiling. More players multiply replication, physics and NPC cost.
- Set max players in game settings in Studio (game-level setting). Match it to the mode: a 4v4 needs 8, not 50.
- Turn on `Workspace.StreamingEnabled` for any large map or any game targeting phones. Test with it on from day one; adding it late breaks scripts.
- In scripts, never assume a far-away instance exists on the client. Use `WaitForChild` with a timeout, or `Instance.AncestryChanged`, or tag-based listening with `CollectionService:GetInstanceAddedSignal`.
- Choose model streaming mode per model (`Model.ModelStreamingMode`): `Default` for normal; `Atomic` for models that must arrive whole (a building with a door script); `Persistent` for things that must always exist (the lobby spawn, round-state parts); `PersistentPerPlayer` for per-player models; `Nonatomic` lets parts arrive piecemeal.
- `Persistent` models never stream out, so use them sparingly; each one costs memory on every client.
- Enum `StreamingIntegrityMode`: `Default`, `Disabled`, `MinimumRadiusPause`, `PauseOutsideLoadedArea`. For fast movement or combat games, `PauseOutsideLoadedArea` stops the player falling into unloaded holes at a cost of a short freeze.
- Enum `StreamingPauseMode`: `Default`, `Disabled`, `ClientPhysicsPause`. Leave default unless you have a custom loading screen.
- Put server-only logic in `ServerStorage` and server NPC rigs near their players. Far NPCs the client has not streamed are invisible, which is fine for gameplay but not for scripts that expect the client to see them.

## Rules for scripts under streaming
- Server scripts see everything; client scripts see only streamed-in instances.
- Do not call `workspace.Map.Door` on the client without `WaitForChild`.
- Raycasts on the client hit only loaded geometry. Do authoritative hit checks on the server.
- Teleport a player and wait for the destination area: use `Player:RequestStreamAroundAsync(position)` before spawning (UNVERIFIED locally, check the API reference).

## How server size changes cost
- Replication: per-player updates scale roughly with players times nearby moving objects.
- NPCs: each NPC runs AI on the server and replicates to every player in range. Halving NPC count often saves more than halving players.
- Physics: unanchored parts near many players cost the most.
- Chat, remotes and leaderboards grow with players; throttle them (see [[13-Luau-Engineering/cooldowns-and-rate-limits]]).

## Quick decision table
| Game type | Players per server | Streaming |
|---|---|---|
| Round-based arena | 8 to 16 | on if map is large |
| Co-op wave defence | 4 to 8 | on, enemies persistent per player |
| Social hub or tycoon world | 20 to 50 | on, required |
| Obby | 10 to 30 | on if long |

UNVERIFIED: ranges are starting points, not Roblox guidance.

## Test steps
1. Turn StreamingEnabled on and play with a small target radius to expose missing-instance bugs.
2. Join with 2 clients, walk one far away, and confirm gameplay still works.
3. Record client memory in the Developer Console before and after (see [[14-QA-Optimization/microprofiler-and-stats-guide]]).

## Mistakes
- `StreamingEnabled` on with scripts that index far parts directly.
- Making the whole map `Persistent` (defeats streaming).
- Large server size with expensive NPCs on mobile.
- Never testing streaming in a Studio Play session with a tiny radius.

## Related
[[12-Multiplayer-NPCs/matchmaking-and-lobbies]], [[12-Multiplayer-NPCs/bots-to-fill-servers]], [[14-QA-Optimization/mobile-performance-cookbook]], [[14-QA-Optimization/performance-budgets]], [[14-QA-Optimization/lag-sources-ranked]]

## For our Roblox games
Copy the principle: size the server to the mode, stream big maps, mark critical models Persistent, load-test with weak devices. Never copy another game's map layouts, building models or asset IDs. Zero spend: all settings are built into Studio.
