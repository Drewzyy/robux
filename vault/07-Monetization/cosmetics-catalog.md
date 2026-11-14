---
description: Capes, hats, accessories, trails, auras, pets, emotes, titles: which cosmetics to make for a game, how to attach them in Roblox, and what to earn versus sell.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: cosmetics, capes, hats, accessories, trails, auras, pets, titles, name colour, emotes, skins, shop items, accessory, attachment
sources: Roblox Accessory and Attachment instances (model knowledge); 07-Monetization/microtransaction-playbook; 07-Monetization/offer-pressure-limits
verify: UNVERIFIED: prices are starting points; test them against your own sales.
tags: [monetization, playbook]
---
# Cosmetics catalog

## Do this
- Make in-experience cosmetics (items that exist only inside your game). Publishing to the Roblox Marketplace (UGC) costs Robux and needs approval, which breaks zero spend; skip it.
- Attach hats, capes, and wings as `Accessory` objects with a Handle part and an Attachment matching the character's (HatAttachment, BodyBackAttachment, WaistBackAttachment, and so on); Roblox welds them automatically.
- Capes: a thin MeshPart or several hinged parts on BodyBackAttachment; animate sway with a small looping CFrame tween on the client.
- Models come from the Modeler agent (Blender) or free CC0 sources; low poly (under 2,000 triangles each).
- Earn some, sell some: about half the catalog earnable by playing keeps it fair and gives everyone a goal.

## Catalog ideas
| Type | Examples | Earn or sell | Price (Robux) |
|---|---|---|---|
| Hats | crowns, halos, caps, horns, flower crowns, glowing headphones | earn common ones, sell rare | 25 to 150 |
| Capes | royal, hero, flame, galaxy, seasonal | sell | 75 to 250 |
| Wings / back items | wings, backpacks, jetpacks | sell | 100 to 300 |
| Trails | rainbow, fire, ice, stars, music notes | earn first, sell premium | 50 to 200 |
| Auras | glow ring, flames, snowfall, petals | sell | 100 to 300 |
| Pets (cosmetic) | floating companion that follows | earn and sell | 75 to 400 |
| Movement styles | a pack from [[09-Animation-VFX/free-animation-library]] | sell | 50 to 150 |
| Titles and name colours | "Champion", gradient names | earn | free |
| Win / spawn effects | fireworks, lightning, confetti | sell | 50 to 200 |
| Seasonal | holiday hats, snow trails | limited by season, not fake scarcity | 50 to 200 |

## Selling well (honest)
- Let players try it on in the shop (preview on their own character).
- Show cosmetics on other players: being seen is the reason to buy.
- Bundles: 3 related items at about 30% off, stated plainly.
- Follow [[07-Monetization/offer-pressure-limits]].
