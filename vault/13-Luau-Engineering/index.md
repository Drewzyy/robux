---
description: Code patterns the agents must not guess: Rojo layout, framework choice, remotes and security, ProfileStore, toolchain, performance budgets.
type: section-index
status: verified
last_checked: 2026-11-09
sources: Research report C2
verify: scripts/check_vault.py --full checks every link below resolves and every status matches the note.
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
