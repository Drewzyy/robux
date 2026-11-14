---
description: Quest and mission design for Roblox: objective types, quest chains, markers, rewards scaling, daily quests, and tracking rules that keep new players guided without long text.
type: reference
status: draft
last_checked: 2026-11-14
keywords: quests, missions, objectives, quest chain, tracker, daily quests, rewards, tutorial quest, guidance, fetch quest, kill quest, markers
sources: Author working knowledge of game design practice (no pages opened 2026-11-14); Roblox Creator Hub docs (create.roblox.com/docs, titles only, not opened 2026-11-14)
verify: General practice. Reward ratios and timings are starting points. UNVERIFIED: any claim about quest completion rates; measure in your own analytics.
tags: [game-design, reference]
---
# Quest and mission design

Quests turn the core loop into goals with a clear end. Players should always know what to do, where, and what they get.

## Do this
- Give every quest one verb and one number: "Collect 10 crystals", "Defeat 5 slimes".
- Show a tracker with progress (3/10) and a world marker (beam or arrow) at all times.
- Keep the first quest under 90 s and finish it with a reward. See [[04-Onboarding/first-60-seconds-checklist]].
- Chain quests of rising size: 1 min, 2 min, 4 min, then a 10 min milestone. See [[05-Progression/goal-gradient-and-session-goals]].
- Offer 3 daily quests: 1 easy (2 min), 1 medium (5 min), 1 hard (10 min). Refresh daily at a fixed UTC time and show a timer.
- Pay with the thing the next step needs: a quest at an iron mine rewards iron tools, not random coins.
- Let players accept 1 main and up to 3 side quests at once; more overwhelms.
- Allow abandon and restart with no penalty.

## Quest types (pick a mix)
- Gather: collect N items, teaches the map.
- Defeat: kill N or a boss, teaches combat. See [[02-Game-Design/combat-design-basics]].
- Deliver: carry an item to an NPC, teaches geography.
- Explore: find N secrets, rewards curiosity. See [[03-Game-Psychology/curiosity-and-mystery]].
- Social: team up with a friend for a bonus, supports play-with-friends.
- Skill: finish a course under a time, rewards mastery.

## How to build it in Roblox
- Quest table in a ModuleScript: id, title, goal type, target count, reward, next quest id.
- Progress per player saved as a small table; server increments on validated events only.
- Use ProximityPrompt for talk and turn-in; UI banner on quest complete with a short sound.
- Quest markers: Beam between attachments or a BillboardGui, hidden when the player is within 15 studs.
- Daily reset: compare last reset day number from os.time to avoid client clocks.
- Log completions so you can find quests that nobody finishes.

## Numbers (playtest starting points)
- Quest reward worth 3-5 minutes of normal earning, scaled so it feels generous. See [[02-Game-Design/economy-math]].
- Chain milestone every 5th quest with a cosmetic or pet.
- Dialogue under 20 words per quest.
- Daily quest completion target 60-70 percent for the easy one.

## Mistakes
- Quests that require reading three paragraphs.
- Collection counts that ignore server respawns, so players wait on nodes.
- Fetch quests with long empty walks.
- Daily quests that punish a missed day with lost streaks. See [[03-Game-Psychology/streaks-and-daily-rewards]].
- Reward too small to notice.

## Playtest check
- Ask a tester to say what the quest is after closing the dialogue; if they cannot, shorten the line.
- Watch for players walking away from a marker; the marker is hidden or unclear.
- Check the average time per quest against the table above.
- Remove any quest finished by under 5 percent of players in week one.

## For our Roblox games
Copy the structure: one verb plus a number, trackers, markers, rising chains, mixed types, daily tiers. Never copy named quests, NPCs, or dialogue from existing games. Write original quest text and rewards.
