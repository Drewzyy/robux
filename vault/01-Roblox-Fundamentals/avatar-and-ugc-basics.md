---
description: Avatar structure (15 body parts, accessories, layered clothing) and how UGC items, in-game avatar item sales and in-experience creation work, with the 4k triangle budget and safety limits.
type: reference
status: draft
last_checked: 2026-11-15
keywords: avatar, ugc, accessories, layered clothing, rigid accessories, cage mesh, avatar setup, marketplace, in-experience creation, avatar creation token, creator store, emotes, body parts
sources: creator-docs content/en-us/avatar/index.md (read 2026-11-15); creator-docs content/en-us/avatar/create.md (read 2026-11-15); creator-docs content/en-us/avatar/layered-accessories/specifications.md (read 2026-11-15); creator-docs content/en-us/avatar/rigid-accessories/specifications.md (read 2026-11-15); creator-docs content/en-us/production/monetization/avatar-items.md (read 2026-11-15); creator-docs content/en-us/avatar/in-experience-creation.md (read 2026-11-15); creator-docs content/en-us/production/monetization/avatar-creation-token.md (read 2026-11-15); creator-docs content/en-us/safety.md (read 2026-11-15)
verify: Numbers (15 body parts, 4k triangles, 4 bone influences) read in docs 2026-11-15. Marketplace commission rates and seller rules are on pages not in the clone: UNVERIFIED. Selling UGC needs verification and our studio is zero spend, so treat this as awareness only.
tags: [roblox-fundamentals, reference]
---
# Avatar and UGC basics

Every player brings an avatar that persists across games. Know what it is made of before you touch costumes, accessories or avatar-based features.

## Do this
- Design with the player's own avatar in mind: leave it alone unless the game needs a uniform look. Keep head and body scale readable on phones.
- If you create accessories, keep each rigid or layered accessory under 4,000 triangles, and limit bone influences to 4 per vertex when you skin it yourself.
- Use Studio's Avatar Setup and Automatic Skinning Transfer instead of hand rigging. Test with the mannequin and project files before upload.
- Treat player-made content (outfits, names, builds) as untrusted: pre-moderate, make it short lived, or give only preset options.
- Watch your Safety dashboard avatar channel after you add an avatar editor. Roll back if reports spike.
- Do not sell avatar items in a game without the item's creator adding your start place to the item's sale location.
- Keep ratings in mind: ratings cover only content you make, not what players wear.

## Parts of an avatar
- Geometry: 15 mesh body parts under one Model (head, upper and lower torso, and upper arm, lower arm, hand, upper leg, lower leg, foot on each side). Players can mix parts.
- Rigging armature: how the body moves.
- Animations: movement and emotes that express personality.
- Attachment points and cage meshes: where accessories attach and how clothing wraps.
- What you wear: rigid accessories (hats, glasses), layered accessories and clothing that deform with the body, makeup, classic clothing, emotes and animation packs.

## Tools
- Importer (meshes with PBR textures, rigging, skinning), Avatar Setup (generates missing partitions, rig, skin weights, cages, facial animation, attachments), Automatic Skinning Transfer for layered and facial accessories.
- Third party options named in the docs include Blender (free) and GIMP (free). Maya is paid: skip.

## UGC in games
- In-game avatar item sales: the item creator adds your start Place ID to sale locations; you enable the item by Asset ID at Monetization > Avatar Items. You earn a Marketplace commission as game owner.
- In-experience creation: AvatarCreationService lets players build and buy avatar bodies or makeup in your game. Needs an avatar creation token bought with Robux and Marketplace creator requirements. Items go to the player's inventory but cannot be listed or resold.
- Creator Store limits per 30 days: verified accounts 200 mesh, image or model and 100 audio assets; unverified 10 each. Audio under 10 seconds needs verification.
- Creator Store rules bar obfuscated code, remote asset loading (require by id, loadstring, InsertService), and oversized scripts. See [[10-Art-Environment/no-toolbox-rule]] and [[07-Monetization/creator-store-and-ugc]].

## Safety design for kids
- Free-form user creation is a 16+ content flag in the questionnaire. Avoid it for a kid-focused game.
- Moderate avatar editors, filter names, and avoid private-space themes.
- Do not run auctions or scarcity pressure on avatar items sold to children.

## Quick checklist before using avatar features
1. Does the feature need the player's real avatar, or will a simple preset do?
2. Is every text field filtered and every shared image or outfit moderated?
3. Does any rated content change (free-form creation, media sharing)? Resubmit the questionnaire.
4. Is the feature playable with chat and sharing switched off for young players?
5. Are all meshes and textures made by us or clearly licensed?

## Common mistakes
- Importing a body without the 15 part split, so Avatar Setup has to invent partitions.
- Exceeding the 4,000 triangle accessory budget, or caging layered items badly so clothing clips.
- Letting players name or draw things with no filter or review.
- Using a Toolbox accessory with hidden scripts.

## For our Roblox games
Copy the principle: respect the player's avatar, use presets for self-expression, and give cosmetic rewards that are earned or clearly priced.

Never copy: another game's outfits, branded characters or accessory art, and never use a model from the Toolbox without checking it ([[01-Roblox-Fundamentals/ip-and-toolbox-rules]]). No paid avatar items or paid tools for our studio.
