--!strict
-- Pure night schedule: sawtooth difficulty with rest beats. All numbers are STARTING VALUES (design doc section 6).
local NightPlan = {}

NightPlan.LENGTH = 540
NightPlan.TUTORIAL_LENGTH = 120
NightPlan.FOG_AT = 270
NightPlan.SOLO_EQ = 2.4 -- 1 human + 2 bots at 0.7 each
NightPlan.FOUR_EQ = 4
NightPlan.BOT_WEIGHT = 0.7
NightPlan.FLICKER_FROM = 45
NightPlan.HUSHHOG_FROM = 165
NightPlan.COST = { Mumble = 1, Flicker = 1, Hushhog = 3 } :: { [string]: number }

export type Beat = { name: string, start: number, stop: number, solo: number, four: number }

NightPlan.BEATS = {
	{ name = "Warm-up", start = 0, stop = 45, solo = 1, four = 2 },
	{ name = "Rise 1", start = 45, stop = 135, solo = 3, four = 5 },
	{ name = "Rest", start = 135, stop = 165, solo = 0, four = 0 },
	{ name = "Rise 2", start = 165, stop = 255, solo = 4, four = 7 },
	{ name = "Rest", start = 255, stop = 270, solo = 0, four = 0 },
	{ name = "Fog event", start = 270, stop = 315, solo = 3, four = 5 },
	{ name = "Rest", start = 315, stop = 345, solo = 0, four = 0 },
	{ name = "Rise 3", start = 345, stop = 450, solo = 5, four = 9 },
	{ name = "Rest", start = 450, stop = 480, solo = 0, four = 0 },
	{ name = "Final push", start = 480, stop = 540, solo = 6, four = 10 },
} :: { Beat }

function NightPlan.length(tutorial: boolean): number
	if tutorial then
		return NightPlan.TUTORIAL_LENGTH
	end
	return NightPlan.LENGTH
end

-- The tutorial night plays the same shape compressed into 120 s.
function NightPlan.planTime(elapsed: number, tutorial: boolean): number
	if tutorial then
		return elapsed * NightPlan.LENGTH / NightPlan.TUTORIAL_LENGTH
	end
	return elapsed
end

function NightPlan.beatAt(planT: number): Beat?
	for _, beat in NightPlan.BEATS do
		if planT >= beat.start and planT < beat.stop then
			return beat
		end
	end
	return nil
end

-- Humans count 1, bots count 0.7.
function NightPlan.equivalent(humans: number, bots: number): number
	return humans + bots * NightPlan.BOT_WEIGHT
end

-- Intensity points allowed alive at once. +5% per night, capped at night 10.
function NightPlan.target(planT: number, equivalent: number, nightNumber: number): number
	local beat = NightPlan.beatAt(planT)
	if beat == nil then
		return 0
	end
	local frac = math.clamp((equivalent - NightPlan.SOLO_EQ) / (NightPlan.FOUR_EQ - NightPlan.SOLO_EQ), 0, 1)
	local base = beat.solo + (beat.four - beat.solo) * frac
	local growth = 1 + 0.05 * (math.clamp(nightNumber, 1, 10) - 1)
	return base * growth
end

-- alive = intensity points currently alive or telegraphed. rand() returns [0, 1).
function NightPlan.pickCreature(
	planT: number,
	tutorial: boolean,
	alive: number,
	target: number,
	rand: () -> number
): string?
	local options: { { kind: string, weight: number } } = {}
	local function consider(kind: string, weight: number)
		if alive + NightPlan.COST[kind] <= target + 1e-6 then
			table.insert(options, { kind = kind, weight = weight })
		end
	end
	consider("Mumble", 3)
	if not tutorial then
		if planT >= NightPlan.FLICKER_FROM then
			consider("Flicker", 2)
		end
		if planT >= NightPlan.HUSHHOG_FROM then
			consider("Hushhog", 1)
		end
	end
	if #options == 0 then
		return nil
	end
	local total = 0
	for _, option in options do
		total += option.weight
	end
	local roll = rand() * total
	for _, option in options do
		roll -= option.weight
		if roll < 0 then
			return option.kind
		end
	end
	return options[#options].kind
end

return NightPlan
