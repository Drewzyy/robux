---
description: Ready Luau for a favorite reminder after 6 minutes of real play (once per player, never rewarded) and playtime gifts every few minutes, server-authoritative and saved.
type: reference
status: verified
last_checked: 2026-11-15
keywords: favorite reminder, favorite popup, 6 minutes, playtime gifts, free rewards, timed rewards, session rewards, retention, PromptSetFavorite, template, luau
sources: creator-docs AvatarPromptResult.yaml (PromptSetFavorite result enum, read 2026-11-15); 06-Retention/favorite-and-follow-prompts; 04-Onboarding/fast-early-rewards
verify: Builds, lints and formats in the studio's build gate. Calling PromptSetFavorite with the place id as an Asset is UNVERIFIED for experiences; the card falls back to pointing at the star button if the call fails. Not run in a live server.
tags: [development-templates, reference]
---
# Engagement template: favorite reminder and playtime gifts

Two small systems the Game Score looks for. The server owns all timing and rewards; the client only shows cards.

## Do this
- Copy the three files below into the game; require `PlaytimeServer` from the server entry (`PlaytimeServer.start()`).
- Change `GIFT_EVERY` and `GIFT_COINS` to fit the economy in [[02-Game-Design/economy-math]]; first gift at 2 minutes, then every 5 by default.
- The favorite card shows once per player after 6 minutes of active play (idle time does not count). It never gives a reward: rewarding favorites is treated as banned (see [[06-Retention/favorite-and-follow-prompts]]).
- Hook `grantCoins` into the game's real currency and save, and add the reveal juice from [[08-UI-UX/reward-moments-and-juice]].
- Keep the card one tap to close and never show it mid-action, per [[07-Monetization/offer-pressure-limits]].

`src/shared/PlaytimeRules.luau`
```lua
--!strict
local PlaytimeRules = {}

PlaytimeRules.FAVORITE_AFTER = 6 * 60
PlaytimeRules.GIFT_FIRST = 2 * 60
PlaytimeRules.GIFT_EVERY = 5 * 60
PlaytimeRules.GIFT_COINS = 25
PlaytimeRules.IDLE_AFTER = 90

-- Seconds of active play needed before gift number `n` (1-based).
function PlaytimeRules.giftDue(n: number): number
	return PlaytimeRules.GIFT_FIRST + (n - 1) * PlaytimeRules.GIFT_EVERY
end

return PlaytimeRules
```

`src/server/PlaytimeServer.luau`
```lua
--!strict
local DataStoreService = game:GetService("DataStoreService")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local PlaytimeRules = require(ReplicatedStorage.Shared.PlaytimeRules)

local store = DataStoreService:GetDataStore("Engagement")

local remote = Instance.new("RemoteEvent")
remote.Name = "Engagement"
remote.Parent = ReplicatedStorage

type State = { active: number, lastInput: number, gifts: number, askedFavorite: boolean }
local states: { [Player]: State } = {}

local PlaytimeServer = {}

-- Replace with the game's real currency grant.
local function grantCoins(player: Player, amount: number)
	local stats = player:FindFirstChild("leaderstats")
	local coins = stats and stats:FindFirstChild("Coins")
	if coins and coins:IsA("IntValue") then
		coins.Value += amount
	end
end

local function load(player: Player)
	local ok, asked = pcall(function()
		return store:GetAsync("fav_" .. player.UserId)
	end)
	states[player] = { active = 0, lastInput = os.clock(), gifts = 0, askedFavorite = ok and asked == true }
end

local function tick(player: Player, st: State, dt: number)
	if os.clock() - st.lastInput > PlaytimeRules.IDLE_AFTER then
		return
	end
	st.active += dt
	if st.active >= PlaytimeRules.giftDue(st.gifts + 1) then
		st.gifts += 1
		grantCoins(player, PlaytimeRules.GIFT_COINS)
		remote:FireClient(player, "gift", PlaytimeRules.GIFT_COINS)
	end
	if not st.askedFavorite and st.active >= PlaytimeRules.FAVORITE_AFTER then
		st.askedFavorite = true
		remote:FireClient(player, "favorite")
		task.spawn(function()
			pcall(function()
				store:SetAsync("fav_" .. player.UserId, true)
			end)
		end)
	end
end

function PlaytimeServer.start()
	Players.PlayerAdded:Connect(load)
	for _, p in Players:GetPlayers() do
		task.spawn(load, p)
	end
	Players.PlayerRemoving:Connect(function(p)
		states[p] = nil
	end)
	-- The client reports that the player is active; the server only trusts the time.
	remote.OnServerEvent:Connect(function(player, kind)
		local st = states[player]
		if st and kind == "active" then
			st.lastInput = os.clock()
		end
	end)
	task.spawn(function()
		while true do
			local dt = task.wait(1)
			for player, st in states do
				tick(player, st, dt)
			end
		end
	end)
end

return PlaytimeServer
```

