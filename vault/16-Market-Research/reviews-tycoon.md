---
description: What players praise and hate in Roblox tycoons: build-up satisfaction, bugs, lag and paywalls, turned into build rules and a pre-launch checklist.
type: reference
status: draft
last_checked: 2026-11-15
keywords: tycoon, base building, droppers, conveyor, plot, rebirth, game pass, lag, bugs, reviews, churn
sources: https://devforum.roblox.com/t/unable-to-progress-tycoon-incremental-due-to-immense-physics-lag-on-low-end-devices/3587271 (snippet, read 2026-11-15); https://devforum.roblox.com/t/garden-tycoon-what-can-i-improve/1433698 (snippet, read 2026-11-15); https://www.thetoptens.com/roblox/reasons-quit-roblox/ (snippet, opinion list, read 2026-11-15)
verify: Themes come from web search snippets, vendor review analyses and forum threads; review phrasing is paraphrased, all frequencies and percentages are UNVERIFIED, pages were not opened.
tags: [16-market-research, reference]
---
# Reviews: tycoon games
What players say makes them stay and what makes them quit in this niche, written as rules for building. Evidence is thin and mostly secondary, so treat rankings as hypotheses to test with our own data.

## Do this
- Pay out within 5 seconds of spawn: first dropper should earn before the player reads anything.
- Show a ghosted next purchase on the plot so the next goal is always in sight.
- Fix physics lag first: unanchor as few parts as possible and test on low-end devices (a DevForum thread reports a tycoon unplayable from physics lag).
- Bugs on the first loop (stuck doors, vehicles, missing buttons) cost more players than missing features; test the first 10 minutes daily.
- Add a reason to come back: offline income capped at 2-4 hours, or a daily cash drop.
- Offer pass perks that save time (2x cash, auto-collect), never exclusive core buildings.

## Top 6 loved things (paraphrased review language)
1. Watching the base grow. Phrasing: 'it was so satisfying to see my factory fill up.'
2. Clear next-buy goals. Phrasing: 'always knew what to buy next.'
3. Customization of the base look. Phrasing: 'I decorated it my way.'
4. Playing with friends on a shared or linked plot. Phrasing: 'we raced to finish our bases.'
5. Short loops that reward quick sessions. Phrasing: 'I could do a bit and log off.'
6. Surprises such as secret rooms or bosses. Phrasing: 'found a hidden room, felt smart.'

## Top 6 hated things (paraphrased review language)
1. Bugs in core features; one DevForum tycoon creator saw metrics rise just from fixing a vehicle bug.
2. Physics lag on low-end devices that blocks progress.
3. Paywalls: best items only with Robux. Phrasing: 'cash grab.'
4. Boring mid-game: long waits with nothing new to see.
5. Confusing or text-heavy tutorials forced on the player.
6. Early stuck states, such as walking around with nothing to do for minutes.

## Review-to-mechanic translation

### Build this
- Ghost preview of next buy with price.
- Cheap early buys (under 30 seconds each) then slower curve.
- Cash-per-second readout.
- Quest or arrow for the first 3 buys.
- Save plot state reliably (test leave and rejoin).

### Never do this
- Never gate the main building line behind a pass.
- Never leave a stuck state without an escape (reset button).
- Never ship a physics-heavy conveyor without a part count cap.
- Never force a long text tutorial.

## 10-point pre-launch checklist
1. [ ] First buy within 15 seconds.
2. [ ] Arrow or ghost for next buy.
3. [ ] Part count measured on a low-end phone.
4. [ ] Rejoin restores the plot.
5. [ ] First 10 minutes played with no bug in two full runs.
6. [ ] Reset or unstuck button exists.
7. [ ] Mid-game has a new visual change every 5 minutes.
8. [ ] Passes save time only.
9. [ ] Offline income capped and explained.
10. [ ] Group play (shared plot or leaderboard) tested.

## Evidence notes
- Source type: search snippets of vendor analyses, press and DevForum threads. No page was opened.
- Ranking of loves and hates is an editorial judgement from the repeated themes; counts are UNVERIFIED unless a figure and source are given inline.
- Test the hates list against our own playtest comments and Roblox analytics (leave rates, favorites) after launch.

## Related
- [[03-Game-Psychology/what-players-love-and-hate]]
- [[16-Market-Research/roblox-hit-patterns]]
- [[06-Retention/churn-signals-and-fixes]]

## For our Roblox games
- Copy the principle: give players the loved things above for free and early, and remove the hated things before launch, especially unfair pay walls, hidden odds and bugs in the first 10 minutes.
- Never copy names, characters, art, music, logos or UI skins from any game named here. Keep prompts to buy player-initiated and show odds and prices plainly, since our audience includes children.
