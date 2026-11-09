--!strict
-- Pure hint picker. Three words at most, never a wall of text: teach by doing.
local Tutorial = {}

export type State = {
	phase: string,
	tutorial: boolean, -- guided: tutorial night, or this player is still in their first pops
	carrying: boolean,
	anyLit: boolean,
	popped: number, -- this player's own pops
	oilLow: boolean,
}

function Tutorial.hint(state: State): string
	if state.phase ~= "Night" then
		return ""
	end
	if not state.anyLit then
		if state.carrying then
			return "Pour it in."
		end
		return "Fuel the lamp."
	end
	if state.tutorial and state.popped == 0 then
		return "Light it up."
	end
	if state.tutorial and state.popped == 1 then
		return "Aim at glow." -- design says "Aim at the glow." (4 words); kept to the 3-word cap
	end
	if state.oilLow then
		return "Refuel!"
	end
	return ""
end

return Tutorial
