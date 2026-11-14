---
description: How often to ship Roblox game updates with a small team and zero spend: a weekly rhythm, update sizes, naming, patch notes, testing and rollback habits, and how cadence ties to retention.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: update cadence, release schedule, weekly update, patch notes, hotfix, rollback, versioning, live ops, small team, update size, changelog
sources: vault notes 15-LiveOps/liveops-playbook, live-ops-calendar-template, weekly-review-template, consulted 2026-11-14; https://create.roblox.com/docs/production/game-design/liveops-essentials (cited in liveops-playbook, not opened this session)
verify: Cadence numbers are practice for small teams, UNVERIFIED against Roblox data. Algorithm effects of updates are UNVERIFIED.
tags: [liveops, playbook]
---
# Update cadence

Regular change keeps players returning. A small team should pick a rhythm it can keep for months. Base rhythm in [[15-LiveOps/liveops-playbook]]; calendar in [[15-LiveOps/live-ops-calendar-template]].

## Do this
- Pick a fixed weekly slot (same weekday and hour) for the main update. Predictability helps players and you.
- Use three sizes: hotfix (any time, under 1 hour), weekly update (1 to 3 changes), monthly or seasonal update (a bigger feature or event).
- Give every update one goal from [[06-Retention/decision-tree]] and one metric to watch.
- Ship at a low-traffic hour for your audience so a bad deploy hurts fewer players; check Creator Analytics for your peak hours.
- Test in a private test place or a staging experience (free) before publishing to the live game.
- Keep the last good version ready to roll back; use Roblox version history in Studio (verify current UI).
- Post patch notes in game, in Discord and on the DevForum or group (see [[15-LiveOps/community-management]]).
- Never ship a big update on a Friday evening without a person available to fix it.
- Do not skip a week silently; if you skip, say so and say when the next one lands.

## Update sizes
| Type | Scope | Time budget | Example |
|---|---|---|---|
| Hotfix | One bug or exploit | Under 1 hour | Fix dupe, crash |
| Weekly | 1 to 3 small items | 1 to 3 days of work | New item, balance, UI fix |
| Monthly | One feature or event | 1 to 2 weeks | New zone, new event |
| Season | Theme plus rewards | 3 to 6 weeks | See [[15-LiveOps/event-design]] |

## Weekly rhythm (example)
- Mon: review last week's metrics ([[15-LiveOps/weekly-review-template]]), choose the goal.
- Tue to Wed: build and self-test.
- Thu: private test with friends or test server, fix.
- Fri or weekend: publish at a planned hour, watch errors for 2 hours.
- Weekend: community check, collect feedback.
Adjust days to fit your schedule; keep the order.

## Patch notes
- Title with the version or theme name; 3 to 6 bullets; lead with what players gain.
- Include "Fixed" and "Balance" short lists. Admit mistakes; credit bug reporters.
- Put a "What's new" card in game ([[04-Onboarding/returning-player-welcome-back]]).

## How to build it in Roblox
- Keep a `GAME_VERSION` string in a shared module and log it with every analytics event, so you can compare cohorts across updates.
- Use feature flags (a config module or attributes on a folder) to turn features on and off without republishing. Zero cost, large safety.
- Keep reward and economy numbers in data tables, not scattered in scripts, so balance changes are small.
- Watch errors via the Developer Console and Creator Dashboard after each publish (see [[06-Retention/analytics-reading-guide]]).

## Mistakes
- Updates too big to test, with bugs that hurt retention.
- Changing economy numbers with no announcement.
- Burning content in one drop and having nothing for weeks (see [[15-LiveOps/content-treadmill-vs-systems]]).
- Cadence that burns out the team; pick a pace you can hold for 6 months.

## Rollback checklist
- Know the last good version before you publish.
- Keep data schema changes backward compatible so rollback does not corrupt saves.
- Announce a rollback in the same channels as the update.
- Reward players affected by a bad update with a small free boost.

## For our Roblox games
Copy the principle: a fixed rhythm, one goal per update, staged testing, clear patch notes. Never copy another game's update names, art, music or logos; write original patch notes.
