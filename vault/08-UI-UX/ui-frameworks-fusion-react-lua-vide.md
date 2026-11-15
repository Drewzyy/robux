---
description: Compare Fusion, React Lua, Vide and Roact for building Roblox UI in code, with stars, licenses, a tiny example each, and which one an agent should pick.
type: reference
status: draft
last_checked: 2026-11-15
keywords: fusion, react-lua, roact, vide, reactive ui, luau ui library, state binding, hydrate, createRoot, scopes, roblox gui code, declarative ui
sources: https://github.com/dphfox/Fusion (README, docs/tutorials, src, read 2026-11-15); https://github.com/jsdotlua/react-lua (README, docs/api-reference/react-roblox.md, read 2026-11-15); https://github.com/centau/vide (README, docs, read 2026-11-15); GitHub search results 2026-11-15
verify: Stars and licenses are from GitHub search on 2026-11-15. API shapes read from cloned source. Fusion 0.4 is a dev version, so UNVERIFIED that the exact call names are stable. Performance claims between libraries are UNVERIFIED.
tags: [ui-ux, reference]
---
# UI frameworks: Fusion, React Lua, Vide (and legacy Roact)

Code-driven UI libraries let a script describe a ScreenGui as a function of state instead of hand-placing instances. All four below are free and open source.

| Library | Stars | License | Style |
|---|---|---|---|
| dphfox/Fusion | 817 | MIT | State objects (Value, Computed), scopes for cleanup |
| jsdotlua/react-lua | 582 | MIT | Port of ReactJS 17 (components, hooks, createRoot) |
| Roblox/react-luau | 591 | MIT | Roblox's read-only mirror of the same port |
| centau/vide | 358 | MIT | Solid-style signals, functions as reactive values |
| Roblox/roact | 624 | Apache-2.0 | Older React-like library, superseded by React Lua |

## Do this
- Pick ONE UI library per game and keep it for the whole project. Mixing two means two cleanup models.
- Default choice for a new Luau-only game: Fusion. It is small, has strict types, and its scope array makes cleanup obvious.
- Choose React Lua when the team (or the agent) already knows React hooks, or the UI has deep trees with many reusable components. It is larger, so budget load time.
- Choose Vide when you want the lightest reactive layer and are fine with a smaller community.
- Do not start new work on Roact. Treat it as legacy; React Lua ships a roact-compat layer for migration (see react-lua/docs/migrating-from-legacy).
- Keep UI code on the client. State that matters (coins, inventory) lives on the server; UI only reads a copy.
- Destroy every scope or root when a screen closes. See [[13-Luau-Engineering/memory-leak-checklist]].

## What each one solves
- Fusion: binds an instance property to a state object, so changing the value updates the label next frame. Objects take a scope as their first argument and register their own destroy function there; doCleanup(scope) destroys them in reverse order.
- React Lua: you write components as functions that return elements, store local state with hooks, and mount with ReactRoblox.createRoot. Re-rendering is diffed for you.
- Vide: a source is a getter/setter function, and any property set to a function re-runs when its sources change. Cleanup is tied to scopes.

## Short examples (our style, not copied)
Fusion counter:
```luau
--!strict
local Fusion = require(game.ReplicatedStorage.Packages.Fusion)
local scope = Fusion.scoped(Fusion)
local coins = scope:Value(0)

scope:New "TextLabel" {
	Parent = playerGui.ScreenGui,
	Text = scope:Computed(function(use) return `Coins: {use(coins)}` end),
}

-- later, when the screen closes:
Fusion.doCleanup(scope)
```
React Lua mount:
```luau
local React = require(Packages.React)
local ReactRoblox = require(Packages.ReactRoblox)

local function CoinLabel(props: { coins: number })
	return React.createElement("TextLabel", { Text = `Coins: {props.coins}` })
end

local root = ReactRoblox.createRoot(Instance.new("Folder"))
root:render(ReactRoblox.createPortal(React.createElement(CoinLabel, { coins = 5 }), playerGui))
-- on close: root:unmount()
```

## Pitfalls
- Fusion applies state changes on the next frame, so reading a property right after set() shows the old value (documented in the Fusion tutorial).
- Fusion is on a 0.4 dev branch in the clone; pin the exact version in wally.toml and re-read its docs before upgrading.
- React Lua is a large package. Test mobile load time and memory before committing.
- Forgetting root:unmount() or doCleanup() leaks connections every time a menu opens.
- Roact and React Lua share names (createElement, Component); copy-pasted old forum code often mixes them and fails.
- Do not build a full custom UI for the first prototype if plain instances plus a short script would do. See [[08-UI-UX/roblox-ui-production]].

## For our Roblox games
- Copy the principle: UI is a function of state, one owner per screen, cleanup on close, server holds the truth.
- Use the libraries as packages from Wally with their licenses kept in the package folder; MIT and Apache-2.0 allow this.
- Never copy another game's layouts, icons, colors, fonts or sound. Build our own look from [[08-UI-UX/design-system]].
- Never use a UI library to hide odds or push a purchase; purchase prompts only open from a player tap.
