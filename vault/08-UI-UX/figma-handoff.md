---
description: How UI moves between BrewixGames, Figma and Roblox Studio: SVG mockups with named layers, editing in Figma, and free plugins that turn a Figma frame into Roblox UI.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: figma, ui handoff, svg, mockup, layers, import, roblox studio plugin, screengui, frame, design to studio, figma mcp
sources: Figma MCP server docs, plans and rate limits (developers.figma.com/docs/figma-mcp-server, search summary read 2026-11-14); DevForum: Figma Importer Assistant (devforum.roblox.com/t/2961561), roimport (devforum.roblox.com/t/4731967), search summaries read 2026-11-14
verify: Plugin features and Figma's free-plan read limit (6 or 20 a month; Figma's pages disagree) are from search summaries, UNVERIFIED. The Figma login path was not tested from the build sandbox.
tags: [ui-ux, playbook]
---
# Figma handoff

Two ways to get a screen into Figma. Both end the same way: Andrew sends the Figma frame to Studio with a free plugin, and Frame writes the scripts behind it.

## Do this
- Draw every screen mockup as SVG at 1920x1080 with one `<g>` per UI element, and give each group an `id` that is the Roblox instance name in PascalCase: `ShopFrame`, `BuyButton`, `CoinLabel`, `ItemList`.
- End each id with its kind so importers pick the right class: `...Button` (TextButton or ImageButton), `...Label` (TextLabel), `...Frame` (Frame), `...Scroll` (ScrollingFrame), `...Image` (ImageLabel).
- Use real text elements (`<text>`), not text turned into shapes, and plain rectangles with `rx` for rounded corners. Avoid filters, masks and blend modes: importers drop them.
- Keep one top-level group per screen named like the ScreenGui: `HudGui`, `ShopGui`.
- Colours as hex from the style kit; fonts by the Roblox font name from [[08-UI-UX/fonts-and-text]].
- With the Figma login on: write frames into the file Andrew names; read a file at most once per run (free plan reads are few); if the Figma tools are missing or refuse, fall back to SVG.
- After Andrew imports into Studio, check sizes use Scale (not Offset) and add UIAspectRatioConstraint and UIScale per [[08-UI-UX/mobile-first-checklist]].

## Path A: file handoff (default, no login)
1. Planner or Frame writes the SVG mockups in the run's `mockups/` folder.
2. Andrew presses Edit in Figma on the Game Plan page. The folder and Figma open.
3. Andrew drags the SVGs into a Figma file. Named groups arrive as named layers.
4. Andrew edits, then sends the frame to Studio with a free plugin.

## Path B: Figma login (optional, Settings)
Frame and Planner get Figma's own MCP server and draw frames into Andrew's file directly. It uses Andrew's Figma sign-in, never an API key. Writing is not limited; reading is limited on the free plan.

## Free plugins that turn a Figma frame into Roblox UI
| Plugin | Notes |
|---|---|
| Figma Importer Assistant | Free Studio plugin, manual or automatic import with a companion Figma plugin |
| roimport | Free Figma-to-Roblox plugin |
Check each plugin's page is still free before installing. Never install a paid one.

## For our Roblox games
Copy the workflow, not anyone's designs: Figma community files and other games' screens are references for layout only. Never import another game's logos, icons or art.
