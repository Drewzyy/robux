---
description: Cmdr command console, Adonis admin system, and Iris/Plasma immediate-mode debug UI for Roblox: setup, the server-side permission rule, stars, licenses, and when to use each in a kid-safe game.
type: reference
status: draft
last_checked: 2026-11-15
keywords: cmdr, adonis, admin commands, debug console, iris, plasma, imgui, beforerun hook, permissions, moderation, developer tools, kick, ban
sources: https://github.com/evaera/Cmdr (README.md, docs/03-setup.md, 04-commands.md, 06-permissions.md, 53-security.md, read 2026-11-15); https://github.com/SirMallard/Iris (README.md, read 2026-11-15); https://github.com/Epix-Incorporated/Adonis (search listing only); GitHub search 2026-11-15
verify: Stars and licenses from GitHub search 2026-11-15. Cmdr behavior read from its docs. Adonis was not cloned, so its features are UNVERIFIED beyond its one-line description. Cmdr docs use a newer Luau `const` keyword that may not run in every Studio version, UNVERIFIED.
tags: [luau-engineering, reference]
---
# Admin and debug tools: Cmdr, Adonis, Iris

Teams need a way to give coins, skip a round or inspect state during testing. Use a proper command framework, not ad hoc chat commands.

| Repo | Stars | License | Use |
|---|---|---|---|
| evaera/Cmdr | 547 | MIT | Typed command console with autocomplete |
| Epix-Incorporated/Adonis | 516 | MIT | Full server administration system |
| SirMallard/Iris | 362 | MIT | Immediate-mode debug UI (Dear ImGui style) |
| matter-ecs/plasma | 24 | MIT | Immediate-mode widgets for Matter debugging |

## Do this
- Add Cmdr for developer-only tools in every game. Register commands on the server only; it replicates the definitions to clients for you.
- Cmdr blocks all non-default commands in live games until you register at least one server-side BeforeRun hook. Write that hook first.
- Deny every group by default. Allow named groups (Admin, Dev) to a short list of UserIds or a group rank you control.
- Keep public groups tiny: only the harmless utilities (help, echo) may be open to everyone.
- Put the real check in a SERVER hook. A client hook is only for fast feedback, since exploiters control their client.
- Split a command into a definition module (Name, Args with types, Group) and a `NameServer` module that runs on the server. Cmdr validates argument types on both sides.
- Gate every debug command with `RunService:IsStudio()` or an admin check so they cannot ship open.
- Use Iris for in-game debug panels (frame times, entity counts, state viewers). Build it client-side and load it only for allowlisted users.
- Choose Adonis only if you want a ready moderation product; it is big and brings its own UI and settings.

## Command example (our style)
```luau
-- GiveCoins.luau (definition)
return {
	Name = "givecoins",
	Description = "Give coins to players (dev only).",
	Group = "Admin",
	Args = {
		{ Type = "players", Name = "targets", Description = "Who gets coins" },
		{ Type = "integer", Name = "amount", Description = "1 to 100000" },
	},
}
-- GiveCoinsServer.luau
return function(_context, targets: { Player }, amount: number): string
	amount = math.clamp(amount, 1, 100000)
	for _, p in targets do DataService:AddCoins(p, amount, "admin") end
	return `Gave {amount} coins to {#targets} players.`
end
```
```luau
-- server hook: only allowlisted UserIds may run Admin commands
registry:RegisterHook("BeforeRun", function(context)
	if context.Group == "Admin" and not ALLOWED[context.Executor.UserId] then
		return "No permission."
	end
end)
```

## Pitfalls
- Registering hooks through RegisterHooksIn clones them to the client too; never put secrets in a hook file.
- Commands that change data must go through the same DataService validation as player actions, not write profiles directly.
- Admin commands that touch children (kick, ban, teleport) need logging: who ran what, when.
- An open Cmdr console in a live game with a weak hook is a full exploit surface.
- Iris code runs per frame; creating widgets for hundreds of entities costs frames. Draw summaries.
- Never let a debug command grant paid items or bypass purchase flow in production builds.

## For our Roblox games
- Copy the principle: server-enforced permissions, typed arguments, audit log, debug tools off for normal players.
- Use these as packages with licenses kept. Do not copy Adonis code into our scripts.
- Never ship commands that let staff grant themselves unfair advantages in public servers.
- Related: [[13-Luau-Engineering/remotes-and-security]], [[13-Luau-Engineering/remote-validation-cookbook]], [[14-QA-Optimization/index]].
