---
description: Canonical Rojo layout: default.project.json mapping, file naming (.server.luau, .client.luau), one bootstrapper per side, frozen shared data.
type: reference
status: verified
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2; kanzerog rojo-roblox-skill project-structure; carmodyofficial-ops roblox_systems_architecture
verify: Verified 2026-11-09: the layout below, extracted into a temp project, passed scripts/gates/build_gate.py (rojo build 7.7.1, selene, stylua, lune 0.11.1). See LOG.md Phase 3 spot checks.
---
# Project structure

## Mapping
| Filesystem | Becomes | Rule |
|---|---|---|
| `src/shared` | `ReplicatedStorage.Shared` | both sides require it; never put secrets here, exploiters can read it |
| `src/server` | `ServerScriptService.Server` | server only; clients cannot see it |
| `src/client` | `StarterPlayer.StarterPlayerScripts.Client` | cloned into each player on join |
| `src/ReplicatedFirst` | `ReplicatedFirst` | runs earliest on client (loading screen) |
| `Packages` (Wally) | `ReplicatedStorage.Packages` | third-party libraries |

```json
{
  "name": "my-game",
  "tree": {
    "$className": "DataModel",
    "ReplicatedStorage": {
      "Shared": { "$path": "src/shared" },
      "Packages": { "$path": "Packages" }
    },
    "ServerScriptService": { "Server": { "$path": "src/server" } },
    "StarterPlayer": {
      "StarterPlayerScripts": { "Client": { "$path": "src/client" } }
    }
  }
}
```
Drop the `Packages` line if the project has no Wally packages (Rojo fails on a missing path).

## File naming
- `.luau` is a ModuleScript (does not run alone).
- `init.server.luau` or `*.server.luau` runs on the server; `*.client.luau` runs on the client.
- Prefer `.luau` over `.lua`.

## Rules
- One bootstrapper per side: one server Script and one client LocalScript require an ordered list of modules, call `:Init()` on all, then `:Start()` on all. Two phases avoid ordering races.
- No loose Scripts scattered in the tree.
- Group by feature in large codebases (`features/combat/{server,client,shared}`).
- No circular `require`; move shared types and constants to a leaf module.
- Never require across the client/server boundary; shared code only through ReplicatedStorage.
- `shared/Data` is content (items, prices, shops); `shared/Configs` is tuning knobs. Freeze both with `table.freeze`. Server validation and client display read the same source.

## Studio files every project carries
`selene.toml` (`std = "roblox"`), `stylua.toml`, `tests/run.luau`. See [[13-Luau-Engineering/headless-test-gate]].
