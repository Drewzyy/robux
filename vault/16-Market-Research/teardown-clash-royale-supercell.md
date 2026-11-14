---
description: Why Clash Royale and Supercell games work: 3-minute matches, chest timers, trophy matchmaking, card collection and clan play, with session-length and chest rules for Roblox.
type: teardown
status: draft
last_checked: 2026-11-14
keywords: clash royale, supercell, chests, session length, matchmaking, trophies, clans, card upgrades, ladder, seasons, free to play
sources: general knowledge of Supercell public design talks and press (no page re-read 2026-11-14); https://www.pocketgamer.biz/how-king-grew-pet-rescue-saga-to-1-billion-and-sustained-it-for-over-a-decade/ (search result, snippet only, for live-ops comparison)
verify: UNVERIFIED: chest durations, trophy numbers and matchmaking rules are from memory and changed over versions. Verify before copying numbers.
tags: [market-research, teardown]
---
# Teardown: Clash Royale and Supercell design
Fast head-to-head matches with timers that tell you when to come back. The lessons are about session length and pacing, not cards.

## Do this
- Make a match 3 to 4 minutes so it fits any gap in a day; a player should be able to finish one before a bus stop.
- Offer a reward after each win: a chest slot that opens on a timer (for example 3 minutes for a small, 8 hours for a big). Only 4 slots so players must choose.
- Show a clear next reward on the main screen (progress bar with the next chest) so every match moves it.
- Use trophy matchmaking: pair players of similar rating so win rate stays near 50 percent. Place new players against easier opponents (bots allowed) for the first 5 matches.
- Soft-protect losers: a loss costs fewer trophies at low levels, and the ladder has floors you cannot fall under.
- Add clans or friend groups with donations (request cards, give cards). Giving feels good and ties people together. See [[03-Game-Psychology/play-with-friends]].
- Rotate a limited-time challenge weekly (special rules, free entry) so players try new things.

## Why it got huge
- Short, tense matches with clear rules a child learns in minutes.
- Visible progress after each session through chests and cards.
- Supercell's habit: ship, watch data, kill what does not work. UNVERIFIED detail.
- Easy spectating and clan chat made it social.

## Why people play
- Competition and mastery in short bursts.
- Collecting cards and making decks (identity).
- Climbing the ladder.

## Why they stay
- Layered progression: card levels, arena unlocks, deck experiments.
- Clans and friendly matches.
- Seasonal ladder resets give a new climb.

## Why they come back
- Chest timers finishing: a built-in reason to open the app. Use as a gentle pull, not a threat.
- Daily and weekly quests, new season, balance changes.
- See [[03-Game-Psychology/streaks-and-daily-rewards]] and [[03-Game-Psychology/reward-schedules]].

## Session length numbers (suggested)
| Item | Value |
|---|---|
| Match | 3 to 4 min |
| Typical session | 2 to 4 matches (10 to 15 min) |
| Chest short timer | 3 to 10 min |
| Chest long timer | 8 hours, skip with free gems earned |
| Season | 4 weeks |

## Roblox build notes
- Matchmaking with MemoryStore queues keyed by rating bands, widening the band every 5 seconds waited. See roblox-server-data.
- Bots when queue exceeds 20 seconds; label them in code, not necessarily on screen.
- Chest state saved as unlock os.time so it works offline.
- Server decides every outcome; client only shows. See roblox-security.

## Mistakes
- Timers that only exist to sell speedups: kids notice and leave.
- Rating loss streaks without a safety net.
- Long match plus a lose-fast design: frustration.

## For our Roblox games
Copy: short rounds, chests with timers and a free track, rating bands, donations, weekly challenge. Never copy: names, troops, towers, chest art, UI frames, sound, characters. Related: [[03-Game-Psychology/player-psychology]].
