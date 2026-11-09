local Fog = require("../src/shared/Fog")

return function()
	assert(Fog.pick(0) == "ThickFog")
	assert(Fog.pick(0.5) == "Gust")
	assert(Fog.pick(0.999) == "LanternRain")
	assert(Fog.pick(1) == "LanternRain", "roll of exactly 1 must not index out of range")

	assert(Fog.effects("ThickFog", 10).reachMul == 0.7)
	assert(Fog.effects("ThickFog", 45).reachMul == 1, "effect ends after its duration")
	assert(Fog.effects(nil, 0).reachMul == 1)
	assert(Fog.effects("LanternRain", 5).fuelPerSec > 0)
	assert(Fog.effects("LanternRain", 25).fuelPerSec == 0)
	assert(math.abs(Fog.effects("Gust", 3).drift) <= 0.15)
end
