---
description: Zap, Blink, ByteNet, BridgeNet2 and rbx-net for typed, compact Roblox remote networking, with a Zap schema example, stars, licenses and server-validation pitfalls.
type: reference
status: draft
last_checked: 2026-11-15
keywords: zap, blink, bytenet, bridgenet2, packet, remote event, buffer networking, idl, serialization, rate limit, networking library, rbx-net
sources: https://github.com/red-blox/zap (README.md, docs/intro/getting-started.md, docs/config/events.md, read 2026-11-15); https://github.com/frrazer/roblox-game-boilerplate (AGENTS.md, wally.toml, read 2026-11-15); Knit ARCHIVAL.md on typed remotes (read 2026-11-15); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. Zap README says docs are for 0.6.x and a rewrite is in progress, so syntax may change. Bandwidth savings claims are the projects' own and UNVERIFIED by us.
tags: [luau-engineering, reference]
---
# Networking libraries: Zap and the buffer family

Every network library is a wrapper over RemoteEvent, UnreliableRemoteEvent and RemoteFunction. The gain is typed schemas, smaller packets and automatic validation.

| Repo | Stars | License | Approach |
|---|---|---|---|
| red-blox/zap | 190 | MIT | Schema file compiled to Luau, buffer packing |
| 1Axen/blink | 192 | MIT | Schema compiler (IDL) for buffer networking |
| ffrostfall/ByteNet | 185 | MIT | Runtime buffer serializer |
| ffrostfall/BridgeNet2 | 71 | MIT | Batched remotes |
| roblox-aurora/rbx-net | 108 | MIT | Multi-language networking framework |

## Do this
- Define every remote once in a schema (Zap or Blink) and generate code from it. Do not hand-write RemoteEvent names in scripts.
- Give each event a direction. In Zap, `from: Server` or `from: Client`, and a call style such as ManyAsync. Two-way events are not supported there; add a second event instead.
- Type every field (u8, u16, u32, string, boolean) with the smallest size that fits. A clamp-able value should have a range.
- Treat generated validation as a first filter only. Still check game rules on the server: owns the item, cooldown, distance, stock.
- Rate-limit each client event per player. See [[13-Luau-Engineering/cooldowns-and-rate-limits]].
- Use unreliable events for fast cosmetic data (positions for effects) and reliable events for anything that changes state.
- Commit generated output files, or generate in the check script, so builds are repeatable.
- If you do not want a code generator, copy the lighter pattern: one Packets module that declares each packet name and its argument checks.

## Zap schema example (our style)
```
opt server_output = "src/server/Net.luau"
opt client_output = "src/client/Net.luau"

event BuyItem = {
	from: Client,
	type: Reliable,
	call: SingleAsync,
	data: (ItemId: u16, Count: u8),
}

event CoinsChanged = {
	from: Server,
	type: Reliable,
	call: ManyAsync,
	data: u32,
}
```
```luau
-- server
Net.BuyItem.SetCallback(function(player, itemId, count)
	ShopService:TryBuy(player, itemId, math.clamp(count, 1, 10))
end)
-- client
Net.CoinsChanged.SetCallback(function(coins) hud:SetCoins(coins) end)
```
(Callback method names vary by Zap version; check the 0.6.x docs. UNVERIFIED for the rewrite.)

## Pitfalls
- A typed schema does not prove the request is fair. A valid u16 item id can still be one the player does not own.
- Buffers make traffic harder to read but are not security. Never rely on obscurity.
- Library upgrades can change generated file format; pin versions and regenerate in one commit.
- Batching libraries add up to a frame of delay; do not use them for input that needs instant response.
- Knit's author notes that Luau types are not enforced at runtime over the network, so runtime checks are always required.
- Large strings and tables in one event can hit remote size limits; send ids, not whole tables.

## For our Roblox games
- Copy the principle: one declared contract per message, small types, server-side rules, per-player limits.
- Use the libraries as packages with MIT licenses kept; do not copy another game's packet or remote names.
- Never design remotes that let a client name a price, reward amount or odds.
- Related: [[13-Luau-Engineering/networking-hardening]], [[13-Luau-Engineering/remote-validation-cookbook]], [[13-Luau-Engineering/remotes-and-security]].
