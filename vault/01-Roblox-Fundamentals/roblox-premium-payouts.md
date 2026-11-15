---
description: How Roblox pays creators for Premium and Plus subscribers and engagement: Creator Rewards (5 Robux daily, 35% audience expansion), Plus earnings, deprecated Premium payouts, safe design.
type: reference
status: draft
last_checked: 2026-11-15
keywords: premium, roblox plus, creator rewards, engagement payouts, daily engagement, audience expansion, active spender, share link, private server payout, premium purchase modal, devex, robux
sources: creator-docs content/en-us/creator-rewards.md (read 2026-11-15); creator-docs content/en-us/production/monetization/engagement-based-payouts.md (read 2026-11-15); creator-docs content/en-us/production/monetization/roblox-plus.md (read 2026-11-15)
verify: Numbers (5 Robux, 10 minutes, 35 percent of first 100 USD, 100 DAU, 60 day hold, 750 and 100 Robux Plus caps) read in docs 2026-11-15. Real earnings depend on Roblox and may change: treat as variable, UNVERIFIED for any forecast. Premium vs Plus overlap beyond the docs is UNVERIFIED.
tags: [roblox-fundamentals, reference]
---
# Roblox Premium, Plus and engagement payouts

Roblox pays creators beyond direct sales. The old Premium engagement payout was replaced in 2025, so plan around Creator Rewards and Plus. Money rules for our own sales are in [[07-Monetization/roblox-plus-and-subscriptions]].

## Do this
- Do not budget on these payouts. Roblox may change them; read the page before relying on any figure.
- Optimize for the behavior they reward: players who stay 10+ minutes, return, and bring new players, not for Premium status itself.
- Never gate gameplay behind Premium or Plus, and never show a Premium or Plus prompt as a paywall on entry.
- Show the in-game Plus or Premium prompt only after a player taps a clearly labeled button, and never with fake urgency. Check MembershipType first to skip existing members.
- Describe member perks honestly in the game description. Do not promise Robux or rewards you do not control.
- Offer cosmetic or convenience perks only, with no tactical advantage over non-members.
- Check Monetization > Creator Rewards in the Creator Hub for each game and Analytics > Share Links for links.

## Creator Rewards (replaced engagement-based payouts on July 24, 2025)
- Daily Engagement: 5 Robux for each Active Spender who plays your game 10+ minutes that day, if it is one of the first three games they launch that day.
- Active Spender: spent at least 9.99 USD on Robux, Premium or UGC subscriptions in the past 60 days, and not a new or reactivated user in that window.
- Audience Expansion: when a New User or Reactivated User (not logged in for 60 days) is attributed to you and plays 10+ minutes that day, you get a 35 percent share of their first 100 USD of qualifying purchases anywhere on Roblox in their first 60 days.
- Attribution paths: your Share Link, your direct link, or a search of your exact game name as their first session of the day. The game needs an average 100+ DAU for the following 60 days.
- Pay is in Earned Robux with a 60 day hold. Cashing out follows DevEx terms ([[07-Monetization/roblox-wallet]]).
- Dashboard: rewards earned, daily rewarded active spenders, rewarded signups and reactivations with benchmark percentiles.

## Roblox Plus (docs page)
- Subscribers get 10 percent off eligible purchases, rising to 20 percent from month three. Roblox pays the discount, so your per-sale earnings stay the same.
- Up to 750 Robux for each new Plus subscriber you bring via PromptRobloxSubscriptionPurchase (starts once the subscription is paid, not on free trial).
- Up to 100 Robux per subscriber who spends 60+ minutes a month in your paid private servers.
- 10 percent of Robux a Plus subscriber sends to another user inside your game with PromptRobuxTransferAsync (DevEx eligible).
- Plus access includes Marketplace selling and free paid private servers.

## Legacy: engagement-based payouts
- Paid by share of time Premium members spent in your game. Marked deprecated; the Premium purchase modal (PromptPremiumPurchase) still exists.
- The old guidance still applies: do not paywall, do not promise outside rewards, no tactical advantage for members.

## Worked example (from the docs)
An Active Spender plays game A for 2 minutes, B for 5, C for 10, D for 5, A for 10 more, then E for 15. Games A (12 minutes total), C (10) and E (15) each earn 5 Robux. B and D fall short of 10 minutes. In a second example the fourth game visited earns nothing even with 15 minutes, because only the first three launched that day count.

## Checklist before any member-facing feature
1. Does the game stay fully playable for non-members?
2. Is the perk cosmetic or convenience only?
3. Is the prompt behind a labeled button the player taps?
4. Does the text say exactly what the member gets, in plain words?
5. Is it hidden for players who cannot buy (region or parental limits)? See age-and-region-policies.

## What this means for design
- A mid-session player who is having fun is worth more than a long grind. Build clear 10 to 20 minute sessions with a good ending.
- Daily engagement counts only if you are among a player's first three games: strong reasons to open the game first (a daily reward they can skip) help, but do not guilt or punish a missed day. See [[03-Game-Psychology/streaks-and-daily-rewards]].
- Share links need a landing moment for brand new players: easy controls, fast reward ([[04-Onboarding/first-60-seconds-checklist]]).
- Children cannot (and should not be asked to) spend to qualify you. Never mention payouts to players.

## For our Roblox games
Copy the principle: earn from real engagement and honest invites. Use creator rewards as upside, not a plan.

Never copy: another game's Premium perk lists, art or wording. No prompts that guilt players into subscribing, no purchase prompts except from a player tap, and no paid ads to chase new users.
