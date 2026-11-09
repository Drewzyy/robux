local Tutorial = require("../src/shared/Tutorial")

local function state(overrides)
	local base = { phase = "Night", tutorial = true, carrying = false, anyLit = false, popped = 0, oilLow = false }
	for key, value in overrides do
		base[key] = value
	end
	return base
end

return function()
	assert(Tutorial.hint(state({})) == "Fuel the lamp.")
	assert(Tutorial.hint(state({ carrying = true })) == "Pour it in.")
	assert(Tutorial.hint(state({ anyLit = true })) == "Light it up.")
	assert(Tutorial.hint(state({ anyLit = true, popped = 1 })) == "Aim at glow.")
	assert(Tutorial.hint(state({ anyLit = true, popped = 2 })) == "")
	assert(Tutorial.hint(state({ anyLit = true, popped = 2, oilLow = true })) == "Refuel!")
	assert(Tutorial.hint(state({ tutorial = false, anyLit = true, popped = 1 })) == "")
	assert(Tutorial.hint(state({ phase = "Lobby" })) == "")
	-- Every hint is three words or fewer: no text wall.
	for _, text in { "Fuel the lamp.", "Pour it in.", "Light it up.", "Aim at glow.", "Refuel!" } do
		local words = 0
		for _ in string.gmatch(text, "%S+") do
			words += 1
		end
		assert(words <= 3)
	end
end
