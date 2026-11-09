--!strict
-- Pure argument checks used by every server remote handler.
local Validate = {}

function Validate.isFiniteNumber(value: any): boolean
	return type(value) == "number" and value == value and value ~= math.huge and value ~= -math.huge
end

function Validate.isOneOf(value: any, allowed: { string }): boolean
	if type(value) ~= "string" then
		return false
	end
	return table.find(allowed, value) ~= nil
end

-- Wraps any angle into [-pi, pi).
function Validate.wrap(angle: number): number
	return (angle + math.pi) % (math.pi * 2) - math.pi
end

-- Returns a wrapped angle, or nil when the input is not a plausible angle.
function Validate.angle(value: any): number?
	if not Validate.isFiniteNumber(value) then
		return nil
	end
	if math.abs(value) > 1000 then
		return nil
	end
	return Validate.wrap(value)
end

return Validate
