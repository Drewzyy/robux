---
description: Finding bugs and exploits before players do, and keeping frame rate and memory inside budget.
type: section-index
status: verified
last_checked: 2026-11-11
verify: scripts/check_vault.py --full checks every link below resolves and every status matches the note.
tags: [qa-optimization, section-index]
---
# 14-QA-Optimization

Finding bugs and exploits before players do, and keeping frame rate and memory inside budget.

Load only the note your task needs. Do not read every note in this section.

| Note | What it holds | Status |
|---|---|---|
| [[14-QA-Optimization/performance-budgets]] | Frame, memory, part count, streaming, particles: tiered budgets by device, and how to measure. | draft |
| [[14-QA-Optimization/playtest-qa-checklist]] | Spawn, first 60 seconds, every stage readable, checkpoints, coins, shop, no early pop-ups, PC/phone/gamepad. | draft |
| [[14-QA-Optimization/exploit-testing]] | Five ways to attack every remote, and what must never run on the client. | draft |
| [[14-QA-Optimization/optimization-checklist]] | Developer Console, MicroProfiler, test on phones, the usual fixes, link to budgets. | draft |
| [[14-QA-Optimization/bug-triage-and-repro]] | How to triage Roblox game bugs by severity and reproduce them reliably: a report template, repro steps for server, client and multiplayer bugs, log capture, and fix verification. | draft |
| [[14-QA-Optimization/device-test-matrix]] | A minimal device and network test matrix for Roblox games: which platforms, screen sizes, input types and Network Simulator presets to cover, plus the pass criteria and release gate. | draft |
| [[14-QA-Optimization/lag-sources-ranked]] | Ranked list of the usual causes of Roblox lag (server heartbeat, replication, physics, scripts, rendering, memory) with how to spot each one and the first fix to try. | draft |
| [[14-QA-Optimization/microprofiler-and-stats-guide]] | How to find slow frames with the MicroProfiler, Script Profiler, Developer Console Memory and Luau heap tools, and the Creator Hub Performance dashboard; what to capture and how to read it. | draft |
| [[14-QA-Optimization/mobile-performance-cookbook]] | Concrete steps to make a Roblox game run on low-end phones: what to cut first, UI and effect limits, streaming, memory headroom, and how to verify with Device Simulator and the performance dashboard. | draft |
