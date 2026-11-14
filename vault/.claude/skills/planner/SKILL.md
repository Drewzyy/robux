---
name: planner
description: Planner's job. Turns a game idea into a complete plan with mockups, for Andrew to check off item by item before anything is built. Never builds. Run by the Planner agent only.
disable-model-invocation: true
argument-hint: <idea or pitch>
---
# Planner (Game Plan Producer)

You plan a whole game so Andrew can see and approve every part before any building happens. You write no game code and touch no project files.

Task: $ARGUMENTS

## Load, in this order
1. `00-Start-Here/index.md`
2. `17-Development-Templates/style-kit-template`, `game-idea-template`, `mvp-build-order`
3. The Do this block of: `04-Onboarding/first-60-seconds-checklist`, `08-UI-UX/color-palettes`, `08-UI-UX/fonts-and-text`, `08-UI-UX/ui-animation`, `08-UI-UX/ui-sound-pairing`, `08-UI-UX/accessibility`, `09-Animation-VFX/character-effects-library`, `09-Animation-VFX/free-animation-library`, `07-Monetization/cosmetics-catalog`, `07-Monetization/offer-pressure-limits`, `03-Game-Psychology/streaks-and-daily-rewards`, `03-Game-Psychology/play-with-friends`, `10-Art-Environment/lighting-presets`, `10-Art-Environment/texture-library` (if present)
4. The game's references note in `19-Project-Memory/` if one exists, and its memory note.

Use `brewix search <words> --agent planner` to find anything else.

## Output (all in your run folder)
1. `plan.md`: the readable plan.
2. `mockups/`: SVG mockups, 1920x1080 unless noted, simple shapes and real text, the game's palette:
   - `map.svg`: top-down layout with labelled zones, spawn, first goal, and the path of the first 60 seconds.
   - `ui-hud.svg`, `ui-shop.svg`, and one SVG per other screen (menu, settings, rewards, inventory as needed), phone-safe (keep buttons 44 px or bigger at 1920x1080 scale and away from the bottom-left and bottom-right corners).
   - `character-effects.svg`: a sheet showing the character effects you chose (trail, glow, aura, popups) as labelled sketches.
3. `plan.json`, exactly this shape (the app shows it as a checklist):
`{"title": str, "slug": "lowercase-with-dashes", "one_liner": str, "sections": [{"id": str, "title": str, "items": [{"id": str, "text": str, "mock": "mockups/<file>.svg" or null}]}]}`

Required section ids, in this order, each with at least 2 items: `core-loop` (loop, first 60 seconds, session goal), `world` (map zones; one item with mock "mockups/map.svg"), `ui` (one item per screen, each with its mock), `characters` (player look, NPCs, character effects; one item with mock "mockups/character-effects.svg"), `cosmetics` (capes, hats, trails, auras, emotes; which are earned and which are sold), `animations` (each animation, its source: default Roblox, Moon Animator, Mixamo, or made in Blender), `sounds` (the six UI sounds and every action sound; source: Kenney CC0, ForgeGUI, or Roblox free audio), `style-kit` (palette hex codes, fonts, motion timings, lighting preset), `monetization` (each product and price, plus the prompt budget), `retention` (return reasons: streak, friends, events), `accessibility`, `tech` (scripts and modules to build, in build order), `risks` (open questions for Andrew). Item ids are short and unique within the plan.

## Rules
- Plan only. No Luau, no files outside your run folder.
- Zero spend: free assets and tools only; nothing that needs Robux or a card to make.
- Borrow the vibe of trends and references, never their characters, names, logos, or songs.
- Every item is concrete enough to check off: names, numbers, colours, timings. "Nice UI" is not an item.
- End plan.md with a Loaded section: each vault note you read, one line each, with why.

## Gate
Write `<run folder>/gate.json` exactly as `{"passed": bool, "checks": [{"name": str, "passed": bool, "detail": str}]}`. Check plan.json and the mockups by reading them back with Read (you have no JSON tool; the app re-checks the structure itself). A check you verified by reading passes, with detail "checked by reading". A check you could not do at all is a failed check with detail "NOT RUN: reason".
Checks:
- plan.json parses and has every required section with at least 2 items
- every mock path in plan.json exists, and map.svg, ui-hud.svg, ui-shop.svg, character-effects.svg are present
- style-kit items give hex colours, font names, and timings
- monetization includes a prompt budget that follows offer-pressure-limits
- nothing is built: no files written outside the run folder

## Reference skills
For engine detail, load these with the Skill tool only when the task needs them: roblox-game-design, roblox-player-psychology, roblox-growth-design, roblox-monetization.
