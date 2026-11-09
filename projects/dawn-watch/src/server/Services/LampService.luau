--!nonstrict
-- Server-owned lamp state: fuel, keeper, beam aim. The client only ever sends an aim intent.
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Shared = ReplicatedStorage:WaitForChild("Shared")
local Aim = require(Shared.Aim)
local Config = require(Shared.Config)

local LampService = {}

local WARM = Color3.fromRGB(255, 205, 120)

function LampService:Init(services)
	self.services = services
	self.stations = {}
end

function LampService:Start()
	for _, id in Config.STATION_ORDER do
		self:_build(id)
	end
end

function LampService:_build(id)
	local spec = Config.STATIONS[id]
	local folder = self.services.World.folder
	local pos = Vector3.new(spec.pos[1], spec.pos[2], spec.pos[3])

	local lamp = Instance.new("Part")
	lamp.Name = id
	lamp.Shape = Enum.PartType.Ball
	lamp.Size = Vector3.new(3, 3, 3)
	lamp.Position = pos
	lamp.Anchored = true
	lamp.Color = Color3.fromRGB(70, 70, 80)
	lamp.Material = Enum.Material.Glass
	lamp.Parent = folder

	local light = Instance.new("PointLight")
	light.Range = 22
	light.Color = WARM
	light.Brightness = 0
	light.Parent = lamp

	local beam = Instance.new("Part")
	beam.Name = id .. "Beam"
	beam.Anchored = true
	beam.CanCollide = false
	beam.CanQuery = false
	beam.CanTouch = false
	beam.Material = Enum.Material.Neon
	beam.Color = WARM
	beam.Transparency = 1
	beam.Size = Vector3.new(0.6, 0.6, 1)
	beam.Parent = folder

	local gui = Instance.new("BillboardGui")
	gui.Size = UDim2.fromOffset(90, 24)
	gui.StudsOffset = Vector3.new(0, 3.5, 0)
	gui.AlwaysOnTop = true
	gui.Parent = lamp
	local back = Instance.new("Frame")
	back.Size = UDim2.new(1, 0, 0, 10)
	back.Position = UDim2.fromOffset(0, 12)
	back.BackgroundColor3 = Color3.fromRGB(20, 20, 30)
	back.BorderSizePixel = 0
	back.Parent = gui
	local fill = Instance.new("Frame")
	fill.Size = UDim2.fromScale(0, 1)
	fill.BackgroundColor3 = WARM
	fill.BorderSizePixel = 0
	fill.Parent = back
	local title = Instance.new("TextLabel")
	title.Size = UDim2.new(1, 0, 0, 12)
	title.BackgroundTransparency = 1
	title.TextColor3 = Color3.new(1, 1, 1)
	title.TextStrokeTransparency = 0.5
	title.TextSize = 12
	title.Font = Enum.Font.GothamBold
	title.Text = spec.label
	title.Parent = gui

	local prompt = Instance.new("ProximityPrompt")
	prompt.ActionText = "Tend"
	prompt.ObjectText = spec.label
	prompt.HoldDuration = Config.HOLD_RELIGHT
	prompt.MaxActivationDistance = 12
	prompt.RequiresLineOfSight = false
	prompt.Parent = lamp

	local station = {
		id = id,
		spec = spec,
		pos = pos,
		lamp = lamp,
		light = light,
		beam = beam,
		fill = fill,
		prompt = prompt,
		title = title,
		fuel = 0,
		keeper = nil,
		aim = spec.restAim,
		targetAim = spec.restAim,
		effAngle = spec.restAim,
		effReach = spec.reach,
		outTime = 0,
		botTimer = 0,
	}
	self.stations[id] = station

	prompt.Triggered:Connect(function(player)
		self:_onTend(player, station)
	end)
end

function LampService:Reset()
	for _, id in Config.STATION_ORDER do
		local st = self.stations[id]
		st.fuel = 0
		st.keeper = nil
		st.aim = st.spec.restAim
		st.targetAim = st.spec.restAim
		st.outTime = 0
		st.botTimer = 0
	end
	for _, player in Players:GetPlayers() do
		player:SetAttribute("Station", nil)
		player:SetAttribute("Oil", 0)
	end
end

function LampService:_assign(player, st)
	for _, other in self.stations do
		if other ~= st and other.keeper == player then
			other.keeper = nil
		end
	end
	if st.keeper ~= nil and st.keeper ~= player and st.keeper.Parent ~= nil then
		st.keeper:SetAttribute("Station", nil)
	end
	st.keeper = player
	st.targetAim = st.aim
	player:SetAttribute("Station", st.id)
end

-- Anyone may pour oil (the floating oil-runner role). Pouring or tapping an unkept lamp makes you its keeper.
function LampService:_onTend(player, st)
	if self.services.Night.phase ~= "Night" then
		return
	end
	local keeperGone = st.keeper == nil or st.keeper.Parent == nil
	if player:GetAttribute("Oil") == 1 then
		st.fuel = math.min(Config.FUEL_MAX, st.fuel + Config.OIL_PER_POUR)
		player:SetAttribute("Oil", 0)
		if keeperGone or st.keeper == player then
			self:_assign(player, st)
		end
	elseif keeperGone then
		self:_assign(player, st)
	end
