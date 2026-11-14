---
description: What top YouTubers do with titles, thumbnails, the first 30 seconds and retention, translated into Roblox icons, thumbnails and the first 60 seconds of play.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: youtube, packaging, title, thumbnail, hook, first 30 seconds, retention, mrbeast, mark rober, kreekcraft, flamingo, click promise, expectations, first minute
sources: Colin and Samir podcast with MrBeast, Jan 29 2025, via Sportskeeda summary (read 2026-11-14); creatorhandbook.net "10 lessons from MrBeast" (read 2026-11-14); spotterstudio.com "How pro YouTubers make intros" (read 2026-11-14); 1of10.com "How Mark Rober averages 36M views" and "How to hook viewers in the first 30 seconds" (read 2026-11-14); pulse.bot "Mark Rober's new YouTube strategy" (read 2026-11-14); vidIQ Rober breakdown via rosetta.to (read 2026-11-14)
verify: Mostly second-hand summaries of interviews, not primary transcripts. Dream, Sidemen, KreekCraft and Flamingo specifics are UNVERIFIED (no source read); only general patterns are listed for them. No leaked documents used.
tags: [art-environment, playbook]
---
# YouTube packaging lessons for Roblox

The same attention problem exists on YouTube and on the Roblox Home page: a tile, a promise, then a few seconds to prove it. This note keeps the principles and drops the video-only parts. Builds on [[10-Art-Environment/thumbnail-mrbeast-logic]] (do not repeat its formula) and [[06-Retention/discovery-signals]].

## Do this
- Treat icon, thumbnails, title and the first 60 seconds as ONE promise. Whatever the tile shows must be on screen within 10 seconds of spawning, as the same image if possible.
- Within the first 5 seconds of play, show the promised thing (the huge tower, the boss, the pet egg). Rober-style videos open on the exact shot of the thumbnail; copy that: spawn camera or spawn point faces the promise.
- Reassure early. MrBeast has said that in the first seconds he must show the title and thumbnail are real. In Roblox: give one real, visible payoff (a coin burst, a first win, a first jump) inside 30 seconds.
- Spend about a third of design time on the idea plus its title and thumbnail, before building. One creator described moving from about 5% to about 30% of time there (source unattributed; treat as a heuristic). Write the title, icon sketch and the first-minute beat BEFORE coding the loop.
- Name the game as a curiosity question or a clear stake, not a brand. Short, concrete, one idea (e.g. "Survive X", "Escape Y", "Race to the top"), 20-40 characters, no emoji spam.
- Make 3-5 thumbnail candidates that differ by ONE variable (subject, crop, colour, text) so a test teaches something. See [[10-Art-Environment/roblox-icon-and-thumbnail-patterns]].
- Never open with a logo, a loading card longer than a few seconds, or a wall of text. Players drop at the first dead moment.
- Update thumbnails when the game updates: a new reward or area is a new reason to click.

## What the creators say (paraphrased)
- MrBeast (Colin and Samir, Jan 2025, via summary): the title and thumbnail set an expectation; the opening must confirm it fast; he joked thumbnails matter less with autoplay, but the first seconds then matter as much. Another recap says he stresses meeting or beating expectation in the first 30 seconds. UNVERIFIED wording.
- Mark Rober (analyst summaries): clicks come from a curiosity question plus a title and thumbnail that say the same thing; the opening shows the promised result so the viewer feels paid for the click; he uses thumbnail testing and has said weak thumbnails hurt discovery.
- Dream, Sidemen (UNVERIFIED, general observation only): a recognisable face or group, one high-stakes scenario, a title that states the challenge plainly. Roblox equivalent: a recognisable character or mascot reused across thumbnails.
- KreekCraft, Flamingo (UNVERIFIED): Roblox event and game-trend videos lean on bright avatar close-ups, big numbers and an obvious "new" or "update" cue. Roblox equivalent: the same cues on the tile.

## Retention in the first 60 seconds (game version)
| Seconds | Beat | Rule |
|---|---|---|
| 0-5 | Promise visible | Spawn facing the hero element; no menu blocking the view |
| 5-15 | First action | Player can move and do the core verb with at most one prompt |
| 15-30 | First payoff | Visible reward with sound and number pop (see [[11-Audio/sound-feedback-map]]) |
| 30-45 | Raise stakes | Slightly harder second action, a visible goal bar |
| 45-60 | Open loop | Show the next bigger thing (locked door, next pet, next zone) |

## Mistakes
- Thumbnail shows an area, boss or prize the player cannot reach in the first 5 minutes. Bounce goes up and the algorithm learns the tile disappoints.
- Title and icon tell different stories.
- Optimising click-through alone. A tile that gets clicks but loses players in 30 seconds scores worse.
- Copying a famous creator's face, logo or exact layout.

## Title rules that carry over
- One idea per title. If it needs "and", split it into the title and the thumbnail text.
- Use a concrete number or stake when true: "100 floors", "1 hour", "last one standing".
- Put the hook word first; Home cards truncate long names on phones.
- Do not use all caps for the whole name; one capitalised word is enough.
- Keep the same naming pattern across updates so returning players recognise it.
- Avoid claims the game cannot keep ("free Robux", "no lag"): moderation risk and instant bounce.

## Test plan (zero spend)
1. Write 3 titles and 3 thumbnails before building the opening.
2. Pick the pair whose promise is easiest to show in the first 5 seconds of play.
3. After launch, change one thing per week and compare the same weekday, not different weeks.
4. Judge by play-through (clicked and stayed 1+ minute), not clicks alone. See [[06-Retention/discovery-signals]].
5. Keep a log: date, change, before and after numbers, in the run folder.

## Why it works
- Clicking is a prediction: "this will be fun in the way the picture says". The first minute settles the bet.
- A match raises trust, which raises return visits; a mismatch trains players to ignore the tile.
- Short first loops give a fast win, which is the strongest early retention lever creators and designers both cite.
- Autoplay-style browsing (Home, recommendations) means the first seconds decide, not the description.

## For our Roblox games
Copy: promise-match between tile and spawn view, early confirmation, curiosity-question naming, one-variable thumbnail tests, planning packaging before building. Never copy: any creator's name, face, catchphrase, logo, channel art, thumbnails or titles; do not imply an affiliation with a YouTuber or a Roblox event.
Related: [[10-Art-Environment/thumbnail-pipeline]], [[08-UI-UX/thumbnail-icon-spec]], [[08-UI-UX/game-ui-patterns-from-hits]].
