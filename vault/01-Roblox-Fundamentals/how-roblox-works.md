---
description: Client and server, services, places and experiences, Studio, and what publishing means. The mental model every agent needs first.
type: reference
status: draft
last_checked: 2026-11-11
sources: https://create.roblox.com/docs/scripting/security/client-server-boundary
verify: UNVERIFIED: written from model knowledge; check against https://create.roblox.com/docs/scripting/security/client-server-boundary
tags: [roblox-fundamentals, reference]
---
# How Roblox works (the short version)

## Experiences, places, Studio
- An **experience** is the game players join. It holds one or more **places** (levels or worlds). Each place is built in **Roblox Studio** and saved to Roblox.
- **Saving** keeps a private copy. **Publishing** makes the change live for players. BrewixGames never publishes: Andrew does that in Studio.

## Client and server
- Every running game has one **server** (Roblox's computer) and one **client** per player (their device).
- `Script` runs on the server. `LocalScript` runs on a player's client. `ModuleScript` is shared code required by either.
- **Never trust the client.** Anything a client sends can be faked by an exploiter. The server decides money, damage, inventory, and rewards. See [[13-Luau-Engineering/remotes-and-security]].
- Clients and server talk through `RemoteEvent` (fire and forget) and `RemoteFunction` (ask and wait). Server code must validate every argument: type, range, rate, and whether this player is allowed to do it right now.

## Services you meet first
| Service | What it is for |
|---|---|
| Workspace | The 3D world everyone sees |
| Players | Who is in the server |
| ReplicatedStorage | Shared modules and remotes, visible to clients |
| ServerScriptService / ServerStorage | Server-only code and assets, hidden from clients |
| StarterGui / StarterPlayer | What each player gets when they join |
| DataStoreService | Saving player data between sessions (use the playbook) |
| TweenService, RunService | Smooth motion; per-frame updates |

## What costs money (for us: nothing)
Building, testing, saving, and publishing an experience is free. Earning Robux is optional (gamepasses, developer products). Ads and paid assets cost money, and BrewixGames never uses them.
