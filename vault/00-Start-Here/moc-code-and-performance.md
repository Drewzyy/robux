---
description: Map of notes on Luau engineering, project setup, security, data saving, testing, bugs, profiling, and low-end phone performance.
type: reference
status: draft
last_checked: 2026-11-15
keywords: luau, code, rojo, datastore, remotes, security, performance, profiler, memory leaks, lag, qa, testing, toolchain, bugs
sources: vault notes only, read 2026-11-15 (front matter of each linked note)
verify: Built from note descriptions; each linked note carries its own source and verify line.
tags: [start-here, reference]
---
# Code and Performance
Use when writing code, fixing bugs, hardening against exploits, or making the game run fast.

## Do this
- Read first, step 1: [[13-Luau-Engineering/smallest-complete-change]] (how agents write code).
- Read first, step 2: [[13-Luau-Engineering/remotes-and-security]] (never trust the client).
- Read first, step 3: [[14-QA-Optimization/optimization-checklist]] (stay in budget).
- Then pick the section below that matches your task and read only those notes.

## How to use this map
- Read the three notes under Do this, then open one section below.
- Each note has its own Do this list; act on that, not on this page.
- If a note is marked UNVERIFIED, tune from your own analytics.

## Set up
- [[13-Luau-Engineering/project-structure]]: Rojo layout and naming.
- [[13-Luau-Engineering/toolchain]]: Rojo, Lune, Selene, StyLua, Wally.
- [[13-Luau-Engineering/framework-choice]]: Knit, Flamework or custom.
- [[13-Luau-Engineering/service-controller-template]]: server and client bootstrappers.
- [[13-Luau-Engineering/smallest-complete-change]]: write less, reuse first.
- [[13-Luau-Engineering/index]]: section index.

## Data and security
- [[13-Luau-Engineering/datastore-playbook]]: ProfileStore, session locking.
- [[13-Luau-Engineering/remotes-and-security]]: validation order, NaN, rate limits.
- [[13-Luau-Engineering/remote-validation-cookbook]]: copy-paste validators.
- [[13-Luau-Engineering/networking-hardening]]: transport and layered validation.
- [[13-Luau-Engineering/cooldowns-and-rate-limits]]: timestamp cooldowns.
- [[14-QA-Optimization/exploit-testing]]: attack your own build.
- [[07-Monetization/processreceipt-template]]: safe purchase code.

## Patterns
- [[13-Luau-Engineering/state-machines]]: typed FSM.
- [[13-Luau-Engineering/object-pooling]]: reuse instances.
- [[13-Luau-Engineering/tween-and-animation-cleanup]]: tween lifecycle.
- [[13-Luau-Engineering/memory-leak-checklist]]: connections and player-keyed tables.
- [[17-Development-Templates/engagement-template]]: favorite reminder code.

## Testing and bugs
- [[13-Luau-Engineering/headless-test-gate]]: rojo build, Selene, StyLua, Lune.
- [[13-Luau-Engineering/common-bugs-checklist]]: pre-commit review list.
- [[14-QA-Optimization/bug-triage-and-repro]]: severity and repro.
- [[14-QA-Optimization/playtest-qa-checklist]]: playtest QA.
- [[14-QA-Optimization/device-test-matrix]]: devices and networks.

## Performance
- [[14-QA-Optimization/optimization-checklist]]: measure and budget.
- [[14-QA-Optimization/performance-budgets]]: tiered budgets, 16.67 ms frame.
- [[14-QA-Optimization/lag-sources-ranked]]: usual causes of lag.
- [[14-QA-Optimization/microprofiler-and-stats-guide]]: find slow frames.
- [[14-QA-Optimization/mobile-performance-cookbook]]: low-end phone steps.
- [[12-Multiplayer-NPCs/server-size-and-streaming]]: StreamingEnabled.
- [[10-Art-Environment/3d-model-checklist]]: cheap models.

## Agent rules
- [[17-Development-Templates/anti-slop-checklist]]: quality gate for output.
- [[17-Development-Templates/agent-output-format]]: how agents report.

## For our Roblox games
- Copy the principle: server authority, validate every argument, save safely, measure before optimizing. Never copy code you cannot license, and never use Toolbox scripts.
- Only free tools: no paid plugins or services.
- Mark untested behaviour UNVERIFIED in your report.
