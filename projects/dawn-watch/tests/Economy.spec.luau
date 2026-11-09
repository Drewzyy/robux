local Economy = require("../src/shared/Economy")

return function()
	assert(Economy.glow(1, false) == 30)
	assert(Economy.glow(3, false) == 50)
	assert(Economy.glow(2, true) == 50, "daily bonus is +10")
	assert(Economy.glow(0, false) == 30, "never below 1 star: trying is not punished")

	assert(Economy.stars(0) == 3)
	assert(Economy.stars(0.1) == 3)
	assert(Economy.stars(0.25) == 2)
	assert(Economy.stars(0.4) == 2)
	assert(Economy.stars(0.41) == 1)
	assert(Economy.stars(1) == 1)

	assert(Economy.outFraction({ 0, 0, 0 }, 540) == 0)
	assert(math.abs(Economy.outFraction({ 54, 54, 54 }, 540) - 0.1) < 1e-9)
	assert(Economy.outFraction({}, 540) == 0)

	local first = Economy.nextUpgrade(0)
	assert(first ~= nil and first.name == "Lamp Polish" and first.remaining == 40)
	local second = Economy.nextUpgrade(40)
	assert(second ~= nil and second.name == "Longer Wick" and second.remaining == 40)
	assert(Economy.nextUpgrade(220) == nil)
end
