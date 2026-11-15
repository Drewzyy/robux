---
description: Ranked table of 30 popular Roblox and Luau GitHub repos with stars, licenses, why each matters for building games, and which vault note covers it; skips spam and exploit repos.
type: reference
status: draft
last_checked: 2026-11-15
keywords: top roblox repos, github, luau, rojo, wally, fusion, react-lua, profilestore, cmdr, knit, jecs, stylua, luau-lsp, lune, open source, libraries
sources: GitHub repository search API, queries roblox, luau, roblox framework, roblox ui, rojo, roblox-ts, roblox networking, roblox state management (read 2026-11-15); cloned READMEs for the repos marked "read"
verify: Stars and licenses are from GitHub search on 2026-11-15 and change daily. "Read" means we cloned and read its README and key source; others are search-description only (UNVERIFIED details). Spam and exploit-tool repos were excluded on purpose.
tags: [start-here, reference]
---
# Top Roblox repos (ranked by stars)

Stars measure attention, not quality. This list keeps only repos useful for building our own games with free tools. Already covered elsewhere: Roblox/creator-docs, Roblox/studio-rust-mcp-server, TabooHarmony/roblox-brain, EL4CTEO/blender-skills, gamedev-skills/awesome-gamedev-agent-skills.

## Do this
- Pick libraries from this table only after reading the covering note and its pitfalls.
- Check the license column before adding anything. MIT and Apache-2.0 are fine as packages; MPL-2.0 tools are fine to run; GPL code is never copied into our scripts.
- Pin exact versions in wally.toml and write down the date you checked it.
- Treat unfamiliar repos with "2026" or "Best ... 2026" in the description as spam until proven otherwise; many have fake star counts near 70.
- Never run or recommend exploit, executor, macro or "script hub" repos; they violate Roblox rules.
- Re-run the search quarterly; stars and maintenance change.

## Ranked table

