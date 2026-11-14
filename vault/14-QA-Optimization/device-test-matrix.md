---
description: A minimal device and network test matrix for Roblox games: which platforms, screen sizes, input types and Network Simulator presets to cover, plus the pass criteria and release gate.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: device matrix, test matrix, mobile, tablet, console, gamepad, Network Simulator, Device Simulator, safe area, aspect ratio, low-end, input types, release gate, controller emulator
sources: creator-docs content/en-us/studio/device-simulator.md (read 2026-11-14); creator-docs content/en-us/studio/network-simulator.md (presets table, read 2026-11-14); creator-docs content/en-us/studio/controller-emulator.md (read 2026-11-14); creator-docs content/en-us/production/game-design/design-for-roblox.md (read 2026-11-14)
verify: Network preset numbers are copied as figures from the local Network Simulator doc. Device categories come from the Device Simulator doc. Which real phones to buy is UNVERIFIED; with zero spend use any old phone you or friends already own.
tags: [qa-optimization, checklist]
---
# Device test matrix

You cannot test everything. Test a small matrix that covers the biggest risks: small screens, weak hardware, touch input, gamepad input and bad networks.

## Do this
- Cover five device classes: low-end phone, mid or new phone, tablet, desktop, console or gamepad. Test touch, mouse and keyboard, and gamepad input at least once each.
- Use Studio first: the Device Simulator toolbar (device categories per the docs) for screen size, pixel density and on-screen keyboard; the Controller Emulator for gamepad flows.
- Use the Network Simulator presets for latency: Ideal Fiber, Wired Broadband, Home Wi-Fi, Standard Mobile (4G), Bad Connection (3G). Test your core loop on at least Home Wi-Fi, 4G and 3G.
- Use at least one real low-end phone for performance. Simulators do not reproduce CPU or GPU limits.
- Run the full matrix before each major release, and a smoke test (join, play one round, buy one item, leave) on 3 devices before every update.
- Gate the release: no S1 or S2 bug on any row, frame rate meets target on the low-end phone, no clipped UI on any aspect ratio.

## Matrix

| Row | Device | Input | Network | Check |
|---|---|---|---|---|
| 1 | Low-end Android phone | touch | 4G | 30 fps, no OOM in 15 min, UI fits |
| 2 | Mid iPhone or Android | touch | Home Wi-Fi | 60 fps target, safe area ok |
| 3 | Tablet (large aspect) | touch | Home Wi-Fi | UI scales, no tiny controls |
| 4 | Desktop | mouse + keyboard | Wired | all hotkeys, window resize |
| 5 | Gamepad (emulator or console) | gamepad | Wired | every button reachable, focus navigation works |
| 6 | Any phone | touch | 3G preset | playable at about 150 ms in, 180 ms out |

Preset values for reference, from the docs table (latency / jitter / loss): 4G inbound 45 ms / 20 ms / 0.40%, outbound 55 ms / 30 ms / 0.50%. 3G inbound 150 ms / 70 ms / 0.50%, outbound 180 ms / 90 ms / 0.50%.

## What to check per row
- Layout: nothing hidden by notches or the top bar; buttons at least 44 px; text readable at the smallest screen.
- Input: every action has a touch button, a key and a gamepad binding. No hover-only interactions on touch.
- Performance: frame time, memory growth, battery heat after 15 minutes (see [[14-QA-Optimization/mobile-performance-cookbook]]).
- Network: shots, pickups, shop and round start all work at 3G; client prediction corrects without rubber-banding; no stuck "loading" states.
- Streaming: no falling through the world after teleport on slow connections.
- Permissions: purchase prompts, social prompts and chat work or degrade politely.

## Network test script
1. Open the Network Simulator in Studio and apply the preset before the playtest starts (the docs say large changes mid-run distort results).
2. Play the core loop three times.
3. Record any action with visible delay above 300 ms, any desync, any stuck state.
4. Repeat with packet loss presets or custom values, then try high jitter.

## Release gate checklist
- [ ] Row 1 meets frame and memory targets
- [ ] No clipped UI on rows 2 and 3
- [ ] Gamepad can finish the core loop (row 5)
- [ ] 3G run completes without stuck states (row 6)
- [ ] Performance dashboard shows no new crash or OOM spike after the previous update (see [[14-QA-Optimization/microprofiler-and-stats-guide]])
- [ ] Exploit smoke test done (see [[14-QA-Optimization/exploit-testing]])

## Mistakes
- Testing only on the fastest device.
- Trusting a simulator for performance.
- Testing 3G only after launch complaints.
- Forgetting gamepad and then losing console players.
- Skipping the smoke test for "small" updates.

## Related
[[14-QA-Optimization/playtest-qa-checklist]], [[14-QA-Optimization/bug-triage-and-repro]], [[14-QA-Optimization/performance-budgets]]

## For our Roblox games
Copy the principle: a small fixed matrix, weak device first, network presets, a hard release gate. No names, art or assets are involved. Zero spend: Studio simulators plus phones you already have; do not buy test devices or paid testing services.
