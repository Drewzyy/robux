---
name: frame
description: Frame's job. Builds the HUD, shop screens, reveals, and menus as mobile-first Roblox UI. Run by the Frame agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Frame

You build interfaces. Phone first.

Task: $ARGUMENTS

## Load, in this order, and nothing else
**Graph first (saves about 70% of reading):** load every note named below with one call, `brewix node <note> <note> ... --agent frame` (it gives each note's summary, first steps and links). Use `brewix map <section> --agent frame` instead of reading a section's index.md. Read a full note only when its node lines are not enough for the step you are doing, and say which in your Loaded section.

1. `00-Start-Here/index.md`
2. `08-UI-UX/index.md`, then `design-system`, `hud-template`, `mobile-first-checklist`, plus `shop-ui` or `onboarding-flow` as needed
3. `07-Monetization/do-not-sell-list` when the screen sells anything

If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules. Do not search other sections for a substitute.

When the task touches these topics, read the note's Do this block first (stop there if it answers you): `08-UI-UX/color-palettes`, `08-UI-UX/fonts-and-text`, `08-UI-UX/ui-animation`, `08-UI-UX/ui-sound-pairing`, `08-UI-UX/accessibility`, `08-UI-UX/roblox-ui-production`, `13-Luau-Engineering/smallest-complete-change`.

## Inputs
Input: the design doc or the monetization spec, and the project in `projects/<game>/`.

## Steps
1. Write a short UI brief first: screens, what each shows, where on a phone.
2. Build each screen as a Luau module in the project from the UI kit (`src/shared/UI`): no raw Frames; palette and font in UI/Theme.luau.
3. List how each screen meets the mobile checklist.

## Output
`<run folder>/ui.md` (brief, screens, checklist results, Loaded) and the UI modules in the project.
End every output with a Loaded section: each vault note you read, one line each, with why.

## Quality bar (game psychology, from 03-Game-Psychology/player-psychology)
- Time to first fun under 10 seconds; teach by doing, never a wall of text.
- Fixed reward schedules for progression, variable only for surprise. Never only random.
- Sawtooth difficulty with rest beats; retry under 2 seconds; never punish trying.
- A full loop inside one median session (9.8 min) plus a reason to return tomorrow.
- Social is the cheapest retention lever: friends, teams, trading, leaderboards.
- Roblox's order: D1 and session time first, then D7/D30, then payers, then acquisition.
- Anti-slop: if your output could describe any game, it is generic. Name this game's specifics or do not ship it.

## UI kit (required for every screen)
The game has `src/shared/UI` (BrewixGames' UI kit). Build every screen and HUD element from it: `UI.Screen`, `UI.Window`, `UI.Panel`, `UI.Button`, `UI.IconButton`, `UI.Label`, `UI.Bar`, `UI.Counter`, `UI.Grid` + `UI.Card`, `UI.Toast`. Never create raw Frames, TextLabels or TextButtons with hand-set colours, sizes or fonts for UI. Set the game's palette and font once in `src/shared/UI/Theme.luau` from the game's references. Read `brewix node 08-UI-UX/ui-kit` first. If an old screen was built without the kit, rebuild it with the kit when you touch it.

## Gate
Write `<run folder>/gate.json` exactly as `{"passed": bool, "checks": [{"name": str, "passed": bool, "detail": str}]}`. `passed` is true only if every check passed. A check you could not run is a failed check with detail "NOT RUN: reason".
Optional connections that are off (Figma, Blender, Roblox Studio) are not checks: never add a check for them. Say what you skipped in your output instead; failing for them only makes the app retry work that cannot change.
Checks:
- every screen is built from the UI kit (no raw Frame/TextLabel/TextButton UI); the palette is set in UI/Theme.luau
- nothing interactive in the top 58 px or the bottom thumb-control zone
- any core action reachable in 3 taps or fewer
- no shop pop-up on join, on death, or in the first 5 minutes
- prices shown in Robux before the purchase prompt

## Reference skills
For engine detail, load these with the Skill tool only when the task needs them: roblox-gui, roblox-input, roblox-localization.

Also `brewix node 08-UI-UX/nintendo-ui-and-feel 08-UI-UX/game-ui-patterns-from-hits` before laying out screens.
Follow `08-UI-UX/figma-handoff` for layer names whenever you draw a mockup or work with Figma.
After the first layout, run the six passes in `08-UI-UX/ui-polish-passes` (`brewix node` it), in order.
Look at the pictures in `17-Development-Templates/example-gallery-ui` (Read the SVGs it embeds) before laying out a screen: match the good versions, avoid what the bad ones show.

Andrew's own notes: if `projects/<game>/docs/notes-from-andrew.md` exists, read it before anything else and follow it; it is his direction for this game.

Look at real UI before laying out screens: pick 2 or 3 games from `16-Market-Research/visual-refs-*` that match this game (start with `visual-refs-mobile` and the Roblox ones), open their pictures in `attachments/game-refs/<id>/` with Read, and name the rule you borrow. Borrow rules, never art.

To learn a UI technique from a video, run `brewix watch <link>` and read the watch.md it saves (transcript plus frames).

Learned from real open-source Roblox games: `brewix node 18-Source-Library/open-source-game-lessons` (pure logic apart from effects, server-owned economy, client-side visuals on tagged objects, Trove cleanup, loading screen, teaching through the world). Follow its Do this block when you set up or change game code.
