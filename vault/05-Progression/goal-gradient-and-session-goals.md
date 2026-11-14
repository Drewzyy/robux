---
description: Always show a next goal: goal-gradient and endowed-progress effects, progress bar rules, near-miss feedback, and session-end hooks, with a server-side Luau session goal tracker.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: goal gradient, endowed progress, progress bar, near miss, session goal, Zeigarnik, peak end, session end hook, next goal, quest, checklist, Kivetz, Nunes Dreze
sources: Kivetz, Urminsky and Zheng, The Goal-Gradient Hypothesis Resurrected, Journal of Marketing Research 2006 (from memory, not opened this session); Nunes and Dreze, The Endowed Progress Effect, Journal of Consumer Research 2006 (from memory, not opened); Clark et al., Gambling near-misses enhance motivation to gamble, Neuron 2009 (from memory, not opened); Zeigarnik 1927 (from memory); existing vault notes progression-systems, core-loop-template
verify: Effects are well-known findings but the papers were NOT re-read this session; effect sizes are UNVERIFIED and from consumer studies, not games. Near-miss research concerns gambling; we use it only for free, skill-based feedback. All timings are heuristics.
tags: [progression, playbook]
---
# Goal gradient and session goals

People work harder the closer they feel to a goal. Make the next goal always visible and close. See [[05-Progression/progression-systems]] (endowed progress rule) and [[02-Game-Design/loop-stack]].

## Do this
- Show exactly one primary next goal on the HUD at all times (bar, price tag, "3/5 stars"). Secondary goals live in a menu.
- Start every bar partly filled (endowed progress): 10-20% on first view, and a free first step done for them.
- Session goal size: finishable in 5-12 min. Chain goals so finishing one reveals the next at 60-80% of a similar size (heuristic).
- Make bars non-linear toward the end if needed: the last 20% should feel quick; award a tick for partial sub-actions.
- Near-miss feedback for skill moments only ("1 stud short", ghost of best time, 4/5 stars): show how close, and what changes next attempt.
- Session-end hooks: when the player is about to leave (goal done, backpack full, round over), show the next goal plus a timer or reward waiting ("egg hatches in 20 min", "daily chest tomorrow").
- End sessions on a peak: celebrate the win with sound and animation before showing the next goal (peak-end).
- Never fake progress or rubber-band the bar to push spending; goals must reflect real, earned progress.

## Why it works
- Goal gradient: effort accelerates as the goal nears (Hull 1932 in animals; Kivetz et al. 2006 in loyalty cards).
- Endowed progress: a card with 2 free stamps on 12 completed more than a blank 10-stamp card (Nunes and Dreze 2006, numbers from memory, UNVERIFIED).
- Zeigarnik: unfinished tasks stay in mind, which is the pull to come back.
- Near-miss: near wins increase motivation to continue (Clark et al. 2009, gambling context). In skill games it signals "you can do this"; do not stage near-misses to sell luck items.

## Progress bar rules
| Rule | Value |
|---|---|
| Update on each action | within 1 frame, animate the fill 0.2-0.4 s |
| Label | numbers plus bar (37/50), never bar alone |
| Segments | 3-8 tick marks so progress feels chunked |
| Completion | burst, sound, reward icon flies into counter |
| Reset | next bar appears immediately, partly filled |
| Mobile | bar at least 12 px tall, top or side, not under thumbs |
Pair with the GUI skill for layout on phones.

## Luau: session goal tracker (server)
```lua
--!strict
export type Goal = { id: string, target: number, progress: number, reward: number }

local goals: {[number]: Goal} = {}

local function newGoal(userId: number, id: string, target: number, reward: number, headStart: number): Goal
	local g: Goal = { id = id, target = target, progress = math.floor(target * headStart), reward = reward }
	goals[userId] = g
	return g
end

-- returns true when completed; caller grants reward and calls newGoal for the next
local function addProgress(userId: number, amount: number): (boolean, Goal?)
	local g = goals[userId]
	if not g then return false, nil end
	g.progress = math.min(g.progress + amount, g.target)
	return g.progress >= g.target, g
end
```
Fire a RemoteEvent with (progress, target) to the client for display only; the server owns progress. Clean up on PlayerRemoving. Persist only the long-term goal, not the session bar.

## Session-end hook checklist
- Player has a goal at 60-90% done when they leave (check with analytics: share of sessions ending mid-goal).
- At least one timer is running that rewards returning (crop, egg, restock) with a push of the info in the welcome-back screen ([[03-Game-Psychology/streaks-and-daily-rewards]]).
- Leave screen is not a guilt trip: no "are you sure" pressure for kids.

## Mistakes
- Goal list of 10 equal quests: no single next step.
- Bar at zero on first view.
- Tiny goal reward that does not change play.
- Staged near-misses on paid random rolls (ethics and policy risk, see [[02-Game-Design/rng-and-drop-tables]]).

## For our Roblox games
Copy: one visible goal, head-start fill, peak-then-next-goal flow, timers at exit. Never copy another game's quest names, bar skins, UI art, or sounds.
