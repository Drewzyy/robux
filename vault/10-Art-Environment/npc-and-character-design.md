---
description: Designing NPCs, pets and mascots for Roblox: silhouette, color roles, face and expression, voice lines, idle behavior, readability at small size, and an original-character checklist.
type: reference
status: draft
last_checked: 2026-11-14
keywords: npc, character design, mascot, silhouette, expression, dialogue, idle, enemy design, shop keeper, pets, readability, color role, original character
sources: Author working knowledge of environment and character art (no pages opened 2026-11-14); Roblox Creator Hub docs, Terrain and avatar pages (titles only, not opened 2026-11-14)
verify: General character design practice. Numbers are starting points. UNVERIFIED: marketing impact of mascots on Roblox.
tags: [art-environment, reference]
---
# NPC and character design

NPCs guide, sell, threaten, and charm. A good one reads in a second and is yours alone. See [[10-Art-Environment/art-direction-for-roblox]] and [[09-Animation-VFX/emotes-and-idle-personality]].

## Do this
- Start with a silhouette test: fill the character in black; it should still be identifiable at 30 studs.
- Assign a role color: friendly NPCs warm (yellow, green), shop blue, danger red or purple, quest givers a unique accent.
- Give each important NPC one memorable feature (hat, tool, tail) and one personality trait (cheerful, grumpy, sleepy).
- Keep faces simple: 2 eyes, 1 mouth, 3-5 expressions (neutral, happy, surprised, angry, sad).
- Add an idle animation and a greeting when the player is within 12-15 studs.
- Mark quest NPCs with an icon above the head and a soft glow.
- Write voice lines of 6-12 words; max 3 lines per visit. See [[02-Game-Design/quest-and-mission-design]].
- Make enemies read as threats: asymmetrical shapes, sharp points, glowing eyes, clear weak points. See [[02-Game-Design/combat-design-basics]].
- Pets and mascots: big head, big eyes, round shape for appeal; check how they look in a crowd of 6. See [[02-Game-Design/genre-playbooks/pet-collecting]].

## Why it works
- Characters carry emotional attachment and shared jokes.
- Role colors and icons reduce text and teach players quickly.
- Unique designs support brand recognition and legal safety.

## Character sheet template
- Name and one-line role.
- Silhouette sketch plus 3 color swatches.
- Personality in 3 words.
- 3 voice lines.
- Idle, talk, and react animations.
- Where they appear and why.

## How to build it in Roblox
- Build with R15 or custom rigs; use Humanoid or AnimationController for non-player characters.
- NPC head turn: a script that rotates the Neck toward the nearest player within range, easing 0.3 s.
- Dialogue: ProximityPrompt opens a short bubble UI with typewriter text under 1.5 s per line.
- Use free tools like Blender for custom meshes and the Animation Editor for animations. See [[10-Art-Environment/3d-model-checklist]].
- Keep triangle counts low (under 2000 for small NPCs) for mobile; UNVERIFIED limit, test.
- Use Humanoid display name hidden and a custom BillboardGui label.

## Numbers (starting points)
- Greeting radius 12-15 studs; icon visibility 40 studs.
- Dialogue line length 6-12 words; 3 lines per visit.
- Palette per character: 3 colors plus 1 accent.
- Enemy design: 3-6 variants per biome with clear size classes (small, mid, boss).
- Pet scale about 0.3-0.5 of avatar height.

## Mistakes
- Look-alike NPCs that confuse quests.
- Long speeches that block play.
- Dark outfits in dark zones.
- Characters that copy a known IP, even loosely.
- No idle animation, so NPCs look like statues.

## For our Roblox games
Copy the method: silhouette test, role colors, one feature plus one trait, short lines. Never copy another game's characters, mascots, names, catchphrases, or voice clips. Invent your own cast and keep a sheet for each.
