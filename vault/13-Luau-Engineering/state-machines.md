---
description: A small typed finite state machine in Luau for NPCs, rounds, abilities and UI: transition tables, enter/exit hooks, timeouts, and rules that prevent stuck or illegal states.
type: reference
status: draft
last_checked: 2026-11-14
keywords: state machine, FSM, finite state machine, transitions, NPC states, round phases, enter exit hooks, luau types, union type, stuck state, timeout
sources: creator-docs content/en-us/resources/battle-royale/core-scripts.md (state-driven design, read 2026-11-14); creator-docs content/en-us/characters/pathfinding.md (read 2026-11-14)
verify: Pattern and code written from model knowledge, not executed here: UNVERIFIED by running. Luau type syntax is standard but check with luau-analyze or your headless gate.
tags: [luau-engineering, reference]
---
# State machines

A state machine holds exactly one state at a time and allows only listed transitions. It replaces tangles of booleans (`isAttacking`, `isStunned`, `isDead`) that disagree with each other.

## Do this
- Define states as a string-literal union type so typos fail type checking: `type State = "Idle" | "Chase" | "Attack" | "Dead"`.
- Write the transition table as data: `{ [State]: { [State]: true } }`. `transition(to)` refuses anything not listed.
- Give each state optional `enter` and `exit` hooks. Put all setup (play animation, set speed) in `enter` and all teardown (disconnect, cancel tween) in `exit`.
- Track `enteredAt = os.clock()` and give each state a max duration where it makes sense. A watchdog moves stuck states to a safe state.
- Make `Dead` terminal: no transitions out except an explicit `reset`.
- One owner per machine: only the machine changes its own state. Others send events (`machine:send("Hit")`).
- Log transitions in a debug build (`warn(from, "->", to)`); invaluable for bug reports.
- Use one machine per entity, and one shared update loop, not a `while true` per machine.

## Implementation

```lua
--!strict
export type State = "Idle" | "Chase" | "Attack" | "Stunned" | "Dead"

type Hooks = {
	enter: ((from: State?) -> ())?,
	exit: ((to: State) -> ())?,
	update: ((dt: number) -> ())?,
	maxDuration: number?,
	timeoutTo: State?,
}

local TRANSITIONS: { [State]: { [State]: boolean } } = {
	Idle = { Chase = true, Stunned = true, Dead = true },
	Chase = { Idle = true, Attack = true, Stunned = true, Dead = true },
	Attack = { Chase = true, Stunned = true, Dead = true },
	Stunned = { Idle = true, Dead = true },
	Dead = {},
}

local Machine = {}
Machine.__index = Machine

export type Machine = typeof(setmetatable(
	{} :: {
		state: State,
		enteredAt: number,
		hooks: { [State]: Hooks },
	},
	Machine
))

function Machine.new(hooks: { [State]: Hooks }): Machine
	return setmetatable({
		state = "Idle" :: State,
		enteredAt = os.clock(),
		hooks = hooks,
	}, Machine)
end

function Machine.transition(self: Machine, to: State): boolean
	local allowed = TRANSITIONS[self.state]
	if not allowed[to] then
		return false
	end
	local from = self.state
	local exitHook = self.hooks[from] and self.hooks[from].exit
	if exitHook then
		exitHook(to)
	end
	self.state = to
	self.enteredAt = os.clock()
	local enterHook = self.hooks[to] and self.hooks[to].enter
	if enterHook then
		enterHook(from)
	end
	return true
end

function Machine.update(self: Machine, dt: number)
	local hooks = self.hooks[self.state]
	if not hooks then
		return
	end
	if hooks.maxDuration and hooks.timeoutTo and os.clock() - self.enteredAt > hooks.maxDuration then
		self:transition(hooks.timeoutTo)
		return
	end
	if hooks.update then
		hooks.update(dt)
	end
end

return Machine
```

Drive many machines from one `RunService.Heartbeat` connection that loops a list.

## Where to use it
- NPC AI: see [[12-Multiplayer-NPCs/npc-ai]] and [[12-Multiplayer-NPCs/boss-fight-patterns]].
- Rounds: [[12-Multiplayer-NPCs/round-system-pattern]].
- Player abilities: `Ready` to `Casting` to `Cooldown`.
- UI flows: menu screens with strict back/forward rules.

## Hierarchical tip
Keep it flat until you need more. If many states share behaviour (all "combat" states can be stunned), add a group check in `transition`, not a second machine.

## Mistakes
- Changing state by writing `self.state = "X"` directly, skipping hooks.
- Hooks that yield (`task.wait`) so the state changes under them. Spawn with `task.spawn` or use timeouts.
- Forgetting to cancel tweens and connections in `exit` (see [[13-Luau-Engineering/tween-and-animation-cleanup]]).
- No timeout on `Attack` or `Stunned`, so a missed animation event freezes the NPC.
- Booleans plus a state machine together (two sources of truth).

## For our Roblox games
Copy the principle: union-typed states, transition table, enter/exit hooks, timeouts. No names, art or assets are involved. Zero spend: plain Luau.
