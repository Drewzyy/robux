---
description: Pre-release QA gate for a Roblox update: Studio test modes (up to 8 clients, Team Test, Device and Network Simulator, Player Emulator), policy checks, and post-release monitoring.
type: checklist
status: draft
last_checked: 2026-11-15
keywords: release, qa, checklist, playtest, team test, device simulator, network simulator, player emulator, multi-client, performance, error report, crash rate, rollback
sources: creator-docs content/en-us/studio/testing-modes.md (read 2026-11-15); creator-docs content/en-us/studio/device-simulator.md (read 2026-11-15); creator-docs content/en-us/studio/network-simulator.md (read 2026-11-15); creator-docs content/en-us/production/analytics/performance.md (read 2026-11-15); creator-docs content/en-us/production/analytics/error-report.md (read 2026-11-15); creator-docs content/en-us/production/analytics/alerts.md (read 2026-11-15); creator-docs content/en-us/production/publishing/publish-games-and-places.md (read 2026-11-15)
verify: Tools and menu names read in the docs 2026-11-15. Device and Network Simulator are marked beta in the docs and need the New Device Simulator beta feature turned on. Pass thresholds are our own suggestions, UNVERIFIED unless noted.
tags: [qa-optimization, checklist]
---
# Release QA checklist

One pass before every publish, plus a watch window after. Gameplay depth is in [[14-QA-Optimization/playtest-qa-checklist]]; this list adds release-specific tool checks.

## Do this
- Run each step below on the exact build you will publish, then publish only when all boxes are ticked.
- Test as a client and server, not only in the single Play mode: use Server & Clients with 2 clients (up to 8 allowed) and F7 to start.
- Test on a phone-sized device in Device Simulator and on at least one real phone ([[14-QA-Optimization/device-test-matrix]]).
- Test under Home Wi-Fi, Standard Mobile (4G/LTE) and Bad Connection (3G) presets with Network Simulator (network-lag-and-latency).
- Use Player Emulator with a test profile to check locale, text stretch (Elongate), pseudolocalized strings and region and policy toggles.
- Set up alerts before release, then watch Performance and Error Report for the new place version.
- Keep a rollback plan: know the last good version before publishing.

## Before publish
1. Output window clean in a full playtest: no red errors, warnings reviewed.
2. First 60 seconds on a fresh test account complete without help ([[04-Onboarding/first-60-seconds-checklist]]).
3. Core loop, shop, save and load, and a rejoin all work. Test leaving mid-action and rejoining.
4. Two or more clients see the same world state; no client-only objects that should be shared.
5. Remotes validated and rate limited ([[13-Luau-Engineering/remote-validation-cookbook]]); exploit pass ([[14-QA-Optimization/exploit-testing]]).
6. Purchases only open from a player tap; no purchase or odds hidden. Paid random item odds are visible ([[07-Monetization/paid-random-items-checklist]]).
7. PolicyService restricted paths tested through Player Emulator: no dead ends when features are off.
8. Content ratings still match content; resubmit the questionnaire if anything changed (experience-guidelines-and-maturity-questionnaire).
9. Text filter on all player text; no placeholder names, art or sounds from other games.
10. UI readable at small phone sizes, with touch targets large enough, and works with gamepad where supported ([[08-UI-UX/mobile-first-checklist]]).
11. Memory and frame budget checked ([[14-QA-Optimization/performance-budgets]]).
12. Thumbnail, icon and description match what the update contains.

## Test mode cheat sheet
| Need | Tool |
|---|---|
| One player quick check | Play or Run |
| Client and server separation | Server & Clients, 1 or 2 clients (max 8) |
| Friends testing with you | Team Test, one session at a time, End Session kicks all |
| Phone, tablet, console screens | Device Simulator (beta) |
| Controller input | Controller Emulator |
| Slow or lossy connection | Network Simulator (beta) |
| Language, text length, region policy | Player Emulator |
| Automated runs | StudioTestService, StudioDeviceSimulatorService (Studio only) |

## After publish (first hour, then first day)
- Restart servers if the update must reach everyone (publishing-steps).
- Performance dashboard (needs 100+ DAU): client crash rate, out-of-memory exits, client frame rate, memory, server frame rate near 60. Break down by Place Version to compare with the old build.
- Error Report: filter New errors since the latest version; look at the Count column and stack traces (crash-and-error-reports).
- Alerts: crash rate over 5 percent for 5 minutes is the docs example.
- Retention and D1 for the new cohort after a day.
- If crash rate or errors spike, roll back first and diagnose second.

## Severity guide
- Blocker: data loss, crash on join, purchase takes Robux without delivering, exploit that grants currency.
- Major: broken core loop step, unreadable UI on phone, repeated errors in Error Report.
- Minor: visual glitch, typo, sound clipping.
- Never ship with a blocker; ship with minor only if logged ([[14-QA-Optimization/bug-triage-and-repro]]).

## For our Roblox games
Copy the principle: test every release in client-server mode, on small screens and bad networks, with policy toggles, then watch the new version's charts.

Never copy: another game's content or test scripts. No paid testing services or devices; use free Studio tools and friends.
