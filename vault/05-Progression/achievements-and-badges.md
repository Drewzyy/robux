---
description: Designing achievements and Roblox badges: categories, goal sizes, reward types, badge limits, and how to use them to guide players and signal status without grinding or spam.
type: reference
status: draft
last_checked: 2026-11-14
keywords: achievements, badges, milestones, trophies, completionist, roblox badges, awards, goals, collection, secret achievements, titles
sources: Author working knowledge of progression design (no pages opened 2026-11-14); Roblox Creator Hub docs, badges and DataStore pages (titles only, not opened 2026-11-14)
verify: Badge API usage is from memory; UNVERIFIED: current badge cost, limits or rules. Check the Roblox Creator Hub badges page before creating badges.
tags: [progression, reference]
---
# Achievements and badges

Badges are public, permanent proof of experience. Use them to guide play and reward milestones. See [[03-Game-Psychology/social-status-and-flex]] and [[03-Game-Psychology/curiosity-and-mystery]].

## Do this
- Create 15-30 badges for launch in five categories: welcome, progress, skill, collection, secret.
- Give a "Welcome" badge on first join. The player sees a reward in the first minute.
- Make progress badges at easy-medium-hard steps (for example 10, 100, 1000 of an action).
- Add skill badges for fair challenges (win without damage, finish under a time).
- Add collection badges for sets (all pets of one world).
- Hide 2-3 secret badges for explorers and wordless clues.
- Pair badges with in-game rewards: title, small currency, cosmetic frame.
- Update with each content release: 1-3 new badges per update.

## Why it works
- Milestones give long-term goals; completionists are a durable audience.
- Badges show up on profiles and aid discovery and trust (UNVERIFIED how much on Roblox).
- Achievements teach features by pointing to them.

## Design the ladder
- Easy: 80 percent of players reach in the first session.
- Medium: 30-40 percent reach in week one.
- Hard: 5-10 percent in a month.
- Rare: 1 percent, mastery or secrets.
- Keep a mix so every player sees at least 3 next steps.

## How to build it in Roblox
- Use BadgeService:AwardBadge on the server and check UserHasBadgeAsync before awarding.
- Keep your own achievement table too (progress counters), since badges are only on or off.
- Show an in-game toast with icon, name, and a sound when earned.
- A badge page UI listing locked and earned with hints (secrets show ???).
- Rate limit checks so awarding does not spam the service.
- Reward grants happen from your own data, not from the badge call.

## Numbers (starting points)
- 15-30 badges at launch; 5-8 per category.
- Goal sizes grow about 10 times each step (10, 100, 1000).
- Toast display 3 s; do not stack more than 2 at once.
- Reward worth 1-5 minutes of earning for small, a cosmetic for big ones. See [[02-Game-Design/economy-math]].

## Mistakes
- Grind-only badges that need 100 hours.
- Badges for trivial actions that flood the profile.
- Missable badges tied to old events with no return path.
- Awards without feedback or sound.
- Forgetting server-side checks, enabling exploiters to grant themselves badges.

## Playtest check
- Count how many badges a fresh player earns in 10 minutes; target 2-3.
- Try to award a badge from a client script to confirm the server blocks it.

## For our Roblox games
Copy the structure: five categories, 10x ladders, secret badges, and in-game titles. Never copy another game's badge names, icons, or text. Draw your own badge art with free tools.
