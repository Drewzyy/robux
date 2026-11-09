-- Runs every pure-logic spec with Lune. Stops with error() on the first failure.
local specs = {
	{ "Validate", require("./Validate.spec") },
	{ "Aim", require("./Aim.spec") },
	{ "Beam", require("./Beam.spec") },
	{ "Fog", require("./Fog.spec") },
	{ "NightPlan", require("./NightPlan.spec") },
	{ "Economy", require("./Economy.spec") },
	{ "Tutorial", require("./Tutorial.spec") },
	{ "Config", require("./Config.spec") },
}

for _, spec in specs do
	local ok, err = pcall(spec[2])
	if not ok then
		error(`{spec[1]} spec failed: {err}`)
	end
	print(`ok  {spec[1]}`)
end
print("all specs passed")
