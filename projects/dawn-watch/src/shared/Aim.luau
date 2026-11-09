--!strict
-- Pure beam aiming. The server turns beams toward the requested angle at a capped speed.
local Aim = {}

local TWO_PI = math.pi * 2

function Aim.wrap(angle: number): number
	return (angle + math.pi) % TWO_PI - math.pi
end

-- Shortest signed difference target - current, in [-pi, pi).
function Aim.diff(current: number, target: number): number
	return Aim.wrap(target - current)
end

function Aim.step(current: number, target: number, maxSpeed: number, dt: number): number
	local delta = Aim.diff(current, target)
	local maxStep = maxSpeed * dt
	if math.abs(delta) <= maxStep then
		return Aim.wrap(target)
	end
	if delta > 0 then
		return Aim.wrap(current + maxStep)
	end
	return Aim.wrap(current - maxStep)
end

-- Bots aim at the creature closest to the tower. Input: list of { angle, dist }.
function Aim.botTarget(creatures: { { angle: number, dist: number } }): number?
	local best: number? = nil
	local bestDist = math.huge
	for _, c in creatures do
		if c.dist < bestDist then
			bestDist = c.dist
			best = c.angle
		end
	end
	return best
end

return Aim
