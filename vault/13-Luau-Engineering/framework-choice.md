---
description: Knit vs Flamework vs a custom bootstrapper, with the decision rule. Pick one and never mix two DI frameworks.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2; carmodyofficial-ops roblox_systems_architecture; anzordatunasvhili-beep roblox_key_knit CLAUDE.md
verify: Community comparison. Before adopting Knit, check its Wally page (wally.run) for the current version and maintenance status: UNVERIFIED here.
---
# Framework choice

| | Knit | Flamework (roblox-ts) | Custom bootstrapper |
|---|---|---|---|
| Shape | Services (server) and Controllers (client); `.Client` table auto-exposes remotes | decorator DI (`@Service`, `@Controller`, `@Component`), compile-time resolution | a two-phase loader plus a small service table |
| Strength | simple, battle-tested, auto networking | best typing and testability | no magic, full control |
| Weakness | weaker typing, runtime string lookups | needs the TypeScript toolchain | you write the plumbing |
| Lifecycle | `KnitInit` then `KnitStart` | decorators | `Init` then `Start` |

## Decision rule
- Game 1, solo, Luau: **custom bootstrapper** (fewest moving parts, nothing to install, every line readable by agents). See [[13-Luau-Engineering/service-controller-template]].
- Fast Luau team that wants auto networking: Knit.
- Large typed codebase: Flamework with roblox-ts.
- Pick one. Never mix two DI frameworks.

## Knit rules if chosen
One `Knit.Start()` per side, never twice. Add all services before `Knit.Start()`. Pure utilities are plain modules, not services.
