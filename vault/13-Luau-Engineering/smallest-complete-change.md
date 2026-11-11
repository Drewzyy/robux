---
description: How agents write game code: the smallest change that fully works, reuse before adding, never cut safety, and say what was skipped.
type: playbook
status: verified
last_checked: 2026-11-11
keywords: ponytail, lean code, simple, yagni, minimal, refactor, review, over-engineering, bloat, shortcut
sources: https://github.com/DietrichGebert/ponytail (MIT), skills/ponytail and skills/ponytail-review
verify: Condensed from ponytail's skill text on 2026-11-11; a rule set, not a factual claim.
tags: [luau-engineering, playbook]
---
# Smallest complete change

Adapted from ponytail (DietrichGebert/ponytail, MIT). Applies to code only: build every feature, polish item, and accessibility option the task asks for; build each with the least new code.

## Do this
1. Before writing, read the code you touch and list every place the change must reach: callers, configs, UI that shows the value, saves.
2. Take the first option that fully works: not needed at all (skip it and say so) > already in this project (a module, helper, or pattern) > a Roblox service or Luau built-in > the vault's library (`13-Luau-Engineering/lib/economy`) > one clear line > the minimum code.
3. No wrappers, options, config, or "for later" code nobody asked for. Deleting beats adding. Keep the project's existing structure (Services, Controllers, Configs).
4. Bug fix: find every caller of the function first, then fix the root cause once in the shared code.
5. New logic that matters (money, saves, purchases, damage, a parser, a loop with edge cases) gets one small test or assert-based check (Lune test or `brewix gate build`).
6. A deliberate shortcut gets a comment: `-- shortcut: <the limit>, <when to upgrade>`.
7. Never cut: server-side validation of remotes, purchase grants in ProcessReceipt, data-loss protection, accessibility, anything Andrew asked for.
8. End your output with one or two lines: what you skipped or did not check, and any risk.

## Reviewing (Playtest, Tuner, Director)
Order: correct, safe, holds with a full server, tested, fast, lean. Every finding needs a concrete case ("this input leads to this wrong result"); no case, no finding. Group as Must fix, Should fix, Nice to have; give file:line, the problem, the smallest fix, and what happens if skipped.
