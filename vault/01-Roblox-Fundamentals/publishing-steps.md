---
description: Publishing a Roblox game: first publish, place settings, early testers, release, audience and access, private servers, fees and refund thresholds, with limits like 5 new releases per day.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: publish, release, place, start place, early testers, audience, access settings, private servers, universe id, genre, publishing fee, expedited review, exclude from recommendations, playtest
sources: creator-docs content/en-us/production/publishing/publish-games-and-places.md (read 2026-11-15); creator-docs content/en-us/production/publishing/kids-and-select.md (read 2026-11-15); creator-docs content/en-us/production/monetization/private-servers.md (read 2026-11-15); creator-docs content/en-us/discovery.md (read 2026-11-15)
verify: Steps, limits and fee numbers read in the docs on 2026-11-15. Menu names can change with Studio updates. Group permission details live on a docs page not present in the clone: UNVERIFIED.
tags: [roblox-fundamentals, playbook]
---
# Publishing steps

From a Studio file to a public game. Companion checklists: [[01-Roblox-Fundamentals/pre-launch-policy-checklist]] and [[17-Development-Templates/launch-checklist]].

## Do this
- File > Publish the first time. Fill icon, thumbnail, name, optional promotional text, genre (and optional subgenre) and description, then Save. Roblox creates a universe with a UniverseID.
- Keep the name stable. Renaming often makes the game harder to find under old names.
- Write the first description sentence as a plain summary with genre keywords. No repeated words or unrelated keywords, which can demote the game.
- Invite up to 10 early testers (File > Invite early testers) before the public release. They see a preview watermark and can report.
- Answer the ratings question at release. "No" to age-restricted content gives Content May Vary and 16+ age-checked reach. Complete the full questionnaire to get precise ratings.
- Release deliberately: new games per creator are limited to 5 first-time releases per day (revoking and rereleasing does not count).
- Restart live servers after you replace the start place or push a risky change.

## Places
- A game has one start place and optional extra places (for example lobby, dungeon, island). Add one with File > Publish As > pick the game > Add as a new place.
- Change the start place by overwriting: open the new place, Publish As, pick the current start place, Overwrite. Do not use plain Publish, which overwrites the place with itself.
- Extra places add teleport loads and separate memory. Keep a single place if the map fits streaming budgets ([[14-QA-Optimization/performance-budgets]]).

## Access and audience
- New games are private to the owner, anyone with Edit access and group members with the right role.
- Audience > Manage Access (Publish settings > Manage Audience): Playtest tab has early testers and optional playtesters; Public tab has a Friends toggle (friends only when on, everyone when off, within age and geographic limits).
- Optional: Audience > Advanced Settings > Exclude from recommendations for a limited-reach public game.
- All release actions (first release, place updates, Publish As to another game, audience changes) must meet release requirements.

## Reaching younger players
- 16+ only: age-checked creator account in good standing, 2+ days old, content rating answered.
- All ages (Kids and Select): age check plus verification (facial estimation under 18, government ID 18+), 2-step verification, a Minimal, Mild or Moderate rating, a fee or subscription, and evaluation.
- Evaluation: first only age-checked 16+ players; needs 250 unique plays by highly engaged age-checked players within 60 days, plus a safety review. Track in Audience > Reach.
- Fee path: a one-time refundable 1,000 Robux per game, or an active Roblox Plus or Premium for 2 consecutive months. Refund if the game keeps 25 highly engaged players for 60 days without moderation.
- An expedited review costs 50,000 Robux (48 hour review, refundable after 90 days under conditions). Zero spend studio: use the 16+ path and organic evaluation, not fees.

## Private servers
- Enable at Audience > Access Settings > Allow private servers. The game must be public. Can be free or a monthly Robux price.
- You cannot combine private servers with paid access (Robux or local currency).
- Changing the price cancels all active subscriptions and messages those players.
- Under 13s may not be able to join, depending on parental settings. Plus subscribers get paid private servers free and you can earn up to 100 Robux per subscriber for 60+ monthly minutes.
- Check the free option for co-play: friend play is a positive discovery signal (discovery-and-recommendations).

## Metadata rules that affect reach
- Custom thumbnail required for Home recommendation.
- No monetary reward bait in names or thumbnails; no spoken audio or voice-overs in thumbnails; use original images.
- Gameplay video, once approved, appears first on the details page.

## Update flow after launch
- Publish updates from Studio with File > Publish to Roblox, then restart servers if the change must reach live players at once. Without a restart, old servers keep the old version until they close.
- Before any update that touches content, check whether a questionnaire answer changed and resubmit.
- After release, watch Analytics > Monitoring > Performance and Error Report for the new place version ([[14-QA-Optimization/performance-budgets]]).

## For our Roblox games
Copy the principle: publish privately, test with invited friends, rate honestly, then release to 16+ and earn the all-ages evaluation organically. Keep one reusable publish checklist.

Never copy: another game's name, icon, thumbnail or description text. Never pay a fee, buy ads or promise rewards in the description.
