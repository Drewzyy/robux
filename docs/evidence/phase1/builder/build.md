# Coin counter, server snippet

STUB: all `13-Luau-Engineering` notes (remotes-and-security, datastore-playbook, project-structure, headless-test-gate, common-bugs-checklist) do not exist yet. The section index is itself `status: stub`. I used the skill's hard rules only. No vault claim is used here.

## What this is
A snippet, not a project. It would live at `src/server/Services/CoinService.luau`. A Rojo layout is not produced.

UNVERIFIED (no vault note, could not check docs):
- ProfileStore API names (`ProfileStore.New`, `StartSessionAsync`, `Reconcile`, `AddUserId`, `EndSession`, `OnSessionEnd`, `Profile.Data`) are from memory of the community module. Confirm them against the version you vendor.
- The require path `ServerStorage.Packages.ProfileStore` is a placeholder. Add the module through Wally or vendoring.

## Code

```luau
--!strict
-- CoinService: server owns coins. Client only says "I touched this coin".
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local ServerStorage = game:GetService("ServerStorage")

local ProfileStore = require(ServerStorage.Packages.ProfileStore) -- UNVERIFIED path
local store = ProfileStore.New("PlayerData_v1", { Coins = 0 }) -- UNVERIFIED API

local COIN_VALUE = 1
local MAX_TOUCH_DISTANCE = 15 -- studs, tune to coin size and latency
local MIN_INTERVAL = 0.1 -- seconds between accepted claims per player

local CoinService = {}
local profiles: { [Player]: any } = {}
local lastClaim: { [Player]: number } = {}
local remote: RemoteEvent

function CoinService.Init()
	remote = Instance.new("RemoteEvent")
	remote.Name = "CoinTouched"
	remote.Parent = ReplicatedStorage
end

local function onLoad(player: Player)
	local profile = store:StartSessionAsync(`Player_{player.UserId}`, {
		Cancel = function()
			return player.Parent ~= Players
		end,
	})
	if not profile then
		player:Kick("Could not load your data. Please rejoin.") -- never play on a default profile
		return
	end
	profile:AddUserId(player.UserId)
	profile:Reconcile()
	profile.OnSessionEnd:Connect(function()
		profiles[player] = nil
		player:Kick("Data session ended. Please rejoin.")
	end)
	if player.Parent ~= Players then
		profile:EndSession()
		return
	end
	profiles[player] = profile
	player:SetAttribute("Coins", profile.Data.Coins) -- replicate for UI only
end

local function onClaim(player: Player, coin: unknown)
	-- validate: type, existence, tag, plausibility, rate
	if typeof(coin) ~= "Instance" or not coin:IsA("BasePart") then return end
	if not coin:IsDescendantOf(workspace) or not coin:HasTag("Coin") then return end
	if coin:GetAttribute("Collected") then return end
	local profile = profiles[player]
	if not profile then return end
	local now = os.clock()
	if now - (lastClaim[player] or 0) < MIN_INTERVAL then return end
	local root = player.Character and player.Character:FindFirstChild("HumanoidRootPart")
	if not (root and root:IsA("BasePart")) then return end
	if (root.Position - coin.Position).Magnitude > MAX_TOUCH_DISTANCE then return end
	local dist = (root.Position - coin.Position).Magnitude
	if dist ~= dist or dist == math.huge then return end -- NaN / inf guard

	lastClaim[player] = now
	coin:SetAttribute("Collected", true) -- single-claim; respawn logic resets it
	profile.Data.Coins += COIN_VALUE
	player:SetAttribute("Coins", profile.Data.Coins)
	coin:Destroy() -- or hide and respawn
end

function CoinService.Start()
	remote.OnServerEvent:Connect(onClaim)
	Players.PlayerAdded:Connect(onLoad)
	for _, p in Players:GetPlayers() do
		task.spawn(onLoad, p)
	end
	Players.PlayerRemoving:Connect(function(player)
		lastClaim[player] = nil
		local profile = profiles[player]
		if profile then
			profiles[player] = nil
			profile:EndSession() -- leave-save
		end
	end)
	-- ProfileStore autosaves while a session is active (UNVERIFIED: check your version).
	game:BindToClose(function()
		for _, profile in profiles do
			profile:EndSession()
		end
		-- TODO: ProfileStore may need a wait for saves to flush before shutdown. UNVERIFIED.
	end)
end

return CoinService
```

## Design to code
| Point | Where |
|---|---|
| Earn coins from client RemoteEvent | `CoinTouched`, handler `onClaim` |
| Server owns state | `profile.Data.Coins`. The `Coins` attribute is a read-only mirror for UI. |
| Validate args | `onClaim`: Instance type, BasePart, in workspace, `Coin` tag, not collected, profile loaded, rate limit, distance, NaN/inf guard |
| Saves between sessions | ProfileStore session, `Reconcile`, leave-save via `EndSession`, `BindToClose` |
| Failed load | Kick, no default play |

Notes:
- The coin parts must have the `Coin` CollectionService tag (set in Studio or by a spawner). The client does `remote:FireServer(coinPart)` on `Touched` for its own character.
- The NaN guard is on the computed distance. The only client argument is an Instance, so no raw number is accepted.
- Multi-server note: a coin the client sees in this server can only be claimed in this server. Fine for the snippet.

## Gate
Nothing was run. Bash was denied, so I could not check which tools are installed, and nothing was linted or built. See gate.json. `passed` is false.

## Open questions
- Should coins respawn, or are they one-time per server?
- Is `MAX_TOUCH_DISTANCE = 15` acceptable? It needs a playtest.
- Which ProfileStore version and path does the studio vendor?

## Loaded
- `00-Start-Here/index.md`: required first read.
- `13-Luau-Engineering/index.md`: Builder home section. It is a stub.
- Tried and missing: `remotes-and-security`, `datastore-playbook`, `project-structure`, `headless-test-gate`, `common-bugs-checklist`.
