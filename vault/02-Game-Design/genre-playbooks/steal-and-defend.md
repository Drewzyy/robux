---
description: Steal-and-defend (base raiding) playbook for Roblox: collect, defend your base, steal from others, with fairness rules, protection timers, and anti-grief design so raiding stays fun for victims.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: steal, raid, base defense, brainrot style, lock, protection timer, grief, heist, collect and steal, base, raiding, risk reward
sources: Author working knowledge of Roblox collect-and-steal style games (no pages opened 2026-11-14); Roblox Creator Hub docs, ProximityPrompt page (title only, not opened 2026-11-14)
verify: Design principles are general. UNVERIFIED: which specific games lead the genre now and any figures about them; do not rely on player counts.
tags: [game-design, playbook]
---
# Steal-and-defend playbook

Loop: acquire valuables, place them in your base to earn passively, defend them, and raid other bases for rarer ones. It mixes tycoon income with light PvP.

## Do this
- Keep the steal a short, visible action: a 2-4 s hold on a ProximityPrompt with an alarm that alerts the owner.
- Give owners a defense: base lock for 30-60 s (cost or cooldown), traps, or a chase that returns the item if they tag the thief within 10-15 s.
- Make losing an item cheap to recover: a stolen item is marked on the map or can be bought back at a fair price.
- Protect newbies: 5-10 minutes of base lock at start, and match small players to small servers.
- Show a clear risk and reward ladder: common items easy to steal, rare items need more time or gear.
- Cap thefts per base per hour (for example 3) so one player cannot be farmed.
- Reward defenders: bonus coins for a successful catch, or a "guard" title.
- Tie income to holding items so players log in to collect. See [[05-Progression/pets-and-gear-tiers]].

## Why it works
- Loss risk adds tension to a simple income loop. See [[03-Game-Psychology/loss-aversion-used-fairly]].
- Stealing makes a funny, clippable story; defending gives a real skill.
- Simple rules are readable by under 13 players. See [[03-Game-Psychology/age-groups-and-what-they-want]].

## How to build it in Roblox
- Item ownership lives on the server; a steal transfers ownership only after the hold completes and the thief is in range.
- Base lock: a timestamp and a server check; the door is a collidable part toggled by the server.
- Chase return: record last holder and a 10-15 s window; if tagged, return the item to the base.
- Teleport thieves back to a safe plaza if they carry loot too long, to avoid endless chases.
- Save item list per player via DataStore; do not save items in transit.
- Add a report and block option for harassment.

## Numbers (playtest starting points)
- Hold time to steal 3 s; lock duration 45 s; lock cooldown 5 min or a coin cost.
- Newbie protection 10 min.
- Steal cap 3 per base per hour; buy-back 60-80 percent of item value.
- Server size 8-16 players so raids feel personal.

## Mistakes
- Victims lose progress with no recourse; they leave and never return.
- Lock cost that only rich players afford.
- Max-level players raiding new bases; separate by progress tier.
- Items stuck in a thief's inventory after a disconnect; return them to the owner.
- Too many alerts that spam the owner.

## Fairness checks
- A new player should lose at most 1 item in the first hour.
- A veteran should never be able to steal more than twice in a row from one target.
- Log steals for review to detect alt-account farming.

## Playtest check
- Have two testers raid each other for 10 minutes and ask which side felt cheated.
- Disconnect a thief mid-carry and verify the item returns to its owner.

## For our Roblox games
Copy the risk, reward and defense triangle, protection timers, and buy-back. Never copy any current game's item characters, meme names, base layouts, or sound clips. Invent an original collectible set and theme.
