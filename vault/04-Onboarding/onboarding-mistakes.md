---
description: Checklist of common onboarding failures on Roblox (text walls, early shop, slow first reward, locked camera, hard first level, forced signups) with a fix for each. Audit before launch.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: onboarding mistakes, tutorial, bounce, text wall, cutscene, first minute, new player, funnel drop, shop, loading, audit, checklist
sources: vault notes 04-Onboarding/onboarding-flow, 04-Onboarding/first-60-seconds-checklist, 06-Retention/decision-tree, consulted 2026-11-14; general design practice (model knowledge)
verify: Thresholds (20 percent drop, 30 s) are heuristics, UNVERIFIED against Roblox data. Confirm with your own funnel events.
tags: [onboarding, checklist]
---
# Onboarding mistakes

Use this as a pre-launch audit. Each item names the symptom, the fix, and the metric to watch. Pair with [[04-Onboarding/first-60-seconds-checklist]].

## Do this
- Run this list on every build that touches the first 5 minutes.
- Record the first session on a phone screen at least once per update. Mistakes show up on small screens first.
- Treat any step where more than 20 percent of new players quit as a bug, not a taste issue (heuristic, UNVERIFIED).
- Fix the first mistake in the list you fail before touching later ones; earlier mistakes cost more players.
- Re-test after each fix with fresh accounts (new profile, no saved data).

## The list
| # | Mistake | Symptom | Fix |
|---|---|---|---|
| 1 | Slow load, black screen | Bounce before first input | Lean loading screen, stream content, show progress text |
| 2 | Cutscene or long intro | Players walk away | Cut to under 5 s and make it skippable; or drop it |
| 3 | Text wall or multi-page how-to | Nobody reads it | Max 8 words per prompt, see [[04-Onboarding/tutorial-patterns]] |
| 4 | First reward after 60 s or more | Low early tick events | Reward within 30 s, see [[04-Onboarding/first-session-rewards]] |
| 5 | Shop or Robux prompt in minute 1 | Quick leave, bad reviews | Delay any purchase prompt until after the first upgrade |
| 6 | Hard first obstacle | Early deaths cluster | Soften, add checkpoint, see [[04-Onboarding/new-player-protection]] |
| 7 | Several currencies at once | Confusion, no purchases | Show one currency, reveal the next at a milestone |
| 8 | Controls unexplained on mobile | Players cannot move or act | Show touch UI hints, test on phone first |
| 9 | Camera or movement locked during prompts | Frustration | Never lock controls for a hint |
| 10 | Goal unclear | Wandering | One visible objective, arrow or beam to it |
| 11 | No save feedback | Fear of losing progress | Toast: "Saved", autosave on each reward |
| 12 | Same tutorial forced on veterans | Rage quit on rejoin | Persist `tutorialDone`, offer skip |
| 13 | Social wall (invite pop-ups, group join) | Overwhelm | Social prompts only after 3 to 5 min |
| 14 | Spawn facing away from the objective | Disorientation | Spawn looking at the first target |
| 15 | Sounds off or loud on join | Scare, mute | Calm baseline mix, feedback sounds short |

## How to find the real cause
- Build a funnel: join, first input, first reward, first upgrade, minute 5, session end. See [[06-Retention/event-taxonomy]] and [[06-Retention/analytics-reading-guide]].
- Look at where the biggest drop sits, then match it to a row above.
- Compare device types (phone, tablet, PC, console). A drop on phone only points to UI size or controls.
- Check the thumbnail and title promise against what the first minute delivers; a mismatch causes bounce (see [[08-UI-UX/thumbnail-icon-spec]]).

## How to prioritize fixes
1. Anything that blocks the first input (load, controls, camera).
2. Anything that delays the first reward.
3. Anything that causes early death or confusion.
4. Everything else (polish, extra tips).

## Re-check cadence
- Every update: rerun rows 1, 4, 8.
- Monthly: full list, new-account playthrough, and funnel review in [[15-LiveOps/weekly-review-template]].

## Mini audit script (5 minutes)
1. Create a fresh test account, join on a phone-sized window.
2. Start a stopwatch at the join click. Note the time of first input, first reward, first upgrade.
3. Count words shown on screen before the first reward.
4. Try to die in the first minute; note what it costs.
5. Leave and rejoin; confirm the tutorial does not repeat and progress is kept.
6. Write down the one thing that confused you and fix that first.

## Examples of fixes (generic)
- Long intro cutscene: replace with a 3 s fly-in that ends on the objective, input enabled from second 3.
- Shop popup at join: move to a small button, show once after the first upgrade.
- Hard first jump: widen the platform and add a checkpoint right before it.

## For our Roblox games
Copy the principle: audit the first five minutes against a fixed list and treat early quits as bugs. Never copy any specific game's tutorial wording, characters, art, music or logos.
