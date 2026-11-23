---
description: The UI kit every game gets in src/shared/UI. Build every screen from its parts for a finished, consistent look on phone and PC.
type: playbook
status: draft
last_checked: 2026-11-23
keywords: ui kit, components, button, panel, window, counter, progress bar, shop grid, card, toast, theme, uiscale, uicorner, uistroke, uigradient, tween, premium ui
sources: Kit code in studio/kits/ui (tested by building every part as real Roblox instances in Lune). Layout and safe-area rules from create.roblox.com/docs (ScreenInsets, SafeAreaCompatibility, adaptive design, appearance modifiers, UI animation). Library survey from GitHub (Fusion MIT, react-lua MIT, Vide MIT). Tap targets from WCAG 2.1 target size.
verify: The style numbers (radii, stroke widths, tween times) are our convention, tuned by eye against popular simulators; no official source gives them. UNVERIFIED in Studio until a game is played with it.
tags: [ui-ux, playbook]
---
# The UI kit

## Do this
- Every game has `src/shared/UI` (the app copies it in once the game has code). Require it: `local UI = require(ReplicatedStorage.Shared.UI)`.
- Build screens ONLY from kit parts. Never make raw Frames and TextLabels with hand-set colours: that is what made our old UI look like grey boxes.
- Change the look in `UI/Theme.luau` (colours, font, sizes, timings), never inline in a screen. Pick the game's palette there first, from the game's references.
- Lay out with `UI.List` and `UI.Grid`, never by hand-positioning repeated items.
- One primary (green) button per screen; close is the red X the Window gives you.

## Parts
| Part | Use | Returns |
|---|---|---|
| `UI.Screen(name)` | One ScreenGui per screen; safe area and device scaling done | ScreenGui |
| `UI.Window(screen, {Title, Size=Vector2})` | Shop, inventory, settings: title, close X, pop open | `win.Body`, `win:Open()`, `:Close()`, `:Toggle()` |
| `UI.Panel(parent, {Size, Position, Color})` | Any box: rounded, outlined, gradient, shadow | `panel.Body` (padded) |
| `UI.Button(parent, {Text, Color, Icon, OnClick, Width})` | Every button: hover grows, press squashes | TextButton |
| `UI.IconButton(parent, {Icon or Text, Color, OnClick})` | Round menu buttons on the HUD edges | TextButton |
| `UI.Label(parent, {Text, TextSize, Color})` | Outlined text in the game font | TextLabel |
| `UI.Bar(parent, {Value, Text, Color})` | Progress, health, XP; `bar:Set(0.6, "6/10")` animates | table |
| `UI.Counter(parent, {Icon, Value})` | Currency pill; `counter:Set(n)` shows 1.2K and pops | table |
| `UI.Grid(parent, cellSize)` + `UI.Card(grid, {Title, Image, ButtonText, RarityColor, OnClick})` | Shops, pets, inventories | ScrollingFrame / Frame |
| `UI.Toast(screen, text, color)` | Short messages at the top | Frame |
| `UI.abbreviate(n)` | 1250 to "1.3K" | string |

## Layout rules
- Everything is in design pixels at 1280x720; `UI.Screen` scales it (phones about 0.68x with a touch bonus, PCs up to 1.3x). Do not add your own UIScale.
- Buttons: 72 design px tall (about 49 px on a phone), at least 8 px apart. Nothing under 56 design px is tappable.
- Text: Body 22, Button 26, Title 36. Never `TextScaled` without `UITextSizeConstraint`.
- HUD: currency counters top left, menu IconButtons down the left or right edge, the main action bottom centre. Keep the bottom-left (thumbstick) and bottom-right (jump) corners clear on phones.
- Always set `AnchorPoint` with `Position`.

## Feel
- Every button already has hover and press motion; windows pop open with Back easing; counters pop on change. Add a sound on Activated ([[08-UI-UX/ui-sound-pairing]]).
- Reward moments: `UI.Toast` plus a Counter pop; bigger moments see [[08-UI-UX/reward-moments-and-juice]].

## Mistakes the kit prevents
Default grey frames, Offset-only sizes, mixed fonts, no outlines, no press feedback, instant pop-in, hand-positioned lists, buttons too small for thumbs.

## Example
```lua
local UI = require(ReplicatedStorage.Shared.UI)
local screen = UI.Screen("Shop")
local shop = UI.Window(screen, { Title = "Egg Shop", Size = Vector2.new(820, 540), Layout = false })
local grid = UI.Grid(shop.Body)
for i, egg in Eggs do
	UI.Card(grid, { Title = egg.Name, Image = egg.Icon, ButtonText = UI.abbreviate(egg.Price),
		RarityColor = UI.Theme.Colors[egg.Rarity], LayoutOrder = i, OnClick = function() buy(egg) end })
end
local hud = UI.Screen("Hud")
local coins = UI.Counter(hud, { Icon = COIN_ICON, Value = 0, Position = UDim2.fromOffset(16, 16) })
UI.IconButton(hud, { Icon = SHOP_ICON, Color = UI.Theme.Colors.Gold, Position = UDim2.new(0, 16, 0.5, 0),
	AnchorPoint = Vector2.new(0, 0.5), OnClick = function() shop:Toggle() end })
```
