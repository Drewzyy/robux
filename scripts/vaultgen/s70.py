from common import note
S = "70-competitors"
R = "Research report (Nov 9 2026) C7"
SNAP = "Tracker numbers are a Nov 2026 snapshot as quoted in the report and move hourly: UNVERIFIED until re-read on Rolimons or RoVitals with a date."

note(f"{S}/teardown-template", "The 10 fields to record for any game: identity, core loop, progression, monetization, first 60 seconds, retention, social, packaging, performance, copy vs avoid.", "template", "draft",
 R, "Template. A filled teardown is verified by dating every number and linking its tracker page.", r"""
# Teardown template

Date every number and name its source.

1. **Identity:** creator, launch date, visits, 30-day peak CCU, DAU range, approval/like ratio, last update date, update cadence.
2. **Core loop** in one sentence, plus verb every 5 s, reward every 30 s, session goal, weekly goal.
3. **Progression:** currencies, multipliers, rebirth/prestige, collection, offline progress, scarcity, events.
4. **Monetization:** full product list with prices; gamepass vs developer product vs subscription; what is NOT sold; first-purchase price; premium anchor.
5. **First 60 seconds:** spawn, goal visibility, time to first success, pop-ups, tutorial friction.
6. **Retention mechanics:** daily rewards, saved progress, best times and leaderboards, friends and co-play, limited items, weekly events.
7. **Social:** server size, spectating, trading, friends leaderboard, private servers.
8. **Packaging:** icon, thumbnails, title, description, genre tags; the promise made and whether the game keeps it.
9. **Performance and mobile:** part-count feel, load time, mobile playability, visible crash signals.
10. **Copy vs avoid:** three things to steal and three to avoid, naming the mechanic, not the theme.
""")

def td(name, title, desc, body, src):
    note(f"{S}/{name}", desc, "teardown", "draft", R + "; " + src, SNAP, body)

td("teardown-tower-of-hell", "Tower of Hell", "Tower of Hell: random tower, no checkpoints, 8-minute rounds; retention from mastery and rivalry, not progression. Still updated after 8 years.", r"""
# Teardown: Tower of Hell

1. **Identity:** YXceptional Studios. 29.4B visits, 30-day peak CCU 71,822, approval 74%. Updated Oct 11, 2026; maintained for 8 years.
2. **Core loop:** climb a randomly generated tower with no checkpoints in an 8-minute round.
3. **Progression:** minimal; the game is the system.
4. **Monetization:** VIP servers with settings (skip rounds, tower size, shop lock). Full product list not recorded: UNVERIFIED.
5. **First 60 s:** drop into a live round.
6. **Retention:** mastery, rivalry, spectating, "one more run".
7. **Social:** shared rounds and spectating.
8. **Packaging:** not recorded.
9. **Performance:** not recorded.
10. **Copy:** readable stages, short rounds, rivalry, spectating. **Avoid:** copying it worse than the original; no-checkpoints is a deliberate bet, not a default.
""", "god plan Section 2.2 (not in repo); studiokrew.com top games 2026")

td("teardown-grow-a-garden", "Grow a Garden", "Grow a Garden: plant-wait-harvest loop, offline growth, 5-minute clock restocks, weekly events. Fastest game to 1B visits.", r"""
# Teardown: Grow a Garden

1. **Identity:** launched late March 2025 (sources say Mar 25 or 26). About 35-36B visits by late 2026 (35.9B as of Sep 28, 2026 in one source). Peaked at 22.3M concurrent on Aug 23, 2025. Fastest ever to 1B visits (33 days). Update cadence: weekly; 9 update days in the 30 days before Nov 9, 2026.
2. **Core loop:** buy seeds, plant, wait, harvest, sell, buy better seeds.
3. **Progression:** seed tiers, mutations, weather, pets.
4. **Monetization:** pets from loot boxes; exclusive items claimable only while online. Full list UNVERIFIED.
5. **First 60 s:** not recorded.
6. **Retention:** offline growth, 5-minute restocks of shared stock, weekly events, four reward schedules at once. Event spikes are huge then fall fast (god plan: Ghoul Garden launch day peak 100,577, next day 32,326).
7. **Social:** visiting gardens. Grow a Garden 2 (June 2026) adds night raids on other gardens.
8. **Packaging:** not recorded.
9. **Performance:** not recorded.
10. **Copy:** offline progress from saved timestamps, clock-based scarcity, weekly events, layered schedules. **Avoid:** assuming a simple loop needs no economy tuning; the economy and the clock separate keepers from copies.

Mechanics detail: [[10-game-design/genre-playbooks/farming]].
""", "geeiq.com grow-a-garden; rocode.app grow-a-garden guide; pvpbank.com 2026 hot games")

td("teardown-blox-fruits", "Blox Fruits", "Blox Fruits: action RPG with long progression arcs, rare fruit chase, PvP and trading. Players commit when rewards feel earned and visible.", r"""
# Teardown: Blox Fruits

1. **Identity:** Gamer Robot Inc. 40.3B+ visits (one genre list), 400K-800K DAU (one 2026 analysis).
2. **Core loop:** explore islands, fight, level up, chase rare fruit abilities.
3. **Progression:** extensive levelling, rare fruit spawns, bosses.
4. **Monetization:** not recorded in the research: UNVERIFIED.
5. **First 60 s:** not recorded.
6. **Retention:** long arcs (months); rewards that feel earned and visible.
7. **Social:** PvP, trading.
8-9. Not recorded.
10. **Copy:** long arcs, visible rewards, the ability chase. For an obby, the equivalent is stage packs, best times, and leaderboards, not levels. **Avoid:** trading or PvP depth in game 1.
""", "games.studio23.london top genres; studiokrew.com")

