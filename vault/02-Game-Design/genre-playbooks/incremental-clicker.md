---
description: Incremental and clicker playbook for Roblox: click, buy generators, multiply, prestige. Covers cost growth, offline gains, first 10 minutes pacing, and how to avoid a blank spreadsheet feel.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: incremental, clicker, idle, generators, cost growth, prestige, offline progress, multiplier, auto click, number go up, big numbers
sources: Author working knowledge of incremental game design (no pages opened 2026-11-14); Roblox Creator Hub docs, DataStore page (title only, not opened 2026-11-14)
verify: Cost growth ranges are common community conventions; UNVERIFIED as exact industry standards. Playtest every curve.
tags: [game-design, playbook]
---
# Incremental and clicker playbook

Loop: click or wait, earn currency, buy generators and multipliers, reach a wall, prestige. The hook is watching numbers grow and always having a near upgrade.

## Do this
- Let the first click earn value and the first purchase arrive in under 20 s. See [[04-Onboarding/first-60-seconds-checklist]].
- Use generators with cost growth of 1.07 to 1.15 per unit owned (cost = base * rate^owned). Test both ends and pick by feel.
- Add automation at minute 3-5 (auto-clicker or first idle generator) so the player feels progress while thinking.
- Show per-second income and time to next upgrade. Players love seeing "buyable in 12 s".
- Add a wall every 10-20 minutes and offer prestige right as growth slows. See [[05-Progression/rebirth-and-prestige-design]].
- Include offline earnings capped at 2-8 hours at 25-50 percent rate to reward return without punishing absence.
- Layer currencies: soft (coins), mid (gems from milestones), prestige (tokens). Cap at 3 types at launch.
- Add visual juice: number pops, coin bursts, bigger generators visibly growing. See [[09-Animation-VFX/game-feel-juice]].

## Why it works
- Constant small wins and a visible next step (goal gradient). See [[05-Progression/goal-gradient-and-session-goals]].
- Low skill floor, wide audience, fits idle sessions on phones.
- Prestige resets give replay without new content.

## How to build it in Roblox
- Store numbers as doubles for early game; switch to a mantissa and exponent pair if values pass 1e300.
- Format numbers (K, M, B, T, then scientific); one shared formatter.
- Tick income on the server at 1 s; the client interpolates the display.
- Offline gain: save a timestamp, compute on join from server os.time, cap hours.
- Store generator counts in a compact table; save every 60 s and on leave.
- Rate limit click remotes (for example 20 per second) to stop auto-click exploits.

## Numbers (playtest starting points)
- First 10 purchases within 3 minutes.
- Wall at 12-15 minutes before the first prestige offer.
- Prestige multiplier gives 1.5 to 3 times speed on the next run.
- Total of 8 to 12 generator tiers at launch; each tier costs about 10 to 15 times the one before. See [[02-Game-Design/economy-math]].

## Mistakes
- Pure spreadsheet with no theme; give each generator a character, a place or an animation.
- Dead time: nothing to buy for 5 minutes.
- Numbers so large players lose meaning; use readable names and milestones.
- Click remote without server cap; instantly exploitable.
- Paywalling the idle loop instead of offering convenience extras.

## Analytics to watch
- Median time to first prestige; aim 15-30 min for first, shorter after.
- Share of players who return day 2 with offline gains available.
- Where players quit relative to wall timing.

## Playtest check
- Idle for 10 minutes and check offline gains match your formula.
- Try one million clicks via a script to confirm the server cap holds.

## For our Roblox games
Copy the loop: click, generators, walls, prestige, capped offline gains. Never copy another title's generator names, art, or number tuning exactly. Theme your own world and tune curves from your own playtests.
