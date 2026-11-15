---
description: Step by step for the Roblox Content Ratings Questionnaire: ratings, who can play each one, how to answer, resubmit rules, moderation risk, and the 13+, 16+ and 18+ triggers.
type: checklist
status: draft
last_checked: 2026-11-15
keywords: content ratings, questionnaire, IARC, content maturity, minimal, mild, moderate, restricted, content may vary, resubmit, kids and select, age-restricted, moderation
sources: creator-docs content/en-us/production/promotion/content-ratings.md (read 2026-11-15); creator-docs content/en-us/safety.md (read 2026-11-15); creator-docs content/en-us/production/publishing/publish-games-and-places.md (read 2026-11-15)
verify: Read in the cloned docs on 2026-11-15. This version names a "Content May Vary" label and IARC ratings, and differs from the older vault note content-maturity-guide, which should be re-checked against it. Exact per-question wording is in the docs: UNVERIFIED here beyond what is listed.
tags: [roblox-fundamentals, checklist]
---
# Experience guidelines and the maturity questionnaire

Every public game needs accurate content ratings. A game with no ratings is restricted for all players. This note is the working checklist; the label summary lives in [[01-Roblox-Fundamentals/content-maturity-guide]].

## Do this
- Fill in name, description and genre first (Publish settings > Game details), then go to Publish settings > Content ratings > Get a rating. Plan 15 minutes to 1 hour.
- Answer for the most mature or extreme content a player can meet, not the average. Count only content you made, not avatar clothing players bring.
- Choose "Game" at the app type question. Social or communication apps are not supported.
- Resubmit the questionnaire whenever an update changes any answer (new blood, a new chat feature, gambling-like content). Do it in the same release.
- Keep assets clean too: realistic or excessive violence, blood, crude humor or romance in images, meshes or audio can be moderated whatever the label says.
- For a kid-friendly target, design for Minimal or Mild. Anything that forces Moderate or higher shuts out part of the youngest audience.
- Use PolicyService to change content by player instead of raising the whole game's rating.

## Ratings and who can play (per the docs)
| Rating | Can be reached by |
|---|---|
| Minimal or Mild | Roblox Kids (5 to 8) and Select (9 to 15) |
| Moderate | Select (9 to 15) and standard accounts (16+) |
| Content May Vary | Age-checked 16+ only |
| Restricted | Age-verified 18+ only |

Reaching under 16 also needs the all-ages publishing requirements. See age-and-region-policies.

## Two outputs
- IARC rating: third party, localized by region (ESRB, PEGI, USK, ClassInd and others), shown only in some regions. Roblox pays the IARC fee.
- Roblox content maturity rating: shown in every region.
- The Preview step lists regions where your answers make the game unavailable.

## Answers that trigger age limits
- Unplayable gambling content: 13+.
- Viewing content from other games: 13+.
- Social hangout without private spaces, free-form user creation, media sharing from gameplay, a main theme of a sensitive issue: 16+.
- Social hangout with private spaces, extended generative AI interaction, heavy realistic blood: 18+ age-verified.
- Any player to player communication (text, voice, emotes) means answer yes to the communication question. Most games do.

## Process notes
- Under-18 creators may wait a few days for ratings. Age-verified 18+ creators see more question variants and compare IARC examples.
- If a game has content only for 18+, servers restart to remove everyone not age-verified 18+, and Studio removes other collaborators.
- Moderation checks answers. A mismatch can mean a notice, replaced or removed ratings, or action on the game and account. Repeated misanswers escalate.
- Appeals: Creator Hub > game > Safety > Issues and appeals, or roblox.com/report-appeals. An appeal is not guaranteed to succeed.
- Kids and Select games had a date of January 26, 2027 for assigned IARC ratings in the docs, after which a game may be removed from that audience.

## Safety tools the guidelines ask for
- Text filter on all player-visible text (TextService:FilterStringAsync), including pet names and signs.
- Ban API (BanAsync) and kick for abuse; IsVerified for gating trade or ranked areas.
- Watch Safety > Overview abuse reports per 1,000 playtime hours, split by Avatar, Chat, Voice, Experience and Audio.
- Avoid designs centered on private spaces such as bedrooms for games open to under 18.

## For our Roblox games
Copy the principle: rate honestly, rate for the worst case, resubmit on every content change, and aim our games at Minimal or Mild. Add the questionnaire and PolicyService check to [[01-Roblox-Fundamentals/pre-launch-policy-checklist]].

Never copy: another game's ratings or descriptors, art, or audio. Never understate content to widen reach.
