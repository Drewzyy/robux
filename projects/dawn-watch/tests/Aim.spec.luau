local Aim = require("../src/shared/Aim")

local function near(a: number, b: number): boolean
	return math.abs(a - b) < 1e-6
end

return function()
	-- Turn is capped by speed * dt.
	assert(near(Aim.step(0, 1, 2, 0.25), 0.5))
	-- Close enough: lands on target.
	assert(near(Aim.step(0, 0.1, 2, 0.25), 0.1))
	-- Takes the short way round the circle (from just below pi to just above -pi).
	local stepped = Aim.step(math.pi - 0.1, -math.pi + 0.1, 1, 0.05)
	assert(near(Aim.diff(math.pi - 0.1, stepped), 0.05))
	-- Bots chase the creature closest to the tower.
	assert(near(Aim.botTarget({ { angle = 1, dist = 80 }, { angle = 2, dist = 20 } }) :: number, 2))
	assert(Aim.botTarget({}) == nil)
end
