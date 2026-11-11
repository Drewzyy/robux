---
description: ProfileStore by default: session locking, autosave plus leave plus BindToClose, failed-load kick, UpdateAsync, throttling, 4 MB limit.
type: playbook
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2; mattqdev ProfileService 2026 guide; ivar-anon roblox-datastores; justozy datastore anti-patterns; chguo999 roblox-datastores
verify: Community references. ProfileStore's exact API is UNVERIFIED here: read its official docs (and Wally page) before writing code against it, and test save/load in Studio with API access enabled.
tags: [luau-engineering, playbook]
---
# DataStore playbook

## Default
**ProfileStore**, the successor to ProfileService, for new work. ProfileService is acceptable (maintenance mode). DataStore2: no session locking, development slowed since 2022; do not use for new games. Raw DataStoreService has no retry, locking, or reconciliation.

## Non-negotiables
- Every DataStore call in `pcall` with retry and exponential backoff.
- Read-modify-write uses `UpdateAsync` (atomic), never `GetAsync` then `SetAsync`.
- One server owns a player at a time (session locking).
- Save on all three: autosave interval, player leaving, and `game:BindToClose`.
- Store only JSON-encodable data: no Instances, CFrames, mixed-key or cyclic tables.

## The number one data-loss bug
Saving only on `PlayerRemoving`. When a server crashes or shuts down, that save may never fire. `game:BindToClose` does fire, with about 30 seconds. Roblox restarts every server on each game update, so without a shutdown flush, progress since the last autosave is lost every update. Save in parallel inside BindToClose; sequential saves can run past 30 seconds.

## Failed loads
If a load fails, do not let the player play on a default profile: the next save overwrites their real data. Kick the player (or refuse to save). A failed load must never look like a new player.

## Throttling and limits
- Writes to the same key are throttled (about one write per key per 6 seconds).
- Check `DataStoreService:GetRequestBudgetForRequestType` before non-critical writes.
- Do not save on every currency change: change in memory, let autosave (ProfileStore default 300 s) and leave-save persist. Lowering the autosave interval is counterproductive.
- 4 MB per key after JSON encoding.
- Do not yield inside an `UpdateAsync` callback.

## Studio
Enable "Studio Access to API Services" or DataStores fail silently in Studio. DataStores do not work from a LocalScript.

## Extra rules (from the roblox-datastores skill, gamedev-skills/awesome-gamedev-agent-skills, Apache-2.0)
- Gate saves on "load succeeded" (`~= true`), not "load did not fail": a player can leave while the load is still running.
- The UpdateAsync callback must not yield and may run more than once; do no side effects inside it (no granting, no remotes).
- Retry only writes that are safe to repeat (writing the full state is safe, IncrementAsync is not); confirm with an uncached read when it matters.
- BindToClose gets at most about 30 seconds: save all players in parallel and wait.
- Studio uses the same live data as players once API access is on; test in a separate place.
