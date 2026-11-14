---
description: Why Vampire Survivors and roguelikes create the one-more-run loop (short runs, random builds, meta unlocks) and how to build it cheaply in Roblox.
type: teardown
status: draft
last_checked: 2026-11-14
keywords: vampire survivors, roguelike, roguelite, one more run, auto attack, meta progression, build variety, short runs, unlocks, hades, low cost
sources: https://www.gamesradar.com/games/roguelike/i-see-a-lot-of-publishers-i-dont-like-vampire-survivors-creator-made-his-own-publisher-to-share-the-luck-and-says-too-many-companies-try-to-exploit-the-platforms-just-to-make-money (search result, snippet only); https://gamedeveloper.com/design/vampire-survivors-development-sounds-like-an-open-source-fueled-fever-dream (title only); https://www.gamespot.com/articles/vampire-survivors-2-is-unlikely-would-to-offer-something-radical-new/1100-6511982/ (snippet only)
verify: UNVERIFIED: run lengths and numbers are from memory of the game (typical 30 minute runs). The creator stated he does not know exactly why it succeeded; our reasons are analysis.
tags: [market-research, teardown]
---
# Teardown: Vampire Survivors and roguelikes
Tiny input (move only), huge screen of effects, and a build that is different every run. The one-more-run loop is a pattern, not a genre.

## Do this
- Make a run 10 to 20 minutes in Roblox (the original is about 30) with a clear boss or timer end.
- Cut input to one thing: movement. Attacks fire automatically. This works on phones and controllers.
- Level up every 30 to 60 seconds early (first level within 15 seconds) and offer 3 random choices each time. Choices are the game.
- Make synergies discoverable: two items that combine into a stronger one. Show a hint on the pickup so players plan.
- Run end gives permanent currency (gold) even on failure, which buys unlocks, so a loss still progresses. See [[03-Game-Psychology/reward-schedules]].
- Unlock new characters or items by conditions (reach minute 10, kill 500) so each run has a side goal. This feeds [[03-Game-Psychology/motivation-coverage-checklist]].
- Restart in one tap: from death screen to a new run under 5 seconds.
- Keep a power spike every 2 to 3 minutes (a chest, an evolution) so the run never plateaus.

## Why it got huge
- Almost no learning: anyone can play in 10 seconds.
- Screen fills with effects and numbers as the build grows; power fantasy is visible. See [[09-Animation-VFX/game-feel-juice]].
- Low price and low spec made it widely available. UNVERIFIED sales not cited.
- Clips of absurd late-game builds spread.

## Why people play
- Power fantasy: weak to screen-clearing in 20 minutes.
- Experimentation: what if I take this combo?
- Low pressure: no aiming skill.

## Why they stay
- Meta progression: each run feeds an unlock list.
- Many characters with different starts.
- Secrets and hidden unlocks.

## Why they come back
- "One more run": a death shows exactly what you could have done differently, and a new build is a fresh idea.
- New content updates.
- Short runs fit any gap.

## Run structure (suggested)
| Minute | Event |
|---|---|
| 0 to 2 | 3 level ups, easy enemies |
| 2 to 5 | first elite and chest |
| 5 to 10 | enemy density doubles; evolutions |
| 10 to 15 | mini boss, power peak |
| 15 to 20 | final boss or survive timer |

## Roblox build notes
- Enemy count: cap 80 active at once; use simple parts, no per-enemy scripts (one server loop moves all). See roblox-performance and roblox-npc-ai.
- Pick-3 UI with large cards; touch targets over 44 dp.
- Save only meta currency and unlock flags; the run state lives in memory.
- Use free marketplace or self-made art; one color palette keeps it cohesive.
- Damage numbers: pooled labels, max 20 visible.

## Mistakes
- Auto-attack with no choice: boring. Choices are the engine.
- Meta unlocks that make early runs trivial.
- Lag from too many enemies and effects on mobile.

## For our Roblox games
Copy: movement-only input, pick-3 level ups, synergy evolutions, failure-pays meta currency, instant restart. Never copy: vampire theme, characters, names, music, UI art. Related: [[03-Game-Psychology/player-psychology]].
