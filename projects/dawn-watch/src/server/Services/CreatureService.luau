--!nonstrict
-- Server-owned creature state. Creatures drift in on telegraphed lanes; beams hold them until they pop.
local Debris = game:GetService("Debris")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Shared = ReplicatedStorage:WaitForChild("Shared")
local Beam = require(Shared.Beam)
local Config = require(Shared.Config)
local NightPlan = require(Shared.NightPlan)

local CreatureService = {}

local COLORS = {
	Mumble = Color3.fromRGB(150, 170, 210),
	Flicker = Color3.fromRGB(170, 230, 240),
	Hushhog = Color3.fromRGB(190, 160, 220),
}

function CreatureService:Init(services)
	self.services = services
	self.creatures = {}
	self.pending = {}
	self.lanes = {}
	self.popped = 0
end

function CreatureService:Start()
	local folder = Instance.new("Folder")
	folder.Name = "Creatures"
	folder.Parent = self.services.World.folder
	self.folder = folder

	for index, angle in Config.LANE_ANGLES do
		local lane = Instance.new("Part")
		lane.Name = "Lane" .. index
		lane.Anchored = true
		lane.CanCollide = false
		lane.CanQuery = false
		lane.CanTouch = false
		lane.Material = Enum.Material.Neon
		lane.Color = Color3.fromRGB(255, 205, 120)
		lane.Transparency = 1
		local length = Config.SPAWN_DIST
		lane.Size = Vector3.new(1.2, 0.2, length)
		local mid = Vector3.new(math.cos(angle) * length / 2, Config.SEA_Y - 0.4, math.sin(angle) * length / 2)
		local far = Vector3.new(math.cos(angle) * length, Config.SEA_Y - 0.4, math.sin(angle) * length)
		lane.CFrame = CFrame.lookAt(mid, far)
		lane.Parent = self.folder
		self.lanes[index] = lane
	end
end

function CreatureService:Reset()
	for _, c in self.creatures do
		c.part:Destroy()
	end
	self.creatures = {}
	self.pending = {}
	self.popped = 0
end

-- Telegraph a lane now; the creature appears Config.TELEGRAPH_SECONDS later.
-- opts (optional, first-minute help): { fast = true } uses the short telegraph and near spawn,
-- { ring = true } puts a highlight ring on the creature so the player sees what to aim at.
function CreatureService:Schedule(kind, laneIndex, now, opts)
	local fast = opts ~= nil and opts.fast == true
	table.insert(self.pending, {
		kind = kind,
		lane = laneIndex,
		at = now + (if fast then Config.FIRST_TELEGRAPH else Config.TELEGRAPH_SECONDS),
		dist = if fast then Config.FIRST_SPAWN_DIST else Config.SPAWN_DIST,
		ring = opts ~= nil and opts.ring == true,
	})
end

-- Intensity points alive or already telegraphed.
function CreatureService:AlivePoints()
	local total = 0
	for _, c in self.creatures do
		total += NightPlan.COST[c.kind]
	end
	for _, p in self.pending do
		total += NightPlan.COST[p.kind]
	end
	return total
end

function CreatureService:Snapshot()
	local list = {}
	for _, c in self.creatures do
		table.insert(list, { angle = c.angle, dist = c.dist })
	end
	return list
end

function CreatureService:_spawn(p)
	local spec = Config.CREATURES[p.kind]
	local part = Instance.new("Part")
	part.Name = p.kind
	part.Shape = Enum.PartType.Ball
	part.Size = Vector3.new(spec.size, spec.size, spec.size)
	part.Color = COLORS[p.kind]
	part.Material = Enum.Material.SmoothPlastic
	part.Anchored = true
	part.CanCollide = false
	part.CanQuery = false
	part.Parent = self.folder
	if p.ring then
		local ring = Instance.new("Highlight")
		ring.FillTransparency = 1
		ring.OutlineColor = Color3.fromRGB(255, 230, 160)
		ring.DepthMode = Enum.HighlightDepthMode.AlwaysOnTop
		ring.Adornee = part
		ring.Parent = part
	end
	table.insert(self.creatures, {
		kind = p.kind,
		spec = spec,
		lane = p.lane,
		part = part,
		dist = p.dist,
		age = 0,
		progress = 0,
		angle = Config.LANE_ANGLES[p.lane],
	})
end

function CreatureService:_pop(c)
	local burst = Instance.new("Part")
	burst.Anchored = true
	burst.CanCollide = false
	burst.CanQuery = false
	burst.Transparency = 1
	burst.Size = Vector3.new(1, 1, 1)
	burst.Position = c.part.Position
	burst.Parent = self.folder
	local emitter = Instance.new("ParticleEmitter")
	emitter.Rate = 0
	emitter.Lifetime = NumberRange.new(0.8, 1.2)
	emitter.Speed = NumberRange.new(8, 14)
	emitter.SpreadAngle = Vector2.new(180, 180)
	emitter.Color = ColorSequence.new(Color3.fromRGB(255, 230, 160))
	emitter.LightEmission = 1
	emitter.Parent = burst
	emitter:Emit(25)
	Debris:AddItem(burst, 2)
	self.services.Lamps:FlashAll()
	-- TODO(UNVERIFIED asset): giggle/chime sounds need real audio ids from Andrew's approved assets. None invented.
end

function CreatureService:_remove(index)
	local c = self.creatures[index]
	c.part:Destroy()
	table.remove(self.creatures, index)
end

-- now = seconds into the night. Returns the number of creatures popped this tick.
function CreatureService:Tick(dt, now)
	for i = #self.pending, 1, -1 do
		if now >= self.pending[i].at then
			self:_spawn(self.pending[i])
			table.remove(self.pending, i)
		end
	end

	local beams = self.services.Lamps:LitBeams()
	local poppedNow = 0
	for i = #self.creatures, 1, -1 do
		local c = self.creatures[i]
		c.age += dt
		c.dist -= c.spec.speed * dt
		c.angle = Config.LANE_ANGLES[c.lane] + Beam.zigzag(c.age, c.spec.zigAmp, c.spec.zigFreq)

		local hits, rate = 0, 0
		for _, b in beams do
			if Beam.covers(b.angle, b.halfWidth, b.reach, c.angle, c.dist) then
				hits += 1
				rate = math.max(rate, b.workRate)
			end
		end
		c.progress = Beam.advance(c.progress, dt, hits, c.spec.beams, rate, c.spec.decay)

		if c.progress >= c.spec.hold then
			self:_pop(c)
			self:_remove(i)
			self.popped += 1
			poppedNow += 1
		elseif c.dist <= Config.TOUCH_DIST then
			-- East lanes threaten the Harbor lamp, west lanes the Stormside lamp.
			local target = if math.cos(Config.LANE_ANGLES[c.lane]) >= 0 then "Harbor" else "Stormside"
			self.services.Lamps:Dim(target, c.spec.penalty)
			self:_remove(i)
		else
			local bob = math.sin(c.age * 2) * 0.4
			c.part.Position =
				Vector3.new(math.cos(c.angle) * c.dist, Config.SEA_Y + 1 + bob, math.sin(c.angle) * c.dist)
			c.part.Transparency = 0.1 + 0.6 * math.min(1, c.progress / c.spec.hold)
		end
	end

	-- Lane glow: lit while a creature is telegraphed or on it.
	local active = {}
	for _, p in self.pending do
		active[p.lane] = true
	end
	for _, c in self.creatures do
		active[c.lane] = true
	end
	for index, lane in self.lanes do
		lane.Transparency = if active[index] then 0.5 else 1
	end
	return poppedNow
end

return CreatureService
