---
description: What players actually buy in Roblox games, ranked by value to them, and the line we do not cross with a young audience.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: microtransactions, developer products, game passes, starter pack, vip, 2x, boosts, cosmetics, conversion, monetization
sources: https://create.roblox.com/docs/production/game-design/starter-pack-design (search summary); https://create.roblox.com/docs/production/monetization (search summary)
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Check against the Monetization pages before shipping.
---
# Microtransaction playbook

## What sells (and is fair)
| Offer | Type | Why it works |
|---|---|---|
| Starter pack | developer product, shown on first spawn, can be time-limited | Roblox docs: often +2 to 10% revenue; must be the best value in the game and obviously so |
| 2x coins / 2x XP | game pass | permanent, easy to understand |
| VIP | game pass | chat tag, VIP area, small daily bonus, cosmetic flair: status players can show |
| Cosmetics (trails, auras, pets, emotes) | dev products or passes | self-expression, seen by friends |
| Convenience (auto-collect, extra slots, faster hatch) | game pass | saves time without beating other players |
| Temporary boosts (15 to 30 min 2x luck or coins) | developer product | buy again and again; a short, fun burst |
| Server boosts | developer product | social spending: helps everyone, the buyer is thanked ([[07-Monetization/server-boosts]]) |

## Do this
- Make the first purchase easy and generous (starter pack) and visible from the first spawn; Roblox's docs say gating it behind tutorials reduces sales.
- Spend depth: offers from about 25 to 1,000+ Robux so a converted player has a next step.
- Write exactly what the player gets. Vague descriptions cause regret and refunds.
- Show offers at natural moments (after a win, near a locked door), not as pop-ups mid-action.

## The line (young audience, Roblox rules)
- Real discounts only (Roblox requires presentation to be open and fair).
- No pay-to-win against other players in competitive modes; buy speed or style, not victory.
- No fake scarcity and no countdown pressure built to rush children.
- Paid random items follow [[07-Monetization/paid-random-items-checklist]] and PolicyService.
- Grant purchases only in ProcessReceipt on the server ([[07-Monetization/processreceipt-template]]).
