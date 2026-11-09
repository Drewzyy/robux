# Design: Sunny Sips (working title), beach lemonade tycoon

Note: every number below is a **starting value to tune** in playtest.

## 1. Pitch and genre
Squeeze, pour and serve lemonade to beach-goers, then spend the coins to grow a tiny stand into a beach-front lemonade empire. Genre: tycoon. Core verb: **serve** (squeeze, pour, hand over).

## 2. Core loop
- **Every 5 s (verb):** a customer walks up, you hold the squeeze prompt, pour, and hand over the cup. (starting value: 5 s per sale)
- **Every 30 s (reward):** about 5 sales gives about 25 coins, enough for the next upgrade, which you can see and feel at once. (starting values)
- **Every session (goal, ~10 min):** finish Stand Tier 1 and unlock Tier 2, the Boardwalk Cart. (starting value)
- **Every week (goal):** fill the Beach Menu, 6 flavors, one new flavor unlocked per daily return. (starting value)

## 3. First 60 seconds (all values are starting values)
- **0-5 s, spawn and orient:** The player spawns facing a stand with a glowing lemon crate. A customer is already walking up and shows a lemonade icon. There is no text wall. One arrow points at the crate.
- **5-10 s, first core verb:** The player holds the prompt at the crate to squeeze (1.5 s), then the cup auto-fills and they tap the customer to serve. Coins pop and a happy sound plays. First fun lands at about 8 s.
- **10-30 s, first reward:** Customers 2 to 5 arrive about 4 s apart, and each pays 5 coins. At 25 coins the "Striped Umbrella" pad glows. Stepping on it buys it. More customers now come, and the sand shows a visible sun-shade.
- **30-60 s, open loop:** A second serving window opens (cost 40 coins, hit at about 50 s). The goal bar for "Tier 2: Boardwalk Cart" appears at 20%. A sign says "Friends' stands are down the beach". The player always knows the next purchase and its price.

## 4. Mechanics
1. **Squeeze and pour (hold and release).** The player holds the prompt, then releases at the green band for "Perfect" (+1 coin tip). It teaches rhythm. It goes wrong if hold timing is too strict, so a miss still serves, just with no tip.
2. **Heat meter (beach weather).** Sun strength cycles between cool and hot every ~90 s. Hot means more customers and cold-drink demand, cool means a rest beat. It teaches timing upgrades to peaks. It goes wrong if the cool phase feels like a punishment, so base income never drops to zero.
3. **Ice cooler (stock).** Ice melts slowly in hot spells. Refilling it is a free tap at the crate and boosts price per cup. It teaches light maintenance. It goes wrong if it nags, so no fail state and no lost coins.
4. **Seagull shoo.** A seagull sometimes lands on the counter. Tap it to shoo and earn a small coin bonus. It is a surprise, the variable-schedule beat. If ignored it leaves with nothing stolen.
5. **Upgrade pads.** Walk onto a pad to buy (umbrella, window, blender, sign). Visible results teach cause and effect. A pad the player can't afford yet shows its price and a progress ring.

## 5. Progression
- **Fixed:** the shown upgrade ladder (umbrella 25, window 40, blender 90, sign 150 coins; starting values). Tier 2 unlock is a fixed bar. One new flavor each daily return is a fixed schedule the player can see.
- **Variable (surprise only):** seagull bonus and rare "Celebrity Beach-goer" customers tip extra. Nothing needed to progress is random.

## 6. Economy curve (sawtooth)
Rise: each upgrade is 4 to 6 sales away early on, stretching to ~20 sales by Tier 2 (starting values). Rest beat: the cool phase and the moment after each purchase, when income is automatic. Retry under 2 s: there are no fail states. A missed pour re-pours instantly. Sources: sales, tips, seagull bonus. Sinks: upgrades, flavors. Keep all prices in one config table. UNVERIFIED: no economy-worksheet in the vault, so the pacing ratios are my own starting guesses.

## 7. Social hooks (cheapest first)
1. **Join friends:** the beach is a shared server, and friends' stands sit in a row.
2. **Shared goal:** a "Beach Party" bar fills from everyone's sales, and when full it triggers a short crowd boom for all (starting value 200 cups).
3. **Friend leaderboard:** cups served today, friends only.
4. **Helping:** a visitor can carry a cup to a friend's queue. Both earn coins.
Invite prompts come after the first upgrade, never before.

## 8. Lobby and layout
There is no separate lobby. The player spawns on sand at their stand with the sea behind it. Left is the crate, centre the counter, right the upgrade pads in a short row. A boardwalk at the horizon shows the Tier 2 cart as a silhouette goal. Friends' stands line the beach, visible but a short walk away.

## 9. What we will not do
- No pay-to-win and no gated finish. The whole game is reachable free.
- No punishing the free path: no stolen coins, no timers that expire.
- No paid random items in game 1.
- Nothing is proposed for sale in this document. Monetization is for Merchant later.

## 10. Open questions and UNVERIFIED
- STUB / missing vault notes: `10-game-design/index` is a stub (status: stub). The notes `game-design-doc-template`, `core-loop-template`, `first-60-seconds-checklist`, `genre-playbooks/tycoon` and `economy-worksheet` do not exist. I designed from the architect skill's rules.
- UNVERIFIED: the player-psychology claims (9.8-minute median session, the Cutting et al. 2023 study, Roblox's priority order) come from the skill text. I could not confirm them in the vault because `player-psychology` was not loaded (it is only listed as planned).
- UNVERIFIED: all timings, prices and counts are untuned guesses.
- Q: Is the working title "Sunny Sips" acceptable? Andrew decides.
- Q: Should the Beach Party bar be per server or per friend group? Decision needed before Builder starts.
- Q: Is a daily-return flavor reward fine for ages? Check `60-policy` before any streak-type reward ships (not loaded here).

## 11. Loaded
- `00-router/index.md`: required first read, routed the task to `10-game-design`.
- `10-game-design/index.md`: the section index. It is a stub, and all listed notes are marked planned.
- Tried to read `game-design-doc-template`, `core-loop-template`, `first-60-seconds-checklist`, `genre-playbooks/tycoon` and `economy-worksheet`: all five were missing, so no content was used.
- No other section was loaded. I proposed nothing for sale, so `40-monetization/do-not-sell-list` was not needed.
