---
description: Server-wide boosts a player buys for everyone in the server: design, timer, thanks, stacking, and safe code.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: server boost, server luck, server-wide, social purchase, boost, luck, developer product, timer
sources: https://deltiasgaming.com/?p=364175 (Plants vs Brainrots server luck, 149 Robux, 15 minutes)
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Check against the Monetization pages before shipping.
tags: [monetization, playbook]
---
# Server boosts

A player pays once and the whole server gets a boost for a time. Example seen: Plants vs Brainrots sells server luck for 149 Robux for 15 minutes.

## Why it works
The buyer gets status (everyone sees who paid), other players get a gift, and players stay longer while it runs. It is social spending, not pay-to-win.

## Do this
- Developer product (can be bought again), not a game pass.
- Typical shape: 2x luck or 2x coins for the whole server, 10 to 15 minutes, 99 to 199 Robux.
- Announce: banner "Alex boosted the server! 2x Luck for 15:00", a sound, and a timer at the top for everyone.
- Stacking: extend the timer (cap the total, for example 60 minutes) rather than stacking multipliers.
- Thank the buyer: a title or badge "Server Hero", a leaderboard of boosters this session.
- The boost lives on the server only (a server attribute with the end time); clients read and display it. Grant only in ProcessReceipt; handle the server shutting down by not promising time beyond this server.
- Tell buyers it applies to this server only, before they pay.
