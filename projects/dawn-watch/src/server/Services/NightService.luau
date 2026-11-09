--!nonstrict
-- Server owns the night timer, phase, spawning, fog event and the dawn payout.
-- Phases: "Lobby" -> "Night" -> "Dawn" (8 s result) -> "Lobby". Glow is IN MEMORY ONLY this build (no saving).
local Lighting = game:GetService("Lighting")
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local RunService = game:GetService("RunService")

local Shared = ReplicatedStorage:WaitForChild("Shared")
local Config = require(Shared.Config)
local Economy = require(Shared.Economy)
local Fog = require(Shared.Fog)
local NightPlan = require(Shared.NightPlan)

local DAWN_RESULT_SECONDS = 8
local HUD_INTERVAL = 0.25
local SPAWN_CHECK_INTERVAL = 1

local NightService = {}

function NightService:Init(services)
	self.services = services
	self.rng = Random.new()
	self.phase = "Lobby"
	self.nightNumber = 0
	self.tutorial = false
	self.t = 0
	self.fogKind = nil
	self.fogStart = 0
	self.lastSchedule = -100
	self.spawnClock = 0
	self.hudClock = 0
	self.dawnClock = 0
	self.stars = 0
	self.glowAward = 0
	self.firstPopPaid = false
end

function NightService:Start()
	self:_buildStartLantern()

	local function setup(player)
		player:SetAttribute("Oil", 0)
		player:SetAttribute("Glow", 0)
		-- First player in starts the tutorial night after a beat, so they spawn facing the dark lamp.
		if self.phase == "Lobby" and self.nightNumber == 0 then
			task.delay(1, function()
				if self.phase == "Lobby" and self.nightNumber == 0 then
					self:StartNight()
				end
			end)
		end
	end
	Players.PlayerAdded:Connect(setup)
	for _, player in Players:GetPlayers() do
		setup(player)
	end
	Players.PlayerRemoving:Connect(function(player)
		self.services.Lamps:PlayerLeft(player)
	end)

	RunService.Heartbeat:Connect(function(dt)
		self:_tick(dt)
	end)
end

function NightService:_buildStartLantern()
	local part = Instance.new("Part")
	part.Name = "StartLantern"
	part.Shape = Enum.PartType.Ball
	part.Size = Vector3.new(2.5, 2.5, 2.5)
	part.Position = Vector3.new(-7, Config.GALLERY_Y + 1.5, 9)
	part.Anchored = true
	part.Material = Enum.Material.Neon
	part.Color = Color3.fromRGB(255, 205, 120)
	part.Parent = self.services.World.folder

	local prompt = Instance.new("ProximityPrompt")
	prompt.ActionText = "Start Night"
	prompt.ObjectText = "Lantern"
	prompt.HoldDuration = 0 -- retry in under 2 s
	prompt.MaxActivationDistance = 10
	prompt.RequiresLineOfSight = false
	prompt.Parent = part
	prompt.Triggered:Connect(function()
		if self.phase == "Lobby" then
			self:StartNight()
		end
	end)
end

function NightService:StartNight()
	self.nightNumber += 1
	self.tutorial = self.nightNumber == 1
	self.t = 0
	self.fogKind = nil
	self.lastSchedule = -100
	self.spawnClock = 0
	self.stars = 0
	self.glowAward = 0
	self.firstPopPaid = false
	self.services.Creatures:Reset()
	self.services.Lamps:Reset()
	Lighting.ClockTime = 0
	self.phase = "Night"
end

