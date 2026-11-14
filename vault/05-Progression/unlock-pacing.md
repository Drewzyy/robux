---
description: How fast to unlock features, areas, gear and abilities in a Roblox game: time-based unlock schedule for the first hour, first week and first month, with a drip rule that avoids overload and drought.
type: reference
status: draft
last_checked: 2026-11-14
keywords: unlock, pacing, drip feed, feature gating, level gating, early game, first hour, content schedule, progression, tutorial, new player experience
sources: Author working knowledge of progression design (no pages opened 2026-11-14); Roblox Creator Hub docs, badges and DataStore pages (titles only, not opened 2026-11-14)
verify: Schedules are playtest starting points, UNVERIFIED as universal. Compare against your own session-length and day-1 retention numbers.
tags: [progression, reference]
---
# Unlock pacing

When something unlocks matters as much as what it is. Too fast overwhelms; too slow bores. Use a drip schedule. See [[05-Progression/progression-curves]] and [[05-Progression/progression-systems]].

## Do this
- Unlock one new thing every 2-4 minutes in the first 20 minutes (a tool, area, ability, or pet slot).
- Unlock a bigger thing (new area or mechanic) every 15-25 minutes in the first hour.
- Show the next unlock as a locked preview with its requirement, so there is always a goal.
- Gate by play, not only by money: mix level, quest, and currency requirements.
- Teach each mechanic before the next appears; never unlock 2 new systems at once.
- Keep a visible "unlocked 7/30" meter to show breadth.
- Add a catch-up path: returning players unlock faster for a while.
- Reserve a few unlocks as surprises for the first-week stretch; do not spoil all in the UI.

## Why it works
- Unlocks are the cheapest reward you can give: a new toy.
- Regular drips keep curiosity and goal pull alive. See [[05-Progression/goal-gradient-and-session-goals]].
- Spaced content smooths long-term retention without more build work.

## A sample schedule
- Minute 0-2: movement and first collect.
- Minute 2-5: first tool upgrade (goal one).
- Minute 5-10: second zone door becomes visible, locked.
- Minute 10-20: pets or companions system opens, with 1 free pet.
- Minute 20-40: first rebirth teaser or advanced feature. See [[05-Progression/rebirth-and-prestige-design]].
- Hour 1-3: social feature (trade, clan, party).
- Day 2-3: daily quests and cosmetics shop depth. See [[03-Game-Psychology/streaks-and-daily-rewards]].
- Week 1: first endgame mode preview.
- Week 2-4: seasonal event content.

## How to build it in Roblox
- Unlock table in a ModuleScript: id, requirement type (level, quest, currency, time), value, tutorial popup id.
- On unlock fire a RemoteEvent for a UI banner, sound, and a pulsing button.
- Store unlocked IDs in a DataStore as a short list.
- Use a server tick to check requirements; never trust the client.
- Feature flags per unlock to adjust pacing without a code change.
- Track time-to-unlock per feature; flag any over 2x the plan.

## Numbers (starting points)
- Minute 0-20: 6-8 unlocks.
- Hour 1: 12-15 unlocks total.
- First 5 sessions: roughly 60 percent of the base content visible.
- Locked previews shown at most 2 ahead.
- Catch-up multiplier 1.5 to 2 times for 3 days after a 7 day absence.

## Mistakes
- Dumping all menus at spawn.
- Long droughts of 15 minutes with nothing new.
- Gating the fun mechanic behind a 2 hour grind.
- Unlocks with no tutorial; players never find them.
- Locking basic quality of life behind paywalls.

## For our Roblox games
Copy the drip principle: small unlock every few minutes, one big one every 20, visible previews, catch-up for returners. Never copy another game's unlock order, system names, or UI. Design your own tree.
