local Validate = require("../src/shared/Validate")

return function()
	assert(Validate.isFiniteNumber(1.5))
	assert(not Validate.isFiniteNumber(0 / 0), "NaN must be rejected")
	assert(not Validate.isFiniteNumber(math.huge), "inf must be rejected")
	assert(not Validate.isFiniteNumber(-math.huge))
	assert(not Validate.isFiniteNumber("1"))
	assert(not Validate.isFiniteNumber(nil))

	assert(Validate.isOneOf("Harbor", { "Harbor", "Crown" }))
	assert(not Validate.isOneOf("Nope", { "Harbor", "Crown" }))
	assert(not Validate.isOneOf(5, { "Harbor" }))

	assert(Validate.angle(0 / 0) == nil)
	assert(Validate.angle(math.huge) == nil)
	assert(Validate.angle(5000) == nil)
	assert(Validate.angle("x") == nil)
	local a = Validate.angle(math.pi * 3)
	assert(a ~= nil and a >= -math.pi and a < math.pi)
end
