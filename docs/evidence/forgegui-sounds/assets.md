# Dawn Watch: sound effects request sheet

Project: `projects/dawn-watch` (sheet: `projects/dawn-watch/sounds.json`, design: `projects/dawn-watch/docs/design.md`). Sounds only. No Studio session needed.

## Assets
| Need | Choice | Source | Asset ID or file | License | Status |
|---|---|---|---|---|---|
| 41 sound effects (all in sounds.json) | Generate from the prompts in the sheet | ForgeGUI (Andrew generates; UNVERIFIED that ForgeGUI makes audio, see `80-assets/forgegui`) | all `null` | all `null` until Andrew fills | 41 of 41 need generating |

Check of the existing 40 entries against the design: ids unique, no invented asset IDs, prompts say length, character and what to avoid, tone is cozy (no horror, matching section 9 of the design). All kept unchanged; none were wrong.

Coverage: core verbs (pickup, pour, ignite, aim, hold, pop for all 3 creatures), telegraph, lamp dim/out/relight, 3 fog variants, rest and night start, dawn, stars, Glow, upgrades, daily and weekly rewards, UI tap/back, tutorial hint, bot handoff, lane ping.

Added 1 entry (a gap): `lamp_relight_hold`, for the 2 s held relight, which had only a completion sound (`lamp_relight`).

## Dropped and why
Nothing dropped. No Toolbox audio, no copyrighted melodies (prompts forbid them), no look-alike characters or branded sounds.

## Needs Andrew
- Generate **41 sounds** in ForgeGUI (all of them still need generating), upload to Roblox (moderated), paste `rbxassetid://<number>` into `asset_id` and the license line into `license`.
- **13 are first-60-seconds sounds. Generate these first:** oil_pickup, oil_pour, lamp_ignite, beam_sweep, lane_telegraph, mumble_spawn, beam_hold_charge, mumble_pop, oil_low_warning, glow_tick, ui_tap, tutorial_arrow_hint, lamp_brighten.
- Confirm ForgeGUI generates audio, its price (studio budget is $0), and its license for monetized Roblox games. If it does not, change `source` in the sheet.
- Open questions: none beyond the above. `beam_hum` is the only loop; check it loops cleanly.

## Loaded
- `00-router/index.md`: required first read.
- `80-assets/index.md`: section index.
- `80-assets/audio-pipeline.md`: sounds.json shape and roles.
- `80-assets/forgegui.md`: tool status (unverified for audio) and hand-off.
- `80-assets/asset-sources-and-licenses.md`: license recording rules.
- `80-assets/no-toolbox-rule.md`: no Toolbox.
- `80-assets/3d-model-checklist.md`: required by the skill; not relevant to sounds.
- `60-policy/ip-and-toolbox-rules.md`: IP and Toolbox rules.
- `projects/dawn-watch/sounds.json` and `docs/design.md`: the inputs.
