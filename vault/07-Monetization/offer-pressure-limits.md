---
description: How many offers and purchase prompts a game can show before players leave, where they belong, and the persuasion lines BrewixGames never crosses.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: popups, purchase prompts, offers, too many, monetization pressure, prompt budget, upsell, ad fatigue, dark patterns, persuasion, shop
sources: Roblox Creator Docs contextual-purchases.md (creator-docs repo, read 2026-11-14); DevForum threads on purchase prompts (devforum.roblox.com/t/ability-for-players-to-disable-purchase-prompts/1587039)
verify: Roblox's docs say to be deliberate about how many sales prompts you use but give no number; the numbers below are this studio's rules, not Roblox's. Tune them with your own analytics.
tags: [monetization, playbook]
---
# Offer pressure limits

Roblox's own guidance (contextual purchases): offer at natural pauses, like between rounds or in a pre-round shop, and "be thoughtful about how many sales prompts you decide to use". Players on the DevForum complain most about prompts at random moments and prompts that appear while they are pressing something else. The limits below are the studio's starting rules.

## Do this (studio prompt budget)
- **Purchase prompts only from a tap on a Buy button.** Never call a purchase prompt on a timer, on join, on death, or when the player pressed something else.
- **Offers, not prompts, when unasked:** an offer is a small card the player can ignore; a prompt is Roblox's purchase dialog. Unasked offers are limited:
  - none in the first 2 minutes of a player's first session (the starter pack shows as a shop badge and a card in the lobby, never a blocking popup);
  - at most 1 unasked offer per 10 minutes, and at most 3 per session;
  - only at natural pauses: round end, lobby, a win, an unlock screen. Never mid-action.
- **One tap to close,** with "No thanks" as easy to hit as "Buy". After a "No thanks", that offer stays hidden for the rest of the session.
- **The shop is always one tap away** with a dot badge when something new is there. Badges replace popups.
- **Bundle complementary items** into one offer instead of several prompts.

## Signs it is too much (watch weekly)
- D1 or session length drops after adding an offer.
- Most players close an offer within a second (they did not read it).
- Reviews or comments mention "pay to win", "too many ads", or "popups".
If any shows up, remove the newest offer first.

## Persuasion we use (honest, and it works)
Clear value ("2x coins forever"), showing items on other players (social visibility), previews before buying (try the trail for 30 seconds), satisfying purchase feedback (sound, burst, the item appearing on you), anticipation (next unlock shown greyed out), and fair bundles.

## Persuasion we never use
Subliminal or hidden messages, flashing or rapid images, fake countdowns or fake scarcity, guilt or pressure copy ("your pet will be sad"), offers that look like Roblox's own system dialogs, pre-ticked extras, or anything aimed at getting young players to buy without understanding. These break Roblox's rules for fair presentation, can get a game moderated, and lose players' trust.
