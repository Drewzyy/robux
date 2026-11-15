---
description: What age tiers (Kids 5-8, Select 9-15, 16+, 18+), under-13 limits and region rules change in a Roblox game, and the PolicyService flags to read before gating features per player.
type: reference
status: draft
last_checked: 2026-11-15
keywords: age tiers, under 13, kids, select, policyservice, region, regional pricing, IARC, paid random items, trading, ads allowed, private servers, chat, notifications, age check
sources: creator-docs content/en-us/production/publishing/kids-and-select.md (read 2026-11-15); creator-docs content/en-us/production/promotion/content-ratings.md (read 2026-11-15); creator-docs content/en-us/production/monetization/paid-random-items.md (read 2026-11-15); creator-docs content/en-us/safety.md (read 2026-11-15); creator-docs content/en-us/production/monetization/regional-pricing.md (read 2026-11-15); creator-docs content/en-us/production/monetization/private-servers.md (read 2026-11-15); creator-docs content/en-us/production/promotion/experience-notifications.md (read 2026-11-15); creator-docs content/en-us/studio/testing-modes.md (read 2026-11-15)
verify: Tiers, 13/16/18 triggers, PolicyService names and private server and notification limits read in docs. The PolicyService class page is not in the cloned docs, so its full field list is UNVERIFIED. Region by region rules are not listed in the docs.
tags: [roblox-fundamentals, reference]
---
# Age and region policies

Roblox decides who can see and play a game from the account's age tier, the game's ratings, the player's region and parental settings. Design so the game still works when a feature is switched off for a player.

## Do this
- Call PolicyService:GetPolicyInfoForPlayerAsync on the server when a player joins and cache the result for the session. Gate each feature on its own flag, never on a guessed age.
- Treat any restricted flag as a normal state: hide the feature, show a plain message, and keep the rest of the loop playable.
- Do not collect or show anything personal from kids. Run all player text through TextService:FilterStringAsync.
- Do not require chat, voice, trading or private servers for progress. Under 13s may have them off.
- Never rely on a notification opt-in prompt for under 13s: it does not appear for them.
- Test in Studio with Test > Player Emulator: turn on Enable Test Profile and pick a Region; the docs say the region choice changes the policy toggles that GetPolicyInfoForPlayerAsync returns. Repeat after each content change (see release-qa-checklist).
- Re-answer the ratings questionnaire when a feature changes content (experience-guidelines-and-maturity-questionnaire).

## Account tiers
| Tier | Accounts | Ratings reachable | Chat |
|---|---|---|---|
| Roblox Kids | self-declared or age-checked 5 to 8 | Minimal, Mild | Off by default; parent can enable after age check |
| Roblox Select | 9 to 15 (self-declared 9+ until checked) | Minimal, Mild, Moderate | Off while self-declared; introduced gradually after age check |
| Roblox | age-checked 16+ | all except Restricted | On by default where the region allows |
| Restricted | age-verified 18+ | Restricted content | n/a |

Parents can also allow a specific game for a child with the Approved Experience control (not for Restricted).

## What under 13 changes in a game (from the docs)
- Private servers: under 13s may not be able to join them, depending on privacy and parental controls.
- Notification permission prompt (ExperienceNotificationService:PromptOptIn) is not shown under 13.
- Content flags at 13+: unplayable gambling content, viewing other games' content.
- Early testers under 9 need parent approval; under 16 cannot preview above their parent-allowed rating.
- Paid random items and paid item trading are restricted per user by PolicyService (below).
- Social media links are visible only to age-verified 16+.
- Analytics age buckets include under 9, 9 to 12 and 13 to 15, and groups with fewer than 3 users are hidden.

## PolicyService flags named in the docs
- ArePaidRandomItemsRestricted: true means the player may not use paid random generators (Robux or currency bought with Robux).
- IsPaidItemTradingAllowed: false means no trading of paid items or paid random outcomes.
- IsContentSharingAllowed: false means hide sharing of screenshots, video or images that others can see.
- AreAdsAllowed: false means ad units show a fallback image.
- Also covered by the service: brand content and approved external social links (field names UNVERIFIED).

Allowed treatments when paid random items are restricted: an unpaid earnable path, a fixed disclosed order, direct purchase of the outcome priced near expected value, hiding the item, blocking with a clear message, or moving the player out of that area. See [[07-Monetization/paid-random-items-checklist]].

## Region rules
- IARC maps your answers to local authorities (ESRB for the Americas, PEGI for Europe, USK Germany, ClassInd Brazil, GRAC Korea, ACB Australia and more). Preview in the questionnaire lists regions where the game becomes unavailable.
- Some questions exist for local law (for example Korea national identity, terrorism). Answering yes means the content must be removed before submitting.
- Chat is on by default only "if available in your region". Premium is not available in every country.
- Regional pricing (Managed Pricing) is on by default for passes. Prices never drop more than 70 percent below your default and never exceed it. Use GetUsersPriceLevelsAsync to stop gift or trade arbitrage between price levels.
- Economic location uses signals such as billing history and VPN use and falls back to the default price.
- Tax, payouts and local currency purchases vary by country: see [[07-Monetization/roblox-wallet]].

## For our Roblox games
Copy the principle: one server check per join, per-feature flags, and graceful fallbacks so every tier gets a full game. Default our games to a Kids-friendly design with no required social features.

Never copy: another game's age gating text, art or feature names. Never try to detect or guess a child's age or location ourselves, and never push restricted players toward a workaround.
