---
description: Which public Roblox game and template repos are worth reading (Slither, Matter's Roomba example, Chickynoid demo, boilerplates), what each teaches, their licenses, and which archives to avoid.
type: teardown
status: draft
last_checked: 2026-11-15
keywords: open source roblox game, slither, matter example, chickynoid demo, game template, boilerplate, uncopylocked, reference project, receipt processing, rewards, reflex, react, learning
sources: https://github.com/littensy/slither (README.md, package.json, src layout, products/process-receipt.ts, read 2026-11-15); https://github.com/matter-ecs/matter (example/README.md, read 2026-11-15); https://github.com/easy-games/chickynoid (README.md, read 2026-11-15); https://github.com/frrazer/roblox-game-boilerplate (README.md, read 2026-11-15); GitHub searches 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. Only Slither's layout and receipt code were read in detail. Many search hits for "roblox open source game" are SEO spam with fake star counts around 70; treat any unfamiliar repo with 2026 in its description as untrusted and unreviewed.
tags: [luau-engineering, teardown]
---
# Open-source Roblox games worth studying

Real open-source Roblox games are rare. These are the ones that teach something and have clear licenses. Read them; do not copy art or assets.

| Repo | Stars | License | What it teaches |
|---|---|---|---|
| littensy/slither | 204 | MIT | Full casual game in roblox-ts: React UI, Reflex state, Lapis saves, product and reward services |
| matter-ecs/matter (example/) | 121 | MIT | ECS game "Attack of the Killer Roombas" with hot reload |
| easy-games/chickynoid (example/) | 266 | MIT | Server-authoritative movement, mods for weapons and hitpoints |
| frrazer/roblox-game-boilerplate | 108 | Unlicense | Agent-ready project skeleton with Lifecycle, Packets, ProfileStore |
| BrookenRecord/tabletop-island | 22 | none listed | Social game for tabletop games (license missing, read only) |
| StayBlue/librebox-demo | 51 | MIT | Roblox-compatible open engine experiment, not for production |

## Do this
- Read Slither's folder split before designing ours: `client` (alerts, components, hooks, providers, store), `server` (players, products, rewards, world, store) and `shared` (store slices, remotes, constants). Each server feature owns small services.
- Copy the idea of a product registry: a map from product id to a handler function, and ProcessReceipt that looks the handler up, runs it in pcall, and returns NotProcessedYet if the player or handler is missing. Write our own version.
- Study its split of state: a shared store slice for saves, with selectors, so client and server agree on shape.
- Study Matter's example for hot reload: edit a system file while playing and see it update without restart. Use this idea for tuning in Studio.
- Study Chickynoid's mod system: weapons, hit points and kill bricks are mods that plug into the core, not core edits.
- Use the Frazer boilerplate AGENTS.md as a model for our own agent rules (naming table, one check command, never edit generated folders).
- Before reading any other "open source game" repo, check the license file, the real commit history, and that the code is not an exploit script or a loader.
- Avoid uncopylocked archives (for example a 148-star library of 700 uncopylocked games with no license). Uncopylocked does not mean free to reuse, and copying such games risks takedowns.

## Slither receipt pattern (our own sketch)
```luau
--!strict
local handlers: { [number]: (Player) -> () } = {}
local Products = {}

function Products.Register(productId: number, grant: (Player) -> ())
	handlers[productId] = grant
end

MarketplaceService.ProcessReceipt = function(receipt)
	local player = Players:GetPlayerByUserId(receipt.PlayerId)
	local grant = handlers[receipt.ProductId]
	if not player or not grant then return Enum.ProductPurchaseDecision.NotProcessedYet end
	local ok = pcall(grant, player)
	return ok and Enum.ProductPurchaseDecision.PurchaseGranted or Enum.ProductPurchaseDecision.NotProcessedYet
end
return Products
```
Add receipt-id de-duplication in saved data so a retried receipt never grants twice.

## Pitfalls
- Slither is TypeScript (roblox-ts); its patterns carry over, its syntax does not. Do not paste .ts code into Luau.
- Its shop sells money packs with a cheerful popup. In our games, odds and prices must be plain, and prompts open only after a player tap.
- Template repos drift out of date quickly; check the date of the last commit and package versions.
- Licenses cover code, not always art, music or logos. Slither credits Catppuccin colors, kenney.nl assets and Fluent Emoji; each has its own license (UNVERIFIED terms).
- Matter's example and Chickynoid's demo assume their own project layouts; extracting one piece takes work.

## For our Roblox games
- Copy the principle: small services per feature, product-handler registry, shared state slices, mods for extension, hot reload for tuning.
- Never copy another game's name, art, icons, music, UI layout or reward tables, even from MIT repos.
- Keep our reward and shop systems child-safe: no countdown pressure, no hidden odds, no auto-opening purchase prompts.
- Related: [[13-Luau-Engineering/frameworks-knit-and-alternatives]], [[13-Luau-Engineering/ecs-and-character-controllers-jecs-matter-chickynoid]], [[07-Monetization/index]].
