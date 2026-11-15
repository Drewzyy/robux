---
description: Knit (archived), AeroGameFramework, Flamework and NevermoreEngine compared, plus the plain Lifecycle pattern from a public-domain boilerplate that replaces them, with stars and licenses.
type: reference
status: draft
last_checked: 2026-11-15
keywords: knit, framework, service controller, aerogameframework, flamework, nevermore, lifecycle, archived, module loader, roblox architecture, comm, component
sources: https://github.com/Sleitnick/Knit (README.md, ARCHIVAL.md, read 2026-11-15); https://github.com/frrazer/roblox-game-boilerplate (AGENTS.md, src/shared/Lifecycle.luau, read 2026-11-15); https://github.com/Quenty/NevermoreEngine; GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. Knit archival is stated in its own README and ARCHIVAL.md. Flamework and Nevermore were not cloned, so their feature claims come from the search description only and are UNVERIFIED.
tags: [luau-engineering, reference]
---
# Frameworks: Knit and its alternatives

A game framework gives every system the same shape (start order, cleanup, access to other systems). Roblox needs very little of this; ModuleScripts already do most of it.

| Repo | Stars | License | Status |
|---|---|---|---|
| Sleitnick/Knit | 631 | MIT | Archived by its author; README says no longer maintained |
| Sleitnick/AeroGameFramework | 224 | MIT | Older predecessor of Knit |
| rbxts-flamework/core | 160 | MIT | TypeScript (roblox-ts) framework |
| Quenty/NevermoreEngine | 635 | MIT | ModuleScript loader plus many reusable modules |
| frrazer/roblox-game-boilerplate | 108 | Unlicense | Rojo + Wally + Lifecycle template written for AI agents |

## Do this
- Do not start a new game on Knit. Its own ARCHIVAL.md says types and intellisense never fit its design, and the author recommends plain ModuleScripts.
- Use the Service (server) and Controller (client) split anyway: services own state and rules, controllers own input, camera and UI. See [[13-Luau-Engineering/service-controller-template]].
- Write a tiny Lifecycle runner (about 100 lines): each system is a table with Name, Init, Start, Destroy. Run all Init first, then all Start, so cross-references are safe.
- In Init, only store references to other systems. In Start, do work and connect events. Never yield in Init.
- Destroy in reverse order of init, and wrap each Destroy in xpcall so one failure cannot block the rest.
- Fail loudly: if any Init or Start errors, destroy what started and raise an error naming the system and phase.
- Replace Knit's remote bridge with a typed networking layer. See [[13-Luau-Engineering/networking-libraries-zap-and-packet]].
- Use roblox-ts and Flamework only if the whole team already writes TypeScript. Do not mix them into a Luau project.

## Lifecycle sketch (our style)
```luau
--!strict
export type System = { Name: string, Init: ((self: any, all: { [string]: any }) -> ())?, Start: ((self: any) -> ())?, Destroy: ((self: any) -> ())? }

local function run(systems: { System })
	local byName = {}
	for _, s in systems do assert(byName[s.Name] == nil, "duplicate " .. s.Name); byName[s.Name] = s end
	for _, s in systems do if s.Init then s:Init(byName) end end
	for _, s in systems do if s.Start then s:Start() end end
	return function() for i = #systems, 1, -1 do local d = systems[i].Destroy; if d then pcall(d, systems[i]) end end end
end
```

## What the boilerplate teaches
- Naming rules are written down (PascalCase files, `_camelCase` private fields, PascalCase packet names) and an AGENTS.md tells agents to run one check script after every change.
- Server-only code lives in src/server, shared code in src/shared; everything in shared is visible to exploiters.
- Tunable numbers sit in frozen Config tables.
- Rule worth copying: controllers request, services decide. The client never decides outcomes.

## Pitfalls
- Frameworks that auto-require every ModuleScript in a folder hide load order bugs. Keep an explicit ordered list.
- Dependencies between systems that form a cycle will deadlock a naive Init. Keep a one-way order: Data, then Economy, then Shop.
- Nevermore is a large module set with its own loader; adopting it means adopting its conventions. Take single modules only if the license is kept.
- The boilerplate's own rule "no comments in code" is its style, not ours; keep short why-comments.

## For our Roblox games
- Copy the principle: explicit start order, one owner per piece of state, server decides, one command checks everything.
- The boilerplate is Unlicense (public domain), so its structure may be reused, but write our own code rather than pasting files.
- Never copy another game's service names, remote names or tuning numbers.
- Related: [[13-Luau-Engineering/framework-choice]], [[13-Luau-Engineering/project-structure]], [[13-Luau-Engineering/toolchain-wally-rojo-pesde]].
