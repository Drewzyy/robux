---
description: Copy-paste starter: server and client bootstrappers with two-phase Init/Start, one service, one controller, frozen shared config, a Lune test.
type: template
status: verified
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2 (two-phase bootstrap pattern)
verify: Verified 2026-11-09: these files, written to a temp project, passed scripts/gates/build_gate.py (rojo build, selene, stylua --check, lune tests). Remote validation is pattern-only; test in Studio.
---
# Service and controller template

Files, relative to the project root. Copy, then rename `Coins`.

`src/server/init.server.luau`
```lua
-- Server bootstrapper: require services, Init all, then Start all.
local Services = script:WaitForChild("Services")

local order = {
	require(Services.CoinService),
}

for _, service in order do
	service:Init()
end
for _, service in order do
	service:Start()
end
```

`src/server/Services/CoinService.luau`
```lua
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Config = require(ReplicatedStorage.Shared.Config)
local Validate = require(ReplicatedStorage.Shared.Validate)

local CoinService = {}
local coins: { [Player]: number } = {}
local remote: RemoteEvent

function CoinService:Init()
	remote = Instance.new("RemoteEvent")
	remote.Name = "RequestCollect"
	remote.Parent = ReplicatedStorage
end

function CoinService:Start()
	Players.PlayerAdded:Connect(function(player)
		coins[player] = 0
	end)
	Players.PlayerRemoving:Connect(function(player)
		coins[player] = nil
	end)
	remote.OnServerEvent:Connect(function(player, amount)
		-- Never trust the client: validate type and range, drop silently.
		if not Validate.isFiniteNumber(amount) then
			return
		end
		if amount < 1 or amount > Config.MaxCollectPerRequest then
			return
		end
		coins[player] = (coins[player] or 0) + math.floor(amount)
	end)
end

return CoinService
```

`src/shared/Config.luau`
```lua
return table.freeze({
	MaxCollectPerRequest = 5,
})
```

`src/shared/Validate.luau` (pure, testable in Lune)
```lua
local Validate = {}

function Validate.isFiniteNumber(x: any): boolean
	return type(x) == "number" and x == x and x ~= math.huge and x ~= -math.huge
end

return table.freeze(Validate)
```

`src/client/init.client.luau`
```lua
-- Client bootstrapper: same two-phase shape for controllers.
local controllers = {}
for _, c in controllers do
	c:Init()
end
for _, c in controllers do
	c:Start()
end
```

`tests/run.luau`
```lua
local Validate = require("../src/shared/Validate")

assert(Validate.isFiniteNumber(3), "3 is finite")
assert(not Validate.isFiniteNumber(0 / 0), "NaN rejected")
assert(not Validate.isFiniteNumber(math.huge), "inf rejected")
assert(not Validate.isFiniteNumber("3"), "string rejected")
print("all tests passed")
```

The real game sends what was collected (a coin part), not an amount; the server decides the amount. This template shows the validation shape only. See [[13-Luau-Engineering/remotes-and-security]].
