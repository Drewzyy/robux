--!strict
-- Pure data. No Instances, so Lune can load it. Every number is a STARTING VALUE to tune in playtest (design doc).
local Config = {}

Config.GALLERY_Y = 20
Config.SEA_Y = 3
Config.SPAWN_DIST = 110
Config.TOUCH_DIST = 12
Config.TELEGRAPH_SECONDS = 3
Config.SPAWN_GAP_SECONDS = 3

Config.FUEL_MAX = 100
Config.OIL_PER_POUR = 40
Config.LOW_FUEL = 25
Config.HOLD_TEND = 0.4
Config.HOLD_RELIGHT = 2

Config.HUMAN_TURN = 3.0 -- rad/s, server-enforced
Config.BOT_SPEED_FACTOR = 0.7
Config.BOT_REFUEL_DELAY = 6
Config.TUTORIAL_GRACE = 30 -- lamps cannot run dry for the first 30 s of the tutorial night
Config.BOT_START_TUTORIAL = 20

-- "Fast First Minute" (design change). STARTING VALUES to tune in playtest.
Config.FIRST_BEAM_HOLD = 0.4 -- hold to light a lamp that has never been lit this night (was 2)
Config.FIRST_SPAWN_DIST = 60 -- studs out for the first / welcome Mumble (was 110)
Config.FIRST_TELEGRAPH = 2 -- seconds of telegraph for the first / welcome Mumble (was 3)
Config.FIRST_LANE = { Harbor = 2, Stormside = 4, Crown = 2 } -- pre-aimed lane per station
Config.SECOND_LANE = { Harbor = 1, Stormside = 3, Crown = 1 } -- second Mumble needs a real aim
Config.LAMP_FLASH_SECONDS = 0.6
Config.FIRST_RUN_POPS = 3 -- pops that pay the guided Glow schedule and show guidance
Config.TUTORIAL_CAP_SECONDS = 60 -- real seconds the tutorial alive-cap stays at 2

Config.AIM_MIN_INTERVAL = 0.04 -- server rate limit per player (25 msgs/s max)

-- Four lanes: two on the east (Harbor) side, two on the west (Stormside) side. Angles in radians around the tower.
Config.LANE_ANGLES = { -0.25, 0.25, math.pi - 0.25, math.pi + 0.25 }

Config.STATION_ORDER = { "Harbor", "Stormside", "Crown" }

Config.STATIONS = {
	Harbor = {
		label = "Harbor Lamp",
		pos = { 10, 23, 0 },
		restAim = 0,
		halfWidth = 0.1,
		reach = 120,
		workRate = 1,
		burn = 0.8,
	},
	Stormside = {
		label = "Stormside Lamp",
		pos = { -10, 23, 0 },
		restAim = math.pi,
		halfWidth = 0.3,
		reach = 70,
		workRate = 0.6,
		burn = 0.8,
	},
	Crown = {
		label = "Crown Lens",
		pos = { 0, 26, 0 },
		restAim = 0,
		halfWidth = 0.18,
		reach = 130,
		workRate = 1,
		burn = 1.3,
	},
}

-- speed studs/s, hold seconds of beam needed, beams needed at once, decay of progress per second when unlit
Config.CREATURES = {
	Mumble = { speed = 8, hold = 1.5, beams = 1, decay = 0.5, penalty = 30, size = 4, zigAmp = 0, zigFreq = 0 },
	Flicker = { speed = 14, hold = 0.8, beams = 1, decay = 1, penalty = 25, size = 2.5, zigAmp = 0.12, zigFreq = 5 },
	Hushhog = { speed = 5, hold = 3, beams = 2, decay = 0.5, penalty = 60, size = 7, zigAmp = 0, zigFreq = 0 },
}

return Config
