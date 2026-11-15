---
description: Test and design for latency, jitter and packet loss on Roblox: Network Simulator presets (8 to 180 ms), a repeatable test routine, client-side feedback patterns, and server frame rate checks.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: latency, ping, jitter, packet loss, network simulator, lag, prediction, rollback, server authority, replication, retry, timeout, microprofiler, mobile
sources: creator-docs content/en-us/studio/network-simulator.md (read 2026-11-15); creator-docs content/en-us/studio/testing-modes.md (read 2026-11-15); creator-docs content/en-us/production/analytics/performance.md (read 2026-11-15)
verify: Preset values, ranges and the test routine read in docs 2026-11-15. The simulator is beta and affects Studio playtests only. Which delay players accept for each action type is our judgement: UNVERIFIED.
tags: [qa-optimization, playbook]
---
# Network lag and latency

Players on phones and weak Wi-Fi see delay, uneven updates and lost packets. Design so the game still feels right, and test with Studio's Network Simulator. Ranked causes are in [[14-QA-Optimization/lag-sources-ranked]] and security side in [[13-Luau-Engineering/networking-hardening]].

## Do this
- Give instant local feedback (sound, animation, button state) the moment a player acts, then confirm with the server. Never leave a button looking dead while waiting.
- Block double submits: disable the button or ignore repeats while a request is in flight, and make server handlers safe to receive twice.
- Show a loading state that is different from a failure, and give a timeout with a safe retry.
- Treat unreliable updates (movement, effects) as skippable: the next update replaces the old one, with no stale state left on screen.
- Recover after a bad connection ends: resync state without a rejoin.
- Keep the server frame rate at 60. Docs: drops in server frame rate can lower client frame rate and increase latency.
- Test the same action under Ideal Fiber, Home Wi-Fi, 4G and 3G presets and write down the results.

## Network Simulator
- Enable: File > Beta Features > New Device Simulator, restart Studio. Then Test > Device Simulator, and the Network pill above the viewport.
- Works on Studio playtests only. It does not change live players.
- Controls per direction: latency 0 to 1000 ms, jitter 0 to 1000 ms, packet loss 0.00 to 0.50 percent (0.50 means half a percent).
- Ping is round trip, so inbound plus outbound delay both count (10 ms each way adds about 20 ms).
- Selecting a preset only stages values until you press Apply; applying takes effect in the running playtest.
- In a Team Test, geography already adds real latency and loss on top of the simulation.

| Preset | Inbound (latency, jitter, loss) | Outbound |
|---|---|---|
| Ideal Fiber | 8 ms, 0, 0 percent | 8 ms, 0, 0 |
| Wired Broadband | 25, 3, 0 | 25, 3, 0 |
| Home Wi-Fi | 30, 12, 0.20 | 30, 15, 0.30 |
| Standard Mobile 4G | 45, 20, 0.40 | 55, 30, 0.50 |
| Bad Connection 3G | 150, 70, 0.50 | 180, 90, 0.50 |

For a true zero-delay comparison set every value to 0 as a custom setting; Ideal Fiber still adds 8 ms each way.

## Repeatable routine (docs)
1. Pick a journey that crosses client and server: ability use, inventory update, replicated movement.
2. Define success: response time, visible feedback, timeout behavior, corrections, duplicate requests, recovery.
3. Baseline on Ideal Fiber.
4. Repeat on Home Wi-Fi or 4G.
5. Repeat on 3G as a resilience test: the game must stay understandable.
6. Change only inbound or only outbound to find which path hurts.
7. Fix, rerun, compare. Apply the condition before starting a run, because big mid-run changes upset congestion control.

## Diagnose
- Developer Console shows average ping and logs.
- MicroProfiler network capture shows sent and received traffic per frame, to see if replication or physics dominates ([[14-QA-Optimization/microprofiler-and-stats-guide]]).
- MicroProfiler also separates a network delay from a frame-time problem.
- Keep the same scenario and mode between captures.

## Design tips by feature
| Feature | Tip |
|---|---|
| Shop and purchase | Spinner, one tap one request, show result only from the server |
| Combat | Local hit feedback now, damage truth from the server ([[02-Game-Design/pvp-fairness-and-balance]]) |
| Movement of others | Interpolate, never teleport on every packet |
| Round timers | Use server time stamps, not countdown on each client |
| Trading | Confirm with a clear final screen from the server ([[12-Multiplayer-NPCs/trading-system-safety]]) |

## For our Roblox games
Copy the principle: act locally, confirm on the server, survive a 3G connection, and measure with the same routine each release.

Never copy: another game's netcode or exploit-prone shortcuts. Do not trust the client for rewards or currency to hide lag.
