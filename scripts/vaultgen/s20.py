from common import note
S = "20-luau-rojo"
R = "Research report (Nov 9 2026) C2"

note(f"{S}/project-structure", "Canonical Rojo layout: default.project.json mapping, file naming (.server.luau, .client.luau), one bootstrapper per side, frozen shared data.", "reference", "verified",
 R + "; kanzerog rojo-roblox-skill project-structure; carmodyofficial-ops roblox_systems_architecture",
 "Verified 2026-11-09: the layout below, extracted into a temp project, passed scripts/gates/build_gate.py (rojo build 7.7.1, selene, stylua, lune 0.11.1). See LOG.md Phase 3 spot checks.", r"""
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
`selene.toml` (`std = "roblox"`), `stylua.toml`, `tests/run.luau`. See [[20-luau-rojo/headless-test-gate]].
""")

note(f"{S}/framework-choice", "Knit vs Flamework vs a custom bootstrapper, with the decision rule. Pick one and never mix two DI frameworks.", "reference", "draft",
 R + "; carmodyofficial-ops roblox_systems_architecture; anzordatunasvhili-beep roblox_key_knit CLAUDE.md",
 "Community comparison. Before adopting Knit, check its Wally page (wally.run) for the current version and maintenance status: UNVERIFIED here.", r"""
# Framework choice

| | Knit | Flamework (roblox-ts) | Custom bootstrapper |
|---|---|---|---|
| Shape | Services (server) and Controllers (client); `.Client` table auto-exposes remotes | decorator DI (`@Service`, `@Controller`, `@Component`), compile-time resolution | a two-phase loader plus a small service table |
| Strength | simple, battle-tested, auto networking | best typing and testability | no magic, full control |
| Weakness | weaker typing, runtime string lookups | needs the TypeScript toolchain | you write the plumbing |
| Lifecycle | `KnitInit` then `KnitStart` | decorators | `Init` then `Start` |

## Decision rule
- Game 1, solo, Luau: **custom bootstrapper** (fewest moving parts, nothing to install, every line readable by agents). See [[20-luau-rojo/service-controller-template]].
- Fast Luau team that wants auto networking: Knit.
- Large typed codebase: Flamework with roblox-ts.
- Pick one. Never mix two DI frameworks.

## Knit rules if chosen
One `Knit.Start()` per side, never twice. Add all services before `Knit.Start()`. Pure utilities are plain modules, not services.
""")

note(f"{S}/service-controller-template", "Copy-paste starter: server and client bootstrappers with two-phase Init/Start, one service, one controller, frozen shared config, a Lune test.", "template", "verified",
 R + " (two-phase bootstrap pattern)",
 "Verified 2026-11-09: these files, written to a temp project, passed scripts/gates/build_gate.py (rojo build, selene, stylua --check, lune tests). Remote validation is pattern-only; test in Studio.", r"""
# Service and controller template

Files, relative to the project root. Copy, then rename `Coins`.

`src/server/init.server.luau`
```lua
-- Server bootstrapper: require services, Init all, then Start all.
local Services = script:WaitForChild("Services")

local order = {
	require(Services.CoinService),
}

for _, service in order do
	service:Init()
end
for _, service in order do
	service:Start()
end
```

`src/server/Services/CoinService.luau`
```lua
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Config = require(ReplicatedStorage.Shared.Config)
local Validate = require(ReplicatedStorage.Shared.Validate)

local CoinService = {}
local coins: { [Player]: number } = {}
local remote: RemoteEvent

function CoinService:Init()
	remote = Instance.new("RemoteEvent")
	remote.Name = "RequestCollect"
	remote.Parent = ReplicatedStorage
end

function CoinService:Start()
	Players.PlayerAdded:Connect(function(player)
		coins[player] = 0
	end)
	Players.PlayerRemoving:Connect(function(player)
		coins[player] = nil
	end)
	remote.OnServerEvent:Connect(function(player, amount)
		-- Never trust the client: validate type and range, drop silently.
		if not Validate.isFiniteNumber(amount) then
			return
		end
		if amount < 1 or amount > Config.MaxCollectPerRequest then
			return
		end
		coins[player] = (coins[player] or 0) + math.floor(amount)
	end)
end

return CoinService
```

`src/shared/Config.luau`
```lua
return table.freeze({
	MaxCollectPerRequest = 5,
})
```

`src/shared/Validate.luau` (pure, testable in Lune)
```lua
local Validate = {}

function Validate.isFiniteNumber(x: any): boolean
	return type(x) == "number" and x == x and x ~= math.huge and x ~= -math.huge
end

return table.freeze(Validate)
```

`src/client/init.client.luau`
```lua
-- Client bootstrapper: same two-phase shape for controllers.
local controllers = {}
for _, c in controllers do
	c:Init()
end
for _, c in controllers do
	c:Start()
end
```

`tests/run.luau`
```lua
local Validate = require("../src/shared/Validate")

assert(Validate.isFiniteNumber(3), "3 is finite")
assert(not Validate.isFiniteNumber(0 / 0), "NaN rejected")
assert(not Validate.isFiniteNumber(math.huge), "inf rejected")
assert(not Validate.isFiniteNumber("3"), "string rejected")
print("all tests passed")
```

The real game sends what was collected (a coin part), not an amount; the server decides the amount. This template shows the validation shape only. See [[20-luau-rojo/remotes-and-security]].
""")

