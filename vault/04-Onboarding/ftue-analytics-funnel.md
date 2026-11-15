---
description: Instrument the first-time user experience with Roblox funnel events: step plan, LogOnboardingFunnelStepEvent usage, server-only rules, exploiter checks, limits, and how to read drop-off.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: ftue, onboarding funnel, funnel events, analytics service, LogOnboardingFunnelStepEvent, drop-off, d1 retention, journey events, experiments, configs, step names, rate limit
sources: creator-docs content/en-us/production/analytics/funnel-events.md (read 2026-11-15); creator-docs content/en-us/production/analytics/journey-events.md (read 2026-11-15); creator-docs content/en-us/production/analytics/event-types.md (read 2026-11-15); creator-docs content/en-us/production/game-design/onboarding.md (read 2026-11-15); creator-docs content/en-us/production/analytics/get-started.md (read 2026-11-15)
verify: API names, limits (10 funnels, 180 days, 120 + 20 x CCU per minute, 2 to 10 journey stages) read in docs 2026-11-15. The step list and the drop-off alarm numbers are our suggestions: UNVERIFIED. Events only send from the server in published games.
tags: [onboarding, playbook]
---
# FTUE analytics funnel

Onboarding is where most new players leave, so measure it step by step from the first build. Design guidance is in [[04-Onboarding/onboarding-flow]] and [[04-Onboarding/first-60-seconds-checklist]]; event naming rules in [[06-Retention/event-taxonomy]].

## Do this
- Plan 5 to 8 funnel steps before coding. Each step is something a player did, not something shown to them.
- Log each step on the server with AnalyticsService:LogOnboardingFunnelStepEvent(player, stepNumber, stepName) after the player completes it.
- Log step 1 in PlayerAdded ("Player Joined") so the funnel starts the moment someone enters.
- Keep step names short and stable. Changing a step name mid-run splits the data; set the date range after the change to compare cleanly.
- Never send from the client or from Studio: events work only from the server in a published game. For client steps, fire a RemoteEvent and validate the step number on the server before logging.
- Do not log the same step twice on purpose: only the first instance counts, but repeats still use rate limit.
- Read the Funnel page after 100 or more new players and fix the biggest drop first.
- Test changes with Experiments (A/B) and tune values with Configs, as the onboarding guide suggests.

## A starter step plan
| Step | Name | Why |
|---|---|---|
| 1 | Player Joined | Baseline entry |
| 2 | Spawned and Moved | Controls understood |
| 3 | First Core Action | Did the main verb (collect, jump, tap) |
| 4 | First Reward | Saw the first payoff |
| 5 | First Upgrade or Unlock | Understood the loop |
| 6 | Reached 2 Minutes | Stayed past the first decision point |
| 7 | Set First Goal | Saw a next goal (quest, index, pass) |
| 8 | Returned Day 2 | Use retention page, not a funnel step |

Rename to fit the game; keep the count small. Skipped steps auto-complete earlier ones, so log in order.

## How funnels behave (docs)
- A funnel appears once you log it; up to 10 funnels per game appear as tabs.
- A user's saved step expires after 180 days without events; they count as new when they return.
- Filters (platform, age group, country and others) apply to the first step only, so a player who changes device stays in their first device group. Funnels are grouped by the day the player entered.
- One-time funnels (onboarding) need no funnelSessionId. Recurring funnels (shop, upgrades) do: use a GUID per shop open, or a stable key such as playerId-itemId for upgrades. Only the 10 most recent funnelSessionId values per player are kept.
- Rate limit: total AnalyticsService requests per minute is 120 plus 20 times CCU. Custom fields max 3; economy fields group to Other past 8,000 combined values.

## Exploit-safe logging
1. Client tells server "step N done" over a RemoteEvent.
2. Server checks N is an integer in range (for example 1 to maxStep) and that earlier steps happened for that player.
3. Server logs the event, then ignores repeats.
4. Never let the client supply the step name text. Map numbers to names in a server table ([[13-Luau-Engineering/remote-validation-cookbook]]).

## Branching starts: journey events
- Use Journeys when players can take different paths (choose a class, pick a zone).
- 2 to 10 stages, 1 to 10 nodes per stage; names up to 50 characters. Nodes in one stage count as equal progress. Roblox adds an automatic entry stage 0.

## Reading the funnel
- Biggest relative drop first: if step 3 loses a third of players, fix the tutorial prompt, not step 7.
- Compare phone vs desktop, and age group, with filters. A phone-only drop usually means UI or controls.
- Pair with D1 retention and first play bounce: bounce is a negative discovery signal (discovery-and-recommendations).
- After a fix, compare only the date range after release.
- Target: a smooth curve with no step losing more than about a quarter of players (UNVERIFIED rule of thumb).

## Privacy and kids
- Log counts and step names only. Do not log chat, names or any personal text.
- Do not use funnels to profile or pressure individual children.
- Do not run offers triggered by a funnel step that targets a struggling child.

## For our Roblox games
Copy the principle: five to eight server-logged steps from join to first goal, validated, read weekly, fixed biggest drop first.

Never copy: another game's tutorial text, step names with their branding, or art. Do not buy analytics tools; use the free Creator Hub funnel, journey and Explore pages (creator-hub-analytics-tour).