end

function LampService:PlayerLeft(player)
	for _, st in self.stations do
		if st.keeper == player then
			st.keeper = nil
		end
	end
end

function LampService:IsKeeper(player, stationId)
	local st = self.stations[stationId]
	return st ~= nil and st.keeper == player
end

function LampService:SetTargetAim(stationId, angle)
	local st = self.stations[stationId]
	if st ~= nil then
		st.targetAim = angle
	end
end

function LampService:Dim(stationId, amount)
	local st = self.stations[stationId]
	if st ~= nil then
		st.fuel = math.max(0, st.fuel - amount)
	end
end

function LampService:AnyLit()
	for _, st in self.stations do
		if st.fuel > 0 then
			return true
		end
	end
	return false
end

-- True when a human-kept lamp is running low (drives the "Refuel!" hint).
function LampService:OilLow()
	for _, st in self.stations do
		if st.keeper ~= nil and st.fuel < Config.LOW_FUEL then
			return true
		end
	end
	return false
end

function LampService:HumanKeeperCount()
	local count = 0
	for _, st in self.stations do
		if st.keeper ~= nil then
			count += 1
		end
	end
	return count
end

function LampService:OutTimes()
	local list = {}
	for _, id in Config.STATION_ORDER do
		table.insert(list, self.stations[id].outTime)
	end
	return list
end

-- Lit beams for CreatureService: { angle, halfWidth, reach, workRate }.
function LampService:LitBeams()
	local beams = {}
	for _, st in self.stations do
		if st.fuel > 0 then
			table.insert(beams, {
				angle = st.effAngle,
				halfWidth = st.spec.halfWidth,
				reach = st.effReach,
				workRate = st.spec.workRate,
			})
		end
	end
	return beams
end

-- ctx = { active, t, tutorial, fx } where fx comes from Fog.effects.
function LampService:Tick(dt, ctx, creatureSnapshot)
	for _, id in Config.STATION_ORDER do
		local st = self.stations[id]
		if st.keeper ~= nil and st.keeper.Parent == nil then
			st.keeper = nil
		end
		local human = st.keeper ~= nil

		if ctx.active then
			local graceActive = ctx.tutorial and ctx.t < Config.TUTORIAL_GRACE
			if st.fuel > 0 and not graceActive then
				st.fuel = math.max(0, st.fuel - st.spec.burn * dt)
			end
			if ctx.fx.fuelPerSec > 0 then
				st.fuel = math.min(Config.FUEL_MAX, st.fuel + ctx.fx.fuelPerSec * dt)
			end
			if st.fuel <= 0 then
				st.outTime += dt
			end

			-- Bots do the upkeep for unkept stations, slowly.
			if not human and (not ctx.tutorial or ctx.t >= Config.BOT_START_TUTORIAL) then
				if st.fuel < Config.LOW_FUEL then
					st.botTimer += dt
					if st.botTimer >= Config.BOT_REFUEL_DELAY then
						st.fuel = math.min(Config.FUEL_MAX, st.fuel + Config.OIL_PER_POUR)
						st.botTimer = 0
					end
				else
					st.botTimer = 0
				end
			end

			local target = st.targetAim
			local speed = Config.HUMAN_TURN
			if not human then
				speed = Config.HUMAN_TURN * Config.BOT_SPEED_FACTOR
				target = Aim.botTarget(creatureSnapshot) or st.aim
			end
			st.aim = Aim.step(st.aim, target, speed, dt)
			st.effAngle = st.aim + ctx.fx.drift
			st.effReach = st.spec.reach * ctx.fx.reachMul
		end

		self:_render(st, ctx.active)
	end
end

function LampService:_render(st, active)
	local lit = st.fuel > 0
	st.prompt.HoldDuration = if lit then Config.HOLD_TEND else Config.HOLD_RELIGHT
	st.light.Brightness = if lit then 1 + 3 * st.fuel / Config.FUEL_MAX else 0
	st.lamp.Material = if lit then Enum.Material.Neon else Enum.Material.Glass
	st.lamp.Color = if lit then WARM else Color3.fromRGB(70, 70, 80)
	st.fill.Size = UDim2.fromScale(st.fuel / Config.FUEL_MAX, 1)
	st.title.Text = if st.keeper ~= nil
		then st.spec.label .. ": " .. st.keeper.DisplayName
		else st.spec.label .. " (bot)"

	if lit and active then
		local dir = Vector3.new(math.cos(st.effAngle), 0, math.sin(st.effAngle))
		local finish = Vector3.new(dir.X * st.effReach, Config.SEA_Y, dir.Z * st.effReach)
		local length = (finish - st.pos).Magnitude
		st.beam.Size = Vector3.new(0.6, 0.6, length)
		st.beam.CFrame = CFrame.lookAt((st.pos + finish) / 2, finish)
		st.beam.Transparency = 0.55
	else
		st.beam.Transparency = 1
	end
end

return LampService
