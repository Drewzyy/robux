--!strict
-- Pure "Fast First Minute" rules. No Instances and no requires (so Lune and Roblox both load it).
-- Callers pass the Config table in; STARTING VALUES live in Config.
local FirstRun = {}

export type Cfg = {
	HOLD_TEND: number,
	FIRST_BEAM_HOLD: number,
	HOLD_RELIGHT: number,
	TUTORIAL_CAP_SECONDS: number,
}

-- Hold time on a lamp prompt: a lit lamp is a quick tend, a never-lit lamp is a quick first light,
-- a lamp that was lit and went out is the slower relight.
function FirstRun.holdFor(cfg: Cfg, lit: boolean, everLit: boolean): number
	if lit then
		return cfg.HOLD_TEND
	elseif not everLit then
		return cfg.FIRST_BEAM_HOLD
	end
	return cfg.HOLD_RELIGHT
end

-- Tutorial alive-at-once cap in intensity points. Nil means no cap (use the normal plan).
function FirstRun.aliveCap(cfg: Cfg, popped: number, elapsed: number): number?
	if popped <= 0 then
		return 1
	elseif elapsed < cfg.TUTORIAL_CAP_SECONDS then
		return 2
	end
	return nil
end

-- A welcome Mumble is only added when nothing is alive or telegraphed and the player has no pops.
function FirstRun.needsWelcome(playerPops: number, alivePoints: number, alreadyWelcomed: boolean): boolean
	return playerPops <= 0 and alivePoints <= 0 and not alreadyWelcomed
end

return FirstRun
