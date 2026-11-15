---
description: How to make a Robux purchase feel great and fair: instant visible reward, reveal animation, first-purchase bonus, bundles, shown boost timers, free play rich enough that buying is a shortcut.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: purchase feedback, first purchase bonus, starter pack, bundle value, boost timer, shortcut not wall, value per robux, developer product, game pass, receipt, reveal, fair monetization
sources: creator-docs content/en-us/production/game-design/monetization-foundations.md (read 2026-11-15); creator-docs content/en-us/production/game-design/season-pass-design.md (read 2026-11-15); creator-docs content/en-us/production/game-design/balance-virtual-economies.md (read 2026-11-15); vault notes offer-pressure-limits and paid-random-items-checklist (read 2026-11-15)
verify: Roblox docs support clear value, honest descriptions, contextual shops and starter packs shown in the first minutes. The value-per-Robux ratios below are studio starting points, UNVERIFIED, tune with real sales data.
tags: [monetization, playbook]
---
# Rewarding purchases

A purchase should feel like a gift to yourself, never like paying a toll. Docs say players should find value in an item and understand its relevance at once, and descriptions must be accurate and truthful.

## Do this
- Deliver within 1 second of the receipt. In `ProcessReceipt`, grant first, save, then return `PurchaseGranted`. Then play the reveal (see [[08-UI-UX/reward-moments-and-juice]]).
- Make the reward visible on the player: pet follows, trail appears, tool is equipped, number ticks up. A silent stat change feels like nothing.
- Offer a first-purchase bonus once per player: the first buy of any product gives +25 to +50 percent extra of the same thing, shown on the button before buying ("First buy: +50%").
- Bundle things that belong together (boost plus pet plus currency) at 20 to 35 percent better value than buying the parts. Show "You save X" with honest math.
- Boosts show a live timer on the HUD (mm:ss) and survive death, teleport and rejoin. Store the end time, not a countdown, so the full paid time is always delivered.
- Free play must be rich: a player who never pays should still get a reward every few minutes. Buying skips time, it does not unlock fun. See [[07-Monetization/economy-pacing]].
- Never sell power that makes free players lose matches. Sell speed, style, space, convenience.
- Prompts only from a tap, never at random moments, and never to a child in a pressured state (see [[07-Monetization/offer-pressure-limits]]).

## Purchase moment sequence
1. Player taps Buy. Roblox dialog shows (we cannot change it).
2. Confirmed. Within 1 second: sound, particle burst around the player, item shown large for 1.5 seconds.
3. Item is auto-equipped or auto-placed, with "Equipped" label.
4. Receipt line in the shop: "Owned" badge. Never offer it again.
5. Optional: thank-you line, no upsell in the same breath.

## Value per Robux (studio starting points, UNVERIFIED)
| Product type | Price ladder | Value rule |
|---|---|---|
| Currency pack small | 49 to 99 R$ | Baseline: 1 R$ = 1 unit of value |
| Currency pack medium | 199 to 399 R$ | 10 to 15 percent more per R$ than small |
| Currency pack large | 799 to 1499 R$ | 20 to 30 percent more per R$ than small |
| Boost 15 min | 49 to 99 R$ | x2 earn, never above x2 |
| Boost 1 hour | 149 to 249 R$ | Cheaper per minute than 15 min |
| Permanent x2 pass | 399 to 799 R$ | Pays back in about 10 hours of play |
| Starter pack (once) | 99 to 199 R$ | 3x to 5x the value of buying parts |
| Cosmetic | 99 to 499 R$ | Pure style, no power |
See [[07-Monetization/pricing-ladder]] and [[07-Monetization/money-math]].

## Free play so buying is a shortcut
- Every paid item has a free path with a stated time (for example "earn in about 2 hours").
- Show the free path next to the paid button: "Earn it: 2h" and "Skip: 149 R$".
- Cap the paid advantage: a payer is at most 2x faster, not 20x.
- Daily free boosts exist (1 per day, 10 minutes) so players learn what boosts feel like.
- Starter pack is visible as a shop badge in the first minutes, not a popup (see [[04-Onboarding/first-session-rewards]]).

## Fair vs exploitative
- Fair: items that are exactly what they say; refund-friendly clear text; boosts that pause when the player leaves and resume when they return; bonus shown before the buy.
- Exploitative (never): countdown timers that reset, "last chance" with no real end, hiding the price in gems, random paid boxes without shown odds (see [[07-Monetization/paid-random-items-checklist]]), pre-ticked extras, offers that look like Roblox dialogs, pressure copy aimed at children.

## Mistakes
- Granting after a delay or on rejoin only. Players think it failed.
- A boost that silently stacks or overwrites. Show "extended by 15 min".
- A permanent pass that makes free play feel pointless. Keep free rewards flowing.
- Selling the same item twice. Check ownership.

## Related
[[07-Monetization/microtransaction-playbook]], [[07-Monetization/currency-satisfaction]], [[07-Monetization/server-boosts]], [[07-Monetization/cosmetics-catalog]], [[03-Game-Psychology/retention-tricks-catalog]]

## For our Roblox games
Copy the principle: instant, visible, honest value, and free play that already feels good. Never copy another game's item names, characters, art, sounds or logos, and never use hidden odds, fake scarcity or pressure aimed at children.
