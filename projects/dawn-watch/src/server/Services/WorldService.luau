--!nonstrict
-- Builds the blockout with plain Parts: sea, tower, gallery, spawn pad, oil barrels. No Toolbox assets.
local Lighting = game:GetService("Lighting")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Workspace = game:GetService("Workspace")

local Config = require(ReplicatedStorage:WaitForChild("Shared").Config)

local WorldService = {}

local function makePart(parent, name, size, position, color, material)
	local part = Instance.new("Part")
	part.Name = name
	part.Size = size
	part.Position = position
	part.Color = color
	part.Material = material
	part.Anchored = true
	part.TopSurface = Enum.SurfaceType.Smooth
	part.BottomSurface = Enum.SurfaceType.Smooth
	part.Parent = parent
	return part
end

function WorldService:Init(services)
	self.services = services
	local g = Config.GALLERY_Y

	local folder = Instance.new("Folder")
	folder.Name = "DawnWatchWorld"
	folder.Parent = Workspace
	self.folder = folder

	makePart(
		folder,
		"Sea",
		Vector3.new(500, 1, 500),
		Vector3.new(0, 0, 0),
		Color3.fromRGB(14, 26, 54),
		Enum.Material.SmoothPlastic
	)

	local tower = makePart(
		folder,
		"Tower",
		Vector3.new(g, 12, 12),
		Vector3.new(0, g / 2, 0),
		Color3.fromRGB(226, 220, 205),
		Enum.Material.Concrete
	)
	tower.Shape = Enum.PartType.Cylinder
	tower.CFrame = CFrame.new(0, g / 2, 0) * CFrame.Angles(0, 0, math.pi / 2)

	local gallery = makePart(
		folder,
		"Gallery",
		Vector3.new(1, 30, 30),
		Vector3.new(0, g - 0.5, 0),
		Color3.fromRGB(120, 92, 70),
		Enum.Material.WoodPlanks
	)
	gallery.Shape = Enum.PartType.Cylinder
	gallery.CFrame = CFrame.new(0, g - 0.5, 0) * CFrame.Angles(0, 0, math.pi / 2)

	-- Crown Lens pedestal so the top station sits above the gallery.
	makePart(
		folder,
		"CrownPedestal",
		Vector3.new(3, 4, 3),
		Vector3.new(0, g + 2, 0),
		Color3.fromRGB(90, 90, 100),
		Enum.Material.Metal
	)

	local spawn = Instance.new("SpawnLocation")
	spawn.Name = "GallerySpawn"
	spawn.Size = Vector3.new(6, 1, 6)
	spawn.Position = Vector3.new(0, g + 0.5, 8)
	spawn.Anchored = true
	spawn.Neutral = true
	spawn.Duration = 0
	spawn.Transparency = 0.6
	spawn.Parent = folder

	-- Barrels 3 studs from spawn (z = 11) and opposite (z = -11).
	self.barrels = {}
	for index, z in { 11, -11 } do
		local barrel = makePart(
			folder,
			"OilBarrel" .. index,
			Vector3.new(3, 3, 3),
			Vector3.new(0, g + 1.5, z),
			Color3.fromRGB(255, 170, 60),
			Enum.Material.Neon
		)
		local prompt = Instance.new("ProximityPrompt")
		prompt.ActionText = "Grab oil"
		prompt.ObjectText = "Oil barrel"
		prompt.HoldDuration = 0.3
		prompt.MaxActivationDistance = 10
		prompt.RequiresLineOfSight = false
		prompt.Parent = barrel
		table.insert(self.barrels, barrel)
		prompt.Triggered:Connect(function(player)
			player:SetAttribute("Oil", 1)
		end)
	end

	Lighting.ClockTime = 0
end

function WorldService:Start() end

return WorldService
