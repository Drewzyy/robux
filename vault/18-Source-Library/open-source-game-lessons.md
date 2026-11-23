---
description: Techniques taken from reading the code of open-source Roblox games (Maze World, Campingsite, Super Coin World) and Roblox's libraries, for code, UI, onboarding, game feel, world and monetization.
type: playbook
status: draft
last_checked: 2026-11-23
keywords: open source, uncopylocked, example game, maze world, campingsite, coin collector, rojo, trove, testez, wally, collectionservice, streamingenabled, loading screen, preloadasync, onboarding signs, coin animation, game pass, server authoritative shop
sources: Code read 2026-11-23 from github.com/MayGo/maze-world (MIT), github.com/matyasjay/campingsite (license not checked), github.com/thegamerbay/simple-roblox-game (MIT), github.com/Roblox/creator-docs (docs CC-BY-4.0, code MIT), github.com/Sleitnick/RbxUtil (MIT). The DevForum list "Mass uncopylocked 35 free games" could not be opened from the research machine.
verify: Patterns quoted were read in the code. Anything marked UNVERIFIED was not. These are small games; they show clean patterns, not hit-game polish.
tags: [source-library, playbook]
---
# Lessons from open-source Roblox games

## Do this
- Split every gameplay module into **pure logic** (numbers, rules, testable) and **engine effects** (parts, tweens, sounds). Pass random rolls in so tests stay deterministic.
- Clean up with **Trove** (Wally `sleitnick/trove@1.3.0`): `trove:AttachToInstance(obj)`, `trove:Add(connection)`, `trove:Clean()` when done. Every connection has an owner.
- **Server owns the economy**: the client sends only an item id; the server works out the price, checks the coins and returns `false, "Need N coins!"` on failure.
- **Server hitbox, client visuals**: the server spawns an invisible pickup and tags it (`CollectionService:AddTag(coin, "AnimatedCoin")`); the client builds the pretty copy (grows in with Back easing, spins, bobs `math.sin(t*3)*0.5`, no shadow) and a particle burst on pickup. Animation never runs on the server.
- Put **world settings in default.project.json** so they are reviewable: `"Workspace": {"$properties": {"StreamingEnabled": true, "StreamingTargetRadius": 512}}`, and Lighting with its effects.
- **Replace the default loading screen**: `ReplicatedFirst:RemoveDefaultLoadingScreen()`, show the game's logo, `ContentProvider:PreloadAsync` the first assets, a slow scenic camera while loading.
- **Teach through the world**: signs that update with what is still needed ("need 2 logs and 1 stone"), one clear goal reachable in about a minute, tools given at spawn, then a replay prompt.

## Code architecture
- Rojo tree with clear realms: `src/server`, `src/client`, `src/shared`; models as built files mapped with `$path` (`src/Models/*.rbxm`), scripts as source.
- Config modules in ReplicatedStorage for remotes, attributes and stats. Version the save: key `Stats_{Version}` plus a schema version inside the data; bump it when the shape changes. For real saving use ProfileStore or UpdateAsync with session locking ([[13-Luau-Engineering/datastore-playbook]]); these small games do not show it.
- TestEZ specs next to pure modules (`CoinLogic.spec.luau`). Our build checks run `tests/run.luau` with Lune.
- Small games: plain modules plus attributes. UI-heavy games: a state store (Maze World uses Rodux with actions replicated by a network middleware); powerful but verbose.
- Utilities worth using (RbxUtil, MIT): Comm (remotes), Component (tag-driven behaviour), Signal, Shake (camera shake), Spring, Timer.

## UI
- One module per screen (Shop, ShopItem, FinishScreen, Timer, Notification) plus shared parts. In our games the shared parts are the UI kit: [[08-UI-UX/ui-kit]].
- Modals close on an outside click: a full-screen transparent `TextButton` behind the panel, with the panel `Active = true` so clicks do not pass through.
- Hide what the game does not use, with retries (SetCore can fail early): `pcall(StarterGui.SetCore, StarterGui, "TopbarEnabled", false)` in a short retry loop; the same for the Backpack when tools are not used.

## Core loop and feel
- Make value visible: rare pickups (20%) are bigger and a different colour and worth 5x.
- Spawn safely: up to 15 random positions, keep the first where `Workspace:GetPartBoundsInBox(CFrame.new(pos), size)` is empty.
- A doubling price per upgrade (`baseCost * 2^bought`) is a complete, readable upgrade curve for a first slice.
- Finish screens plus leaderboards close each round; offer the next round right away.
- Camera shake and sound design were not found in these games (UNVERIFIED); use RbxUtil Shake and [[08-UI-UX/ui-sound-pairing]].

## World and performance
- StreamingEnabled with a target radius from the project file.
- CollectionService tags join server objects to client behaviour; level designers tag parts instead of editing scripts.
- Collision groups for pass-through abilities and room isolation.
- Trigger zones (fall, sound, logic) as small managers that find named or tagged parts.
- A missing model logs a warning and the game still runs.

## Fair monetization
- Game passes for convenience and fun (High Jump, Ghost Mode), never for progress; currency is earned by play.
- Check ownership with `pcall(MarketplaceService.UserOwnsGamePassAsync, ...)`; on error log it and do not guess. Re-apply character perks on every spawn.
- Prompt on the client, verify on the server. See [[07-Monetization/index]].

## Projects read
| Project | Link | License | Good for |
|---|---|---|---|
| Maze World | github.com/MayGo/maze-world | MIT | A full game: Rojo, shop, passes, pets, leaderboards, loading screen, generated mazes |
| Campingsite | github.com/matyasjay/campingsite | not checked | Tidy Rojo layout, versioned save, onboarding signs |
| Super Coin World | github.com/thegamerbay/simple-roblox-game | MIT | Pure-logic split with tests, Trove, client coin animation, server shop |
| RbxUtil | github.com/Sleitnick/RbxUtil | MIT | Trove, Comm, Component, Shake, Signal |
| Roblox templates | creator-docs resources/templates | see docs | Platformer, Laser Tag, Racing places (not read as code; UNVERIFIED) |
