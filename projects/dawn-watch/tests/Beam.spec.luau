local Beam = require("../src/shared/Beam")

return function()
	assert(Beam.covers(0, 0.1, 100, 0.05, 50))
	assert(not Beam.covers(0, 0.1, 100, 0.2, 50), "outside the beam width")
	assert(not Beam.covers(0, 0.1, 100, 0.05, 150), "beyond reach")
	-- Wide beam covers both east lanes (+-0.25).
	assert(Beam.covers(0, 0.3, 70, -0.25, 40) and Beam.covers(0, 0.3, 70, 0.25, 40))
	-- Wraps around +-pi.
	assert(Beam.covers(math.pi - 0.05, 0.1, 100, -math.pi + 0.05, 10))

	-- A Mumble (hold 1.5 s) pops after 1.5 s of continuous beam.
	local progress = 0
	for _ = 1, 15 do
		progress = Beam.advance(progress, 0.1, 1, 1, 1, 0.5)
	end
	assert(progress >= 1.5 - 1e-6)
	-- Hushhog needs two beams: one beam does not advance it.
	assert(Beam.advance(1, 0.5, 1, 2, 1, 0.5) < 1)
	-- Progress never goes below zero.
	assert(Beam.advance(0.1, 5, 0, 1, 1, 1) == 0)
	-- Flicker zigzag stays within its amplitude.
	for age = 0, 10, 0.37 do
		assert(math.abs(Beam.zigzag(age, 0.12, 5)) <= 0.12 + 1e-9)
	end
end
