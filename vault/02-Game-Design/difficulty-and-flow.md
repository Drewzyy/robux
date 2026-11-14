---
description: Flow channel model, dynamic difficulty adjustment options, obby difficulty pacing, and checkpoint spacing rules with numbers for Roblox obbies and action games.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: flow, flow channel, difficulty, dynamic difficulty, DDA, obby, checkpoint spacing, pacing, sawtooth, rubber banding, retry time, skill challenge balance, Csikszentmihalyi
sources: Hunicke and Chapman, AI for Dynamic Difficulty Adjustment in Games (Hamlet), https://users.cs.northwestern.edu/~hunicke/pubs/Hamlet.pdf (seen in search 2026-11-14, summary only); On Flow and Flow Control, https://www.mediadesign.de/de/news/2017/flow-and-flow-control-how-design-games-player-growth-replay-value-and-great-gaming (search 2026-11-14); Jenova Chen flOw thesis, https://jenovachen.info/abstract (search 2026-11-14); existing note difficulty-curve-worksheet
verify: Flow model and DDA concept from the cited papers (summaries only). All numeric spacing and timing values are our heuristics, UNVERIFIED; confirm by checkpoint funnel data after launch.
tags: [game-design, playbook]
---
# Difficulty and flow

Keep challenge near skill. Worksheet for stage-by-stage tuning: [[05-Progression/difficulty-curve-worksheet]].

## Do this
- Aim for the flow channel: challenge slightly above current skill, never far above (too hard gives anxiety, too easy gives boredom).
- Obby: one new idea per stage, introduce it in a safe spot (no kill floor), repeat with a twist, then combine.
- Checkpoints: space them so a failed attempt costs 10-30 s of replay in early stages, 20-60 s later. Never more than 90 s (UNVERIFIED heuristic).
- Place a checkpoint before every new mechanic and after every hard peak, not before easy filler.
- Retry in under 2 seconds with a single input; no death screen or dialog.
- Use sawtooth pacing: ramp 3-4 stages, then an easy rest stage, then a harder ramp.
- Track stage reach and fail counts per stage; a stage with over 2x the average fail rate or a drop of over 15% of reaching players is too hard (UNVERIFIED thresholds).
- Offer adaptive help, not auto-win: after 8-10 fails on one stage offer a skip or an easier optional path (a Robux skip is allowed, but offer a free route too).

## Flow channel
```
challenge
  ^          anxiety
  |       /  
  |  FLOW CHANNEL  
  |    /           
  |  boredom
  +------------------> skill (grows with practice)
```
The channel moves up and right: as skill rises, challenge must rise or boredom returns. Clear goals, immediate feedback, and a sense of control are the other conditions Csikszentmihalyi lists.

## Dynamic difficulty adjustment (DDA)
Hunicke and Chapman's Hamlet adjusts the game in response to player state to keep them in the channel. Options for us, cheapest first:
| Method | How | Risk |
|---|---|---|
| Player-chosen tiers | Easy/Normal/Hard doors, rewards scale | none, best default |
| Soft assists | wider coyote time, extra checkpoint after N fails | feels patronising if visible |
| Spawn scaling | enemies per player count and recent deaths | exploitable by dying on purpose |
| Reward-side | more health drops after deaths | low |
| Hidden rubber banding | racing catch-up | players notice, reads unfair |
Keep adjustments small, bounded and (where possible) disclosed. Secondary sources warn hidden DDA can undermine a sense of competence.
```lua
--!strict
-- server: easy-mode assist after repeated fails on one stage
local FAILS_FOR_HELP = 8
local fails: {[number]: {[number]: number}} = {} -- userId -> stage -> count

local function onFail(player: Player, stage: number): boolean
	local byStage = fails[player.UserId] or {}
	fails[player.UserId] = byStage
	byStage[stage] = (byStage[stage] or 0) + 1
	return byStage[stage] >= FAILS_FOR_HELP -- true: show optional skip/easier route
end
```
Clear the table on PlayerRemoving.

## Obby pacing template
| Stages | Intent | Fail cost | Notes |
|---|---|---|---|
| 1-5 | teach jump, gap, moving part | under 10 s | near-unfailable, a win by stage 1 |
| 6-15 | one twist each | 10-20 s | checkpoint every stage |
| 16-30 | combos, timing | 20-40 s | rest stage every 5th |
| 31+ | mastery | 30-60 s | optional hard routes for bragging |
- Jump distances: stay within what the default character can do with a margin (R15 default JumpPower 50, WalkSpeed 16: about 11-13 studs gap max, UNVERIFIED; measure in Studio with the actual humanoid values).
- Tower-style games (climb height): total length of 100+ stages needs checkpoint-per-stage plus a visible height meter.

## Mistakes
- Difficulty linked to a clock, not mastery.
- Checkpoint far after a hard part: players quit.
- Hard first stage; players who die in 30 s do not return.
- Death screens longer than the section they replay.

## For our Roblox games
Copy: the principles (flow, sawtooth, checkpoint-per-new-idea, optional help). Never copy another obby's stage layouts, themes, or named obstacles.