note(f"{S}/remotes-and-security", "Never trust the client: validation order, NaN and math.huge, rate limits, server-owned state, RemoteFunction hazards, what replicates.", "reference", "draft",
 R + "; pinheadsk8r, ivar-anon, brockmartin security-hardening references; tabooharmony roblox-networking",
 "Community security references. The NaN rule is checked by the Lune test in service-controller-template. Server Authority status (reported graduated June 2026) is UNVERIFIED; check Creator Docs.", r"""
# Remotes and security

Every RemoteEvent and RemoteFunction argument is attacker-controlled. Exploiters can decompile any replicated script, fire any remote at any rate with any arguments, trigger `Touched` and `ProximityPrompt` from any range, and replay traffic.

## Validate on the server, in order
1. **Type** (`typeof`/`type`). Reject `nil`, wrong types, tables shaped to crash the handler.
2. **Range.** Reject `NaN` (`x ~= x`), `math.huge`, negatives where not allowed. NaN is the dangerous one: it passes `<`/`>` checks and can corrupt saved data.
3. **Permission.** Does this player own this thing, is this action allowed now?
4. **Plausibility.** Distance from the character, cooldown, rate.
Fail by returning early and dropping the request. Do not error loudly to the exploiter.

## Server owns state
Currency, inventory, damage, cooldowns, purchases, progression. Client requests; server decides. Damage is computed by the server from weapon stats, distance, and cooldown, never accepted from the client. Currency math is server-side; the client only displays.

## Rate limits
Limit every remote per player per remote. Roblox's built-in throttle (about 500 requests/second per client) is not a substitute. Use suspicion scoring: accumulate violations, kick at a threshold, do not kick on the first offence.

## RemoteFunction hazards
Never call `RemoteFunction:InvokeClient` for anything you need: a client can never return (the server thread leaks) or return garbage. Use RemoteEvents server to client. Use `UnreliableRemoteEvent` for high-frequency data such as positions.

## What is public
Everything replicated: code in ReplicatedStorage, StarterPlayerScripts, StarterGui; remote names; attribute values. Obfuscation is not security. Design so cheating is impossible or gains nothing.

## Movement and physics
Server Authority is reported (June 2026) as the long-term fix for movement and physics exploits. UNVERIFIED: confirm status in Creator Docs before relying on it.
""")

note(f"{S}/datastore-playbook", "ProfileStore by default: session locking, autosave plus leave plus BindToClose, failed-load kick, UpdateAsync, throttling, 4 MB limit.", "playbook", "draft",
 R + "; mattqdev ProfileService 2026 guide; ivar-anon roblox-datastores; justozy datastore anti-patterns; chguo999 roblox-datastores",
 "Community references. ProfileStore's exact API is UNVERIFIED here: read its official docs (and Wally page) before writing code against it, and test save/load in Studio with API access enabled.", r"""
# DataStore playbook

## Default
**ProfileStore**, the successor to ProfileService, for new work. ProfileService is acceptable (maintenance mode). DataStore2: no session locking, development slowed since 2022; do not use for new games. Raw DataStoreService has no retry, locking, or reconciliation.

## Non-negotiables
- Every DataStore call in `pcall` with retry and exponential backoff.
- Read-modify-write uses `UpdateAsync` (atomic), never `GetAsync` then `SetAsync`.
- One server owns a player at a time (session locking).
- Save on all three: autosave interval, player leaving, and `game:BindToClose`.
- Store only JSON-encodable data: no Instances, CFrames, mixed-key or cyclic tables.

## The number one data-loss bug
Saving only on `PlayerRemoving`. When a server crashes or shuts down, that save may never fire. `game:BindToClose` does fire, with about 30 seconds. Roblox restarts every server on each game update, so without a shutdown flush, progress since the last autosave is lost every update. Save in parallel inside BindToClose; sequential saves can run past 30 seconds.

## Failed loads
If a load fails, do not let the player play on a default profile: the next save overwrites their real data. Kick the player (or refuse to save). A failed load must never look like a new player.

## Throttling and limits
- Writes to the same key are throttled (about one write per key per 6 seconds).
- Check `DataStoreService:GetRequestBudgetForRequestType` before non-critical writes.
- Do not save on every currency change: change in memory, let autosave (ProfileStore default 300 s) and leave-save persist. Lowering the autosave interval is counterproductive.
- 4 MB per key after JSON encoding.
- Do not yield inside an `UpdateAsync` callback.

## Studio
Enable "Studio Access to API Services" or DataStores fail silently in Studio. DataStores do not work from a LocalScript.
""")

