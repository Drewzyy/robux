--!strict
-- Pure hint picker. Three words at most, never a wall of text: teach by doing.
local Tutorial = {}

export type State = {
	phase: string,
	tutorial: boolean,
	carrying: boolean,
	anyLit: boolean,
	popped: number,
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
	if state.oilLow then
		return "Refuel!"
	end
	return ""
end

return Tutorial
