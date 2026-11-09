--!nonstrict
-- Reads mouse / touch / right stick and sends an aim ANGLE as intent. The server decides whether it is allowed,
-- how fast the beam turns, and what it hits.
-- UNVERIFIED: touch and gamepad aiming have not been run in Studio. Mouse path is the reference.
local Players = game:GetService("Players")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local RunService = game:GetService("RunService")
local UserInputService = game:GetService("UserInputService")
local Workspace = game:GetService("Workspace")

local Shared = ReplicatedStorage:WaitForChild("Shared")
local Config = require(Shared.Config)

local player = Players.LocalPlayer

local SEND_INTERVAL = 0.05
local STICK_DEADZONE = 0.3

local AimController = {}

function AimController:Init()
	self.angle = nil
	self.clock = 0
	self.lastSent = nil
end

local function angleFromRay(ray)
	if math.abs(ray.Direction.Y) < 1e-4 then
		return nil
	end
	local t = (Config.SEA_Y - ray.Origin.Y) / ray.Direction.Y
	if t <= 0 then
		return nil
	end
	local hit = ray.Origin + ray.Direction * t
	return math.atan2(hit.Z, hit.X)
end

function AimController:Start()
	self.remote = ReplicatedStorage:WaitForChild("Remotes"):WaitForChild("AimBeam")

	UserInputService.InputChanged:Connect(function(input)
		local camera = Workspace.CurrentCamera
		if camera == nil then
			return
		end
		if input.UserInputType == Enum.UserInputType.MouseMovement then
			local pos = UserInputService:GetMouseLocation()
			self.angle = angleFromRay(camera:ViewportPointToRay(pos.X, pos.Y)) or self.angle
		elseif input.UserInputType == Enum.UserInputType.Touch then
			self.angle = angleFromRay(camera:ScreenPointToRay(input.Position.X, input.Position.Y)) or self.angle
		elseif input.KeyCode == Enum.KeyCode.Thumbstick2 then
			local stick = input.Position
			if math.sqrt(stick.X * stick.X + stick.Y * stick.Y) > STICK_DEADZONE then
				-- Stick up = away from the camera, in the horizontal plane.
				local look = camera.CFrame.LookVector
				local flat = Vector3.new(look.X, 0, look.Z)
				if flat.Magnitude > 1e-4 then
					flat = flat.Unit
					local right = Vector3.new(-flat.Z, 0, flat.X)
					local dir = flat * stick.Y + right * stick.X
					self.angle = math.atan2(dir.Z, dir.X)
				end
			end
		end
	end)

	RunService.Heartbeat:Connect(function(dt)
		self.clock += dt
		if self.clock < SEND_INTERVAL then
			return
		end
		self.clock = 0
		local station = player:GetAttribute("Station")
		if station == nil or self.angle == nil then
			return
		end
		if self.lastSent ~= nil and math.abs(self.lastSent - self.angle) < 0.005 then
			return
		end
		self.lastSent = self.angle
		self.remote:FireServer(station, self.angle)
	end)
end

return AimController
