---
description: Copy-paste Luau validators for RemoteEvent arguments: type checks, number ranges, NaN, strings, enums, instances, distance and ownership checks, and a typed handler wrapper.
type: reference
status: draft
last_checked: 2026-11-14
keywords: remote validation, RemoteEvent, OnServerEvent, type check, NaN, clamp, whitelist, argument validation, exploit, sanity check, typeof, distance check, server authority
sources: creator-docs content/en-us/scripting/security/client-server-boundary.md (not in local clone, title and URL https://create.roblox.com/docs/scripting/security/client-server-boundary, read 2026-11-14 via existing vault note); creator-docs content/en-us/reference/engine/globals.md (typeof, read 2026-11-14)
verify: Validators are standard Luau written from model knowledge and not run in this session: UNVERIFIED by execution, run the headless test gate. The security doc page is not in the local clone.
tags: [luau-engineering, reference]
---
# Remote validation cookbook

Every argument from a client is hostile until checked. These validators go at the top of each `OnServerEvent` handler. For the rules and transport choice see [[13-Luau-Engineering/remotes-and-security]] and [[13-Luau-Engineering/networking-hardening]].

## Do this
- Validate in this order: arg count, types, ranges, enum membership, existence, ownership, distance, state, cooldown. Return early on the first failure.
- Check `typeof(x)`, not `type(x)`, so Roblox datatypes (`Vector3`, `Instance`) are identified.
- Reject NaN and infinity in every number: `x ~= x` is NaN, and `math.abs(x) == math.huge` is infinite.
- Clamp or reject? Reject for security-relevant values (damage, price). Clamp only for harmless values (camera zoom).
- Never trust an `Instance` the client sends: check `IsA`, that it is a descendant of an expected folder, and that the player owns or is near it.
- Never trust names, display strings, prices, damage or "is admin" flags from the client. Look them up server-side from a table keyed by id.
- Wrap with one `validated` helper so every remote gets the same logging and rate limit (see [[13-Luau-Engineering/cooldowns-and-rate-limits]]).
- Test each remote with bad input: nil, wrong type, NaN, huge, negative, empty string, a destroyed instance (see [[14-QA-Optimization/exploit-testing]]).

## Validator library

```lua
--!strict
local Validate = {}

function Validate.isFiniteNumber(x: unknown): boolean
	return typeof(x) == "number" and x == x and math.abs(x) ~= math.huge
end

function Validate.numberInRange(x: unknown, min: number, max: number): boolean
	return Validate.isFiniteNumber(x) and (x :: number) >= min and (x :: number) <= max
end

function Validate.integerInRange(x: unknown, min: number, max: number): boolean
	if not Validate.numberInRange(x, min, max) then
		return false
	end
	return (x :: number) % 1 == 0
end

function Validate.shortString(x: unknown, maxLength: number): boolean
	return typeof(x) == "string" and #(x :: string) <= maxLength
end

function Validate.oneOf(x: unknown, allowed: { [string]: true }): boolean
	return typeof(x) == "string" and allowed[x :: string] == true
end

function Validate.vector3(x: unknown): boolean
	if typeof(x) ~= "Vector3" then
		return false
	end
	local v = x :: Vector3
	return Validate.isFiniteNumber(v.X) and Validate.isFiniteNumber(v.Y) and Validate.isFiniteNumber(v.Z)
end

function Validate.nearCharacter(player: Player, position: Vector3, maxDistance: number): boolean
	local character = player.Character
	local root = character and character:FindFirstChild("HumanoidRootPart")
	if not root or not root:IsA("BasePart") then
		return false
	end
	return (root.Position - position).Magnitude <= maxDistance
end

function Validate.instanceUnder(x: unknown, className: string, container: Instance): boolean
	if typeof(x) ~= "Instance" then
		return false
	end
	local inst = x :: Instance
	return inst:IsA(className) and inst:IsDescendantOf(container)
end

return Validate
```

## Example handler

```lua
--!strict
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local Validate = require(script.Parent.Validate)

local remote = ReplicatedStorage:WaitForChild("BuyItem") :: RemoteEvent

local PRICES: { [string]: number } = { sword = 100, shield = 80 }

remote.OnServerEvent:Connect(function(player: Player, itemId: unknown, quantity: unknown)
	if not Validate.oneOf(itemId, { sword = true, shield = true }) then
		return
	end
	if not Validate.integerInRange(quantity, 1, 10) then
		return
	end
	local price = PRICES[itemId :: string] * (quantity :: number)
	-- check balance on the server, then grant; client never sends price
	print(player.Name, "buys for", price)
end)
```

Parameters after `player` are typed `unknown` on purpose, so the compiler forces a check before use.

## Distance and line of sight
- For interaction remotes: `Validate.nearCharacter(player, target.Position, range + 5)`. The extra 5 studs allows for latency.
- For hit claims: raycast on the server from the player's root to the target. Allow a small tolerance (1 to 2 studs) not a free pass.

## Common bypasses to close
- Table args with extra fields: read only the keys you need, ignore the rest. Check array length before iterating.
- Huge strings or tables sent to eat memory: cap size (`#str <= 100`, `#list <= 50`).
- Same remote spammed 1000 times a second: rate limit per player.
- Fire remote before character loads: guard nil character.
- Wrong state (buy during intermission): check game phase.

## Mistakes
- Using `type()` and letting a `Vector3` pass as a "table" check.
- Forgetting `x ~= x` so NaN slips into position math.
- Accepting `Instance` args from `workspace` anywhere.
- Validating on the client only.

## For our Roblox games
Copy the principle: reject early, typed `unknown` params, table lookups for prices and damage, distance checks. No names, art or assets are involved. Zero spend: plain Luau.
