---
description: Never trust the client: validation order, NaN and math.huge, rate limits, server-owned state, RemoteFunction hazards, what replicates.
type: reference
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2; pinheadsk8r, ivar-anon, brockmartin security-hardening references; tabooharmony roblox-networking
verify: Community security references. The NaN rule is checked by the Lune test in service-controller-template. Server Authority status (reported graduated June 2026) is UNVERIFIED; check Creator Docs.
tags: [luau-engineering, reference]
---
# Remotes and security

Every RemoteEvent and RemoteFunction argument is attacker-controlled. Exploiters can decompile any replicated script, fire any remote at any rate with any arguments, trigger `Touched` and `ProximityPrompt` from any range, and replay traffic.

## Validate on the server, in order
1. **Type** (`typeof`/`type`). Reject `nil`, wrong types, tables shaped to crash the handler.
2. **Range.** Reject `NaN` (`x ~= x`), `math.huge`, negatives where not allowed. NaN is the dangerous one: it passes `<`/`>` checks and can corrupt saved data.
3. **Permission.** Does this player own this thing, is this action allowed now?
4. **Plausibility.** Distance from the character, cooldown, rate.
Fail by returning early and dropping the request. Do not error loudly to the exploiter.

## Server owns state
Currency, inventory, damage, cooldowns, purchases, progression. Client requests; server decides. Damage is computed by the server from weapon stats, distance, and cooldown, never accepted from the client. Currency math is server-side; the client only displays.

## Rate limits
Limit every remote per player per remote. Roblox's built-in throttle (about 500 requests/second per client) is not a substitute. Use suspicion scoring: accumulate violations, kick at a threshold, do not kick on the first offence.

## RemoteFunction hazards
Never call `RemoteFunction:InvokeClient` for anything you need: a client can never return (the server thread leaks) or return garbage. Use RemoteEvents server to client. Use `UnreliableRemoteEvent` for high-frequency data such as positions.

## What is public
Everything replicated: code in ReplicatedStorage, StarterPlayerScripts, StarterGui; remote names; attribute values. Obfuscation is not security. Design so cheating is impossible or gains nothing.

## Movement and physics
Server Authority is reported (June 2026) as the long-term fix for movement and physics exploits. UNVERIFIED: confirm status in Creator Docs before relying on it.