note(f"{S}/common-bugs-checklist", "Pre-commit review list: the bugs that lose data, get exploited, or break on mobile. Builder runs this before finishing.", "checklist", "draft",
 R + " (C2 consolidated)", "Checklist distilled from the other 20-luau-rojo notes; each item points to the note that justifies it.", r"""
# Common bugs checklist

Data ([[20-luau-rojo/datastore-playbook]])
- [ ] Saves on autosave, leave, and `BindToClose`, in parallel at shutdown.
- [ ] Failed load kicks or refuses to save; never plays on a default profile.
- [ ] `UpdateAsync` for read-modify-write; every call in `pcall` with retry.
- [ ] No Instances or CFrames in saved data.

Remotes ([[20-luau-rojo/remotes-and-security]])
- [ ] Every handler checks type, range (NaN, inf, negatives), permission, plausibility.
- [ ] Every remote is rate limited per player.
- [ ] No `InvokeClient` for anything needed.
- [ ] Client never sends amounts, damage, or prices.

Structure ([[20-luau-rojo/project-structure]])
- [ ] One bootstrapper per side, Init then Start.
- [ ] No circular requires; no cross-boundary requires.
- [ ] Shared data frozen; no secrets in ReplicatedStorage.

Performance ([[20-luau-rojo/performance-budgets]])
- [ ] Static parts anchored.
- [ ] No heavy work every Heartbeat.

Monetization
- [ ] Every developer product handled in `ProcessReceipt`, idempotent ([[40-monetization/processreceipt-template]]).

Gate ([[20-luau-rojo/headless-test-gate]])
- [ ] `python3 ../scripts/gates/build_gate.py projects/<game>` passes.
""")

note(f"{S}/toolchain", "Free toolchain: Rojo, Lune, Selene, StyLua, luau-lsp, Wally, Rokit, rbxcloud, Open Cloud. Versions installed and tested in this studio.", "reference", "verified",
 R + "; crates.io; quanticsoul4772 non-MCP tools guide; justinclevenger and el4cteo tooling notes",
 "Verified 2026-11-09: rojo 7.7.1, selene 0.31.0, stylua 2.6.0, lune 0.11.1 installed from crates.io and run by scripts/gates/build_gate.py. Other tools listed are UNVERIFIED here (not installed).", r"""
# Toolchain

## Installed and tested in this studio (2026-11-09)
| Tool | Version | Install | Used for |
|---|---|---|---|
| Rojo | 7.7.1 | `cargo install rojo --version 7.7.1 --locked` | file-based builds (`rojo build`), live sync (`rojo serve`) |
| Lune | 0.11.1 | `cargo install lune --version 0.11.1 --locked` | headless Luau: unit tests, build scripts, `.rbxl` files via `@lune/roblox` |
| Selene | 0.31.0 | `cargo install selene --locked` | Luau linter |
| StyLua | 2.6.0 | `cargo install stylua --features luau --locked` | formatter (`stylua --check src` in the gate) |

Selene's `std = "roblox"` downloads the Roblox API dump on first use. Offline, the studio gate falls back to `scripts/gates/roblox-offline.yml` (globals only) and says so.

## Not installed here (UNVERIFIED versions)
- **Rokit**: pins tool versions in `rokit.toml` (replaces Aftman/Foreman). Faster installs from GitHub releases.
- **Wally**: package manager (`wally.toml`, `wally install` to `Packages/`). Run `wally-package-types --sourcemap sourcemap.json Packages/` after install so types survive.
- **luau-lsp**: language server and CLI type checker; pair with the Luau Language Server Companion Studio plugin.
- **Open Cloud**: REST only, no official SDKs. Community: `rbxcloud` (Rust CLI), `rblx-open-cloud` (Python), OpenBlox (TypeScript). Roblox's `place-ci-cd-demo` shows the Luau Execution API for tests.
- **GitHub Actions**: install, format check, lint, type check, build, tests on every push.

## Studio rule
Publishing through Open Cloud is never run by an agent. `rbxcloud` is denied in `vault/.claude/settings.json`.
""")

