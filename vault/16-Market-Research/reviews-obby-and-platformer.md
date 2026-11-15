---
description: What players praise and hate in Roblox obbies and platformers: fair difficulty, checkpoints, controls and lag, turned into build rules and a checklist.
type: reference
status: draft
last_checked: 2026-11-15
keywords: obby, platformer, checkpoint, difficulty, stages, jump, parkour, rage quit, skips, reviews, controls
sources: https://www.thetoptens.com/roblox/reasons-quit-roblox/ (snippet, opinion list, read 2026-11-15); https://devforum.roblox.com/t/people-leaving-game-really-early-on/2158215?page=3 (snippet, read 2026-11-15)
verify: Themes come from web search snippets, vendor review analyses and forum threads; review phrasing is paraphrased, all frequencies and percentages are UNVERIFIED, pages were not opened.
tags: [16-market-research, reference]
---
# Reviews: obby and platformer games
What players say makes them stay and what makes them quit in this niche, written as rules for building. Evidence is thin and mostly secondary, so treat rankings as hypotheses to test with our own data.

## Do this
- Place a checkpoint every 20-40 seconds of play; losing more than 60 seconds of progress is the top rage-quit cause (UNVERIFIED, general pattern).
- Make every death readable: the player must be able to say what they did wrong.
- Ramp difficulty in waves: easy, easy, medium, hard, then a breather stage.
- Offer a skip stage option, earnable by play, so stuck players stay.
- Keep jump distances 80 percent of the real max to allow for lag and phone controls.
- Test on touch controls and 100 ms ping before launch.

## Top 6 loved things (paraphrased review language)
1. Fair challenge. Phrasing: 'hard but every fail was my fault.'
2. Satisfying movement. Phrasing: 'the jump just feels right.'
3. Variety of stage themes. Phrasing: 'every ten stages looks new.'
4. Racing friends. Phrasing: 'we raced to the top together.'
5. Visible progress through a stage counter. Phrasing: 'I reached stage 100.'
6. Fast restart. Phrasing: 'respawn is instant so I try again.'

## Top 6 hated things (paraphrased review language)
1. Unfair deaths from lag, invisible hitboxes or random kill parts.
2. Long distance back to the last checkpoint.
3. Difficulty spikes with no warning.
4. Bad mobile controls and tiny jump buttons.
5. Paid skips that make the game feel like a paywall.
6. Repeated stage themes and copy-paste obbies (stage reuse).

## Review-to-mechanic translation

### Build this
- Checkpoint spacing rule and spawn anchor.
- Hitbox tests with colored debug parts.
- Difficulty curve spreadsheet before building.
- Earnable skip (e.g. one free per 10 minutes).
- Large mobile jump button.

### Never do this
- Never kill players from something they could not see.
- Never reset all progress on death.
- Never sell skips as the only fix for an unfair stage.
- Never copy another obby's stage layouts.

## 10-point pre-launch checklist
1. [ ] Checkpoint every 20-40 seconds.
2. [ ] Hitbox debug view clean.
3. [ ] Jumps at 80 percent of max.
4. [ ] Touch control test passed.
5. [ ] Ping 150 ms test passed.
6. [ ] Breather stage after each hard stage.
7. [ ] Free skip exists.
8. [ ] New theme every 10 stages.
9. [ ] Instant respawn under 2 seconds.
10. [ ] Friend race or leaderboard works.

## Evidence notes
- Source type: search snippets of vendor analyses, press and DevForum threads. No page was opened.
- Ranking of loves and hates is an editorial judgement from the repeated themes; counts are UNVERIFIED unless a figure and source are given inline.
- Test the hates list against our own playtest comments and Roblox analytics (leave rates, favorites) after launch.

## Related
- [[03-Game-Psychology/what-players-love-and-hate]]
- [[16-Market-Research/teardown-tower-of-hell]]
- [[16-Market-Research/teardown-escape-tsunami-for-brainrots]]

## For our Roblox games
- Copy the principle: give players the loved things above for free and early, and remove the hated things before launch, especially unfair pay walls, hidden odds and bugs in the first 10 minutes.
- Never copy names, characters, art, music, logos or UI skins from any game named here. Keep prompts to buy player-initiated and show odds and prices plainly, since our audience includes children.
