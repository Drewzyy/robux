---
description: Design and script boss fights for co-op Roblox games: phases, telegraphed attacks, health scaling by player count, aggro rules, a typed boss state loop and loot fairness.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: boss fight, boss phases, telegraph, aggro, threat, health scaling, enrage, loot, co-op, attack pattern, state machine, damage tracking
sources: creator-docs content/en-us/production/game-design/core-loops.md (read 2026-11-14); creator-docs content/en-us/characters/pathfinding.md (read 2026-11-14)
verify: Design patterns from model knowledge, UNVERIFIED as tuned values. Playtest with 1, 4 and a full server before trusting any number.
tags: [multiplayer-npcs, playbook]
---
# Boss fight patterns

A boss is a long fight with readable attacks, clear phases and fair rewards. The server runs the whole thing; clients play effects.

## Do this
- Build the boss as a state machine (see [[13-Luau-Engineering/state-machines]]): `Idle`, `Chase`, `Telegraph`, `Attack`, `Recover`, `PhaseShift`, `Dead`.
- Every attack has a telegraph: 0.6 to 1.2 s of visible warning (ground marker, glow, sound) before damage. Damage must come from the server at the end of the telegraph, using the marker's area.
- Phases by health: phase 2 at 66%, phase 3 at 33% (defaults). Each phase adds one new attack or speeds up cooldowns by 15%. Brief invulnerable `PhaseShift` of 2 s for a cinematic and a reset.
- Scale health: `maxHealth = base * (1 + 0.75 * (players - 1))`. Scale damage lightly (1 + 0.1 per extra player). Compute at fight start; if players leave, do not re-scale downward mid-fight.
- Aggro: track threat per player (damage dealt plus healing done). Target the highest threat but switch no more than every 5 s. Add a taunt-free fallback: random target if the top-threat player is out of reach for 4 s.
- Enrage timer: after 5 minutes (UNVERIFIED default) boss damage rises 25% per 30 s, so stalling never wins.
- Track damage per player on the server. Reward by contribution tiers: any hit above 5% of boss health gets full loot access; top 3 get a bonus. Avoids ninja-looters and AFK freeloaders.
- Reset on wipe: if every participant is dead or gone, reset the boss to full health and idle after 8 s.

## Boss loop skeleton

```lua
--!strict
type State = "Idle" | "Chase" | "Telegraph" | "Attack" | "Recover" | "PhaseShift" | "Dead"

type Boss = {
	humanoid: Humanoid,
	root: BasePart,
	state: State,
	threat: { [Player]: number },
	phase: number,
}

local function phaseFor(humanoid: Humanoid): number
	local ratio = humanoid.Health / humanoid.MaxHealth
	if ratio > 0.66 then
		return 1
	elseif ratio > 0.33 then
		return 2
	end
	return 3
end

local function topThreat(boss: Boss): Player?
	local best: Player? = nil
	local bestValue = 0
	for player, value in boss.threat do
		local character = player.Character
		if character and value > bestValue then
			best = player
			bestValue = value
		end
	end
	return best
end

local function step(boss: Boss)
	if boss.humanoid.Health <= 0 then
		boss.state = "Dead"
		return
	end
	local newPhase = phaseFor(boss.humanoid)
	if newPhase ~= boss.phase then
		boss.phase = newPhase
		boss.state = "PhaseShift"
		return
	end
	-- state-specific logic (move, telegraph, damage) goes here
end
```

Run `step` from one loop at 10 Hz. Movement can use PathfindingService with a recompute every 0.5 s.

## Telegraph damage check
Compute hit players at the end of the telegraph with `workspace:GetPartBoundsInBox` or `GetPartBoundsInRadius`, not from touch events. Use an `OverlapParams` that includes only characters.

## Co-op feel
- Show a boss health bar to all participants. Show phase pips on it.
- Give a downed state (revive within 10 s) so one mistake is not a long wait.
- Floating damage numbers per player, color-coded.
- Do not lock out latecomers: allow joining the fight within the first 60 s.

## Mistakes
- Instant damage with no telegraph (feels unfair, rage quits).
- Health scale so high that fights exceed 6 minutes.
- Damage computed on touch events (flaky, exploitable).
- Loot to the last hitter only.
- Boss script with `while true do ... wait()` per ability.

## Related
[[12-Multiplayer-NPCs/enemy-waves-and-spawners]], [[12-Multiplayer-NPCs/npc-ai]], [[12-Multiplayer-NPCs/teams-and-parties]], [[13-Luau-Engineering/cooldowns-and-rate-limits]]

## For our Roblox games
Copy the principle: telegraph, phases, contribution-based loot, wipe reset. Never copy another game's boss names, models, attack animations, themes or sounds. Zero spend: original rigs and free Roblox animation tools.
