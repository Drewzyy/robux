---
description: MrBeast thumbnail logic for Roblox: one subject, one emotion, one prize or stake, 0-3 words; curiosity gap; Splash's workflow and 10-point gate.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Andrew's thumbnail brief (Nov 9 2026), written from 1of10, Thumix, ThumbnailAI, YouThumb, and aaaronmiller/create-viral-content viral-thumbnails reference
verify: The rules are the takeaway; every percentage below is UNVERIFIED (creator-marketing sources, not studies). The 10-point checklist is checked per thumbnail by Splash and partly by scripts/gates/check_stage.py (splash). Real CTR is verified only in Creator Analytics after launch.
---
# Thumbnails: MrBeast logic for Roblox games

**Used by:** Splash. **Also read:** [[50-retention-analytics/discovery-signals]] (play-through rate is the thumbnail's score), [[30-ui-ux/thumbnail-icon-spec]] (sizes, moderation), [[60-policy/ip-and-toolbox-rules]].

## The one rule
A thumbnail's job is to stop the scroll. Not to explain the game. It creates a question the viewer can only answer by clicking.

## The formula
**One clear subject + one strong emotion + one contextual element + minimal text.**
UNVERIFIED (creator-marketing sources): MrBeast's thumbnails average 10-15% CTR where 4-6% is strong; he tests 10-20 variations per video and swaps a thumbnail up to 10 times after publishing. The point that holds regardless of the numbers: it is a repeatable system, not luck.

## Rules
1. **Max 3 elements in frame.** One subject, one object, one text. If a fourth seems needed, it is not.
2. **The face (or avatar) dominates.** 30-40% of the frame, expression pushed past realism (shock, then excitement, then curiosity, then fear), eyes visible, facing the camera, at a rule-of-thirds point. Several faces imply something big is happening. UNVERIFIED figures: faces +35-50% CTR; exaggerated expressions +34% engagement; the brain reads a face in about 100 ms.
3. **Colours: few, saturated, functional.** Max 3 colours, one dominant (60/30/10), each fully saturated, never muted. Complementary pairs (blue and orange, red and cyan, purple and yellow). Clear subject/background separation: bright on dark or dark on bright. Pick colour by the feeling it should trigger (green money, red urgency, gold generosity, blue trust), not by taste. UNVERIFIED: contrasting palettes about +30% CTR.
4. **Text: huge, bold, or nothing.** Most top MrBeast thumbnails have none. When used: 1-3 words (never more than 4), 20-30% of the height, bold condensed sans-serif, all caps, 3-5 px outline, upper or lower third, never over the face, never repeating the title, and it adds something the image does not say. If it is not readable on a phone at arm's length, delete it. UNVERIFIED: more than 4 words about -30% CTR.
5. **Curiosity gap (Loewenstein, information gap theory).** Show the setup or the moment, not the payoff. Enough context to read the genre and the stakes, not enough to answer the question.
6. **One clear story,** readable in about 0.3 seconds (figure attributed to a Microsoft study: UNVERIFIED).
7. **Stakes.** Show the risk, not only the reward.
8. **Consistent branding** across a game's thumbnails: same palette family, text treatment, framing. UNVERIFIED: +80% recognition.

## Applying it to Roblox
- **The face becomes the avatar.** The game's own character, large (30-40%), exaggerated expression and pose, eyes visible, facing camera.
- **The money becomes the game's reward:** coins, wins, a rare pet, the top of the tower, the finish line. Show the prize, not the process.
- **The stakes become the challenge:** a huge jump, a lava floor, a boss, a timer running out.
- **One moment per thumbnail:** about to make the impossible jump; holding the rare prize; facing the boss.
- **Max 3 elements:** one avatar, one prize or hazard, one text at most.
- **Colours from the game,** pushed to full saturation, max 3, one dominant.
- **Text 0-3 words:** a number ("100 STAGES"), a prize ("FREE PET"), or one hook word. Never the game's title.
- **The game's own visuals only.** Studio captures or art of the actual game. No stock images, no other games' art, no real people, no MrBeast likeness, no recognisable characters or logos.
- **Curiosity gap for a game:** the avatar at the base of an impossibly tall tower; a locked door with light behind it; the prize just out of reach. The viewer should ask "can I do that?" or "what is behind that?"
- **The promise must be real.** Whatever the thumbnail shows must happen in the first minutes of play (false advertising hurts first-play bounce, [[50-retention-analytics/discovery-signals]]).

## Sizes (conflict noted)
This brief says Roblox thumbnails are 1280x720. The vault's research ([[30-ui-ux/thumbnail-icon-spec]]) says upload **1920x1080** because smaller images are stretched. Studio rule: make thumbnails at **1920x1080** (same 16:9 framing, so 1280x720 exports cleanly), icon **512x512** as its own crop.

## Splash's workflow
1. **Research** (optional per run): look at MrBeast's 20 most-viewed videos' thumbnails from live sources; record subject, expression, colours, text, element count, story. Update this note's rules only through a reviewed change.
2. **Rule sheet:** this note.
3. **Generate 10-20 variations** as 1920x1080 SVG layout comps plus one 512x512 icon, in `<run folder>/thumbnails/`. Each gets one line: which rule it uses and what question it makes the viewer ask. Embed real captures from `projects/<game>/screenshots/` with a relative `href`; if there are none yet, the comps are layout comps only and say so.
4. **Rank** best to worst with a reason each; name the one to launch with and the one to test next.
5. **Test and iterate** after launch: if Creator Analytics shows a weak play-through rate, swap in the next ranked variation and measure again (72-hour rule). Never change the game to match a thumbnail.

Andrew previews every comp in the review queue and exports the chosen ones to PNG there. Uploading is his action.

## Gate (every thumbnail, before it ships)
- [ ] One clear story in about 0.3 seconds
- [ ] Max 3 elements in frame
- [ ] Avatar large (30-40%), exaggerated expression, eyes visible, facing camera
- [ ] Max 3 colours, full saturation, one dominant, clear subject/background separation
- [ ] Text 0-3 words, huge, bold, all caps, heavy outline, upper or lower third, not over the face, not the title
- [ ] Text readable on a phone at arm's length, or deleted
- [ ] Curiosity gap: setup, not payoff
- [ ] Uses the game's own visuals
- [ ] Looks like the game the player will actually play
- [ ] Distinct from the game's other thumbnails, consistent with its branding

## Sources
- 1of10: https://1of10.com/blog/how-to-make-thumbnails-like-mrbeast/
- Thumix: https://www.thumix.com/blog/mr-beast-style-thumbnail
- ThumbnailAI: https://www.thumbnailai.pro/blog/mrbeast-thumbnail-analysis
- YouThumb: https://www.youthumb.ai/en/blog/mrbeast-thumbnail-secrets-100m-views
- aaaronmiller/create-viral-content viral-thumbnails reference: https://github.com/aaaronmiller/create-viral-content/blob/HEAD/references/viral-thumbnails.md
