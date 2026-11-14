---
description: Running a game community on Roblox at zero spend: Discord, DevForum, group wall, in-game feedback, moderation, patch notes, creator and player relations, response times, and what to avoid.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: community, discord, devforum, roblox group, feedback, moderation, announcements, bug reports, ambassadors, patch notes, social media, zero spend, creators
sources: vault notes 15-LiveOps/liveops-playbook, weekly-review-template, consulted 2026-11-14; general community practice (model knowledge); Roblox community standards not checked, UNVERIFIED
verify: Response-time and ratio numbers are heuristics, UNVERIFIED. Check Roblox community and Discord rules for under-13 players before building channels.
tags: [liveops, playbook]
---
# Community management (zero spend)

The community tells you what breaks and what to build next, and brings players back for free. Use free channels only.

## Do this
- Open three channels: an in-game feedback button, a Roblox group (wall and announcements), and one Discord server. Add a DevForum thread only if you want developer feedback.
- Link the Discord and group in the game's description and in a small in-game menu button.
- Post every update note in all channels on the same day ([[15-LiveOps/update-cadence]]).
- Reply to bug reports within 24 hours with a status (seen, fixing, cannot reproduce); aim for a visible reply even when no fix is ready (UNVERIFIED target).
- Create channels: announcements, bug-reports, suggestions, chat. Keep the count small (under 8) so the server does not feel empty.
- Use forms or a fixed bug template: device, steps, what happened, what expected.
- Reward helpful reporters with a free cosmetic or a credits line in patch notes (not paid).
- Set rules and moderation: clear rules, mute and ban tools, 2 or more moderators once you have over 100 members (UNVERIFIED).
- Keep younger players safe: no private contact requests, no personal data, follow Roblox's and Discord's age rules (verify).
- Hold a small recurring ritual: weekly poll, sneak peek, or Q and A.

## Channel guide
| Channel | Purpose | Cadence |
|---|---|---|
| In-game feedback | Raw bug and idea capture | Always on |
| Roblox group wall | Public announcements | Each update |
| Discord | Chat, support, sneak peeks | Daily check |
| DevForum | Technical discussion, dev peers | When shipping tools |
| Short social posts (free accounts) | Reach new players | 1 to 3 per week |

## Feedback loop
1. Collect: tag each item (bug, balance, feature, other).
2. Triage weekly in [[15-LiveOps/weekly-review-template]]: top 5 items by frequency.
3. Act: fix or reply with a reason.
4. Close: tell the community what shipped from their feedback ("you asked, we did").

## How to build it in Roblox
- In-game feedback form: a TextBox that sends a RemoteEvent to the server, which logs it to a DataStore or an HttpService webhook to your Discord (HttpService must be enabled; use free webhooks, rate-limit and sanitize; verify current rules).
- Show a "Join our group" prompt after a win, never at join. Group membership may give a free perk.
- Add the version and server id to each report automatically.
- Use code redemptions to thank communities ([[15-LiveOps/code-and-reward-drops]]).

## Tone and trust
- Say what is broken; do not hide bugs.
- Never promise dates you cannot keep; say "next week if tests pass".
- Never argue in public; move to a ticket or private note.

## Measure
- Members, weekly active chatters, bug reports per week, time to first reply, and the share of reports closed.
- Compare retention of players who joined the group or Discord versus others (correlation, UNVERIFIED).

## Mistakes
- Opening channels you cannot staff.
- Marketing with paid ads or paid influencers (not allowed in a zero-spend plan).
- Leaking unreleased info by accident; keep private test details in a private channel.
- Collecting personal data from minors.

## For our Roblox games
Copy the principle: free channels, fast visible replies, close the loop on feedback. Never copy another community's server name, branding, mascots, art, music or logos, and never impersonate another creator or staff.
