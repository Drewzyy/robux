--!strict
-- Pure beam-versus-creature rules.
local Beam = {}

local function wrap(angle: number): number
	return (angle + math.pi) % (math.pi * 2) - math.pi
end

function Beam.covers(
	beamAngle: number,
	halfWidth: number,
	reach: number,
	creatureAngle: number,
	creatureDist: number
): boolean
	if creatureDist > reach then
		return false
	end
	return math.abs(wrap(creatureAngle - beamAngle)) <= halfWidth
end

-- Flickers zigzag but stay on their lane.
function Beam.zigzag(age: number, amplitude: number, frequency: number): number
	return math.sin(age * frequency) * amplitude
end

-- Progress grows while enough beams hold the creature, and decays otherwise. Never below 0.
function Beam.advance(
	progress: number,
	dt: number,
	beamsHitting: number,
	beamsNeeded: number,
	workRate: number,
	decay: number
): number
	if beamsHitting >= beamsNeeded then
		return progress + dt * workRate
	end
	return math.max(0, progress - dt * decay)
end

return Beam
