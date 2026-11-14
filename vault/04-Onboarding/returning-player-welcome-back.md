---
description: How to greet lapsed Roblox players: detect absence length, show what changed, give a catch-up boost, restore their goal, and avoid guilt or giant menus. Tiers by 3, 14 and 30+ days.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: returning player, welcome back, lapsed, comeback, catch-up, reengagement, last seen, what's new, absence, win-back, boost, daily reward
sources: vault notes 03-Game-Psychology/streaks-and-daily-rewards, 06-Retention/discovery-signals, consulted 2026-11-14; general live-service practice (model knowledge)
verify: Tier lengths and boost sizes are heuristics, UNVERIFIED against Roblox data. Measure comeback rate per tier before tuning.
tags: [onboarding, playbook]
---
# Returning player: welcome back

A returning player is half new: they remember little and the game has changed. Treat the first 60 s back like a mini onboarding.

## Do this
- Store `lastSeen` (os.time) in the player profile on leave and every autosave. On join, compute days away.
- Use three tiers: 2 to 6 days (short), 7 to 29 days (medium), 30+ days (long). Show a different welcome for each (tiers are a heuristic, UNVERIFIED).
- Show one welcome card, dismissable in one tap, with: what changed since they left (max 3 bullets), a catch-up gift, and the next goal.
- Restore context: put the player back at their last goal or zone, and show a bar of what they were working on ("Zone 4: 70 percent").
- Give a catch-up boost, not a handout: a timed 2x income or XP for 15 to 30 minutes works better than a free item (UNVERIFIED), because it pulls them into play.
- Do not punish absence: no lost streak shame text, no "you missed X" lists longer than one line. If a streak breaks, offer a free save once (see [[03-Game-Psychology/streaks-and-daily-rewards]]).
- For long absences, offer a short refresher: re-show hints for the 2 or 3 core verbs and re-enable fading prompts (see [[04-Onboarding/tutorial-patterns]]).
- Never put the shop on the welcome card.

## Tier table
| Days away | Welcome | Boost | New content to show |
|---|---|---|---|
| 2 to 6 | Toast: "Welcome back" plus daily reward ready | none or small | Newest update note, one line |
| 7 to 29 | Card with 3 changes and a catch-up gift | 2x for 20 to 30 min | New zone, event, or item |
| 30+ | Card plus refresher hints and a gift | 2x for 30 min plus a starter bundle of free currency | Big picture: "here is what is new" |

## What changed list
- Pull from a static `whatsNew` table keyed by version. Show only entries newer than the player's `lastVersionSeen`.
- Order by value to this player: new unlocks they can use now first, cosmetics last.
- Add a "Skip" and a "Show me" button; "Show me" points an arrow at the new thing.

## How to build it in Roblox
- On PlayerAdded, load profile, compute `daysAway`, queue the welcome UI after character load, never before.
- Grant the boost server-side with an expiry timestamp so it survives rejoin.
- Log `returning_welcome_shown` and `returning_welcome_claimed` with tier (see [[06-Retention/event-taxonomy]]). Compare D1-after-return between tiers.
- Win-back outside the game is limited: see [[06-Retention/notifications-and-reengagement]] for what Roblox allows.

## Measure
- Share of lapsed players who return, share who stay 10 plus minutes, and share who return again within 7 days.
- Compare players who saw the welcome card with a holdout (a random 10 percent who see none).

## Sample welcome card (original copy)
- Title: "Welcome back". Body line 1: "New: Zone 5 and the Harvest event". Body line 2: "Your gift: 2x coins for 30 minutes". Buttons: "Play" and "Show me".
- Keep it to one screen, 3 lines, two buttons, and a visible close icon. No countdown pressure.

## Pitfalls
- Showing the card while the character is still loading, so it blocks the first input.
- Boost that expires while the player reads the card. Start the timer on the first action, not on join.
- Giving a better gift to lapsed players than to loyal daily players. Loyal players notice; give them a matching boost on their next daily claim.
- Resetting progress or making the old goal impossible after an update. Migrate saved data and keep old goals valid.
- Welcome flow that depends on a notification having been sent; most lapsed players never see one (see [[06-Retention/notifications-and-reengagement]]).

## Edge cases
- Player returns twice in one day: show the card only once per absence (store `lastWelcomeAt`).
- Player returns during an event: lead with the event, since it is time-limited (see [[15-LiveOps/event-design]]).
- Banned or restricted features changed in the update: explain the change in the card in one line.

## For our Roblox games
Copy the principle: tiered welcome, context restore, timed catch-up boost, no guilt. Never copy another game's welcome art, characters, mascot lines, music or logos; write original copy.
