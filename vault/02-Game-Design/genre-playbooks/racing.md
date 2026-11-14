---
description: Racing playbook for Roblox: vehicle handling feel, track layout, rubber banding, lap structure, unlock pacing for cars, and multiplayer race flow for short sessions.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: racing, roblox racing, cars, track design, rubber banding, laps, drift, vehicle physics, drag race, unlock cars, time trial
sources: Author working knowledge of arcade racing design (no pages opened 2026-11-14); Roblox Creator Hub docs, VehicleSeat and constraints pages (titles only, not opened 2026-11-14)
verify: General practice. Handling numbers are starting points. UNVERIFIED: current genre ranking.
tags: [game-design, playbook]
---
# Racing playbook

Arcade racing wins on feel, short races, and a garage you want to fill. Realism is optional; clear speed sensation is not.

## Do this
- Make a race 60-120 s per lap session, 2-3 laps, and 6-12 racers. Queue the next race in under 20 s.
- Nail handling first: turn response within 0.15 s, forgiving steering at speed, a drift or boost that rewards risk.
- Use landmarks and track edges that make speed visible: pillars, banners, particle streaks, FOV widening with speed (for example 70 to 85).
- Add mild rubber banding: trailing cars get up to a 5-10 percent boost, never a teleport.
- Give a clean first race: a 30 s time trial or solo lap, then a bot-filled race the player can win.
- Unlock cars in tiers (see [[05-Progression/pets-and-gear-tiers]]): each tier faster and different, not only faster.
- Reward every placement with some coins; first place bonus is 2 to 3 times the last.
- Offer shortcuts or alternate lines for skilled players.

## Why it works
- Instant goal (finish first), instant feedback (position, time).
- Collection pull: owning and upgrading cars. See [[05-Progression/unlock-pacing]].
- Rematch loop: "one more race" with different tracks.

## How to build it in Roblox
- Vehicle: a chassis built from constraints (SpringConstraint, HingeConstraint motor) or a simple custom body mover for stable arcade handling; arcade is easier to tune and less buggy.
- Network ownership: give the driver network ownership of their car so steering feels instant.
- Checkpoints: ordered invisible parts; the server validates each pass to stop cheating.
- Race flow: lobby, countdown 3-2-1, race, results, vote for next track.
- Ghost laps stored in a DataStore (position samples every 0.1 s) for time trials.
- Camera: chase camera with slight lag and speed shake; keep it mobile friendly.

## Numbers (playtest starting points)
- Top speed 100-160 studs per second for the starter car; fast tier 1.5x.
- Boost 2 s long, 8 s recharge.
- Track length that takes 35-60 s per lap for a short loop.
- Countdown 3 s, finish timeout 20 s after first place.
- Garage price curve 1x, 3x, 9x per tier; see [[02-Game-Design/economy-math]].

## Mistakes
- Cars that flip or stick on curbs; add a 3 s reset button.
- Bots that cheat visibly or players who wait in empty races.
- Tracks that look the same; give each a theme. See [[10-Art-Environment/map-themes-and-biomes]].
- Pay-to-win speeds; sell skins and trails instead.
- Long loads between races; keep the map streamed or cached.

## Replay hooks
- Daily track challenge with a leaderboard and a free cosmetic for the top 10 percent.
- Weekly "car of the week" featured with a time-limited free unlock path.
- Photo mode at the finish to share builds and liveries.

## Playtest check
- Time 10 laps with the starter car and confirm the spread is under 10 percent; wild variance means handling is too twitchy.
- Test on a phone for steering comfort.

## For our Roblox games
Copy the structure: short races, tier garage, mild rubber band, server-validated checkpoints. Never copy real car brands, licensed names, existing game tracks, or soundtrack. Use generic original vehicle designs and self-made audio.
