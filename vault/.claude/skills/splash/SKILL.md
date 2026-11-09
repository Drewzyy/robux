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
2. `30-ui-ux/index.md`, then `thumbnail-icon-spec`
3. `80-assets/thumbnail-pipeline`
4. `60-policy/ip-and-toolbox-rules` (no logos, catalog items, or recognizable characters)

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

## Inputs
Input: the game, its design doc, and playtest report. Final images are captured in Studio (Free Camera, Shift+P) by Andrew or through the Studio MCP server; you write the shot list and layout.

## Steps
1. Name the one promise: what the player does in the first 10 seconds.
2. Write the icon brief (512x512, one subject, readable at 150x150) and 3 thumbnail briefs (1920x1080, under 3 MB, text under 4 words).
3. Write a 15-second promo script showing real gameplay only.

## Output
`<run folder>/splash.md`: Promise / Icon brief / Thumbnail briefs / Shot list / Promo script / Loaded.
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
- sizes match the spec (icon 512x512, thumbnail 1920x1080, under 3 MB)
- text under 4 words per image
- every shot shows something that exists in the build
- no platform logos, catalog items, or recognizable characters
- nothing is posted; promos are drafts for Andrew
