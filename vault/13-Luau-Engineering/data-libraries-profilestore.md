---
description: ProfileStore (successor to ProfileService) for session-locked player data, plus Lapis and MockDataStoreService, with the start-session pattern, pitfalls, stars and licenses.
type: reference
status: draft
last_checked: 2026-11-15
keywords: profilestore, profileservice, session locking, datastore, player data, lapis, mock datastore, autosave, reconcile, dupe prevention, startsessionasync, saving
sources: https://github.com/MadStudioRoblox/ProfileStore (README.md, docs/index.md, docs/api.md, docs/tutorial/index.md, ProfileStore.luau, read 2026-11-15); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. Behavior (300 second default autosave, nil return on conflict, Cancel option) read from the project docs. UNVERIFIED: how it behaves under live DataStore outages in our game.
tags: [luau-engineering, reference]
---
# Data libraries: ProfileStore, ProfileService, Lapis

Player data needs a session lock so two servers never write the same player at once. That is how item duplication happens in games with trading.

| Repo | Stars | License | Notes |
|---|---|---|---|
| MadStudioRoblox/ProfileStore | 354 | Apache-2.0 | Current library. Single ModuleScript |
| MadStudioRoblox/ProfileService | 332 | Apache-2.0 | Older version; ProfileStore is its successor |
| nezuo/lapis | 82 | MIT | DataStore abstraction with a document model |
| buildthomas/MockDataStoreService | 84 | Apache-2.0 | Offline fake DataStore for tests |

## Do this
- Use ProfileStore for all per-player saved data in new games. Do not write raw DataStore code for player saves.
- Define one template table with defaults (Cash = 0, Items = {}). Call profile:Reconcile() after loading so new fields reach old players.
- Key profiles by the player's UserId as a string.
- Load with StartSessionAsync and pass a Cancel function that returns true when the player has left, so the library stops retrying.
- If StartSessionAsync returns nil, kick the player with a rejoin message. It means another server is mid-claim or the server is shutting down.
- Connect profile.OnSessionEnd to kick the player and clear your cache; the session can end if another server takes the lock.
- Call profile:EndSession() in PlayerRemoving. If you forget, the game makes more and more DataStore requests.
- Call profile:AddUserId(player.UserId) so data deletion requests for privacy rules can be honored.
- Default autosave is 300 seconds (the docs say it was 30 in ProfileService). Do not lower it without a measured reason.
- Never use the Steal option in production; it bypasses the lock. Debugging only.
- Test in Studio with ProfileStore.Mock (in-memory) so test runs do not write to live keys.

## Pattern (our style, short)
```luau
--!strict
local Players = game:GetService("Players")
local ProfileStore = require(ServerPackages.ProfileStore)

local TEMPLATE = { Coins = 0, Pets = {} :: { string } }
local store = ProfileStore.New("PlayerData_v1", TEMPLATE)
local sessions = {}

local function onJoin(player: Player)
	local profile = store:StartSessionAsync(tostring(player.UserId), {
		Cancel = function() return player.Parent ~= Players end,
	})
	if profile == nil then return player:Kick("Data failed to load. Please rejoin.") end
	profile:AddUserId(player.UserId)
	profile:Reconcile()
	profile.OnSessionEnd:Connect(function() sessions[player] = nil; player:Kick("Session ended. Please rejoin.") end)
	if player.Parent == Players then sessions[player] = profile else profile:EndSession() end
end
Players.PlayerAdded:Connect(onJoin)
for _, p in Players:GetPlayers() do task.spawn(onJoin, p) end
Players.PlayerRemoving:Connect(function(p) local s = sessions[p]; if s then s:EndSession() end end)
```

## Pitfalls
- ProfileStore is not for leaderboards or global state; use OrderedDataStores or a separate store for those (the docs say so).
- Data must be JSON-safe: no Instances, no Vector3 or Color3, no mixed tables, no functions. Store plain numbers and strings.
- Changing the store name or key format orphans all saved data. Version the name once ("PlayerData_v1") and migrate inside the template instead.
- Do not read profile.Data from many systems directly. Wrap it in one DataService so changes are validated.
- Purchases: grant items inside ProcessReceipt only after the profile change is recorded, and handle a missing profile by returning NotProcessedYet. See [[13-Luau-Engineering/datastore-playbook]].
- Cross-server gifts: use ProfileStore:MessageAsync, not a second write path.

## For our Roblox games
- Copy the principle: one session-locked profile per player, validated writes, EndSession on leave, mock data in tests.
- Respect Apache-2.0: keep the license and notice with the module. The author asks for optional donations; we do not spend money, so skip it.
- Never copy another game's data keys or template shape; design ours from our own economy.
- Related: [[13-Luau-Engineering/datastore-playbook]], [[13-Luau-Engineering/service-controller-template]].
