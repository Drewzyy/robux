---
description: Dress-up and fashion playbook for Roblox: outfit creation, runway and rating loops, wardrobe collection, creative tools, and moderation-safe design aimed at a young, social audience.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: dress up, fashion, outfit, runway, wardrobe, avatar, style, voting, rating, customization, shopping, creative, modeling
sources: Author working knowledge of Roblox dress-up experiences (no pages opened 2026-11-14); Roblox Creator Hub docs, avatar and HumanoidDescription pages (titles only, not opened 2026-11-14)
verify: General practice. UNVERIFIED: audience share and top-game claims; no figures used.
tags: [game-design, playbook]
---
# Dress-up and fashion playbook

Creative expression plus social rating. Players build looks, show them, and collect more items.

## Do this
- Start with a mirror and 10 free items so the first outfit is done in 60 s.
- Use a themed challenge round: a prompt ("beach party"), 90-120 s to dress, a runway, then votes from other players.
- Rate with simple stars or hearts (1-5) and show results fast; slow results kill excitement.
- Reward participation: everyone gets coins; the top three get bonus coins and a trophy shelf slot.
- Collection: items in rarity tiers with a wardrobe that shows owned and locked silhouettes. See [[05-Progression/achievements-and-badges]].
- Give creative tools: color wheels, layering, accessory sliders, and save slots (5 free).
- Add a photo studio with poses and backdrops; players love sharing screenshots.
- Design for safety: preset text for ratings, no free chat in rating moments, filter all names.

## Why it works
- Identity play and status. See [[03-Game-Psychology/social-status-and-flex]].
- Low conflict, high creativity; strong with under 13 and teen audiences. See [[03-Game-Psychology/age-groups-and-what-they-want]].
- Weekly themes make fresh content cheap.

## How to build it in Roblox
- Apply clothing with HumanoidDescription; keep an allowlist of item IDs you own or created.
- Wardrobe data: list of owned item IDs per player in a DataStore.
- Runway: a TweenService-driven walk path, camera cut to each contestant, and a vote UI.
- Votes validated server side; ignore self-votes.
- Live preview on mobile with a large tap area and a clear Save button.
- Outfits shown to others as a snapshot ViewportFrame or by the live avatar.

## Numbers (playtest starting points)
- Round 3-4 minutes total: 100 s dress, 15 s runway per player when 6 are in, 20 s results. Cap a round at 8 players or run parallel runways.
- Daily free item 1; weekly themed set of 6-10 items.
- Rarity 60 percent common, 25 uncommon, 10 rare, 4 epic, 1 legendary, UNVERIFIED as a ratio; tune in playtest.
- Free save slots 5; extra slots purchased only as convenience.

## Mistakes
- Sales of items that clone real brand clothes; avoid brand logos.
- Inappropriate rating or chat in a young audience.
- Strong items locked behind paywalls so non-payers cannot win rounds; keep rating about creativity.
- Long waits between rounds.
- Unreadable UI on phones.

## Seasonal content ideas
- Rotate prompt themes weekly (seasons, holidays, fantasy, school spirit).
- Run a monthly "stylist of the month" gallery with winners' outfits on a lobby wall.
- Offer collaborative looks where two friends style one avatar together.

## Playtest check
- Time a first outfit from join; target under 90 s.
- Try the full flow on a small phone; buttons must be tappable with a thumb.

## For our Roblox games
Copy the loop: prompt, dress, show, vote, reward. Never copy real fashion brands, celebrity likenesses, specific game items, or mascot characters. Build original clothing sets in your own style.
