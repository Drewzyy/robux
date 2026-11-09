--!nonstrict
-- Display only: dawn bar, 3-word hint, oil and Glow readout, fixed next-reward line, dawn result.
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local TweenService = game:GetService("TweenService")

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
	self.gui = gui

	-- Glow bar toward the next upgrade: starts partly full after the first pops (goal gradient).
	local glowBack = Instance.new("Frame")
	glowBack.Size = UDim2.new(0.6, 0, 0, 8)
	glowBack.Position = UDim2.new(0.2, 0, 1, -16)
	glowBack.BackgroundColor3 = Color3.fromRGB(14, 26, 54)
	glowBack.BorderSizePixel = 0
	glowBack.Parent = gui
	self.glowFill = Instance.new("Frame")
	self.glowFill.Size = UDim2.fromScale(0, 1)
	self.glowFill.BackgroundColor3 = Color3.fromRGB(255, 205, 120)
	self.glowFill.BorderSizePixel = 0
	self.glowFill.Parent = glowBack

	self.banner = label(gui, UDim2.new(1, 0, 0, 60), UDim2.new(0, 0, 0.28, 0), 44)
	self.floater = label(gui, UDim2.new(0.4, 0, 0, 40), UDim2.new(0.3, 0, 0.5, 0), 30)
	self.bannerToken = 0
end

-- Shows text for a moment, then clears it (only if nothing newer replaced it).
function HudController:_flash(target, text, seconds)
	self.bannerToken += 1
	local token = self.bannerToken
	target.Text = text
	task.delay(seconds, function()
		if self.bannerToken == token then
			target.Text = ""
		end
	end)
end

-- First pop: banner. Any paid pop: "+N Glow" floats up toward the Glow readout.
function HudController:_onPop()
	local pops = player:GetAttribute("Pops") or 0
	local reward = player:GetAttribute("LastPopGlow") or 0
	if pops == 1 then
		self:_flash(self.banner, "First light!", 2)
	end
	if reward > 0 then
		self.floater.Position = UDim2.new(0.3, 0, 0.5, 0)
		self.floater.Text = "+" .. reward .. " Glow"
		local tween = TweenService:Create(
			self.floater,
			TweenInfo.new(1.2, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
			{ Position = UDim2.new(0.3, 0, 1, -64) }
		)
		tween:Play()
		task.delay(1.3, function()
			self.floater.Text = ""
		end)
	end
end

function HudController:Start()
	local remote = ReplicatedStorage:WaitForChild("Remotes"):WaitForChild("HudUpdate")
	remote.OnClientEvent:Connect(function(state)
		self.last = state
		self:_render()
	end)
	for _, name in { "Oil", "Glow", "Station", "Pops", "FirstRun" } do
		player:GetAttributeChangedSignal(name):Connect(function()
			self:_render()
		end)
	end
	player:GetAttributeChangedSignal("Pops"):Connect(function()
		self:_onPop()
	end)
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
		tutorial = s.tutorial or player:GetAttribute("FirstRun") == true,
		carrying = carrying,
		anyLit = s.anyLit,
		popped = player:GetAttribute("Pops") or 0,
		oilLow = s.oilLow and player:GetAttribute("Station") ~= nil,
	})

	local nextUp = Economy.nextUpgrade(glow)
	self.glowFill.Size = UDim2.fromScale(if nextUp then math.clamp(glow / nextUp.cost, 0, 1) else 1, 1)
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