`src/client/EngagementCards.client.luau`
```lua
--!strict
local AvatarEditorService = game:GetService("AvatarEditorService")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local UserInputService = game:GetService("UserInputService")

local remote = ReplicatedStorage:WaitForChild("Engagement") :: RemoteEvent
local gui = Instance.new("ScreenGui")
gui.Name = "EngagementCards"
gui.ResetOnSpawn = false
gui.Parent = (Players.LocalPlayer :: Player):WaitForChild("PlayerGui")

local lastPing = 0
UserInputService.InputBegan:Connect(function()
	if os.clock() - lastPing > 10 then
		lastPing = os.clock()
		remote:FireServer("active")
	end
end)

local function card(text: string, button: string?, onTap: (() -> ())?)
	local f = Instance.new("Frame")
	f.AnchorPoint = Vector2.new(0.5, 0)
	f.Position = UDim2.fromScale(0.5, 0.08)
	f.Size = UDim2.fromScale(0.42, 0.12)
	f.BackgroundColor3 = Color3.fromRGB(28, 28, 30)
	f.BackgroundTransparency = 0.1
	local corner = Instance.new("UICorner")
	corner.CornerRadius = UDim.new(0, 14)
	corner.Parent = f
	local label = Instance.new("TextLabel")
	label.BackgroundTransparency = 1
	label.Size = UDim2.fromScale(button and 0.62 or 0.9, 1)
	label.Position = UDim2.fromScale(0.04, 0)
	label.TextScaled = true
	label.TextColor3 = Color3.new(1, 1, 1)
	label.Text = text
	label.Parent = f
	if button and onTap then
		local b = Instance.new("TextButton")
		b.Size = UDim2.fromScale(0.3, 0.6)
		b.Position = UDim2.fromScale(0.67, 0.2)
		b.Text = button
		b.TextScaled = true
		b.BackgroundColor3 = Color3.fromRGB(255, 69, 58)
		b.TextColor3 = Color3.new(1, 1, 1)
		b.Activated:Connect(function()
			onTap()
			f:Destroy()
		end)
		b.Parent = f
	end
	f.Parent = gui
	task.delay(8, function()
		if f.Parent then
			f:Destroy()
		end
	end)
end

remote.OnClientEvent:Connect(function(kind: string, amount: number?)
	if kind == "gift" then
		card(string.format("Playtime gift: +%d coins!", amount or 0))
	elseif kind == "favorite" then
		card("Enjoying it? Add it to your favorites so you can find it fast.", "Favorite", function()
			local ok = pcall(function()
				AvatarEditorService:PromptSetFavorite(game.PlaceId, Enum.AvatarItemType.Asset, true)
			end)
			if not ok then
				card("Tap the star on the game's page to favorite it.")
			end
		end)
	end
end)
```

`tests/run.luau`
```lua
local PlaytimeRules = require("../src/shared/PlaytimeRules")

assert(PlaytimeRules.giftDue(1) == 120, "first gift at 2 minutes")
assert(PlaytimeRules.giftDue(2) == 420, "then every 5 minutes")
assert(PlaytimeRules.FAVORITE_AFTER == 360, "favorite card after 6 minutes")
assert(PlaytimeRules.IDLE_AFTER < PlaytimeRules.GIFT_EVERY, "idle players stop earning before the next gift")
print("engagement rules: ok")
```

## For our Roblox games
Copy the systems: rewards for playing, and a polite, once-only favorite ask. Never pay players for favorites, likes or visits, and never show the card during a fight, a round or a purchase.
