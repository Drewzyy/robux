--!strict
-- Pure Glow rules. STARTING VALUES (design doc section 5). Nothing here is for sale.
local Economy = {}

Economy.BASE = 20
Economy.PER_STAR = 10
Economy.DAILY_BONUS = 10
Economy.FIRST_POP = 10
Economy.NEXT_POPS = 5 -- pops 2 and 3
Economy.GUIDED_POPS = 3

export type Upgrade = { name: string, cost: number }

-- Cumulative Glow thresholds. Spending and upgrade effects are NOT built in the first build.
Economy.UPGRADES = {
	{ name = "Lamp Polish", cost = 40 },
	{ name = "Longer Wick", cost = 80 },
	{ name = "Wide Lens", cost = 140 },
	{ name = "Sea Chime", cost = 220 },
} :: { Upgrade }

-- Lamps out for more than 40% of the night is a 1-star night (never zero: trying is not punished).
function Economy.stars(outFraction: number): number
	if outFraction <= 0.1 then
		return 3
	elseif outFraction <= 0.4 then
		return 2
	end
	return 1
end

function Economy.glow(stars: number, firstNightToday: boolean): number
	local s = math.clamp(stars, 1, 3)
	local total = Economy.BASE + Economy.PER_STAR * s
	if firstNightToday then
		total += Economy.DAILY_BONUS
	end
	return total
end

-- Fixed Glow for a player's next pop, given how many pops they already have: 10, 5, 5, then 0.
function Economy.popReward(popsSoFar: number): number
	if popsSoFar < 0 or popsSoFar ~= popsSoFar then
		return 0
	end
	if popsSoFar < 1 then
		return Economy.FIRST_POP
	elseif popsSoFar < Economy.GUIDED_POPS then
		return Economy.NEXT_POPS
	end
	return 0
end

-- Average share of the night the lamps were dark, 0 to 1.
function Economy.outFraction(outTimes: { number }, length: number): number
	if #outTimes == 0 or length <= 0 then
		return 0
	end
	local sum = 0
	for _, seconds in outTimes do
		sum += seconds
	end
	return math.clamp(sum / (#outTimes * length), 0, 1)
end

-- The fixed next reward the board always shows. Nil once every upgrade threshold is passed.
function Economy.nextUpgrade(totalGlow: number): { name: string, cost: number, remaining: number }?
	for _, upgrade in Economy.UPGRADES do
		if totalGlow < upgrade.cost then
			return { name = upgrade.name, cost = upgrade.cost, remaining = upgrade.cost - totalGlow }
		end
	end
	return nil
end

return Economy