td("teardown-brookhaven", "Brookhaven RP", "Brookhaven RP: goal-free life-sim sandbox, the most visited Roblox game; longevity from social play and constant new secrets.", r"""
# Teardown: Brookhaven RP

1. **Identity:** launched April 2020. 67B+ visits (one 2026 guide; god plan says 55.8B+), 700K-1M+ DAU. Most visited game ever.
2. **Core loop:** none; improvised role-play.
3. **Progression:** none in the usual sense.
4. **Monetization:** not recorded: UNVERIFIED.
5. **First 60 s:** not recorded.
6. **Retention:** friends plus constant updates adding secrets and scenes.
7. **Social:** the whole game.
8-9. Not recorded.
10. **Copy:** social is the strongest retention; friends make the same game different. **Avoid:** assuming every genre can drop goals. An obby needs a finish line and a best time; Brookhaven needs none.
""", "pvpbank.com 2026 hot games guide")

td("teardown-pet-simulator-99", "Pet Simulator 99", "Pet Simulator 99: hatch, upgrade, fuse, and trade pets; diamond economy and collection completion. Trading needs an economy, moderation, and scam protection.", r"""
# Teardown: Pet Simulator 99

1. **Identity:** BIG Games Pets. 250K-500K DAU range.
2. **Core loop:** hatch eggs, upgrade pets, collect diamonds.
3. **Progression:** fusion, rebirth, huge numbers, collection completion.
4. **Monetization:** not recorded in detail: UNVERIFIED. Eggs are random; any paid egg falls under [[40-monetization/paid-random-items-checklist]].
5. **First 60 s:** not recorded.
6. **Retention:** collection, event pets as long holds.
7. **Social:** trading rare pets as assets.
8-9. Not recorded.
10. **Copy:** collections earned by finishing (trails, hats, titles), fusion-like upgrade paths for free players, event items as long holds. **Avoid:** trading in game 1.
""", "studiokrew.com; pvpbank.com")

td("teardown-plus1-speed", "+1 Speed games", "+1 Speed games: number-goes-up-now loop that peaked above 4.6M CCU in Oct 2026; copy the feedback speed, avoid the reskin churn.", r"""
# Teardown: +1 Speed games

1. **Identity:** +1 Speed Keyboard Escape (SecretVerse Studio): 30-day peak CCU 4,611,607, approval 96.9%. +1 Speed Monkey Escape similar. Viral in 2026.
2. **Core loop:** do a simple thing, see a number go up immediately.
3. **Progression:** speed upgrades, racing.
4-9. Not recorded.
10. **Copy:** feedback speed; the "number goes up right now" feeling. **Avoid:** reskin churn. Identical loops reskinned weekly are what retention-based ranking punishes. Do not chase it as a model.
""", "god plan Section 2.2 (not in repo)")

note(f"{S}/tracker-watchlist", "Where to watch competitors for free (Rolimons, RoVitals, BLOXG, YouTube and TikTok, Creator Analytics) with a one-hour-a-week cap.", "reference", "draft",
 R, "Tool list from the report. Each site's availability is UNVERIFIED until opened; Pulse records the date of every number it reads.", r"""
# Tracker watchlist

| Source | What it gives | Notes |
|---|---|---|
| Rolimons (rolimons.com) | live players, 30-day peaks, approval, update dates | free |
| RoVitals (rovitals.com) | same | free |
| BLOXG (bloxg.com) | most-played and trending lists, updated daily | free |
| YouTube and TikTok, search "Roblox obby" weekly | what creators cover; covered games get player spikes | free |
| Creator Analytics, Home Recommendations tab | your own signals vs similar-game benchmarks | replaces trackers for decisions once live |

Rule (god plan): one hour a week on competitor watch, no more.
Pulse records source and date for every number; numbers read secondhand through search summaries are UNVERIFIED.
""")

note(f"{S}/copy-vs-avoid", "Mechanics worth stealing and mechanics to avoid, named specifically, from the six teardowns.", "reference", "draft",
 R + " (C7)", "Synthesis of the six teardowns; no numeric claims.", r"""
# Copy vs avoid

## Copy (mechanics, never art or names)
| Mechanic | From |
|---|---|
| Fast feedback: a number goes up the instant you act | +1 Speed |
| Readable stages, short rounds, spectating | Tower of Hell |
| Offline progress from saved timestamps; clock-based restocks | Grow a Garden |
| Weekly events that keep the core loop | Grow a Garden |
| Collections earned by finishing (trails, titles) | Pet Simulator 99 |
| Long, visible progression arcs | Blox Fruits |
| Friends as the content | Brookhaven |
| Daily rewards and streaks | common pattern |

## Avoid
| Pattern | Why |
|---|---|
| Weekly reskins of the same loop | retention ranking buries it |
| Pay-to-win | breaks the free path; see [[40-monetization/do-not-sell-list]] |
| Trading in game 1 | needs an economy, moderation, scam protection |
| No checkpoints by default | a deliberate bet for skill obbies only |
| Paid RNG as the core loop | policy risk and wrong for a young audience |
""")
print("70 done")
