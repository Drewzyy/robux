---
description: Paid random items: numeric odds summing to 100% shown before purchase, every outcome has value, PolicyService gating per player, fail closed.
type: checklist
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C4 and D1 (B8e, C4); zehn-studio26.com; we-are-family.com; techtimes.com (Korea loot box rules); antimedia.co; andrian-syh monetization-policy
verify: Roblox policy (live May 26, 2026) as reported by secondary sources. Re-read the official Paid Random Items policy in Creator Hub before building any paid random feature.
tags: [monetization, checklist]
---
# Paid random items checklist

Studio rule: **none in game 1.** If a later game uses them, every box must be ticked.

## Disclosure
- [ ] Numeric odds for every possible outcome, shown before purchase, as percentages summing to 100%.
- [ ] If a paid item boosts odds (a Lucky Potion), the adjusted odds are shown too.
- [ ] Every outcome gives something of value. No outcome leaves the player with nothing.

## What counts as paid random
- [ ] Indirect purchases count: a key or ticket bought with Robux and then used for a random item.
- [ ] Paid prize wheels, luck boosts, pity systems, rerolls, enhanced drops, and rate-ups all count.

## Region gating
- [ ] On join, `pcall` `PolicyService:GetPolicyInfoForPlayerAsync` once, cache it for the session, and honour `ArePaidRandomItemsRestricted` per player.
- [ ] **Fail closed:** if the call fails, treat the player as most restricted.
- [ ] Reported restrictions: off for under-18s in the UK and Brazil; off for everyone in Australia, Belgium, and the Netherlands.
- [ ] Where blocked, offer another route: earn it, or buy the item directly. Accepted treatments include an unpaid earnable path, a fixed non-random sequence, guaranteed direct purchase, hiding the feature, or blocking with a message.

## Do not copy
Some large RNG games reportedly keep paid random features on in restricted regions and rely on odds disclosure. That is community-reported practice, not policy, and it is not compliant.

Violations can mean game removal and account suspension. Free random rewards are fine everywhere.
