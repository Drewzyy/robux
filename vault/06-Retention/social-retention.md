---
description: How friends, parties, guilds and trading raise Roblox retention: build order, features, risks, and safety. Social ties are the strongest long-term hook; add them in the right order.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: social, friends, party, guild, clan, trading, co-op, invite, group, leaderboard, gifting, community, retention, safety, mentor
sources: vault notes 03-Game-Psychology/play-with-friends, why-players-return, consulted 2026-11-14; general design practice (model knowledge); Roblox Trust and Safety rules for trading and chat not checked, UNVERIFIED
verify: Effects of social features on retention are widely believed, UNVERIFIED with numbers here. Check Roblox policies on trading, chat and under-13 features before shipping.
tags: [retention, playbook]
---
# Social retention

Players with friends in the game stay longer than solo players. Build social features in layers so new players are not overwhelmed. Psychology: [[03-Game-Psychology/play-with-friends]].

## Do this
- Layer 1 (minute 0 to 5): social by presence. Show other players nearby doing things. No prompts yet.
- Layer 2 (minute 5 to 15): shared reward. Co-op bonus when playing near others (bonus per nearby player, capped).
- Layer 3 (session 2 to 3): invite a friend via the Roblox invite prompt after a win; reward both (see below).
- Layer 4 (day 3 to 7): parties or teams for harder content.
- Layer 5 (week 2 plus): guilds, clans, group ranks, leaderboards.
- Layer 6 (later): trading and gifting, with protections (see [[04-Onboarding/new-player-protection]]).
- Reward the inviter and the friend with small free cosmetics or boosts, never pay-to-invite.
- Never force social to progress. Solo path must exist; social is a faster or richer path.

## Features
| Feature | Retention role | Roblox build |
|---|---|---|
| Friends playing together | Strongest D7 and D30 factor (UNVERIFIED) | SocialService invite prompt, join friend via friends list |
| Party or squad | Shared goals, reasons to coordinate | Server-side party table, party UI, shared quest |
| Guild or clan | Long-term identity | Roblox group ranks or in-game DataStore guilds |
| Trading | Economy and conversation | Server-validated trades, trade lock for new accounts |
| Gifting | Warm return hooks | Free daily gift to a friend, no paid gifting needed |
| Leaderboards | Status, goals | OrderedDataStore, weekly resets, friends-only boards |
| Emotes and wave | Easy interaction | Simple emote buttons, no chat required |

## Rules for trading
- Trade lock for accounts under a level or a day (UNVERIFIED numbers) to stop scam and alt abuse.
- Show both sides' items and a confirm step with a short countdown (3 to 5 s).
- Log all trades. Add a value cap or tax to prevent duplication exploits.
- Check Roblox policy on trading and any item-for-currency rules before building; real-money trading is not allowed (verify in current policy).

## Safety
- Use Roblox's filtered chat and built-in reporting; do not build free-text name or message systems that bypass filtering.
- Keep voice and open chat decisions inside Roblox's platform rules.
- No pressure mechanics that punish players for not inviting.

## How to build it in Roblox
- SocialService:PromptGameInvite for invites, after a success moment.
- Server-side authoritative party and guild tables; save guild membership in DataStore.
- Event tracking: invite_sent, invite_accepted, party_joined, with cohort split (see [[06-Retention/event-taxonomy]]).

## Measure
- D7 and D30 for players who played with a friend versus solo (correlation, not proof).
- Invite acceptance rate, party size, and trades per active player per week.
- Link to the broader view in [[06-Retention/day-1-day-7-day-30-playbook]].

## Mentor and helper loops
- Let veterans earn a small cosmetic for helping new players finish the tutorial or a first boss (verify the helper cannot abuse it with alts).
- Show a friendly "helper" badge; never reveal private data.
- Pair new players in matchmaking with a mentor only if both opt in.

## Quick tests
- Show the invite prompt after the first win versus after the third; compare invite sent and D7.
- Add a co-op bonus and compare session length for grouped versus solo players.

## For our Roblox games
Copy the principle: layered social features, shared goals, safe trading, no forced social. Never copy another game's guild names, trading UI art, characters, music or logos, and never build real-money or off-platform trading hooks.
