---
description: Lessons from reading the code of 12 uncopylocked Roblox games (a casino, a robbery tycoon, team shooters, chess, driving, aim training): systems worth rebuilding and patterns never to copy.
type: playbook
status: draft
last_checked: 2026-11-23
keywords: uncopylocked, casino, blackjack, poker, slots, npc opponents, robbery, heist, bounty, police ai, bots, aim training, capture point, staged objective, round reset, traffic lights, house ownership, placeables, plot, chess, keyframe events, seat lifecycle, profileservice, remote validation, backdoor
sources: Place files from the DevForum post "Mass Uncopylocked | 35 free games and projects" by GamerKreep (17 Mar 2024), read 2026-11-23 with scripts/rbxl_study.luau (scripts extracted, nothing run). Games: Las Vegas at home (597 scripts), Robbing Simulator (1981), Siege Wars, D-Day, WOLF bases, Chess & Checkers, Horror Project, Practice Driving Simulator, RCL aim training.
verify: Read in the code. These games are 3 to 8 years old and partly free-modeled; rebuild the ideas with modern APIs, never copy their scripts or models.
tags: [source-library, playbook]
---
# Uncopylocked games studied

## Do this
- Copy the **ideas**, rebuild the code: server-authoritative, modern APIs (`task.wait`, `workspace:Raycast`, `PivotTo`, `Animator`, `RegisterCollisionGroup`, `GetPartBoundsInBox`), tags and attributes instead of hard-coded names.
- Never copy their scripts or models: they carry Kohl's Admin, Epix and other loaders that `require` outside code or `loadstring` text (backdoor risk), free-model bloat (one game has 1,981 scripts and 67,000 parts) and client-trusted remotes.
- To study another game: `brewix study <link to a .rbxl, a GitHub repo, or a local file>` (scripts, summary and backdoor scan into runs/studies/; runs nothing).

## Best patterns (from Las Vegas at home, the strongest)
- **Physical play beats menus**: sitting on a Seat at a table joins the game; dealer NPCs, 3D cards, voice lines ("Hit", "Bust"). A place feels richer than a screen.
- **One bootstrap, one state machine per table**: a manager finds every `workspace.Games` model and starts its `<Name>Core`; a new table is a copy. Round state lives in Attributes (`RoundPending`, `RoundStarted`, `TimeBeforeTurnEnds`), the UI listens with `GetAttributeChangedSignal`: no custom replication.
- **Seat lifecycle**: on sit, clone the game's GUI into PlayerGui with a reference to the table; it destroys itself when the player stands.
- **Keyframe-synced action**: deal the card when the dealer's animation reaches its `DealCard` keyframe (`KeyframeReached`). Effects that land on the animation feel right.
- **Humans and NPCs share one action API**: NPC opponents answer through the same event a player's remote triggers; random think time (1 to 10 s) makes them feel human. NPCs never spend real saves.
- **Always populated**: a capped pool of NPC gamblers spawns one every 1 to 3 s, so the world never looks empty.
- **One money funnel**: `AdjustCurrency` floors the amount, updates the save and mirrors it to a player Attribute for the HUD. Saves with ProfileService plus Reconcile against a template (session lock, schema growth).
- **Remote hygiene**: check seat and phase, `tonumber`, NaN (`x == x`), min and max, enough cash, floor; per-player per-remote cooldowns.
- **Suspense**: slots roll the result on the server first, then animate the reels to it, with an extra delay when two reels already match.
- **Progression that works**: cash regeneration levels (`1 + level*0.2`, capped), a playtime rank ladder, leaderboards, a rented suite, cosmetic and speed passes. Camera presets on a key (C) and arcade hotkeys.

## Systems worth rebuilding
- **Robbery loop** (Robbing Simulator): timed bagging window (15 s), random payout, cooldown (60 s), alarm, then police arrive; bounty and EXP rise with crime; fence loot at a black market. Lockpicking: hold to pick (use ProximityPrompt `HoldDuration`), cancel if out of range, better doors need a better tool.
- **Combat bots** (police, aim training): nearest target, line-of-sight raycast, aim error grows with distance (`±distance*0.015`), lead the target, random strafe every 0.2 to 0.7 s, magazine and reload, credit the killer, respawn after a delay. Rebuild as one manager with a state machine (idle, chase, shoot, reload) for all bots, PathfindingService to chase, a difficulty table.
- **Objectives** (Siege Wars): staged goals (destroy generators, the shield drops, then the core); capture points that recolour and become team spawns. Rebuild with a zone that counts players per team and a progress meter (`owner`, `progress`, `contested` attributes), not an instant flip on touch.
- **Round reset**: keep a pristine copy of the map in ServerStorage and clone it back, instead of undoing each change.
- **Team loadouts**: on respawn, give every tool stored under the player's team.
- **Traffic lights** (Driving Simulator): phases from a timing table, sensors that detect a VehicleSeat. Car respawn pads with a cooldown.
- **Houses and placeables**: one remote with action strings (Sell, Color, Doors, Lights), sell refunds 50%, placeables owned and cleaned up when the owner leaves.
- **Chess**: keep "reset by cloning the board", but write a real rules module (board table, move generators, server-validated moves) and render from state.

## Never do (seen in these games)
- **Trust the client**: a gun kit lets the client send damage and victim; a house remote lets anyone sell anyone's house; a placeable remote reads the price from a client-sent instance. Validate ownership, range, cooldown and prices on the server, send item ids only.
- **Leak secrets**: a poker table stored each hole card's image in a replicated Attribute, so exploiters could read every hand. Keep hidden state on the server and send each player only their own.
- **Take the stake late**: bets were collected at round start, so cash could be spent in between. Hold the stake when the bet is placed.
- **Unvalidated saves**: a settings remote saved whatever table the client sent. Validate each key.
- **Copy-paste code**: the same 455-line Animate script per NPC, the same 2,358-line gun per weapon, 32 chess pieces named one by one. Use one module and tags.
- **A `while wait()` in every object**, `os.time()` for cooldowns (1 s resolution), DataStore calls without pcall, positional arrays (`data[15]`) instead of named fields.
- **Gambling with Robux**: the casino uses play money only and blocks accounts PolicyService forbids from paid random items. Never let Robux be wagered.

Related: [[18-Source-Library/open-source-game-lessons]], [[13-Luau-Engineering/datastore-playbook]].
