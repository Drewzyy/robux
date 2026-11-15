---
description: When and how to ask players to favorite, invite friends, and opt in to notifications: timing rules, the Roblox APIs, per-player caps, and the unverified rule on rewarding favorites.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: favorite, PromptSetFavorite, AvatarEditorService, invite friends, PromptGameInvite, SocialService, notifications opt-in, PromptOptIn, follow, ask timing, one-time prompt, referral
sources: creator-docs content/en-us/reference/engine/enums/AvatarPromptResult.yaml (read 2026-11-15); creator-docs content/en-us/production/promotion/invite-prompts.md (read 2026-11-15); creator-docs content/en-us/production/promotion/experience-notifications.md (read 2026-11-15); creator-docs content/en-us/production/promotion/referral-system.md (read 2026-11-15)
verify: API names and the opt-in rules are from the docs. The 6 minute and once-per-player timings are studio rules, not Roblox numbers. The docs say nothing about rewarding favorites or likes: UNVERIFIED, so we do not do it.
tags: [retention, playbook]
---
# Favorite and follow prompts

Favorites, invites and notification opt-ins are the three ways a player tells Roblox "bring me back". Each one is a single native dialog, so the whole job is picking the right moment and asking once.

## Do this
- Ask for a favorite only after real play: at least 6 minutes of session time AND a high moment (a win, a rare find, a level up, a boss down). Never on join, on death, or mid-action.
- Ask once per player, ever. Save `AskedFavorite = true` in the player's data store and never ask again, even if they dismissed it. A player who said no meant no.
- Call `AvatarEditorService:PromptSetFavorite()` from a LocalScript only after the player taps something. Our own "Add to favorites" button is the tap; the native dialog then opens. Wire the `PromptSetFavoriteCompleted` event so the UI knows the result (Success, PermissionDenied, Failed per the AvatarPromptResult enum).
- Do NOT give any reward, bonus, or unlock for favoriting, liking, following or visiting. The docs we have do not state the policy (UNVERIFIED), so treat it as banned: we ask, we thank, we never pay.
- Invite prompts: show an "Invite friends" button in the lobby and after a win. Call `SocialService:CanSendGameInviteAsync()` inside `pcall`, then `SocialService:PromptGameInvite()`. Only on a tap.
- Notification opt-in: call `ExperienceNotificationService:CanPromptOptInAsync()` in a `pcall`, then `PromptOptIn()` once, at a moment that promises something ("Get told when the weekend event starts").
- Cap total asks: at most one ask (favorite or notification opt-in) per session, and never two in the same 10 minutes. Counts toward the prompt budget in [[07-Monetization/offer-pressure-limits]].

## The ask sequence (suggested)
| Session | Moment | Ask |
|---|---|---|
| 1 | 6+ min played, after first big win | Favorite card, once |
| 2 | After a daily reward claim | Notification opt-in (13+ only) |
| 3+ | Lobby, any time | Invite friends button stays visible, no popup |

## How each API behaves (from the docs)
- **Favorite**: `AvatarEditorService:PromptSetFavorite()` shows a native dialog and fires a Completed event with an `AvatarPromptResult`. Success means confirmed, PermissionDenied means dismissed, Failed means error.
- **Invites**: default prompt shows friends with Invite buttons. `ExperienceInviteOptions` lets you set `PromptMessage` (keep it short or it will not show), a specific `InviteUser`, a notification asset, and `LaunchData` (max 200 characters) read with `Player:GetJoinData()`.
- **Notifications**: the opt-in modal does not appear for players under 13, players already opted in, or players who saw the prompt in the last 30 days. Text is fixed by Roblox, you cannot customise it. Sending notifications needs a notification string asset and goes out from the server.
- **Referral rewards**: invites are the one place the docs describe rewarding the action. The friend referral system tracks inviter and invitee through `ReferredByPlayerId` and supports rewards for both. This is Roblox's own feature for invites only, not for favorites.

## Fair vs exploitative
- Fair: a thank-you line after a favorite ("Thanks, see you tomorrow"), a clear reason for notifications, a referral reward through Roblox's own referral system that is small and cosmetic.
- Exploitative (never): locking content behind a favorite, "favorite to get 500 coins", repeating the dialog after a dismissal, fake system-looking popups, guilt copy, asking children to bring friends for pressure rewards.

## Mistakes
- Prompting at join. The player has done nothing yet, so conversion is low and trust is lost.
- Assuming the dialog always works. Check the result and show nothing scary on Failed.
- Showing the notification opt-in to under 13 players. The engine will not show it; hide our own button too.
- Counting a favorite ask as free. It still spends attention; log it as an event (see [[06-Retention/event-taxonomy]]).

## Numbers to track (all studio targets, tune with analytics)
- Favorite prompt shown / session reached 6 minutes: should stay under 1 ask per player lifetime.
- Favorite completion rate (Success / shown): log it; if under 10 percent, the moment is wrong, not the wording. UNVERIFIED benchmark, our guess.
- Opt-in completion rate and 30-day cooldown hits: log the OptInPromptClosed event.
- Invite sends per 100 sessions and friends who actually join through GetJoinData().
- D1 and D7 of players who favorited vs those who did not (do not read this as cause, favorites skew to loyal players).

## Example copy (short, kind, no pressure)
- Favorite card: "Having fun? Add us to your favorites so you can find us again." Buttons: "Add to favorites", "Not now".
- Invite: "Play with a friend?" Button opens the native friend list.
- Notifications: "Want a heads-up when the next event starts?" Never promise rewards for saying yes.
- After "Not now": hide the card, set the flag, say nothing else.

## Build checklist
- [ ] Flag stored per player in DataStore, checked server side before the card is shown.
- [ ] Session timer starts at first input, not at join.
- [ ] Moment trigger is a server event (win, rare drop) sent to the client, with a 10 minute minimum gap.
- [ ] Every call wrapped in pcall, every result handled.
- [ ] No reward tied to the result anywhere in the code path.

## Related
[[06-Retention/notifications-and-reengagement]], [[06-Retention/social-retention]], [[03-Game-Psychology/play-with-friends]], [[03-Game-Psychology/social-status-and-flex]]

## For our Roblox games
Copy the principle: ask rarely, after delight, once, through the native dialog, and measure it. Never copy another game's exact wording, button art or mascots, and never attach rewards to favorites or likes. Audience includes children, so no pressure copy.
