---
description: Getting players to bring and stay with friends: invites, friend boosts, co-op goals, shared spaces, and reward-on-join.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: friends, invite, social, co-op, party, friend boost, retention, socialservice, multiplayer, play together
sources: https://create.roblox.com/docs/production/promotion/invite-prompts (search summary); https://devforum.roblox.com/t/2428901
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [game-psychology, playbook]
---
# Play with friends

## Do this
- Prompt invites at a natural pause (after a win, at a party moment) with SocialService:PromptGameInvite. Roblox reported one game got about 10x more friend joins by prompting at pause points (single-studio anecdote).
- Reward on join, not on invite: when a friend arrives through an invite (Player:GetJoinData on the server), reward both. Invite-only rewards get abused.
- Friend boost: +10% to +20% earnings while a friend is in the same server. Show it on screen ("Friend boost +20%").
- Co-op goals that need two people (a door that needs two buttons, a boss with two roles).
- Shared spaces: a hangout area, emotes, sitting, photo spots, trading.
- Make solo play still fun (bots or solo goals), so an empty server is not a dead end.
- Known issue (2025 DevForum): the invite prompt can load slowly; call it after a short wait.