note(f"{S}/performance-budgets", "Tiered performance budgets: 16.67 ms frame at 60 fps, memory by device tier, part counts, particles, streaming; measure on the weakest device.", "reference", "draft",
 R + "; brockmartin performance-optimization; tabooharmony roblox-performance; el4cteo roblox-performance; andrian-syh device-performance",
 "Frame budget is Roblox guidance. Memory, part, and particle numbers differ between community sources: UNVERIFIED as limits, use as tiered targets and measure with MicroProfiler on a low-end phone.", r"""
# Performance budgets

## Frame
60 fps = 16.67 ms per frame (Roblox guidance); 30 fps = 33.3 ms. Scripts should use a few ms at most. One Heartbeat callback taking 8 ms is half a 60 fps frame.
Targets: 60 fps desktop, 45 to 60 mobile, never under 30.

## Memory (sources disagree; tiered targets)
| Tier | Target | Source notes |
|---|---|---|
| Low-end phone | 400 to 500 MB | one reference also gives 1,000 draw calls, 1,000,000 triangles |
| Mobile | under 600 to 800 MB | 1.2 GB called a crash zone in one reference |
| Desktop | about 1.5 GB | |

## World
- About 500 parts max per model; visible scene under 10,000 parts desktop, 5,000 to 7,000 mobile.
- Anchor every static part (unanchored parts enter the physics solver).
- MeshParts sharing a MeshId are instanced (cheap). Unions can be expensive: check triangles in Scene Analysis.
- Particles: about 100/s mobile, 400 desktop in one reference; cap around 50 active emitters in another. Limit shadow-casting lights, beams, trails.
- Load to playable under 10 s, 20 s at most.

## Streaming
On by default for new places. Start `StreamingTargetRadius` at 256 studs, `StreamingMinRadius` about 64, `StreamOutBehavior = Opportunistic`. Set in Studio on Workspace; not settable by script.

## Measure
On the weakest device, not the dev machine. The Studio device emulator is not a memory test (server and client share one process). Tools: MicroProfiler (Ctrl+F6), Script Profiler, Developer Console (F9), Performance Stats, Creator Hub Performance dashboard. Roblox's worked example test set: Infinix Smart 9, Motorola Moto G05, Oppo A18, Amazon Fire HD 10, Samsung Galaxy S22 Ultra.
""")

note(f"{S}/headless-test-gate", "Builder's build gate: rojo build, Selene, StyLua check, and Lune tests via scripts/gates/build_gate.py. What passing means and does not mean.", "reference", "verified",
 "scripts/gates/build_gate.py; Phase 2 verification (LOG.md)",
 "Verified 2026-11-09: gate passes a known-good project and fails a project with a lint error, bad formatting, and a failing test (LOG.md Phase 2).", r"""
# Headless test gate

Run it yourself before finishing (your working folder is the vault):
```
python3 ../scripts/gates/build_gate.py projects/<game>
```
The studio runs the same script again after you finish; your gate.json does not replace it.

## Checks
| Check | Passes when |
|---|---|
| `rojo build` | `default.project.json` builds a place file |
| `selene lint` | no errors or warnings in `src/` |
| `stylua --check` | `src/` is formatted (run `stylua <abs path>/src` to fix) |
| `lune unit tests` | `tests/run.luau` exits cleanly |

A tool that is missing is a failed check ("NOT RUN"), never a pass.

## Writing tests Lune can run
- Pure logic lives in `src/shared` modules that do not touch `game`, Instances, or services.
- `tests/run.luau` requires them by relative path: `require("../src/shared/Economy")`.
- Use `assert(cond, "message")`; the first failure stops the run with a non-zero exit.

## What passing does NOT mean
It does not mean the game works in Roblox. Rojo builds, lint, format, and pure-logic tests say nothing about remotes, physics, UI, or feel. Selene offline cannot check API member names. Live behaviour needs a Studio playtest (Studio MCP) or Andrew.
""")
print("20 done")
