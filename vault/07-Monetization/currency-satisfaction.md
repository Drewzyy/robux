---
description: Making in-game currency feel good: soft vs premium currency, count-up animations, coin sounds, sinks, and clear prices.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: currency, coins, gems, economy, satisfaction, count up, number go up, sink, soft currency, premium currency
sources: https://create.roblox.com/docs/production/game-design/monetization-foundations (search summary); practitioner game-feel sources
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Check against the Monetization pages before shipping.
tags: [monetization, playbook]
---
# Currency satisfaction

## Do this
- One soft currency earned by playing (coins). Optional second currency only if it has a clear job (gems for cosmetics). More than two confuses kids.
- Earning feels good: coins fly to the counter, the counter counts up over 0.3 to 0.6 s (not jumps), a coin sound climbs in pitch for streaks ([[11-Audio/satisfying-sfx]]), a +number pops near the player.
- Show progress to the next purchase ("1,240 / 2,000 for Blue Lamp") so earning always has a target.
- Sinks: upgrades, cosmetics, and rebirth resets give coins a purpose; without sinks numbers inflate and lose meaning.
- Price ladder: first upgrade in under 2 minutes, then steady steps; abbreviate big numbers (1.2M).
- Robux to coins packs: bigger packs give better value per Robux and say so plainly ("+20% bonus").
- Never hide the real Robux price or make the premium currency hard to convert in your head.
