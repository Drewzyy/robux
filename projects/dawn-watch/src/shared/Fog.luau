--!strict
-- The one random element: a mid-night fog event. It never gates progress or Glow.
local Fog = {}

Fog.KINDS = { "ThickFog", "Gust", "LanternRain" }
Fog.DURATION = { ThickFog = 45, Gust = 45, LanternRain = 20 } :: { [string]: number }

export type Effects = { reachMul: number, drift: number, fuelPerSec: number }

-- roll is a number in [0, 1).
function Fog.pick(roll: number): string
	local index = math.clamp(math.floor(roll * #Fog.KINDS) + 1, 1, #Fog.KINDS)
	return Fog.KINDS[index]
end

function Fog.effects(kind: string?, elapsed: number): Effects
	local neutral = { reachMul = 1, drift = 0, fuelPerSec = 0 }
	if kind == nil then
		return neutral
	end
	local duration = Fog.DURATION[kind]
	if duration == nil or elapsed < 0 or elapsed >= duration then
		return neutral
	end
	if kind == "ThickFog" then
		return { reachMul = 0.7, drift = 0, fuelPerSec = 0 }
	elseif kind == "Gust" then
		return { reachMul = 1, drift = math.sin(elapsed * 0.9) * 0.15, fuelPerSec = 0 }
	end
	return { reachMul = 1, drift = 0, fuelPerSec = 2 }
end

return Fog
