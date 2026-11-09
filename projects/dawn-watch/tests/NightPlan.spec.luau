local NightPlan = require("../src/shared/NightPlan")

return function()
	-- Beats tile the whole 9:00 night with no gap or overlap.
	local cursor = 0
	for _, beat in NightPlan.BEATS do
		assert(beat.start == cursor, "gap before " .. beat.name)
		assert(beat.stop > beat.start)
		cursor = beat.stop
	end
	assert(cursor == NightPlan.LENGTH)

	-- Rest beats have zero intensity (sawtooth).
	assert(NightPlan.target(150, 4, 1) == 0)
	assert(NightPlan.target(2.5 * 60 + 1000, 4, 1) == 0, "past the end is 0")
	-- Solo equivalent hits the solo column; four players hit the four column.
	assert(math.abs(NightPlan.target(100, NightPlan.SOLO_EQ, 1) - 3) < 1e-9)
	assert(math.abs(NightPlan.target(100, 4, 1) - 5) < 1e-9)
	-- +5% per night, capped at night 10.
	assert(math.abs(NightPlan.target(100, 4, 2) - 5 * 1.05) < 1e-9)
	assert(NightPlan.target(100, 4, 10) == NightPlan.target(100, 4, 99))

	assert(math.abs(NightPlan.equivalent(1, 2) - 2.4) < 1e-9)
	assert(NightPlan.planTime(60, true) == 270, "tutorial compresses 540 s into 120 s")
	assert(NightPlan.planTime(60, false) == 60)

	-- Tutorial only ever spawns Mumbles; Hushhog is locked until 2:45.
	for roll = 0, 0.99, 0.11 do
		assert(NightPlan.pickCreature(200, true, 0, 10, function()
			return roll
		end) == "Mumble")
		assert(NightPlan.pickCreature(100, false, 0, 10, function()
			return roll
		end) ~= "Hushhog")
	end
	-- Never spawns past the target.
	assert(NightPlan.pickCreature(200, false, 3, 3, function()
		return 0.5
	end) == nil)
	-- Hushhog (cost 3) cannot spawn when only 2 points are free.
	for roll = 0, 0.99, 0.11 do
		assert(NightPlan.pickCreature(200, false, 1, 3, function()
			return roll
		end) ~= "Hushhog")
	end
end
