---
description: Design a Roblox game around voice chat: eligibility and opt-in flags, 13+ and 18+ rules, ratings disclosure, safety reporting, distance falloff, and a no-voice fallback for every feature.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: voice chat, proximity voice, avatar chat, audio device input, audio emitter, mute, age verified, ratings, safety, reports, distance attenuation, opt in, social hangout
sources: creator-docs content/en-us/reference/engine/enums/AvatarChatServiceFeature.yaml (read 2026-11-15); creator-docs content/en-us/reference/engine/enums/VoiceChatDistanceAttenuationType.yaml (read 2026-11-15); creator-docs content/en-us/reference/engine/enums/AudioApiRollout.yaml (read 2026-11-15); creator-docs content/en-us/reference/engine/enums/VoiceChatState.yaml (read 2026-11-15); creator-docs content/en-us/production/promotion/content-ratings.md (read 2026-11-15); creator-docs content/en-us/safety.md (read 2026-11-15); creator-docs content/en-us/production/publishing/account-verification.md (read 2026-11-15)
verify: The Voice chat guide (chat/voice-chat.md) is not in the cloned docs, so enabling steps and exact eligibility rules are UNVERIFIED. Enum facts and rating and safety text were read directly. Age limit for voice itself is not stated in the clone.
tags: [audio, playbook]
---
# Voice chat design

Voice makes social games stronger, and brings the most safety work. Treat it as optional for every player. See also [[12-Multiplayer-NPCs/teams-and-parties]] and age-and-region-policies.

## Do this
- Make every feature work without voice: no quest, trade, reward or rank may require talking.
- Never ask a player to turn voice on. Roblox decides eligibility; players opt in themselves.
- Read the flags from AvatarChatService: both the universe and place audio flags must be on for voice in a place, and the player needs UserAudioEligible and UserAudio.
- Show voice UI (mute button, speaking indicator) only when voice is active for that player. Hide it otherwise without a message that pressures.
- Give an easy way to mute others and report, and keep it large on phones ([[08-UI-UX/accessibility]]).
- Declare communication in the ratings questionnaire (any player to player communication means yes) and remember social hangouts have higher age limits.
- Watch Safety > Overview with the Voice channel filter. A report spike after a voice feature means pause or rework it.
- Keep voice out of any area built around private spaces and out of kid-targeted features.

## Eligibility and flags (from enums)
- AvatarChatServiceFeature is a bitmask: UniverseAudio, PlaceAudio, UserAudioEligible, UserAudio (and the same four for video).
- UserAudioEligible: the user meets requirements such as being age-verified and not banned. It does not mean they opted in.
- The docs say account verification gives access to age-restricted Studio features such as voice chat.
- ChatRestrictionStatus can report a platform restriction (parental controls); expect Restricted for some players.
- VoiceChatState lists Idle, Joining, JoiningRetry, Joined, Leaving, Ended, Failed. Handle Failed with a quiet retry and no punishment.
- MuteState is Muted or Unmuted per participant.

## Spatial voice
- The default voice setup can use AudioDeviceInput and AudioEmitter objects when AudioApiRollout is Enabled; otherwise an internal implementation runs and is hidden.
- VoiceChatDistanceAttenuationType presets: Inverse (inverse-squared, matches an AudioEmitter default) and Legacy (linear squared, min distance 7 and max 80 studs).
- Choose Legacy for a familiar local-chat radius, Inverse for a more natural falloff. Test both in a crowded server.
- Keep voice in its own SoundGroup or bus so you can duck music when people talk (sound-groups-and-mixing).

## Ratings and policy
- Voice and text chat alone do not make a "social hangout", but a game whose main theme is talking with others does, which raises the age limit to 16 (with private spaces, 18 and verified).
- Generative AI that replies to voice counts as AI interaction and must be disclosed; extended AI chat is 18+ verified only.
- Thumbnails and promo video may not contain voice chat conversations.

## Design patterns that need no voice
- Quick chat wheels and emotes.
- Ping and marker systems for teams.
- Visual and sound cues for cooperation.

## Tests before shipping
1. Join with a voice-eligible and a non-eligible account; both must complete the game.
2. Check mute, block and report paths.
3. Check failure states (Failed, no microphone) for clear but calm UI.
4. Review Safety dashboard after one week of live traffic.

## Moderation habits
- Write a short, plain conduct line in the game description and the pause menu.
- Keep a person on call for the first week after any social feature launch.
- Use BanAsync only for clear violations and with a policy-based reason ([[01-Roblox-Fundamentals/moderation-and-appeals]]).
- Use text filtering for any in-game chat bubble or caption feature.

## For our Roblox games
Copy the principle: voice is an optional layer that adds fun for those who choose it, with reporting and mute always close.

Never copy: another game's proximity voice UI or lore. Do not record, store or analyze player voice, and do not reward players for using voice.
