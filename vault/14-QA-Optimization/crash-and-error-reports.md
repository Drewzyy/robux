---
description: Read Creator Hub Error Report, Client crash rate and Server Crashes (out-of-memory snapshots) to find regressions after an update, with custom rules, alerts and a fix routine.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: crash, error report, server crashes, out of memory, memory snapshot, treemap, stack trace, crash rate, alerts, custom rules, place version, memory leak, performance
sources: creator-docs content/en-us/production/analytics/crashes.md (read 2026-11-15); creator-docs content/en-us/production/analytics/error-report.md (read 2026-11-15); creator-docs content/en-us/production/analytics/performance.md (read 2026-11-15); creator-docs content/en-us/production/analytics/alerts.md (read 2026-11-15)
verify: Chart names, filters, 100 DAU and rule limits read in the docs 2026-11-15. Thresholds marked docs example come from the alerts page; other thresholds are ours and UNVERIFIED.
tags: [qa-optimization, playbook]
---
# Crash and error reports

Where Roblox shows what broke in a live game, and how to turn it into a fix. Ties into release-qa-checklist and [[13-Luau-Engineering/memory-leak-checklist]].

## Do this
- After every release, open Monitoring > Performance and Error Report and compare the newest Place Version with older ones (Breakdown by Place Version).
- Treat a rising client crash rate or unexpected out-of-memory exits right after an update as a regression in that update.
- On the Error Report use New errors since and pick the latest version. Fix the highest Count first, then new first-seen errors.
- Expand each row to read the stack trace and find the script and line.
- Use custom rules sparingly: Ignore permanent noise, Group related messages (up to 100 rules; they run top to bottom and stop at the first match).
- For server crashes, split the Crashes chart by reason. Out-of-memory is yours to fix. Platform crashes are Roblox's; file a bug report if they climb.
- Create alerts for crash rate and memory (up to 20 per game, needs 100+ DAU) so the team is told.
- Log each issue with version, platform, count and fix in the bug list ([[14-QA-Optimization/bug-triage-and-repro]]).

## Client crash rate
- Crashes divided by ended sessions, by device type. A dashed line means too few sessions, so the point is noisy; watch Client crash count too.
- Unexpected out-of-memory exits show memory regressions after an update (directionally accurate, may miss some).
- Client memory usage and memory percentage: a line near 100 percent on Android means most Android devices are at risk.
- Client CPU time must stay under 16 ms to reach 60 frames per second.
- Filter by memory group (under 2 GB up to over 8 GB) to find weak devices.

## Server crashes and memory snapshots
- When a server nears out-of-memory, the engine saves a compact JSON summary of the data model before shutdown.
- Pick a snapshot by timestamp, place ID, place version, server version or uptime. A server that crashed minutes after start points to a spike during initialization. A long-uptime crash points to a leak.
- Open the treemap: nodes are sized by memory. Click in to see which services, folders or assets dominate. Use breadcrumbs to find the parent container, then search it in Studio.
- Docs example: ParticleEmitter was half of a Clouds node because heavy assets were cloned at start and never parented or destroyed. Fix by cleanup logic, then check the next update.
- You can download the raw CSV for your own analysis.
- Server targets from the Performance page: keep memory below 50 percent of total (the chart does not show the total, so stay below 3 GB), server frame rate near 60, and server CPU under 16 ms.
- Server memory by age rising over time means a leak in scripts, physics or rendering categories.

## Fix routine
1. Spot: chart spike or alert, with the time and version.
2. Narrow: platform, OS, place, memory group, server or client.
3. Read: stack trace or treemap node.
4. Reproduce in Studio using Server & Clients and the Network and Device simulators if needed.
5. Fix with the smallest change ([[13-Luau-Engineering/smallest-complete-change]]).
6. Ship, then confirm in the same chart on the new version. Keep the note.

## Common causes (from the docs and our checklists)
- Objects cloned but never destroyed; connections never disconnected ([[13-Luau-Engineering/memory-leak-checklist]]).
- Heavy assets loaded at startup rather than streamed.
- Unfiltered spam in output: noisy warnings hide real errors; group them.
- Large textures or many particle emitters on phones.

## Reading three common patterns
- Crash rate jumps only on one platform: look at memory percentage and memory group for that device class; likely textures, meshes or particle counts.
- Errors spike only at the newest version: open New errors since and read the first-seen version; the cause is in that diff.
- Server crashes rise with uptime: server memory by age grows; a leak in scripts, physics or rendering. Restart cadence is only a stopgap.

## Weekly habit
- Monday: look at last 7 days of crash rate and top five errors.
- After each release: compare version breakdown at 1 hour and at 1 day.
- Keep a short log: date, version, symptom, root cause, fix, result.

## For our Roblox games
Copy the principle: every release is compared to the previous version in the same charts, and alerts watch what we cannot.

Never copy: another game's code or error text. Do not collect extra personal data in our own logs; log version, platform and counts only.
