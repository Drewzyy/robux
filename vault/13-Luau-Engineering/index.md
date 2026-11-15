---
description: Code patterns the agents must not guess: Rojo layout, framework choice, remotes and security, ProfileStore, toolchain, performance budgets.
type: section-index
status: verified
last_checked: 2026-11-09
sources: Research report C2
verify: scripts/check_vault.py --full checks every link below resolves and every status matches the note.
tags: [luau-engineering, section-index]
---
# 13-Luau-Engineering

Code patterns the agents must not guess: Rojo layout, framework choice, remotes and security, ProfileStore, toolchain, performance budgets.

Load only the note your task needs. Do not read every note in this section.

| Note | What it holds | Status |
|---|---|---|
| [[13-Luau-Engineering/project-structure]] | Canonical default.project.json mapping, file naming (.server.luau, .client.luau), one bootstrapper per side. | verified |
| [[13-Luau-Engineering/framework-choice]] | Knit vs Flamework vs custom bootstrapper, with a decision rule. Pick one, never mix two DI frameworks. | draft |
| [[13-Luau-Engineering/service-controller-template]] | Copy-paste starter: one service, one controller, shared data module, two-phase Init/Start. | verified |
| [[13-Luau-Engineering/remotes-and-security]] | Never trust the client: validation order, NaN and math.huge, rate limits, server-owned state, RemoteFunction hazards. | draft |
| [[13-Luau-Engineering/datastore-playbook]] | ProfileStore by default: session locking, BindToClose, autosave, failed-load kick, throttling, size limits. | draft |
| [[13-Luau-Engineering/common-bugs-checklist]] | Pre-commit review list of the bugs that lose data or get exploited. | draft |
| [[13-Luau-Engineering/toolchain]] | Rokit, Wally, StyLua, Selene, luau-lsp, Lune, rbxcloud, Rojo, GitHub Actions CI; pinned versions. | verified |
| [[13-Luau-Engineering/headless-test-gate]] | The Builder's build gate: rojo build, Selene, StyLua check, Lune unit tests. What passing means. | verified |
| [[13-Luau-Engineering/networking-hardening]] | Transport table, layered validation, yield race, token bucket, limits, Server Authority. | draft |
| [[13-Luau-Engineering/smallest-complete-change]] | Least new code that fully works, reuse first, never cut safety, name what was skipped. Adapted from ponytail. | verified |
| [[13-Luau-Engineering/cooldowns-and-rate-limits]] | Server-side cooldowns and per-player rate limits in Luau: timestamp cooldowns, token buckets, per-ability tables, cleanup on leave, and numbers for common actions. | draft |
| [[13-Luau-Engineering/memory-leak-checklist]] | Find and prevent Roblox memory leaks in Luau: connections, tables keyed by Player, instances kept alive, threads, with a Maid helper and Developer Console steps to confirm growth. | draft |
| [[13-Luau-Engineering/object-pooling]] | Reuse instances instead of Clone/Destroy: a typed pool for projectiles, hit effects, damage numbers and enemies, with prewarm, reset rules, max size and leak guards. | draft |
| [[13-Luau-Engineering/remote-validation-cookbook]] | Copy-paste Luau validators for RemoteEvent arguments: type checks, number ranges, NaN, strings, enums, instances, distance and ownership checks, and a typed handler wrapper. | draft |
| [[13-Luau-Engineering/state-machines]] | A small typed finite state machine in Luau for NPCs, rounds, abilities and UI: transition tables, enter/exit hooks, timeouts, and rules that prevent stuck or illegal states. | draft |
| [[13-Luau-Engineering/tween-and-animation-cleanup]] | Clean tween and animation lifecycle in Luau: cancel old tweens, use Completed correctly, stop AnimationTracks, disconnect on respawn, and avoid tween fights and leaks. | draft |
| [[13-Luau-Engineering/admin-and-debug-tools-cmdr-adonis-iris]] | Cmdr command console, Adonis admin system, and Iris/Plasma immediate-mode debug UI for Roblox: setup, the server-side permission rule, stars, licenses, and when to use each in a kid-safe game. | draft |
| [[13-Luau-Engineering/data-libraries-profilestore]] | ProfileStore (successor to ProfileService) for session-locked player data, plus Lapis and MockDataStoreService, with the start-session pattern, pitfalls, stars and licenses. | draft |
| [[13-Luau-Engineering/ecs-and-character-controllers-jecs-matter-chickynoid]] | When to use an ECS (jecs or Matter) or a server-authoritative character controller (Chickynoid) in a Roblox game, with stars, licenses, short examples and the cases where plain modules win. | draft |
| [[13-Luau-Engineering/frameworks-knit-and-alternatives]] | Knit (archived), AeroGameFramework, Flamework and NevermoreEngine compared, plus the plain Lifecycle pattern from a public-domain boilerplate that replaces them, with stars and licenses. | draft |
| [[13-Luau-Engineering/game-feel-utilities-rbxutil-shake-spring-timer]] | RbxUtil's Shake, Spring, Timer, TaskQueue, WaitFor, Component and TableUtil modules (MIT) for game feel and plumbing, with settings to start from, pitfalls, and a safe-motion rule for kids. | draft |
| [[13-Luau-Engineering/networking-libraries-zap-and-packet]] | Zap, Blink, ByteNet, BridgeNet2 and rbx-net for typed, compact Roblox remote networking, with a Zap schema example, stars, licenses and server-validation pitfalls. | draft |
| [[13-Luau-Engineering/open-source-roblox-games-worth-studying]] | Which public Roblox game and template repos are worth reading (Slither, Matter's Roomba example, Chickynoid demo, boilerplates), what each teaches, their licenses, and which archives to avoid. | draft |
| [[13-Luau-Engineering/state-and-signals-promise-trove-signal]] | How Signal, Promise and Trove (from RbxUtil and evaera's Promise) handle events, async work and cleanup in Roblox games, plus Charm and Silo for state, with stars, licenses and examples. | draft |
| [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] | Rojo, Wally, pesde, Rokit, StyLua, luau-lsp, Lune and roblox-ts compared: what each does, pinned-version setup for an agent-built game, stars, licenses and pitfalls. | draft |
