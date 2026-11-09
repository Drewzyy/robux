--!nonstrict
-- First-minute guidance, display only: a light pillar and pulsing arrow over the first dark lamp while any player
-- still has no pops, and a highlight on the oil barrels at the "Refuel!" moment.
-- Server-made parts are the same for every player (a per-player version would need a client controller): UNVERIFIED feel.
local RunService = game:GetService("RunService")
local Players = game:GetService("Players")

local GuideService = {}

local CHECK_INTERVAL = 0.2
local GLOW = Color3.fromRGB(255, 205, 120)

function GuideService:Init(services)
	self.services = services
	self.clock = 0
end

function GuideService:Start()
	local folder = self.services.World.folder

	local pillar = Instance.new("Part")
	pillar.Name = "GuidePillar"
	pillar.Size = Vector3.new(1.5, 40, 1.5)
	pillar.Anchored = true
	pillar.CanCollide = false
	pillar.CanQuery = false
	pillar.CanTouch = false
	pillar.Material = Enum.Material.Neon
	pillar.Color = GLOW
	pillar.Transparency = 1
	pillar.Parent = folder
	self.pillar = pillar

	local arrowGui = Instance.new("BillboardGui")
	arrowGui.Name = "GuideArrow"
	arrowGui.Size = UDim2.fromOffset(60, 60)
	arrowGui.AlwaysOnTop = true
	arrowGui.Enabled = false
	arrowGui.StudsOffset = Vector3.new(0, 6, 0)
	arrowGui.Adornee = pillar
	arrowGui.Parent = pillar
	local arrow = Instance.new("TextLabel")
	arrow.Size = UDim2.fromScale(1, 1)
	arrow.BackgroundTransparency = 1
	arrow.Text = "v"
	arrow.TextScaled = true
	arrow.Font = Enum.Font.GothamBlack
	arrow.TextColor3 = GLOW
	arrow.TextStrokeTransparency = 0.3
	arrow.Parent = arrowGui
	self.arrowGui = arrowGui

	self.highlights = {}
	for _, barrel in self.services.World.barrels do
		local h = Instance.new("Highlight")
		h.FillTransparency = 0.7
		h.FillColor = GLOW
		h.OutlineColor = GLOW
		h.Enabled = false
		h.Adornee = barrel
		h.Parent = barrel
		table.insert(self.highlights, h)
	end

	RunService.Heartbeat:Connect(function(dt)
		self:_tick(dt)
	end)
end

function GuideService:_tick(dt)
	-- Pulse every frame, cheap.
	if self.arrowGui.Enabled then
		self.arrowGui.StudsOffset = Vector3.new(0, 6 + math.sin(os.clock() * 6), 0)
	end
	self.clock += dt
	if self.clock < CHECK_INTERVAL then
		return
	end
	self.clock = 0

	local night = self.services.Night.phase == "Night"
	local needPour = false
	local needRefuel = false
	if night then
		for _, player in Players:GetPlayers() do
			local pops = player:GetAttribute("Pops") or 0
			if pops == 0 then
				needPour = true
			end
			local station = player:GetAttribute("Station")
			if
				player:GetAttribute("FirstRun") == true
				and station ~= nil
				and player:GetAttribute("Oil") == 0
				and self.services.Lamps:IsLow(station)
			then
				needRefuel = true
			end
		end
	end

	local pos = if needPour then self.services.Lamps:FirstDarkPosition() else nil
	if pos ~= nil then
		self.pillar.Position = pos + Vector3.new(0, 20, 0)
		self.pillar.Transparency = 0.5
		self.arrowGui.Enabled = true
	else
		self.pillar.Transparency = 1
		self.arrowGui.Enabled = false
	end
	for _, h in self.highlights do
		h.Enabled = needRefuel
	end
end

return GuideService
