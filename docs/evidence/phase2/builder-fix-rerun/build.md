Project: projects/dawn-watch

## What changed
The gate had one failure: `stylua --check`. In `src/client`, a long `self.status.Text = ... .. board` line (around line 97) was not wrapped the way StyLua wants. I ran `stylua` on `projects/dawn-watch/src`. It only re-wrapped that concatenation. There is no logic change, so game behavior is unchanged.

## Gate (python3 ../scripts/gates/build_gate.py projects/dawn-watch)
- First run: rojo build passed, selene passed, stylua --check FAILED, lune passed.
- After formatting, second run: all four passed. The Lune specs that passed were Validate, Aim, Beam, Fog, NightPlan, Economy, Tutorial and Config.
- Selene ran with the offline Roblox std. It checks Roblox globals only, not API members.

## Not checked
I did not audit RemoteEvent argument validation or ProfileStore/BindToClose usage. That was outside the task, which was to fix the formatting and lint failures. Both are marked as not run in gate.json.

## Open questions
None.

## Loaded
- `00-router/index.md`: required first read.
- `20-luau-rojo/index.md`: the Builder's home section. It is a stub, so I fell back to the skill's hard rules. I did not load the `headless-test-gate` note, because the gate script gave all the information needed.
