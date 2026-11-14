---
description: Server-side cooldowns and per-player rate limits in Luau: timestamp cooldowns, token buckets, per-ability tables, cleanup on leave, and numbers for common actions.
type: reference
status: draft
last_checked: 2026-11-14
keywords: cooldown, rate limit, token bucket, os.clock, debounce, spam, remote throttle, ability cooldown, PlayerRemoving, server authority, burst
sources: creator-docs content/en-us/reference/engine/globals.md (os.clock via libraries, read 2026-11-14); creator-docs content/en-us/reference/engine/libraries.md (read 2026-11-14); https://create.roblox.com/docs/scripting/security/client-server-boundary (via existing vault note, not in local clone)
verify: Code written from model knowledge and not executed here: UNVERIFIED by running. Default numbers (cooldowns and rates) are design starting points, UNVERIFIED as limits.
tags: [luau-engineering, reference]
---
# Cooldowns and rate limits

A cooldown limits one ability. A rate limit limits how often a player may call any remote. Both live on the server. The client may show a cooldown ring, but never enforces it.

## Do this
- Store `lastUsed[player][ability] = os.clock()` and compare. Use `os.clock()` (monotonic) for server cooldowns, not `tick()`.
- Check cooldown before doing any work or side effects, then set the timestamp immediately, before any yield, so two calls in the same frame cannot both pass.
- Give every remote a rate limit even if it also has a cooldown. Defaults: movement-like remotes 20 per second, actions 5 per second, UI and shop 2 per second, chat-like 1 per second (UNVERIFIED defaults).
- Use a token bucket for bursty actions: capacity 5, refill 2 per second. It allows a quick combo then throttles spam.
- Do not kick on a single rate violation: drop the call and count it. Kick or flag after 20 violations in 10 s.
- Store state in a table keyed by `Player` and clear it in `PlayerRemoving`, or it leaks (see [[13-Luau-Engineering/memory-leak-checklist]]).
- Client mirrors the cooldown for feel only. If client and server disagree, the server wins silently.
- Server cooldown should be a little shorter than the client one (about 90%) so latency does not reject legit presses.

## Timestamp cooldown

```lua
--!strict
local Players = game:GetService("Players")

local lastUsed: { [Player]: { [string]: number } } = {}

local function tryUse(player: Player, ability: string, cooldown: number): boolean
	local now = os.clock()
	local perPlayer = lastUsed[player]
	if not perPlayer then
		perPlayer = {}
		lastUsed[player] = perPlayer
	end
	local last = perPlayer[ability]
	if last and now - last < cooldown then
		return false
	end
	perPlayer[ability] = now
	return true
end

Players.PlayerRemoving:Connect(function(player)
	lastUsed[player] = nil
end)
```

## Token bucket

```lua
--!strict
local Players = game:GetService("Players")

type Bucket = { tokens: number, updated: number }

local CAPACITY = 5
local REFILL_PER_SECOND = 2

local buckets: { [Player]: Bucket } = {}

local function allow(player: Player): boolean
	local now = os.clock()
	local bucket = buckets[player]
	if not bucket then
		bucket = { tokens = CAPACITY, updated = now }
		buckets[player] = bucket
	end
	local elapsed = now - bucket.updated
	bucket.tokens = math.min(CAPACITY, bucket.tokens + elapsed * REFILL_PER_SECOND)
	bucket.updated = now
	if bucket.tokens < 1 then
		return false
	end
	bucket.tokens -= 1
	return true
end

Players.PlayerRemoving:Connect(function(player)
	buckets[player] = nil
end)
```

Call `allow(player)` as the first line in every `OnServerEvent` handler.

## Choosing numbers
| Action | Cooldown | Rate limit |
|---|---|---|
| Melee swing | animation length, 0.4 to 0.8 s | 3 per s |
| Gun shot | fire rate of the weapon | fire rate plus 20% |
| Ability | design, 3 to 30 s | 2 per s |
| Buy / equip | none | 2 per s |
| Chat emote | 1 s | 1 per s |

UNVERIFIED: starting values.

## Gotchas
- Cooldown set after a `task.wait()` lets spam through during the wait.
- Using `os.time()` (whole seconds) gives coarse cooldowns.
- Not clearing tables on leave (slow leak across a long server life).
- Rate limiting by remote only, not by player, so one cheater blocks everyone.
- Per-character state (reset on respawn) vs per-player state: decide which one a cooldown belongs to.
- Debounce flags in scripts (`local busy = true`) that never reset on error: use `pcall` or set the flag in a `finally`-like pattern.

## Related
[[13-Luau-Engineering/remote-validation-cookbook]], [[13-Luau-Engineering/networking-hardening]], [[12-Multiplayer-NPCs/boss-fight-patterns]], [[14-QA-Optimization/exploit-testing]]

## For our Roblox games
Copy the principle: server timestamps, token buckets, cleanup on leave. No names, art or assets are involved. Zero spend: plain Luau.