| # | Repo | Stars | License | Why it matters | Note |
|---|---|---|---|---|---|
| 1 | luau-lang/luau | 6110 | MIT | The language itself: type system, native codegen | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 2 | JohnnyMorganz/StyLua | 2338 | MPL-2.0 | Formatter that keeps agent diffs clean | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 3 | rojo-rbx/rojo | 1874 | MPL-2.0 | Files on disk synced to Studio (read) | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 4 | roblox-ts/roblox-ts | 1336 | MIT | TypeScript to Luau compiler | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 5 | Roblox/cube | 1322 | NOASSERTION | 3D foundation model; license unclear, not game code | not covered |
| 6 | lune-org/lune | 982 | MPL-2.0 | Run Luau outside Roblox for build and test scripts | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 7 | dphfox/Fusion | 817 | MIT | Reactive UI with scopes (read) | [[08-UI-UX/ui-frameworks-fusion-react-lua-vide]] |
| 8 | matthewdean/roblox-web-apis | 726 | none | Unlicensed list of web APIs; read only | not covered |
| 9 | Quenty/NevermoreEngine | 635 | MIT | Module loader plus reusable modules | [[13-Luau-Engineering/frameworks-knit-and-alternatives]] |
| 10 | Sleitnick/Knit | 631 | MIT | Archived service/controller framework (read) | [[13-Luau-Engineering/frameworks-knit-and-alternatives]] |
| 11 | Roblox/roact | 624 | Apache-2.0 | Legacy React-like UI; superseded | [[08-UI-UX/ui-frameworks-fusion-react-lua-vide]] |
| 12 | Roblox/react-luau | 591 | MIT | Roblox mirror of React port | [[08-UI-UX/ui-frameworks-fusion-react-lua-vide]] |
| 13 | jsdotlua/react-lua | 582 | MIT | Community React 17 port (read) | [[08-UI-UX/ui-frameworks-fusion-react-lua-vide]] |
| 14 | MaximumADHD/Roblox-Client-Tracker | 578 | none | Tracks engine API changes; useful for type files | not covered |
| 15 | JohnnyMorganz/luau-lsp | 567 | MIT | Type checking and autocomplete for agents | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 16 | evaera/Cmdr | 547 | MIT | Typed command console (read) | [[13-Luau-Engineering/admin-and-debug-tools-cmdr-adonis-iris]] |
| 17 | Epix-Incorporated/Adonis | 516 | MIT | Full admin system | [[13-Luau-Engineering/admin-and-debug-tools-cmdr-adonis-iris]] |
| 18 | rojo-rbx/rokit | 514 | MIT | Pins tool versions | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 19 | UpliftGames/wally | 514 | MPL-2.0 | Package manager (read) | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 20 | Ukendio/jecs | 486 | MIT | Fast ECS (read) | [[13-Luau-Engineering/ecs-and-character-controllers-jecs-matter-chickynoid]] |
| 21 | Sleitnick/RbxUtil | 477 | MIT | Signal, Trove, Shake, Spring, Timer (read) | [[13-Luau-Engineering/state-and-signals-promise-trove-signal]] |
| 22 | SirMallard/Iris | 362 | MIT | Immediate-mode debug UI (read) | [[13-Luau-Engineering/admin-and-debug-tools-cmdr-adonis-iris]] |
| 23 | centau/vide | 358 | MIT | Solid-style reactive UI (read) | [[08-UI-UX/ui-frameworks-fusion-react-lua-vide]] |
| 24 | evaera/roblox-lua-promise | 356 | MIT | Promise with cancel and retry (read) | [[13-Luau-Engineering/state-and-signals-promise-trove-signal]] |
| 25 | MadStudioRoblox/ProfileStore | 354 | Apache-2.0 | Session-locked player data (read) | [[13-Luau-Engineering/data-libraries-profilestore]] |
| 26 | MadStudioRoblox/ProfileService | 332 | Apache-2.0 | Older version of ProfileStore | [[13-Luau-Engineering/data-libraries-profilestore]] |
| 27 | rojo-rbx/rbxlx-to-rojo | 301 | MPL-2.0 | One-time place to Rojo conversion | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |
| 28 | littensy/charm | 275 | MIT | Atomic state for roblox-ts (read) | [[13-Luau-Engineering/state-and-signals-promise-trove-signal]] |
| 29 | easy-games/chickynoid | 266 | MIT | Server-authoritative character controller (read) | [[13-Luau-Engineering/ecs-and-character-controllers-jecs-matter-chickynoid]] |
| 30 | seaofvoices/darklua | 250 | MIT | Lua code transformer for bundling | [[13-Luau-Engineering/toolchain-wally-rojo-pesde]] |

## Also worth knowing (below the cut)
- littensy/slither 204 MIT, a full open-source game: [[13-Luau-Engineering/open-source-roblox-games-worth-studying]]
- 1Axen/blink 192 MIT and red-blox/zap 190 MIT, typed networking: [[13-Luau-Engineering/networking-libraries-zap-and-packet]]
- frrazer/roblox-game-boilerplate 108 Unlicense, agent-ready template: [[13-Luau-Engineering/frameworks-knit-and-alternatives]]
- PepeElToro41/ui-labs 203 GPL-3.0 and flipbook-labs/flipbook 125 MIT, UI storybook plugins (GPL: use the tool, never copy its code; not read, UNVERIFIED)
- pesde-pkg/pesde 132 MIT, package manager: [[13-Luau-Engineering/toolchain-wally-rojo-pesde]]
- Sleitnick/RbxCameraShaker 128 MIT: [[13-Luau-Engineering/game-feel-utilities-rbxutil-shake-spring-timer]]

## Left out on purpose
- Exploit, executor, account-manager, macro and script-hub repos (for example NatroMacro, Roblox-Account-Manager); they break Roblox rules.
- Player-side tools (bloxstrap, rbxfpsunlocker, sober, vinegar); they are not for building games.
- Search hits whose description is a keyword-stuffed "2026" pitch with no license.

## For our Roblox games
- Copy the principle: choose maintained, permissively licensed packages, pin versions, write what each one is for.
- Never copy code under GPL, code without a license, or any game's art, names, music or logos.
- Related: [[00-Start-Here/recommended-tools]], [[13-Luau-Engineering/index]].
