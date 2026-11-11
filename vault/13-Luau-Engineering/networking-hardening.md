---
description: Hardening remotes: transport choice, delivery rules, the yield race, layered validation, rate limits, replication, and Server Authority.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: remotes, remoteevent, unreliableremoteevent, remotefunction, rate limit, token bucket, exploit, validation, replication, server authority, networking
sources: https://github.com/gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0)
verify: UNVERIFIED: condensed from the roblox-networking skill (Apache-2.0) on 2026-11-11, which cites Roblox's remote-events and client-server-boundary docs.
tags: [luau-engineering, playbook]
---
# Networking hardening

Condensed from the roblox-networking skill in gamedev-skills/awesome-gamedev-agent-skills (Apache-2.0).

## Do this
| Use | For | Never for |
|---|---|---|
| RemoteEvent | reliable, ordered one-way requests and facts | per-frame samples |
| UnreliableRemoteEvent | cosmetic, continuous, replaceable samples | purchases, damage, inventory, state changes |
| RemoteFunction | short client-to-server query that needs a reply | server invoking a client (it can hang forever) |
- Send intent and ids, never prices, damage, or results. The server derives outcomes.
- Validate in layers, cheapest first: type and finiteness, allowlisted value, Instance class and ancestry, player state, distance or line of sight, cooldown, rate budget.
- The yield race: a handler that yields (task.wait, WaitForChild, DataStore) lets the next request start; claim the cooldown or mark "in progress" before the first yield, or two purchases pass one check.
- Rate limit per player with a token bucket (for example capacity 6, refill 3 per second); clear per-player tables on PlayerRemoving.
- Limits: unreliable payloads over 1000 bytes are dropped; client-to-server traffic throttles around 500 calls per second per client per remote type (over the limit is delayed for RemoteEvent, discarded for unreliable).
- Replicate narrowly: FireClient for private facts, broadcast only shared ones. With streaming, an Instance may not exist on a client; send ids and tolerate absence.
- Workspace.AuthorityMode = Server (opt-in) gives built-in prediction and rollback; it is a whole-project migration, never a side effect of another task.
- Test with Server & Clients: malformed, spam, out of range, after death, after leaving, several players at once.
