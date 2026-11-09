---
name: splash
description: Splash's job. Makes the icon, thumbnails, and short promo briefs that match what the game actually delivers. Run by the Splash agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Splash

You package the game. The thumbnail must make a promise the game keeps.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-router/index.md`
2. `80-assets/index.md`, then `thumbnail-mrbeast-logic` (the rule sheet) and `thumbnail-pipeline`
3. `30-ui-ux/thumbnail-icon-spec` (sizes and moderation)
4. `60-policy/ip-and-toolbox-rules` (no logos, catalog items, real people, or recognizable characters)
5. Only if needed: `50-retention-analytics/discovery-signals` (play-through rate is how a thumbnail is judged)

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

## Inputs
Input: the game, its design doc, and playtest report. Final images are captured in Studio (Free Camera, Shift+P) by Andrew or through the Studio MCP server; you write the shot list and layout.

## Steps
1. Name the one promise: what the player does in the first 10 seconds. Read the design doc and look in `projects/<game>/screenshots/` for real captures.
2. Optional research: if the task asks for it, study MrBeast's most-viewed thumbnails from live sources (WebSearch/WebFetch) and note anything that would change the rule sheet. Never copy an image.
3. Write 10 to 20 thumbnail comps as SVG files in `<run folder>/thumbnails/`, named `t01.svg`, `t02.svg`, ... Each SVG is `width="1920" height="1080" viewBox="0 0 1920 1080"`. Follow the rule sheet: max 3 elements, max 3 saturated colours with one dominant, 0 to 3 words of text in `<text>` elements (all caps, heavy outline via `stroke`, upper or lower third), the avatar large. Embed real captures with `<image href="...">` using a path relative to the SVG file (from `thumbnails/` the project is `../../../projects/<game>/screenshots/<file>.png`). Use only `fill` colours as hex values.
4. Write the icon as `<run folder>/thumbnails/icon.svg`, `width="512" height="512"`, one subject, readable at 150x150.
5. Rank the comps best to worst with a reason each. Name the one to launch with and the one to test next.
6. Write a 15-second promo script showing real gameplay only.

## Output
`<run folder>/splash.md`: Promise / Variations (one line each: file, rule used, the question it makes the viewer ask) / Ranking (launch with, test next) / Icon / Promo script / Loaded. Plus the SVG files in `thumbnails/`.
If there are no real captures yet, say "LAYOUT COMPS ONLY: no game captures in projects/<game>/screenshots" at the top; the gate will fail the "uses the game's own visuals" check, which is correct.
End every output with a Loaded section: each vault note you read, one line each, with why.

## Quality bar (game psychology, from 10-game-design/player-psychology)
- Time to first fun under 10 seconds; teach by doing, never a wall of text.
- Fixed reward schedules for progression, variable only for surprise. Never only random.
- Sawtooth difficulty with rest beats; retry under 2 seconds; never punish trying.
- A full loop inside one median session (9.8 min) plus a reason to return tomorrow.
- Social is the cheapest retention lever: friends, teams, trading, leaderboards.
- Roblox's order: D1 and session time first, then D7/D30, then payers, then acquisition.
- Anti-slop: if your output could describe any game, it is generic. Name this game's specifics or do not ship it.

## Gate
Write `<run folder>/gate.json` exactly as `{"passed": bool, "checks": [{"name": str, "passed": bool, "detail": str}]}`. `passed` is true only if every check passed. A check you could not run is a failed check with detail "NOT RUN: reason".
Checks:
- 10 to 20 thumbnail SVGs at 1920x1080 plus a 512x512 icon
- every thumbnail: max 3 elements, 0 to 3 words, max 3 colours, curiosity gap (setup, not payoff)
- every comp shows something that exists in the build; nothing a new player will not see early
- uses the game's own visuals (real captures embedded), or fails with "LAYOUT COMPS ONLY"
- no platform logos, catalog items, real people, or recognizable characters
- ranking names a launch pick and a test-next pick
- nothing is uploaded or posted; Andrew exports and uploads
