--!nonstrict
-- The one client bootstrapper. Two phases: every Init runs before any Start.
local Controllers = script.Parent.Controllers

local order = {
	require(Controllers.HudController),
	require(Controllers.AimController),
}

for _, controller in order do
	controller:Init()
end
for _, controller in order do
	controller:Start()
end
