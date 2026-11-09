--!nonstrict
-- Owns the two remotes and validates every argument the client sends.
--   AimBeam (client -> server): (stationId: string, angle: number)   intent only, server decides everything.
--   HudUpdate (server -> client): table, display only.
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")

local Shared = ReplicatedStorage:WaitForChild("Shared")
local Config = require(Shared.Config)
local Validate = require(Shared.Validate)

local RemotesService = {}

function RemotesService:Init(services)
	self.services = services
	self.lastAim = {}

	local folder = Instance.new("Folder")
	folder.Name = "Remotes"

	self.aimBeam = Instance.new("RemoteEvent")
	self.aimBeam.Name = "AimBeam"
	self.aimBeam.Parent = folder

	self.hudUpdate = Instance.new("RemoteEvent")
	self.hudUpdate.Name = "HudUpdate"
	self.hudUpdate.Parent = folder

	folder.Parent = ReplicatedStorage
end

function RemotesService:Start()
	self.aimBeam.OnServerEvent:Connect(function(player, stationId, angle)
		self:_onAimBeam(player, stationId, angle)
	end)
	Players.PlayerRemoving:Connect(function(player)
		self.lastAim[player] = nil
	end)
end

-- Order: type -> permission -> rate -> range. Anything wrong is dropped silently.
function RemotesService:_onAimBeam(player, stationId, angle)
	if not Validate.isOneOf(stationId, Config.STATION_ORDER) then
		return
	end
	local clean = Validate.angle(angle) -- rejects non-numbers, NaN, +-math.huge, absurd magnitudes
	if clean == nil then
		return
	end
	if not self.services.Lamps:IsKeeper(player, stationId) then
		return
	end
	local now = os.clock()
	local last = self.lastAim[player]
	if last ~= nil and now - last < Config.AIM_MIN_INTERVAL then
		return
	end
	self.lastAim[player] = now
	self.services.Lamps:SetTargetAim(stationId, clean)
end

function RemotesService:SendHud(payload)
	self.hudUpdate:FireAllClients(payload)
end

return RemotesService
