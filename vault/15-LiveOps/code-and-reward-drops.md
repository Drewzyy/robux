---
description: Redeemable codes and timed reward drops for Roblox games at zero spend: when to use them, code format, expiry, server-side validation, abuse control, community tie-ins, and what to measure.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: codes, redeem, promo code, reward drop, giveaway, milestone, like goal, member count, expiry, anti abuse, datastore, community, free items
sources: vault notes 15-LiveOps/liveops-playbook, 03-Game-Psychology/reward-schedules, consulted 2026-11-14; general live-ops practice (model knowledge); Roblox rules on giveaways and rewards for likes or follows not checked, UNVERIFIED
verify: UNVERIFIED: Roblox policy on rewarding likes, favorites or group joins, and on giveaways. Read the current Community Standards and creator rules before offering any such reward.
tags: [liveops, playbook]
---
# Code and reward drops

Codes are a cheap way to thank players, bring back lapsed ones, and reward a community milestone. Keep them small, safe and server-validated.

## Do this
- Use codes for community moments: a new update, a member milestone, a holiday, a bug apology. Not as the main loop.
- Rewards: small free currency, a boost, a cosmetic. Never a game-breaking item.
- Set an expiry on every code (3 to 14 days is typical, UNVERIFIED) and a one-use-per-player rule.
- Validate on the server only. The client sends the string; the server checks a table, the player's redeemed list and the expiry, then grants.
- Store redeemed codes per player in DataStore so rejoin and server hops cannot repeat them.
- Keep codes in a data table or config so you can add one without a script change (see [[15-LiveOps/update-cadence]]).
- Show clear messages: "Code redeemed", "Already used", "Expired", "Invalid". Rate-limit attempts (e.g. 1 try per second, 10 per minute) to stop guessing.
- Announce codes in Discord and on the group wall ([[15-LiveOps/community-management]]), not in paid ads.
- Do not require leaving Roblox or giving personal data to get a code.
- Check Roblox policy before gating a code on likes, favorites or group joins; if unsure, do not gate (UNVERIFIED).

## Code types
| Type | Example | Notes |
|---|---|---|
| Launch code | Welcome gift for early players | Short expiry |
| Milestone code | Unlocked when the game reaches a player or like goal | Public progress bar builds community |
| Update code | Posted with patch notes | Pulls players to the new update |
| Apology code | After downtime or a bug | Small, sincere |
| Event code | Gives event currency | Ties to [[15-LiveOps/event-design]] |

## Timed reward drops (in-game)
- A server-wide drop every 20 to 30 minutes (a crate, a boss, a bonus window) rewards players who are present (UNVERIFIED timing).
- Show a countdown. Give a small catch-up path to those who miss it.
- Do not drop the best items randomly; keep them for earned goals ([[02-Game-Design/rng-and-drop-tables]]).

## How to build it in Roblox
- Redeem UI: TextBox plus Redeem button, RemoteFunction to the server. Sanitize and trim input, make matching case-insensitive.
- Server: `Codes = { NAME = { reward = ..., expires = os.time-based number } }`; check `profile.redeemed[NAME]`.
- Write a log of redemptions for abuse checks (many accounts from one pattern, sudden spikes).
- Do not ship codes in client-readable scripts; a client script can be inspected, so keep the code table on the server.

## Abuse control
- Alt-account farming: tie high-value codes to account age or playtime thresholds.
- Leaked early codes: use an expiry and a global redemption cap for bigger rewards.
- Duplication: grant through the same server function as other rewards.
- Economy inflation: budget total free currency given by codes per week.

## Measure
- Redemption rate, return within 24 h of a code post, retention of redeemers versus a comparable group (correlation only).
- Currency created by codes as a share of all currency; keep it small (UNVERIFIED, under 5 percent).

## Mistakes
- Codes too generous, which devalue the game.
- Codes with no expiry, which become permanent hand-outs.
- Posting codes in places that attract bots or scammers; use your official channels only.
- Using codes to hide a weak update.

## For our Roblox games
Copy the principle: small, expiring, server-validated rewards tied to community moments. Never copy another game's code names, reward items, art, music or logos, and never make codes depend on paid items.
