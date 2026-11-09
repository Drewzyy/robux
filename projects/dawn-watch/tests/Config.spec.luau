local Config = require("../src/shared/Config")
local NightPlan = require("../src/shared/NightPlan")

return function()
	-- Exactly 3 stations and 3 creature types in MVP.
	assert(#Config.STATION_ORDER == 3)
	for _, id in Config.STATION_ORDER do
		assert(Config.STATIONS[id] ~= nil, id .. " missing")
	end
	local creatureCount = 0
	for kind in Config.CREATURES do
		creatureCount += 1
		assert(NightPlan.COST[kind] ~= nil, kind .. " has no intensity cost")
	end
	assert(creatureCount == 3)

	-- Hushhog is the co-op creature: it needs two beams at once.
	assert(Config.CREATURES.Hushhog.beams == 2)
	-- Telegraph is 3 s so a surprise death is impossible.
	assert(Config.TELEGRAPH_SECONDS == 3)
	-- Lamps cannot run dry for the first 30 s of the tutorial.
	assert(Config.TUTORIAL_GRACE == 30)
	-- Two pours fill an empty lamp; the first pour must light it.
	assert(Config.OIL_PER_POUR > 0 and Config.OIL_PER_POUR <= Config.FUEL_MAX)
	-- Harbor beam must be able to cover a lane from its rest aim after a short turn.
	for _, angle in Config.LANE_ANGLES do
		assert(math.abs(angle) < 10)
	end
end