function NightService:_equivalent()
	local humans = #Players:GetPlayers()
	local bots = math.max(0, #Config.STATION_ORDER - self.services.Lamps:HumanKeeperCount())
	return NightPlan.equivalent(humans, bots)
end

function NightService:_maybeSpawn()
	local planT = NightPlan.planTime(self.t, self.tutorial)
	if self.tutorial and not self.services.Lamps:AnyLit() then
		return -- tutorial: the first Mumble only comes after the player lights a lamp
	end
	if self.t - self.lastSchedule < Config.SPAWN_GAP_SECONDS then
		return
	end
	local target = NightPlan.target(planT, self:_equivalent(), self.nightNumber)
	local alive = self.services.Creatures:AlivePoints()
	local kind = NightPlan.pickCreature(planT, self.tutorial, alive, target, function()
		return self.rng:NextNumber()
	end)
	if kind == nil then
		return
	end
	local lane = self.rng:NextInteger(1, #Config.LANE_ANGLES)
	if self.tutorial and self.services.Creatures.popped == 0 and alive == 0 then
		lane = 2 -- first Mumble comes down the east lane, in front of the Harbor lamp
	end
	self.services.Creatures:Schedule(kind, lane, self.t)
	self.lastSchedule = self.t
end

function NightService:_tick(dt)
	local fx = Fog.effects(self.fogKind, self.t - self.fogStart)
	local active = self.phase == "Night"

	if active then
		self.t += dt
		if not self.tutorial and self.fogKind == nil and self.t >= NightPlan.FOG_AT then
			self.fogKind = Fog.pick(self.rng:NextNumber())
			self.fogStart = self.t
		end
		self.spawnClock += dt
		if self.spawnClock >= SPAWN_CHECK_INTERVAL then
			self.spawnClock = 0
			self:_maybeSpawn()
		end
	end

	local ctx = { active = active, t = self.t, tutorial = self.tutorial, fx = fx }
	self.services.Lamps:Tick(dt, ctx, self.services.Creatures:Snapshot())
	if active then
		local poppedNow = self.services.Creatures:Tick(dt, self.t)
		if poppedNow > 0 then
			self:_onPop()
		end
		if self.t >= NightPlan.length(self.tutorial) then
			self:_endNight()
		end
	elseif self.phase == "Dawn" then
		self.dawnClock += dt
		if self.dawnClock >= DAWN_RESULT_SECONDS then
			self.phase = "Lobby"
		end
	end

	self.hudClock += dt
	if self.hudClock >= HUD_INTERVAL then
		self.hudClock = 0
		self:_sendHud()
	end
end

-- Tutorial reward: the first pop drops a few Glow for everyone tending a lamp.
function NightService:_onPop()
	if self.tutorial and not self.firstPopPaid then
		self.firstPopPaid = true
		for _, player in Players:GetPlayers() do
			if player:GetAttribute("Station") ~= nil then
				player:SetAttribute("Glow", (player:GetAttribute("Glow") or 0) + Economy.FIRST_POP)
			end
		end
	end
end

function NightService:_endNight()
	local length = NightPlan.length(self.tutorial)
	local fraction = Economy.outFraction(self.services.Lamps:OutTimes(), length)
	self.stars = Economy.stars(fraction)
	-- TODO: firstNightToday needs persistence (ProfileStore). Not built in this first build, so always false.
	self.glowAward = Economy.glow(self.stars, false)
	for _, player in Players:GetPlayers() do
		player:SetAttribute("Glow", (player:GetAttribute("Glow") or 0) + self.glowAward)
	end
	Lighting.ClockTime = 6
	self.phase = "Dawn"
	self.dawnClock = 0
	self.services.Creatures:Reset()
end

function NightService:_sendHud()
	local length = NightPlan.length(self.tutorial)
	local fogName = ""
	if self.fogKind ~= nil and self.phase == "Night" and self.t - self.fogStart < Fog.DURATION[self.fogKind] then
		fogName = self.fogKind
	end
	self.services.Remotes:SendHud({
		phase = self.phase,
		night = self.nightNumber,
		tutorial = self.tutorial,
		t = self.t,
		length = length,
		anyLit = self.services.Lamps:AnyLit(),
		oilLow = self.services.Lamps:OilLow(),
		popped = self.services.Creatures.popped,
		fog = fogName,
		stars = self.stars,
		glowAward = self.glowAward,
	})
end

return NightService
