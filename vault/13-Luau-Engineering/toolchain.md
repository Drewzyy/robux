---
description: Free toolchain: Rojo, Lune, Selene, StyLua, luau-lsp, Wally, Rokit, rbxcloud, Open Cloud. Versions installed and tested in this studio.
type: reference
status: verified
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2; crates.io; quanticsoul4772 non-MCP tools guide; justinclevenger and el4cteo tooling notes
verify: Verified 2026-11-09: rojo 7.7.1, selene 0.31.0, stylua 2.6.0, lune 0.11.1 installed from crates.io and run by scripts/gates/build_gate.py. Other tools listed are UNVERIFIED here (not installed).
tags: [luau-engineering, reference]
---
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
