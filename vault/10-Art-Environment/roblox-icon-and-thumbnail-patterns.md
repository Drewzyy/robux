---
description: Common visual patterns in top Roblox game icons and thumbnails (faces, colour, text, numbers, close-ups) and how Roblox thumbnail personalization and experiments work, with a testing routine.
type: reference
status: draft
last_checked: 2026-11-14
keywords: roblox icon, thumbnail, top games, personalization, multi-armed bandit, a/b test, experiments, close-up, saturated colour, text on icon, numbers, QPTR, home page tile
sources: devforum.roblox.com "[Live now] Personalize your thumbnails to attract more users" (search result summary, read 2026-11-14); devforum.roblox.com "Testing Personalized Thumbnails in Home this July" (search summary, 2026-11-14); devforum.roblox.com "Thumbnail Personalization consistently picking worst performing QPTR thumbnail" (search summary, 2026-11-14); create.roblox.com/docs/production/promotion/thumbnails (search summary, 2026-11-14); create.roblox.com/docs/production/experiments (search summary, 2026-11-14); rowatcher.com thumbnail testing articles (third party, 2026-11-14)
verify: Roblox pages could not be opened (network blocked), only search summaries. The top-50 pattern list is an UNVERIFIED general observation from model knowledge, not a fresh count; re-check the live Discover page before relying on percentages. Icon A/B testing: no official evidence found.
tags: [art-environment, reference]
---
# Roblox icon and thumbnail patterns

How hit games dress their tile, and what Roblox itself offers for testing. Sizes and moderation live in [[08-UI-UX/thumbnail-icon-spec]]; composition theory in [[10-Art-Environment/thumbnail-mrbeast-logic]].

## Do this
- Make the icon a single readable subject: a character or creature close-up filling 60-80% of the square, eyes visible, facing camera or slightly off-axis, expression exaggerated.
- Test the icon at 150x150 px and at 100x100 px on a phone. If the subject is not clear in 0.5 seconds, enlarge it or delete detail.
- Use 2-3 colours, high saturation, one strong contrast edge between subject and background (rim light or dark outline of 4-8 px at 512 size).
- Put text on thumbnails, not on the icon (icons are tiny). Max 3 words, a number is the best text ("100 LEVELS", "x2 LUCK").
- Upload 3-5 thumbnails and let Roblox's personalization pick between them (see below). Vary one thing at a time: subject, crop, colour, or text.
- Make the first thumbnail the strongest standalone: it may be the only one shown to some users.
- Refresh the icon and thumbnails for each big update, with a visible "new" cue (new creature, new zone), not a banner saying "UPDATE".
- Log each change with date and the play-through numbers before and after.

## Patterns seen in top Roblox games (UNVERIFIED general observation)
| Pattern | How common | What to copy |
|---|---|---|
| Character or creature close-up | Very common | Big head or hero object, simple background |
| Saturated bright colour, often blue, orange, yellow, red | Very common | Pick one dominant hue, keep rest dark or neutral |
| Strong outline or glow around subject | Common | Separates subject at tiny size |
| Action pose or "about to happen" moment | Common | Jump, swing, grab, chase |
| Numbers or multipliers (x2, 100, 1M) | Common in simulators and tycoons | One number, huge |
| Single word or no text on icon | Most icons | Skip text on icons |
| Real photo faces | Rare for games | Use avatars only |
| Many small elements, UI screenshots | Rare in top tiles | Avoid |

Genre cues: simulators show a pet or tool plus a big number; obbies show a tower or hazard; horror shows a dark scene with one glowing eye or light; anime and RPG show a character with a weapon; tycoons show a building and money.

## Roblox personalization (official, from announcements)
- You can upload multiple thumbnails (docs say up to 10 images or video links per experience; announcement describes picking about five to run).
- Roblox runs a multi-armed bandit: it shows variants, learns which one gets the most engagement for each audience segment, and shifts traffic toward it, typically within hours (announcement summary).
- It is not a classic fixed-split A/B test, so you do not get a clean winner report; you get the system choosing for you.
- A DevForum thread reports it sometimes picks a thumbnail with the lowest quality play-through rate (QPTR). Treat this as an open concern: check your own numbers.
- Roblox has also run translated icon and thumbnail experiments (devforum posts, 2024). Details UNVERIFIED.
- Icon A/B testing: no official feature found. Third-party blogs claim native tests; UNVERIFIED.
- General Experiments (Creator Hub) test in-game and matchmaking changes with a control group; they are separate from thumbnails.

## How to test thumbnails with the tools above
1. Upload 4 thumbnails that differ in one variable each, strongest first.
2. Leave for 7 days, no other big change.
3. Read the Creator Dashboard analytics for impressions, visits and play-through. If a variant is clearly worse, remove it.
4. Next round, keep the winner and change one thing.
5. For the icon, change it only once per 2 weeks and compare the same weekdays.

## Mistakes
- Text too small on the icon.
- Muted or realistic colour grading; tiles with low contrast vanish next to saturated neighbours.
- A thumbnail showing an item the game does not have (moderation and bounce).
- Three or more subjects in frame.
- Changing icon, name and thumbnails in one day; you cannot tell which helped.

## For our Roblox games
Copy: the patterns (close-up, saturated colours, one number, one subject), the one-variable testing habit, and the use of personalization with 4-5 variants. Never copy: another game's characters, art, layout, name or logo; never imitate a top game's icon closely enough to confuse players (moderation and IP risk, see [[01-Roblox-Fundamentals/ip-and-toolbox-rules]]). Related: [[10-Art-Environment/youtube-packaging-lessons]], [[10-Art-Environment/thumbnail-pipeline]].
