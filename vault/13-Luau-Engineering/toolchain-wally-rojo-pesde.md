---
description: Rojo, Wally, pesde, Rokit, StyLua, luau-lsp, Lune and roblox-ts compared: what each does, pinned-version setup for an agent-built game, stars, licenses and pitfalls.
type: reference
status: draft
last_checked: 2026-11-15
keywords: rojo, wally, pesde, rokit, aftman, stylua, luau-lsp, lune, roblox-ts, darklua, package manager, project.json, sourcemap, syncback, toolchain
sources: https://github.com/rojo-rbx/rojo (README.md, AGENTS.md, CHANGELOG.md, read 2026-11-15); https://github.com/UpliftGames/wally (README.md, CHANGELOG.md, read 2026-11-15); https://github.com/frrazer/roblox-game-boilerplate (aftman.toml, wally.toml, default.project.json, AGENTS.md, read 2026-11-15); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. Rojo 7.7.1 is the newest version in the cloned changelog. pesde, Lune, StyLua and luau-lsp were not cloned, so details on them are UNVERIFIED beyond search descriptions.
tags: [luau-engineering, reference]
---
# Toolchain: Rojo, Wally, pesde and friends

Files on disk plus Git give agents a reviewable, diffable project. Rojo syncs them into Studio; Wally or pesde pull in libraries.

| Repo | Stars | License | Job |
|---|---|---|---|
| rojo-rbx/rojo | 1874 | MPL-2.0 | Sync files to Studio, build place files |
| UpliftGames/wally | 514 | MPL-2.0 | Package manager (Cargo-like) |
| pesde-pkg/pesde | 132 | MIT | Newer Luau package manager, multi runtime |
| rojo-rbx/rokit | 514 | MIT | Installs pinned tool versions |
| JohnnyMorganz/StyLua | 2338 | MPL-2.0 | Formatter |
| JohnnyMorganz/luau-lsp | 567 | MIT | Type checking and language server |
| lune-org/lune | 982 | MPL-2.0 | Run Luau scripts outside Roblox |
| roblox-ts/roblox-ts | 1336 | MIT | TypeScript to Luau compiler |
| rojo-rbx/remodel | 176 | MIT | Script place and model files |

## Do this
- Make a Rojo project the source of truth for code. Keep Workspace, Lighting and GUI art in the saved place file edited in Studio, and map only script folders (the boilerplate does this with `$ignoreUnknownInstances`).
- Pin tool versions in a manifest (Rokit reads aftman.toml and foreman.toml files too). The boilerplate pins Rojo 7.7.0, Wally 0.3.2, StyLua 2.5.2, luau-lsp 1.69.0.
- Run `rojo serve default.project.json`, then connect the Rojo plugin in Studio (the boilerplate uses port 34873).
- Install packages through Wally with `wally install`, then run wally-package-types so strict code sees package types. Never edit the generated Packages folder.
- Put shared packages in ReplicatedStorage.Packages and server-only ones (ProfileStore) in a ServerPackages folder under ServerScriptService, so clients never receive them.
- Run the formatter, type check and a `rojo build` in one check script after every change; fix everything before handing off.
- Use `rojo syncback` only for one-time import of an existing place; the README says full automatic conversion is still limited.
- Use Lune for build or test scripts (compare outputs, validate JSON data) that do not need the engine.
- Prefer plain Luau with `--!strict`. Use roblox-ts only when everyone already knows TypeScript.

## Project file sketch (our style)
```json
{
  "name": "OurGame",
  "tree": {
    "$className": "DataModel",
    "ReplicatedStorage": { "$path": "src/shared", "Packages": { "$path": "Packages" } },
    "ServerScriptService": { "$path": "src/server", "ServerPackages": { "$path": "ServerPackages" } },
    "StarterPlayer": { "StarterPlayerScripts": { "$path": "src/client" } }
  }
}
```
```toml
# wally.toml (dependency names are examples, pin real versions)
[dependencies]
Signal = "sleitnick/signal@2.0.3"
Trove = "sleitnick/trove@1.8.0"
[server-dependencies]
ProfileStore = "lm-loleris/profilestore@1.0.3"
```

## Pitfalls
- Without `$ignoreUnknownInstances`, Rojo may delete things you built in Studio. Set it on mapped services.
- Rojo output from `rojo build` is code only. Do not treat it as the shipped place.
- The Wally registry is a Git repo; a missing registry line or wrong realm (shared, server, dev) gives wrong package folders.
- Mixing Wally and pesde in one project doubles the package folders and confuses type tools. Choose one.
- Some tools check licenses: MPL-2.0 tools (Rojo, Wally, StyLua) are fine to run; only modified copies of their own files must stay MPL.
- Studio must have API access enabled and the place published before DataStore tests hit live keys.

## For our Roblox games
- Copy the principle: pinned versions, one check command, code on disk, art in Studio.
- Never copy another team's package list or private tokens; choose packages by need.
- Related: [[13-Luau-Engineering/toolchain]], [[13-Luau-Engineering/project-structure]], [[13-Luau-Engineering/headless-test-gate]].
