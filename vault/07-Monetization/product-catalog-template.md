---
description: Catalog template: gamepass vs developer product vs subscription vs private server, with ID, price, exactly what it does, and why it is fair.
type: template
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C4 and D1; moonetix.com; spawnblox.com; blancmathis monetization reference
verify: Template. A filled copy is checked by Merchant's gate and by scripts/gates/check_stage.py (catalog and do-not-sell sections present).
---
# Product catalog template

| Name | Type | Product ID | Price (R$) | Exactly what it does | Why it is fair | Do-not-sell check |
|---|---|---|---|---|---|---|
| | gamepass / dev product / subscription / private server | (from Creator Dashboard) | | | | pass / fail |

## Which type
- **Gamepass**: one-time, permanent (VIP, 2x coins forever, speed coil, trail, extra inventory, area access). Free to create in Creator Dashboard or the Open Cloud Game Passes API.
- **Developer product**: repeatable consumables (skip stage, coin pack, temporary boost, revive). **Needs `ProcessReceipt`** ([[07-Monetization/processreceipt-template]]) or purchases fail and players are charged for nothing.
- **Subscription**: recurring monthly benefit; not for game 1 ([[07-Monetization/roblox-plus-and-subscriptions]]).
- **Private server**: recurring; around 100 R$/month when added later (god plan, not in repo).

## Rules
- Sell each pass inside its own experience. **Cross-game pass sales were disabled May 30, 2026.** Portals and links to other games are still fine.
- Creating products needs Andrew's account. Agents list them; Andrew creates them.
- No third-party ad SDKs or off-platform reward schemes.
