---
description: Roblox experience notifications and other zero-spend re-engagement: opt-in rules, eligibility, message design, frequency caps (UNVERIFIED), in-game and group channels, and what to measure.
type: reference
status: draft
last_checked: 2026-11-14
keywords: experience notifications, push, reengagement, opt-in, rate limit, open cloud, notification string, lapsed, favorites, follow, group announcement, win-back
sources: https://create.roblox.com/docs/production/promotion/experience-notifications (search result listing seen 2026-11-14, page not opened); https://devforum.roblox.com/t/introducing-experience-notifications/2826474 (search listing 2026-11-14); https://devforum.roblox.com/t/updates-to-notification-rate-limits/3007106 (listing 2026-11-14)
verify: Frequency caps changed over time and sources disagree (one forum post says 1 per 3 days per experience, another mentions 1 per day). UNVERIFIED: read the live Creator Hub page before building. Opt-in and age 13 plus rule is from a search summary, UNVERIFIED.
tags: [retention, reference]
---
# Notifications and re-engagement

Roblox lets experiences send notifications to players who opted in. It is the main zero-spend way to reach lapsed players. The platform controls the rules, and they change, so check the live docs first.

## Do this
- Read the current Experience Notifications page on the Creator Hub before building; treat all limits below as UNVERIFIED.
- Create the notification string (template) in the Creator Dashboard first; there is no default message (per search summary, UNVERIFIED).
- Send only to players who opted in and are eligible. Expect an error when a user cannot receive it (a 400-style response was reported in forum posts, UNVERIFIED).
- Ask for opt-in at a high moment (after a win or on a returning reward), not at join, and explain the value in one line.
- Plan for a hard per-player frequency cap: sources show 1 per 3 days (2024 post) and a later mention of about 1 per day; design for at most 1 per 3 days to be safe (UNVERIFIED).
- Send for a reason: an event start, a finished timer, a friend action, a daily reward ready. Never spam.
- Use the Creator Dashboard notification analytics to compare messages (reported to exist, UNVERIFIED).
- Do not rely on notifications for retention; most lapsed players never opted in. Build the in-game hook first.

## Message design
| Do | Avoid |
|---|---|
| One reason: "Your egg hatched", "Event starts now" | Vague "Come play" |
| Under 10 words, concrete value | Guilt: "We miss you" |
| Time-bound events | Fake urgency |
| Use name mentions only if the API allows (user mentions feature exists per DevForum, UNVERIFIED) | Misleading claims |
Check Roblox's content rules; notifications must follow community standards.

## Other zero-spend channels
- Favorites and follows: prompt after a success. Followers may be notified of updates by the platform (UNVERIFIED how).
- Group wall, group announcements, group shout if available (verify current features).
- Discord server and DevForum thread: see [[15-LiveOps/community-management]].
- In-game: welcome-back card ([[04-Onboarding/returning-player-welcome-back]]), update banner, daily reward.
- Update the game title or thumbnail for events (algorithm and browsing surfaces notice recent updates, UNVERIFIED).

## How to build it in Roblox
- Use the official Luau package from the Creator Store for the opt-in prompt (per summary, UNVERIFIED) and the Open Cloud endpoint for sending server-side.
- Keep the send logic on the server with a per-player `lastNotifiedAt` and a queue; never loop all players at once. Use gentle pacing (below 100 requests per second was suggested in a forum post, UNVERIFIED).
- Store opt-in status and send history in the profile.
- Log notification_sent, notification_opened (if available), and return within 24 h.

## Measure
- Opt-in rate, send success rate, return rate within 24 h versus a holdout.
- Compare notified versus not notified with a random holdout, as notified players are self-selected.
- If return rate under about 2 percent (UNVERIFIED), rewrite the message or drop the send.

## Mistakes
- Spamming and driving opt-outs; opt-outs are permanent per game.
- Sending when nothing new exists.
- Not handling errors, causing retries that spam.
- Using dark patterns; they risk moderation.

## Pre-build checklist
- Live docs read, limits written down with the date you read them.
- Notification string created in the dashboard.
- Opt-in prompt placed after a success moment.
- Server queue with per-player timestamp and error handling.
- Holdout group defined.
- Plan B for players who never opt in (in-game hooks only).

## For our Roblox games
Copy the principle: earn the opt-in, send rarely with one concrete reason, measure against a holdout. Never copy another game's notification copy, mascots, art, music or logos; never pay for ads to replace this channel.
