# Pulse pitch: rising trend for a solo, small, original game
Run: pipe-pitch-to-build-20261109-105542-ca9c--1-pulse. Date: 2026-11-09.

## READ FIRST: data quality
- **No live data was pulled.** I ran 2 WebSearch queries on 2026-11-09 and read only the search summaries. I did not open Rolimons, RoVitals, BLOXG or the Roblox charts pages. Every number below comes from those summaries, was not checked at its source, and is **UNVERIFIED**.
- **Vault gaps.** `16-Market-Research/index` and `02-Game-Design/index` are both `status: stub`. I tried to read `teardown-template`, `tracker-watchlist` and `copy-vs-avoid`. None of the three exists. I did not search other sections. **STUB: 16-Market-Research/index. STUB/MISSING: teardown-template, tracker-watchlist, copy-vs-avoid. STUB: 02-Game-Design/index** (no genre playbook was readable). I fell back to the Pulse skill rules. No competitor teardown matched, so I loaded none.
- The 01-Roblox-Fundamentals section was not loaded, because the skill does not call for it. Policy scores below are my own judgment, not vault-backed. Andrew or Lead should run the pre-launch policy check.

## Data sources (all UNVERIFIED, secondhand via search summary, retrieved 2026-11-09)
| # | Claim | Source as reported | Source date |
|---|---|---|---|
| S1 | Steal An Egg was #1 at 986K CCU. About 9.7M total CCU across about 14K live experiences. | RoWatcher live snapshot (https://www.rowatcher.com/) | 2026-10-14 |
| S2 | "Steal An Egg (and Collect Rare Pets)" reported at 1.6M CCU. The genre is called crowded. Steal An Egg and Ride A Pet ship weekly content. | StudioKrew (https://studiokrew.com/blog/top-roblox-games-september-2026/) | Sep 2026 |
| S3 | Grow a Garden 2 about 560.3K CCU. Brookhaven, Adopt Me, Blox Fruits and Steal a Brainrot sit at 200K to 400K+. | RoWatcher (https://rowatcher.com/news/how-many-people-play-roblox-right-now) | 2026-07-30 |
| S4 | 2026 breakouts: Steal a Brainrot, 99 Nights in the Forest, Fish It, Sell Lemons, Kick a Lucky Block. | forgegui (https://forgegui.com/blog/10-roblox-game-ideas-blowing-up-june-2026) | Jun 2026 |
| S5 | RIVALS 291K+ CCU. | RoWatcher (https://rowatcher.com/news/the-10-fastest-growing-roblox-games-in-march-2026) | Mar 2026 |
| S6 | Week of Sep 7 to 13: platform average CCU up 2% week over week. Puzzle led genres at 39.9% (unclear metric). | RoWatcher (https://rowatcher.com/news/roblox-week-in-numbers-2026-w37) | Sep 2026 |
| S7 | Roblox Wrapped! released Oct 8, peak CCU 245. | RoWatcher / rotrends | Oct 2026 |

Weak evidence, not used for scoring: an auto-generated blog claiming "brainrot is a whole genre" (RocVibaBot, 2026-04-04).
The StudioKrew and RoWatcher figures disagree (1.6M vs 986K for what looks like the same game). This is another reason all numbers are UNVERIFIED.

## Trend chosen
**"Steal / hatch / run" incremental games**: carry or protect a valuable thing in a base while others try to take it (S1, S2, S4). Demand is the strongest I could find, but it is crowded and the top games have weekly content cadence. A solo dev cannot out-content them, so each pitch changes the verb or the social structure.
A second thread is **co-op survive-the-night** (99 Nights in the Forest, S4). It is small in scope and social.

## Candidates table
Scores are 1 to 5. For policy risk, lower is better. "Differs" means how much it differs from the top game in the trend.
| # | Candidate trend | Evidence (UNVERIFIED) | Demand | Solo fit | Retention | Policy risk | Differs | Total (policy inverted) |
|---|---|---|---|---|---|---|---|---|
| 1 | Steal/hatch/run base game | S1, S2 | 5 | 3 | 4 | 3 (random hatching, PvP theft) | 3 | 18 |
| 2 | Co-op survive-the-night | S4 | 4 | 4 | 4 | 1 | 4 | 20 |
| 3 | Fishing incremental (Fish It) | S4 | 4 | 4 | 3 | 2 | 3 | 17 |
| 4 | Farming with clock restocks (Grow a Garden) | S3 | 4 | 4 | 5 | 2 | 2 | 18 |
| 5 | Lucky-block / RNG (Kick a Lucky Block) | S4 | 3 | 5 | 3 | 5 (gambling-adjacent) | 2 | 13 |
| 6 | Tycoon-sell (Sell Lemons) | S4 | 3 | 5 | 3 | 1 | 2 | 16 |
| 7 | Competitive FPS (RIVALS) | S5 | 4 | 1 | 4 | 3 | 2 | 13 |
| 8 | Roleplay sandbox (Brookhaven) | S3 | 4 | 2 | 3 | 4 (moderation load) | 2 | 12 |

Total = demand + solo fit + retention + (6 − policy) + differs.
Candidates 5 to 8 are rejected for policy or scope. The top 3 below merge candidates 1, 2 and 3/4. Their scores are in the pitches.

## Ranked pitches

### 1. Dawn Watch (co-op survive-the-night, candidate 2)
- **Pitch:** Up to 4 friends defend a lighthouse across one 9-minute night. Each person keeps a different lamp alive while fog creatures drift in on a visible, telegraphed path.
- **Core verb:** Tend (refuel and aim lamps). Each player has a station.
- **Comps:** 99 Nights in the Forest (co-op night survival, S4). Not copied: rounds are short and fixed, and each role has a station.
- **Twist:** The night is one complete loop that fits the 9.8-minute median session. Dawn always arrives, so a failed night is never a hard wipe. Fixed rewards (lamp upgrades) come every night. The surprise is a variable "fog event" in the middle of the night. A shared lighthouse upgrade board is the reason to come back tomorrow.
- **Scores:** demand 4, solo fit 4, retention 4, policy risk 1, differs 4.
- **Risks:** Co-op needs other players online, so add bots or solo scaling. Pathing and fog-creature AI is the biggest scope risk. Keep it to 3 creature types.
- **Architect needs to know:** Round length is 9 minutes, retry under 2 seconds, first lamp lit within 10 seconds of spawn. Three stations in MVP. Avoid horror or gore for age rating. Make creatures cute and strange.

### 2. Nest Watch (steal/hatch/run with a different verb, candidate 1)
- **Pitch:** You and a friend share one nest. Your eggs hatch on a fixed visible timer, and raiders from other nests sneak in to tag them. You win by guarding, not by grabbing.
- **Core verb:** Guard (place decoys and traps, then catch raiders). Raiding is secondary and easy.
- **Comps:** Steal An Egg (S1, S2) and Steal a Brainrot (S4). Not copied: no original names, art or characters, and no brainrot IP.
- **Twist:** The "stolen" item is only borrowed. It returns to the nest in 60 seconds, so no one permanently loses progress. Pairs share a nest, which turns the social hook into the core loop.
- **Scores:** demand 5, solo fit 3, retention 4, policy risk 3, differs 3.
- **Risks:** PvP theft can read as griefing. Hatching is fixed-timer and free only, with **no paid random items in game 1**. The top games ship weekly content, so a solo dev falls behind. Treat as the riskiest of the three.
- **Architect needs to know:** Hatch rewards must be deterministic. Show the timer on every egg. Raid damage is capped. Tutorial happens by doing: first egg hatches in under 30 seconds.

### 3. Tide Table (fishing/farming hybrid with real-clock tides, candidates 3 and 4)
- **Pitch:** Cast at a pier where the tide, set by the real clock, decides which fish are biting. Come back at the next tide to catch the fish you missed.
- **Core verb:** Time (cast and reel on a rhythm window).
- **Comps:** Fish It (S4) and Grow a Garden's clock restocks (S3). Not copied: nothing is a copy, and the tide schedule is public to all players.
- **Twist:** The tide schedule is shared. Everyone sees the same tide board, which makes "meet me at high tide" a friend plan. Rewards are fixed per fish type. Rarity is shown on the tide board, not rolled.
- **Scores:** demand 4, solo fit 4, retention 3, policy risk 2, differs 3.
- **Risks:** Fishing is a crowded genre. Retention depends on a daily tide hook that may be too slow for kids. The tide system needs server-time handling, and I have not verified an API for it. UNVERIFIED.
- **Architect needs to know:** A full loop of cast, catch and sell inside 90 seconds. Offline progress is not needed in MVP. Use a fixed tide cycle of about 20 minutes so one session sees two tides.

## Risks (overall)
- All demand numbers are UNVERIFIED and contradictory. Re-check on RoWatcher or Rolimons before Architect commits.
- The steal/hatch/run genre is crowded (S2). Pitch 2 is exposed to that.
- Policy: I did not load 01-Roblox-Fundamentals. Pitch 2 has the most risk (theft framing, hatching). No pitch uses gambling or paid random items. Pre-launch policy check is still needed.
- Vault stubs mean no teardown or genre-playbook check was possible for any pitch.

## Open questions for Andrew
1. Can someone run the live-chart check (Rolimons, RoVitals, BLOXG) so numbers can be verified?
2. Should the missing 16-Market-Research notes be written? Pulse could not use them.
3. Is a 4-player co-op acceptable, or is it solo-friendly only?

## Loaded
- `00-Start-Here/index.md`: required first read, routing.
- `16-Market-Research/index.md`: required section index. It is a stub.
- `02-Game-Design/index.md`: to check pitches against genre playbooks. It is a stub, so no playbook was readable.
- Not found: `16-Market-Research/teardown-template`, `tracker-watchlist`, `copy-vs-avoid`.
