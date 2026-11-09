--!nonstrict
-- The one server bootstrapper. Two phases: every Init runs before any Start. Services never require each other;
-- they get each other through the shared `services` table in Init, so there are no circular requires.
local Services = script.Parent.Services

local order = {
	{ "Remotes", require(Services.RemotesService) },
	{ "World", require(Services.WorldService) },
	{ "Lamps", require(Services.LampService) },
	{ "Creatures", require(Services.CreatureService) },
	{ "Night", require(Services.NightService) },
	{ "Guide", require(Services.GuideService) },
}

local services = {}
for _, entry in order do
	services[entry[1]] = entry[2]
end

for _, entry in order do
	entry[2]:Init(services)
end
for _, entry in order do
	entry[2]:Start()
end
