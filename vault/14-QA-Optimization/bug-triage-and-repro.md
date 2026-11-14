---
description: How to triage Roblox game bugs by severity and reproduce them reliably: a report template, repro steps for server, client and multiplayer bugs, log capture, and fix verification.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: bug triage, repro steps, severity, bug report, output log, error report, crash, regression, place version, team test, minimal repro, priority, hotfix
sources: creator-docs content/en-us/production/analytics/error-report.md (read 2026-11-14); creator-docs content/en-us/production/analytics/crashes.md (read 2026-11-14); creator-docs content/en-us/studio/testing-modes.md (read 2026-11-14)
verify: Triage tiers and templates are model knowledge, UNVERIFIED against any studio standard. The analytics pages named are in the local clone; their contents were skimmed by title, check them for current UI paths.
tags: [qa-optimization, playbook]
---
# Bug triage and repro

A bug you cannot reproduce is a bug you cannot fix. Triage decides order; repro makes the fix possible.

## Do this
- Triage every bug into one of four severities within 24 hours: S1 data loss, exploit, crash or cannot play; S2 core loop broken; S3 wrong but playable; S4 cosmetic.
- Fix order: S1 now (hotfix), S2 this build, S3 next build, S4 when idle. Never let S4 work block S1.
- Treat data loss and duplication as S1 always (see [[13-Luau-Engineering/datastore-playbook]]).
- Treat exploits as S1 until proven harmless (see [[14-QA-Optimization/exploit-testing]]).
- Write the report before fixing: title, severity, place version, platform, steps, expected, actual, frequency, log lines, evidence.
- Reproduce in Studio first: Play (solo), then Local Server with 2 or more clients (Test tab), then a Team Test if it depends on real latency or geography.
- Minimise the repro: remove steps until the bug still happens. Fewer steps means a faster fix.
- Count frequency: "5 of 5", "2 of 10". Intermittent bugs are usually races (event order, `WaitForChild`, streaming, load order).
- After the fix, re-run the exact repro and one neighbor case. Add a regression note to the playtest checklist (see [[14-QA-Optimization/playtest-qa-checklist]]).

## Report template
```
Title: [S2] Round never ends when last player leaves during Results
Version: place v142, 2026-11-14
Where: server (Round script) / client / both
Platform: Studio local server; also seen on Android phone
Steps:
 1. Start round with 2 players
 2. Player B leaves during Results
 3. Wait 30 s
Expected: next Intermission starts
Actual: stuck on Results, timer shows 0
Frequency: 4 of 5
Log: "attempt to index nil with 'Name'" at RoundService:88
Evidence: video or screenshot link
Suspect: participants table not cleared on PlayerRemoving
```

## Where evidence comes from
- Output window and the server log (Developer Console, Log tab, Server).
- Creator Hub error reports and crash pages (analytics, error report, crashes) group errors by message and count; sort by count and look at the newest version.
- Place version breakdown: if it began at v140, diff v139 to v140.
- Network Simulator presets to reproduce lag-only bugs (see [[14-QA-Optimization/device-test-matrix]]).
- Video from the reporter, plus device model and OS.

## Repro patterns
- Race conditions: add `task.wait(0.5)` or reduce speed in one script to widen the window; test with Network Simulator latency added.
- Streaming bugs: use a small streaming radius in a Studio test (see [[12-Multiplayer-NPCs/server-size-and-streaming]]).
- Respawn bugs: reset the character 10 times in a row.
- Join/leave bugs: script a test that adds and removes bots or uses multiple clients.
- Data bugs: test with a fresh account (no data), an old account (old schema) and a rejoin within 5 seconds.
- Mobile bugs: Device Simulator for layout; real phone for performance.

## Triage meeting (15 minutes weekly)
1. Sort open bugs by severity and then frequency.
2. Close duplicates, close "cannot reproduce" after two failed attempts and a request for more info.
3. Assign each S1 and S2 an owner and a day.
4. Check the dashboard for new top errors.

## Mistakes
- "Doesn't work" reports with no steps, version or platform.
- Fixing the symptom (nil check) without finding why it was nil.
- Not retesting on the platform where it was seen.
- Closing a flaky bug after one passing run.
- Mixing feature requests into the bug list.

## Related
[[14-QA-Optimization/playtest-qa-checklist]], [[14-QA-Optimization/microprofiler-and-stats-guide]], [[13-Luau-Engineering/common-bugs-checklist]], [[13-Luau-Engineering/headless-test-gate]]

## For our Roblox games
Copy the principle: severity tiers, minimal repro, frequency counts, version breakdown. No names, art or assets are involved. Zero spend: free Studio and Creator Hub tools.
