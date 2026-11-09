--!nonstrict
-- Display only: dawn bar, 3-word hint, oil and Glow readout, fixed next-reward line, dawn result.
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Shared = ReplicatedStorage:WaitForChild("Shared")
local Economy = require(Shared.Economy)
local Tutorial = require(Shared.Tutorial)

local player = Players.LocalPlayer

local HudController = {}

local function label(parent, size, position, textSize)
	local l = Instance.new("TextLabel")
	l.Size = size
	l.Position = position
	l.BackgroundTransparency = 1
	l.TextColor3 = Color3.fromRGB(255, 236, 190)
	l.TextStrokeTransparency = 0.4
	l.TextSize = textSize
	l.Font = Enum.Font.GothamBold
	l.Text = ""
	l.Parent = parent
	return l
end

function HudController:Init()
	self.last = nil

	local gui = Instance.new("ScreenGui")
	gui.Name = "DawnWatchHud"
	gui.ResetOnSpawn = false
	gui.Parent = player:WaitForChild("PlayerGui")

	local barBack = Instance.new("Frame")
	barBack.Size = UDim2.new(0.5, 0, 0, 14)
	barBack.Position = UDim2.new(0.25, 0, 0, 12)
	barBack.BackgroundColor3 = Color3.fromRGB(14, 26, 54)
	barBack.BorderSizePixel = 0
	barBack.Parent = gui
	self.barFill = Instance.new("Frame")
	self.barFill.Size = UDim2.fromScale(0, 1)
	self.barFill.BackgroundColor3 = Color3.fromRGB(255, 190, 90)
	self.barFill.BorderSizePixel = 0
	self.barFill.Parent = barBack

	self.title = label(gui, UDim2.new(0.5, 0, 0, 24), UDim2.new(0.25, 0, 0, 28), 18)
	self.hint = label(gui, UDim2.new(1, 0, 0, 50), UDim2.new(0, 0, 0.18, 0), 40)
	self.status = label(gui, UDim2.new(0.6, 0, 0, 52), UDim2.new(0.2, 0, 1, -64), 18)
	self.result = label(gui, UDim2.new(1, 0, 0, 60), UDim2.new(0, 0, 0.4, 0), 34)
end

function HudController:Start()
	local remote = ReplicatedStorage:WaitForChild("Remotes"):WaitForChild("HudUpdate")
	remote.OnClientEvent:Connect(function(state)
		self.last = state
		self:_render()
	end)
	for _, name in { "Oil", "Glow", "Station" } do
		player:GetAttributeChangedSignal(name):Connect(function()
			self:_render()
		end)
	end
end

function HudController:_render()
	local s = self.last
	if s == nil then
		return
	end
	local glow = player:GetAttribute("Glow") or 0
	local carrying = player:GetAttribute("Oil") == 1

	self.barFill.Size = UDim2.fromScale(math.clamp(s.t / s.length, 0, 1), 1)

	if s.phase == "Night" then
		local name = if s.tutorial then "First night" else "Night " .. s.night
		self.title.Text = if s.fog ~= "" then name .. "  -  " .. s.fog else name
	elseif s.phase == "Dawn" then
		self.title.Text = "Dawn!"
	else
		self.title.Text = "Step up to the lantern to start the night"
	end

	self.hint.Text = Tutorial.hint({
		phase = s.phase,
		tutorial = s.tutorial,
		carrying = carrying,
		anyLit = s.anyLit,
		popped = s.popped,
		oilLow = s.oilLow and player:GetAttribute("Station") ~= nil,
	})

	local nextUp = Economy.nextUpgrade(glow)
	local board = if nextUp then nextUp.name .. " in " .. nextUp.remaining .. " Glow" else "Board complete"
	self.status.Text = (if carrying then "Carrying oil" else "No oil")
		.. "   |   Glow "
		.. glow
		.. "   |   Next: "
		.. board

	if s.phase == "Dawn" then
		self.result.Text = s.stars .. "/3 stars   +" .. s.glowAward .. " Glow"
	else
		self.result.Text = ""
	end
end

return HudController
