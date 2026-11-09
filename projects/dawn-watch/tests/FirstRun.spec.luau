local Config = require("../src/shared/Config")
local FirstRun = require("../src/shared/FirstRun")

return function()
	-- Never-lit lamp: 0.4 s. Lit lamp: quick tend. Lit then went out: the 2 s relight.
	assert(FirstRun.holdFor(Config, false, false) == 0.4)
	assert(FirstRun.holdFor(Config, true, true) == Config.HOLD_TEND)
	assert(FirstRun.holdFor(Config, false, true) == 2)
	assert(FirstRun.holdFor(Config, false, false) < FirstRun.holdFor(Config, false, true))

	-- Tutorial cap: 1 until the first pop, 2 until 60 s, then the normal plan.
	assert(FirstRun.aliveCap(Config, 0, 5) == 1)
	assert(FirstRun.aliveCap(Config, 0, 90) == 1)
	assert(FirstRun.aliveCap(Config, 1, 20) == 2)
	assert(FirstRun.aliveCap(Config, 3, 59.9) == 2)
	assert(FirstRun.aliveCap(Config, 3, 60) == nil)

	-- Welcome Mumble: only for a player with no pops, an empty sea, and no earlier welcome.
	assert(FirstRun.needsWelcome(0, 0, false))
	assert(not FirstRun.needsWelcome(1, 0, false))
	assert(not FirstRun.needsWelcome(0, 1, false), "creatures already coming: add nothing")
	assert(not FirstRun.needsWelcome(0, 0, true), "one welcome per player")

	-- First beam is pre-aimed: every station has a first lane and a different second lane.
	for _, id in Config.STATION_ORDER do
		local first, second = Config.FIRST_LANE[id], Config.SECOND_LANE[id]
		assert(Config.LANE_ANGLES[first] ~= nil and Config.LANE_ANGLES[second] ~= nil)
		assert(first ~= second)
		-- The lane is on the station's own side (same sign of cos), so its beam can reach it.
		local spec = Config.STATIONS[id]
		assert(math.cos(Config.LANE_ANGLES[first]) * math.cos(spec.restAim) > 0, id)
		assert(math.cos(Config.LANE_ANGLES[second]) * math.cos(spec.restAim) > 0, id)
	end
	-- The first lane is right on the beam when pre-aimed, so the first pop needs no aiming.
	assert(Config.FIRST_SPAWN_DIST < Config.SPAWN_DIST)
	assert(Config.FIRST_TELEGRAPH < Config.TELEGRAPH_SECONDS)
end
