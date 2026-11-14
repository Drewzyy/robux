---
description: Designing rebirth and prestige systems in Roblox: when to offer it, what resets, permanent boosts, cost curves, diminishing returns, and how to avoid players feeling punished for resetting.
type: reference
status: draft
last_checked: 2026-11-14
keywords: rebirth, prestige, reset, multiplier, permanent bonus, endgame, tokens, diminishing returns, ascension, replay, soft reset, simulator
sources: Author working knowledge of progression design (no pages opened 2026-11-14); Roblox Creator Hub docs, badges and DataStore pages (titles only, not opened 2026-11-14); Existing vault note on tycoon rebirth (genre-playbooks/tycoon)
verify: Multipliers and thresholds are starting points. UNVERIFIED as market norms. Always simulate in the economy worksheet.
tags: [progression, reference]
---
# Rebirth and prestige design

A rebirth resets some progress for a permanent advantage. It turns a finished curve into a new run. See [[02-Game-Design/genre-playbooks/tycoon]], [[02-Game-Design/genre-playbooks/incremental-clicker]], [[02-Game-Design/economy-math]].

## Do this
- Offer the first rebirth when growth slows, at about 15-30 minutes of play, and show exactly what it gives.
- Reset the grind (currency, early upgrades), keep the identity (cosmetics, pets, titles, achievements).
- Give a permanent multiplier of at least 1.5 times for the first rebirth so the second run is clearly faster.
- Make runs faster each time: if run 1 takes 20 min, run 2 about 12, run 3 about 8, then taper.
- Unlock something new with each rebirth: area, pet slot, mechanic, or cosmetic frame.
- Show a "rebirth ready" glow and an estimate: "next run about 35 percent faster".
- Allow skipping rebirths; the system is optional for casual players.
- Cap diminishing returns so no player gets stuck: each rebirth costs at most 2-3 times the last.

## Why it works
- Resets reframe a wall into a fresh start with power.
- Players enjoy re-using what they know at higher speed.
- Permanent bonuses are a status marker. See [[03-Game-Psychology/social-status-and-flex]].

## What to reset and what to keep
- Reset: soft currency, tier-1 upgrades, zone progress.
- Keep: rebirth count, tokens, cosmetics, gamepasses, pets, badges.
- Partly keep: a percent of resources (5-10 percent) so the start is not empty.
- Never reset paid items. See [[03-Game-Psychology/loss-aversion-used-fairly]].

## How to build it in Roblox
- A Rebirth module: checks requirement, grants tokens, clears listed data keys, bumps the count, and saves.
- Tokens stored separately and spent in a small permanent upgrade tree (6-12 nodes).
- Confirm screen listing what resets and what stays, with a cancel button.
- Animation sequence of 3-5 s with a sound for the reset moment. See [[09-Animation-VFX/game-feel-juice]].
- Server saves before and after so a crash cannot erase the run.
- Leaderboard: rebirth count board gives a long-term target.

## Numbers (starting points)
- Cost curve: base * 2.5^n for the nth rebirth.
- Multiplier: 1 + 0.5 * n, or 1.25 compounded for 10 rebirths.
- First rebirth at 15-30 min; fifth at about 2-3 hours total.
- Token tree 6-12 nodes, each +5 to 15 percent in a category.
- Soft keep 5-10 percent of resources.

## Mistakes
- Resetting too much or too early; players feel punished.
- Not telling players what they will lose.
- A multiplier so strong that the first run feels pointless in hindsight.
- No new content per rebirth, so the second run feels like a chore.
- Costs that grow faster than rewards.

## Playtest check
- Time run 1 and run 2 with the same tester; the second should be clearly faster.
- Ask what the tester thinks they lose; if wrong, the confirm screen is unclear.

## For our Roblox games
Copy the structure: late-wall offer, partial reset, permanent boost, token tree, new unlock each cycle. Never copy another game's rebirth names, token art, or exact multipliers. Tune from your own data.
