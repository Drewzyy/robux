---
description: Every effect a player character can have (movement, earning, buying, boosts, rewards, status, cosmetics) with how to build each in Roblox and what sound goes with it.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: character effects, double cash glow, aura, trail, speed lines, points popup, purchase effect, reward effect, level up, boost glow, vip, sparkles, particles, highlight, beam
sources: Roblox engine instances ParticleEmitter, Trail, Beam, Highlight, PointLight, BillboardGui (model knowledge); 08-UI-UX/ui-animation; 11-Audio/satisfying-sfx
verify: UNVERIFIED: property values are starting points from practice, not from Roblox's docs. Tune them in Studio; test on a phone.
tags: [animation-vfx, playbook]
---
# Character effects library

## Do this
- Effects run on the client of whoever sees them; the server only says "this happened" (or sets an attribute like `DoubleCash = true`).
- Every effect has a sound ([[11-Audio/satisfying-sfx]]) and an off switch under reduced motion ([[08-UI-UX/accessibility]]).
- Budget: at most 2 looping effects on a character at once; one-shot bursts use `ParticleEmitter:Emit(n)`, not a high Rate.
- Colour meaning stays consistent: gold = money, green = health or growth, blue = speed, purple = rare or magic, red = danger.

## Movement
| Effect | Trigger | Build | Sound |
|---|---|---|---|
| Running dust | moving on the ground | ParticleEmitter at the feet, Rate 8 to 12 while running, grey, Lifetime 0.4 | soft footsteps per material |
| Speed lines / trail | sprint or speed boost | Trail between two attachments on the torso, Lifetime 0.25, white to transparent | rising whoosh |
| Jump puff | jump | Emit(6) small puff at feet | short hop |
| Land impact | landing from height | Emit(10) dust ring + small camera kick | thud scaled by height |
| Double jump ring | second jump | expanding ring part, tween Size up and Transparency to 1 over 0.3 s | airy pop |
| Dash streak | dash | Beam afterimage plus 0.1 s FOV kick | sharp whoosh |
| Footprints | walking (cosmetic) | small decal parts left behind, fade over 2 s | none |

## Earning and progress
| Effect | Trigger | Build | Sound |
|---|---|---|---|
| Points popup | gaining points or cash | BillboardGui "+25" rising 2 studs and fading over 0.6 s; gold for cash | coin tick, pitch rises in streaks |
| Coin magnet | pickups nearby | pickups tween to the character before collecting | rising chime |
| Collect sparkle | item collected | Emit(8) gold sparkles at the item | bright tick |
| Level up | level up | Emit(30) burst + vertical light pillar Beam for 1 s + "LEVEL UP" banner | rising arpeggio |
| Rebirth | rebirth | white flash to transparent, rings rising, halo stays on as a status | big chord |
| Combo meter | streak of actions | number above head grows with each step | pitch climbs a semitone per step |

## Buying and boosts
| Effect | Trigger | Build | Sound |
|---|---|---|---|
| Purchase confetti | any successful purchase | Emit(40) multicolour confetti over the character, shop button bounce | purchase chime + coin jingle |
| Double cash glow | 2x cash boost active | gold Highlight (FillTransparency 0.85, OutlineTransparency 0.3) + gold PointLight (Range 8) + slow sparkles (Rate 4); timer chip in the HUD | soft hum when it starts, chime when it ends |
| Luck boost | luck boost active | green clover sparkles orbiting (two attachments rotating) | twinkle |
| XP boost | XP boost active | blue rising particles from the feet | airy shimmer |
| Server boost giver | someone bought a server boost | everyone sees a beam from the buyer to the sky + banner with their name | fanfare |
| VIP | owns VIP | small crown above the name + gold name colour | none |

## Rewards and status
| Effect | Trigger | Build | Sound |
|---|---|---|---|
| Daily reward | claim | chest opens, items fly to the HUD | unlock + coins |
| Rare drop | rare item | screen-edge glow in the rarity colour + a brief slow-motion 0.2 s | rarity sting |
| Shield | protected | ForceField-material sphere, slow pulse | low hum |
| Heal | healing | green plus signs rising | soft chime |
| Hurt | taking damage | red Highlight flash 0.1 s + small knockback | hit sound |
| Spawn-in | spawn | quick beam down + fade in | whoosh |

## Cosmetic effects players can earn or buy
Trails (rainbow, fire, ice, stars), auras (glow ring, flames, snow), footprints, kill or win effects, emote effects, name colours, titles. See [[07-Monetization/cosmetics-catalog]] for which to sell and for how much.
