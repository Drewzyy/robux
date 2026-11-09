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
local FirstRun = require(Shared.FirstRun)
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
	self.guidedStep = 0 -- 0 nothing guided yet, 1 first Mumble scheduled, 2 second (aim) Mumble scheduled
	self.firstStation = nil
	self.welcomed = {} -- players who already got a welcome Mumble this night
	self.waiting = {} -- first-run players who spawned while no night was running
end

function NightService:Start()
	self:_buildStartLantern()

	local function setup(player)
		player:SetAttribute("Oil", 0)
		player:SetAttribute("Glow", 0)
		player:SetAttribute("Pops", 0) -- this player's own pops; in memory only (no saving this build)
		player:SetAttribute("FirstRun", true) -- guidance shows until Economy.GUIDED_POPS pops
		player.CharacterAdded:Connect(function()
			self:_onSpawn(player)
		end)
		if player.Character ~= nil then
			self:_onSpawn(player)
		end
	end
	Players.PlayerAdded:Connect(setup)
	for _, player in Players:GetPlayers() do
		setup(player)
	end
	Players.PlayerRemoving:Connect(function(player)
		self.services.Lamps:PlayerLeft(player)
		self.welcomed[player] = nil
		self.waiting[player] = nil
	end)

	RunService.Heartbeat:Connect(function(dt)
		self:_tick(dt)
	end)
end

local function pops(player)
	return player:GetAttribute("Pops") or 0
end

-- A first-run player spawns already carrying one pour of oil: no barrel detour before the core verb.
function NightService:_giveStarterOil(player)
	if pops(player) == 0 and player.Character ~= nil then
		player:SetAttribute("Oil", 1)
	end
end

-- T0 for a player is their character spawning. A first-run player never waits for a lantern press.
function NightService:_onSpawn(player)
	if pops(player) > 0 then
		return
	end
	self:_giveStarterOil(player)
	if self.phase == "Night" then
		return -- joined a running night: the welcome Mumble comes when they pour
	end
	self.waiting[player] = true
	if self.phase == "Lobby" then
		task.delay(1, function()
			if self.phase == "Lobby" and player.Parent ~= nil then
				self:StartNight()
			end
		end)
	end
	-- In the Dawn result the night starts when it ends (_tick), a known wait of up to DAWN_RESULT_SECONDS.
end

-- Called by LampService after a pour. Schedules the pre-aimed first or welcome Mumble.
function NightService:OnPoured(player, stationId, isKeeper)
	if self.phase ~= "Night" then
		return
	end
	local lane = Config.FIRST_LANE[stationId]
	if self.tutorial and self.guidedStep == 0 then
		self.guidedStep = 1
		self.firstStation = stationId
		self.welcomed[player] = true
		self:_guidedMumble(stationId, lane, isKeeper)
	elseif
		FirstRun.needsWelcome(pops(player), self.services.Creatures:AlivePoints(), self.welcomed[player] == true)
	then
		self.welcomed[player] = true
		self:_guidedMumble(stationId, lane, isKeeper)
	end
end

function NightService:_guidedMumble(stationId, lane, preAim)
	if preAim then
		self.services.Lamps:PreAim(stationId, Config.LANE_ANGLES[lane])
	end
	self.services.Creatures:Schedule("Mumble", lane, self.t, { fast = true })
	self.lastSchedule = self.t
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
	self.guidedStep = 0
	self.firstStation = nil
	self.welcomed = {}
	self.waiting = {}
	self.services.Creatures:Reset()
	self.services.Lamps:Reset()
	for _, player in Players:GetPlayers() do
		self:_giveStarterOil(player) -- Lamps:Reset zeroed everyone's oil
	end
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
	local popped = self.services.Creatures.popped
	if self.tutorial then
		-- One mechanic at a time: 1 alive until the first pop, 2 until 60 s, then the normal plan.
		local cap = FirstRun.aliveCap(Config, popped, self.t)
		if cap ~= nil then
			target = math.min(target, cap)
		end
	end
	local alive = self.services.Creatures:AlivePoints()
	local kind = NightPlan.pickCreature(planT, self.tutorial, alive, target, function()
		return self.rng:NextNumber()
	end)
	if kind == nil then
		return
	end
	local lane = self.rng:NextInteger(1, #Config.LANE_ANGLES)
	local opts = nil
	if self.tutorial and self.guidedStep == 1 and popped >= 1 and self.firstStation ~= nil then
		-- Second Mumble: the other lane on the same side. It needs a real aim, so it gets a ring.
		self.guidedStep = 2
		lane = Config.SECOND_LANE[self.firstStation]
		kind = "Mumble"
		opts = { ring = true }
	end
	self.services.Creatures:Schedule(kind, lane, self.t, opts)
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
		for _ = 1, poppedNow do
			self:_onPop()
		end
		if self.t >= NightPlan.length(self.tutorial) then
			self:_endNight()
		end
	elseif self.phase == "Dawn" then
		self.dawnClock += dt
		if self.dawnClock >= DAWN_RESULT_SECONDS then
			self.phase = "Lobby"
			-- A first-run player who spawned during the result does not wait for a lantern press.
			if next(self.waiting) ~= nil then
				self:StartNight()
			end
		end
	end

	self.hudClock += dt
	if self.hudClock >= HUD_INTERVAL then
		self.hudClock = 0
		self:_sendHud()
	end
end

-- Fixed per-player pop schedule (Economy.popReward: 10, 5, 5): paid to everyone holding a station.
-- LastPopGlow tells that player's HUD how much to float up; it is set before Pops changes.
function NightService:_onPop()
	for _, player in Players:GetPlayers() do
		if player:GetAttribute("Station") ~= nil then
			local before = pops(player)
			local reward = Economy.popReward(before)
			player:SetAttribute("LastPopGlow", reward)
			if reward > 0 then
				player:SetAttribute("Glow", (player:GetAttribute("Glow") or 0) + reward)
			end
			player:SetAttribute("Pops", before + 1)
			if before + 1 >= Economy.GUIDED_POPS then
				player:SetAttribute("FirstRun", false)
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
