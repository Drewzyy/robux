---
description: How Roblox picks games for Home Recommended For You, Charts and Search per the creator docs: two stages, ranked signals, traps that cut exposure, and what to do about each.
type: reference
status: draft
last_checked: 2026-11-15
keywords: discovery, recommended for you, home, charts, discover page, retrieval, ranking, play through rate, bounce rate, thumbnail, search, sponsored, impressions
sources: creator-docs content/en-us/discovery.md (read 2026-11-15); creator-docs content/en-us/discovery-faq.md (read 2026-11-15); creator-docs content/en-us/production/recommendation.md (read 2026-11-15); creator-docs content/en-us/production/publishing/publish-games-and-places.md (read 2026-11-15)
verify: Stages, signal order, 60 minute cap, custom thumbnail rule and exposure-limiting issues read directly in the docs. The docs give no exact weights or Charts formula: UNVERIFIED how Charts ranks beyond "best-performing content".
tags: [roblox-fundamentals, reference]
---
# Discovery and recommendations

How a new game reaches players on Home, Charts, Search and the game details page, and which player behaviors the docs say feed Recommended For You (RFY). Companion to [[06-Retention/discovery-signals]], which holds the signal list; this note adds the surfaces and the rules that limit exposure.

## Do this
- Ship a custom thumbnail and icon before release. A game with only the default thumbnail is not eligible for Home recommendation impressions.
- Fix the first minute before anything else. Play through rate and first play bounce rate (leaving after a short session, under 60 s and 61 to 180 s) are the top signals; bounce is negative.
- Build for return days, not one long session. Play days per user and playtime per user are measured for Day 1, Days 2 to 7 and Days 8 to 28. Playtime counts at most 60 minutes per user per game per day, so a 3 hour grind adds nothing past the cap.
- Add real friend play: join links, invites and private servers feed "intentional co-play days", an Important signal. See [[12-Multiplayer-NPCs/teams-and-parties]].
- Keep title, icon, thumbnail and description honest and original. Mismatched or copycat metadata is deprioritized and ranks lower in search.
- Read Creator Hub Analytics > Acquisition > Home Recommendations weekly. If impressions drop, check play through rate, play days and bounce first, then spend and co-play.
- Do not chase every signal. The docs say to prioritize core gameplay, retention and accurate metadata.

## How RFY decides (two stages)
1. Retrieval: from all eligible games, picks a subset a user might like using engagement, retention and monetization signals. Only a very low number of plays from any source is needed to be considered (per the FAQ).
2. Ranking: orders the subset per user. It uses only the behavior of users who arrived through RFY. Players brought by ads, friends, search, curation or social media do not count here, though those plays can help retrieval notice the game.
3. The FAQ says the list narrows to roughly 100 games shown on one user's Home. The system tests a game daily on small groups and raises impressions if results improve (explore, then expand).

## Signal order (docs priority)
| Priority | Signal |
|---|---|
| Most important | Play through rate; first play bounce rate (negative); play days per user; playtime per user |
| Important | Intentional co-play days; qualified play sessions; spend days per user; Robux spent per user |

Signals are per-user averages, so a small game with deeply engaged players is not penalized for being small.

## What moves impressions besides your game
- Roblox changes the algorithm and adds or removes signals over time.
- Seasonality: weekly peak usually Saturday, plus summer, back to school and holidays.
- Stronger competing games can reduce your share even if your numbers are flat.
- Ads do not hurt RFY, and ad-acquired users are excluded from RFY signals. A dip after a campaign is not an ad penalty.

## Issues that limit exposure
- Titles that lead with money or giveaways ("Robux! Play now!").
- Metadata that does not match gameplay (dinosaur thumbnail, plain obby inside).
- Non-unique games: same title and visuals or near-identical place files as an existing game.
- Name spam, keyword stuffing and heavy emoji decoration.
- A banner on the Creator Dashboard shows reduced exposure and updates daily. The docs route appeals through Support under "Discovery Content Quality".
- Games can opt out of RFY at Audience > Advanced Settings > Exclude from recommendations.

## Other surfaces
- Home also has Continue Playing, Friends, Sponsored, Curated sorts, Standout Games (hand picked, novel) and Live Events.
- Discover (Charts and trending sorts) showcases best-performing content. The docs give no formula (UNVERIFIED).
- Search is moving toward intent: semantic queries such as "food games" work in supported languages.
- Game details page: keep experience events current, link a group, and add passes or subscriptions. It also shows similar games.
- Notifications can bring lapsed players back; the permission prompt is shown to age 13+ only. See [[06-Retention/notifications-and-reengagement]].
- In-game recommenders (RecommendationService) are a separate tool for your own menus. Log only plays that came from items it served, and log a "quality play" only after about 60 s.

## For our Roblox games
Copy the principle: earn recommendation by making new players stay a minute, come back on three separate days and bring a friend. Plan the first-session path and the return reason together with [[04-Onboarding/first-60-seconds-checklist]] and [[06-Retention/day-1-day-7-day-30-playbook]].

Never copy: another game's title, icon, thumbnail wording or art; money or giveaway bait in names; fake "free Robux" framing. Never buy ads (zero spend studio). Do not use urgency or pressure to inflate play days for children.
