# Economy maths (third-party, unchanged)

`economy_math.luau`, `balance.luau`, and `test_economy_math.luau` are copied unchanged from
gamedev-skills/awesome-gamedev-agent-skills, skills/other-engines/roblox-simulator-economy/scripts
(Apache License 2.0; see LICENSE and NOTICE in this folder). BrewixGames did not modify them.

- Builder copies `economy_math.luau` into a game as a ModuleScript (for example
  `src/shared/EconomyMath.luau`) for prices, buy max, rebirths, offline income, odds, and number
  formatting.
- Test: `lune run test_economy_math.luau` from this folder.
- Pacing table: `lune run balance.luau --print-defaults > economy.json`, edit, then
  `lune run balance.luau economy.json`.

Guide: vault note 07-Monetization/economy-pacing. Path in the vault: 13-Luau-Engineering/lib/economy/.
