---
description: Pre-commit review list: the bugs that lose data, get exploited, or break on mobile. Builder runs this before finishing.
type: checklist
status: draft
last_checked: 2026-11-09
sources: Research report (Nov 9 2026) C2 (C2 consolidated)
verify: Checklist distilled from the other 20-luau-rojo notes; each item points to the note that justifies it.
---
# Common bugs checklist

Data ([[20-luau-rojo/datastore-playbook]])
- [ ] Saves on autosave, leave, and `BindToClose`, in parallel at shutdown.
- [ ] Failed load kicks or refuses to save; never plays on a default profile.
- [ ] `UpdateAsync` for read-modify-write; every call in `pcall` with retry.
- [ ] No Instances or CFrames in saved data.

Remotes ([[20-luau-rojo/remotes-and-security]])
- [ ] Every handler checks type, range (NaN, inf, negatives), permission, plausibility.
- [ ] Every remote is rate limited per player.
- [ ] No `InvokeClient` for anything needed.
- [ ] Client never sends amounts, damage, or prices.

Structure ([[20-luau-rojo/project-structure]])
- [ ] One bootstrapper per side, Init then Start.
- [ ] No circular requires; no cross-boundary requires.
- [ ] Shared data frozen; no secrets in ReplicatedStorage.

Performance ([[20-luau-rojo/performance-budgets]])
- [ ] Static parts anchored.
- [ ] No heavy work every Heartbeat.

Monetization
- [ ] Every developer product handled in `ProcessReceipt`, idempotent ([[40-monetization/processreceipt-template]]).

Gate ([[20-luau-rojo/headless-test-gate]])
- [ ] `brewix gate build projects/<game>` passes.
