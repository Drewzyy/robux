LAYOUT COMPS ONLY: no game captures in projects/dawn-watch/screenshots (the folder does not exist).

# Splash: Dawn Watch launch thumbnails and icon

## Promise
In the first 10 seconds you grab oil from a glowing barrel, pour it into a dark lamp, and the beam sweeps on (design doc, section 3, 0-10 s). By 30 s you swing that beam onto a fog blob (a Mumble) and it pops into sparkles. Every comp shows only things a new player sees in the first minute: the Harbor Lamp, the oil barrel, the beam, a Mumble, a glowing lane, the dawn bar and the lighthouse. No Flicker, Hushhog, Crown Lens, fog event or co-op moment appears, because those come later.

Palette (3 colours, all fully saturated): deep blue `#0A2CA8` (dominant), lamp amber `#FFB800`, white `#FFFFFF` (fog creatures, text). This matches the design doc palette of warm lamp light against deep blue fog.

The avatar in every comp is a **generic placeholder** I drew (round head, blocky body). Replace it with the real Dawn Watch avatar from a Studio capture. It is not a catalog item or a known character.

## Variations
Each comp has 3 elements, tagged by `id` in the SVG.
- **t01** (avatar + dark lamp + "FUEL IT"): rule 4 (action text) and rule 7 (stakes). Question: what happens when that lamp lights?
- **t02** (huge Mumble + avatar + "WHAT IS IT?"): rule 5 (setup, not payoff) and the fear/curiosity expression. Question: what is that thing?
- **t03** (avatar + amber beam + creature silhouette, no text): rule 4 (text is optional) and rule 6 (one story). Question: what is the beam aimed at?
- **t04** (avatar panic face + dimming lamp + "GOING OUT"): rule 7 (stakes). Question: can I save the light?
- **t05** (avatar + half-risen sun + "9 MINUTES"): rule 4 (a number as text). Question: can I last until the sun is up?
- **t06** (avatar + glowing lane + far creature, no text): rule 5, with the telegraphed lane from the design doc. Question: what is coming down that lane?
- **t07** (avatar + glowing oil barrel + "DON'T STOP"): rule 7 (upkeep as the stake). Question: what happens if I stop?
- **t08** (giant avatar face + eye glints in the dark, no text): rule 2 (the avatar dominates, expression pushed). Question: what is staring back?
- **t09** (tiny avatar + tall lighthouse + "TILL DAWN"): the tall-tower curiosity gap from the rule sheet. Question: can I defend that?
- **t10** (avatar + sparkle pop + "LIGHT IT"): the reward moment of the 10-30 s loop. Question: what did I just pop? This is the closest to a payoff of the set, so it is the riskiest for rule 5.

## Ranking
1. **t02**: the biggest face-like shape reads in 0.3 s and "WHAT IS IT?" adds a hook the image does not say. The Mumble is the first creature a player meets, so the promise is kept.
2. **t06**: strongest curiosity gap and it shows a mechanic unique to this game (the telegraphed lane).
3. **t01**: clearest action and the first thing the player really does.
4. **t03**: simple, no text, shows the core verb. A little generic.
5. **t09**: good scale story, but the avatar is tiny, which breaks the "avatar large" rule.
6. **t04**: good stakes, but "GOING OUT" is ambiguous.
7. **t07**: clear, but the barrel is a weak focal point.
8. **t05**: the sun reads, but 9 minutes is a slow promise.
9. **t08**: strong face, but the dark-eyes element could read as horror. The design doc says no horror or age-rating risk.
10. **t10**: gives away the payoff.

**Launch with: t02. Test next: t06.** After launch, if Creator Analytics shows a weak play-through rate, swap to t06 and measure for 72 hours (rule sheet). Do not change the game to fit a thumbnail.

Untested assumptions: I did not render any SVG. Text fit, overlap and the avatar's share of the frame (rule 2 asks for 30-40%) are unchecked; Andrew should preview in the review queue. Placeholder avatars are about 25-45% of frame height and less in area. All CTR claims in the rule sheet are UNVERIFIED.

## Icon
`thumbnails/icon.svg`, 512x512: one subject (a lit lamp: amber disc, white core, amber beam on deep blue). It uses the same palette as the thumbnails. It is simple enough to read at 150x150, but I did not render it to confirm. It is a separate crop and not a resized thumbnail. Replace it with a Studio capture crop once one exists.

## Shot list for Andrew / Studio MCP (Free Camera, Shift+P)
1. A Mumble in close-up, lit by the beam, in warm-on-blue light (for t02).
2. A glowing lane on the water with a Mumble at its end (t06).
3. The avatar at the Harbor Lamp pouring oil, face to camera (t01, t07).
4. The tall lighthouse from the cove, with the avatar at its foot (t09).
Export 1920x1080 PNG under 3 MB. Avoid .webp.

## Promo script (15 s, real gameplay only; record after the build is playable)
- 0-2 s: spawn on the gallery facing the dark lamp; the arrow reads "Fuel the lamp."
- 2-6 s: grab oil from the barrel, pour it in, and the beam sweeps on (lamp lit within 10 s).
- 6-10 s: a lane glows on the water and a Mumble drifts in; drag the beam onto it.
- 10-12 s: it pops into sparkles, the Glow +5 ticks, and the lamp brightens.
- 12-14 s: the dawn bar at the top, and a second lane lights up.
- 14-15 s: end card, no new footage: "Defend the lighthouse until dawn." Title text only; add no scenes that are not in the build.
Capture on one phone-sized and one PC window. Do not show Flicker, Hushhog, Crown Lens, the fog event or the Board unless they are shipped and seen early.

## Open questions for Andrew
1. Should I use the real avatar rig or a custom character? I need a Studio capture to replace the placeholder.
2. Is the name or logo of Dawn Watch to be on the thumbnail? The rule sheet says never the game title in the text, so I left it out.
3. Are the Mumble and the lamp final? I drew simplified shapes from the design doc, not from art.

## Loaded
- `00-Start-Here/index.md`: required first read.
- `10-Art-Environment/index.md`: the section index.
- `10-Art-Environment/thumbnail-mrbeast-logic.md`: the rule sheet (status: draft; all percentages UNVERIFIED).
- `10-Art-Environment/thumbnail-pipeline.md`: the capture steps and sizes.
- `08-UI-UX/thumbnail-icon-spec.md`: sizes, formats and moderation.
- `01-Roblox-Fundamentals/ip-and-toolbox-rules.md`: no logos, catalog items or recognisable characters.
- `projects/dawn-watch/docs/design.md`: the promise, first 60 seconds, creatures and palette.
- Not loaded: `06-Retention/discovery-signals` (not needed). No MrBeast research was asked for in this task, so none was done.
