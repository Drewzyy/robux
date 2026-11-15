---
description: Rules for audio you can use in a Roblox game: who may upload, Creator Store limits (10 vs 100 per 30 days), clips under 10 seconds, IP and moderation risk, and a safe sourcing routine.
type: checklist
status: draft
last_checked: 2026-11-15
keywords: audio assets, audio upload, privacy, creator store, copyright, music, license, moderation, verification, sound id, free sounds, voice-over, thumbnails
sources: creator-docs content/en-us/production/creator-store.md (read 2026-11-15); creator-docs content/en-us/production/publishing/account-verification.md (read 2026-11-15); creator-docs content/en-us/production/publishing/ip-guidelines.md (read 2026-11-15); creator-docs content/en-us/production/promotion/content-ratings.md (read 2026-11-15); creator-docs content/en-us/production/publishing/thumbnails.md (read 2026-11-15); creator-docs content/en-us/sound/objects.md (read 2026-11-15)
verify: The dedicated Audio assets page (audio/assets.md) is not in the cloned docs, so upload price, length caps, and the private-by-default audio permission model are UNVERIFIED here. Limits listed below come from the Creator Store and verification pages that were read.
tags: [audio, checklist]
---
# Audio asset rules and privacy

What you may put into a Roblox game as sound, and what can get audio or the game moderated. Pair with [[11-Audio/free-sound-sources]] for where to get safe clips and [[11-Audio/audio-pipeline]] for our import flow.

## Do this
- Use only audio we made, generated under a license we can read, or CC0 clips. Keep a license line for every file in the repo.
- Verify the creator account (age check or government ID, not phone) before depending on audio distribution: verified accounts can distribute 100 audio assets per 30 days, unverified only 10.
- Expect a short-clip rule for unverified accounts: the docs say verification lets you distribute audio under 10 seconds. Check the Creator Hub upload screen for the live length rule (UNVERIFIED exact caps).
- Never upload commercial music, ripped game sounds, film clips or voices of real people. Copyright covers music, and Roblox handles infringement through notices (ip-guidelines).
- Treat audio as content for ratings: realistic or excessive violence, crude humor or romance in audio can be moderated no matter the label.
- Do not put voice-overs, narration or music with lyrics in thumbnails or promo video: the thumbnail rules bar spoken audio.
- Play assets by asset ID in Sound.SoundId, and test in a published place with a clean account: audio that works for the owner may not load for others if permissions are wrong (permission model UNVERIFIED).
- Cap loudness. Sound.Volume goes 0 to 10 and is multiplicative; docs warn that loud source audio rarely needs more than 2.

## Where audio can come from
| Source | Rule of thumb |
|---|---|
| Creator Store audio | Free-to-use library from Roblox and the community; check the item page |
| Our own recordings | Allowed after upload and moderation |
| Generated sound | Check the tool license and Roblox AI content rules: [[01-Roblox-Fundamentals/ai-content-rules]] |
| Third party libraries | Only with a license that allows commercial game use ([[11-Audio/free-sound-sources]]) |
| Toolbox or random models with sounds | Do not use unreviewed: [[10-Art-Environment/no-toolbox-rule]] |

## Creator Store limits for distributing your own audio
- Per 30 days, verified: 200 mesh, image and model assets, 100 audio assets, 10 plugins. Unverified: 10 each, 2 plugins.
- First time distributing audio you must accept all legal agreements and press Submit.
- You cannot sell or share someone else's composite model that contains restricted assets, with an exception for audio and video also available in the store.

## Privacy and safety
- Voice chat and player recordings are different from asset audio: see voice-chat-design.
- Safety dashboard has an Audio channel: reports about audio in your game show there. Spikes mean a clip is offensive or off-theme.
- If a clip is flagged, the fast fix is to replace the asset ID in your config table and republish.
- Do not embed personal data or real names in audio clips.

## Routine before adding any sound
1. Name the file by role (ui_click_01) and record author and license.
2. Confirm license allows commercial use and no attribution is required, or add credit in the game credits.
3. Upload from a verified account, wait for moderation to pass, then reference the asset ID in a config table.
4. Test on phone speakers at low volume and in headphones.
5. Keep a fallback silent state so a moderated asset does not break gameplay.

## Quick decisions
- Need a click, whoosh or chime: use CC0 packs or the Creator Store library.
- Need music: make a short loop ourselves or use a CC0 track; never a famous song.
- Need a voice: record a teammate with consent, or skip voice and use text and sound cues.
- Unsure about a license: leave it out.

## For our Roblox games
Copy the principle: every sound has a known owner and license, and the game works if one sound gets removed.

Never copy: songs, sound effects ripped from other games or films, or voices from famous people. No paid libraries, paid generators or paid plugins.
