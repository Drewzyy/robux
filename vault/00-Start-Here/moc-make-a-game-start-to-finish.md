---
description: Ordered path from idea to launch and the first 30 days: each step lists the 2 to 5 vault notes to read, in order.
type: reference
status: draft
last_checked: 2026-11-15
keywords: make a game, idea to launch, build order, mvp, steps, roadmap, checklist, first game, workflow, release, plan, launch
sources: vault notes only, read 2026-11-15 (front matter of each linked note)
verify: Built from note descriptions; each linked note carries its own source and verify line.
tags: [start-here, reference]
---
# Make a Game: Start to Finish
Follow the steps in order. Do not skip step 3 (core loop) or step 22 (policy check). Read only the notes named in the step you are on.

## Do this
- Read first, step 1: [[17-Development-Templates/mvp-build-order]] (the order to build things).
- Read first, step 2: [[02-Game-Design/core-loop-template]] (the one loop everything hangs on).
- Read first, step 3: [[17-Development-Templates/launch-checklist]] (the gate before you publish).
- Then pick the section below that matches your task and read only those notes.

## How to use this map
- Read the three notes under Do this, then open one section below.
- Each note has its own Do this list; act on that, not on this page.
- If a note is marked UNVERIFIED, tune from your own analytics.

## Phase 1: Idea and plan
- [[01-Roblox-Fundamentals/how-roblox-works]]: 1. Learn client, server, places and Studio basics first.
- [[17-Development-Templates/game-idea-template]]: 2. Write the idea in the fill-in template.
- [[17-Development-Templates/pitch-template]]: 2. Score candidates and pick one.
- [[16-Market-Research/roblox-hit-patterns]]: 2. Check the idea against what hits share.
- [[16-Market-Research/copy-vs-avoid]]: 2. Know which mechanics to steal and which to skip.
- [[02-Game-Design/core-loop-template]]: 3. Define the verb every 5 seconds and the reward every 30.
- [[02-Game-Design/loop-stack]]: 3. Plan how core, session and long-term loops hand off.
- [[02-Game-Design/game-design-doc-template]]: 4. Fill the one-page design doc.
- [[17-Development-Templates/style-kit-template]]: 4. Fix the look, sound and UI style once.

## Phase 2: Safety check before building
- [[01-Roblox-Fundamentals/policy-index]]: 5. Read the governing rules with their URLs.
- [[01-Roblox-Fundamentals/gambling-line]]: 5. Confirm no mechanic crosses the gambling line.
- [[01-Roblox-Fundamentals/ip-and-toolbox-rules]]: 5. Confirm no copied names, art or music.
- [[07-Monetization/do-not-sell-list]]: 5. Know what must never be sold.

## Phase 3: Set up and block out
- [[17-Development-Templates/mvp-build-order]]: 6. Core loop first, monetization last.
- [[13-Luau-Engineering/project-structure]]: 7. Set up the Rojo layout.
- [[13-Luau-Engineering/toolchain]]: 7. Install the free toolchain.
- [[13-Luau-Engineering/framework-choice]]: 7. Pick one framework and stay on it.
- [[13-Luau-Engineering/service-controller-template]]: 7. Start from the bootstrapper template.
- [[02-Game-Design/world-and-level-design]]: 8. Block out the map with landmarks and sightlines.
- [[10-Art-Environment/art-direction-for-roblox]]: 8. Set silhouettes and shape language.
- [[10-Art-Environment/no-toolbox-rule]]: 8. Build from primitives, no Toolbox models.

## Phase 4: Build the core
- [[13-Luau-Engineering/smallest-complete-change]]: 9. Write the smallest complete change each time.
- [[13-Luau-Engineering/datastore-playbook]]: 10. Save player data safely from day one.
- [[13-Luau-Engineering/remotes-and-security]]: 10. Never trust the client.
- [[00-Start-Here/moc-genres]]: 11. Pick the genre playbook, teardown and review notes.
- [[05-Progression/progression-systems]]: 12. Add currencies, levels and unlocks.
- [[05-Progression/progression-curves]]: 12. Set XP and cost curves.
- [[02-Game-Design/economy-worksheet]]: 13. Put every price in one ModuleScript.
- [[02-Game-Design/economy-math]]: 13. Balance sources and sinks.

## Phase 5: First minutes and polish
- [[04-Onboarding/first-60-seconds-checklist]]: 14. Make the first minute work.
- [[04-Onboarding/tutorial-patterns]]: 14. Teach by doing.
- [[04-Onboarding/fast-early-rewards]]: 14. Give a reward every 10 to 30 seconds early.
- [[08-UI-UX/hud-template]]: 15. Build the HUD.
- [[08-UI-UX/mobile-first-checklist]]: 15. Make it work on phones.
- [[09-Animation-VFX/game-feel-juice]]: 16. Add feedback with millisecond timings.
- [[11-Audio/sound-feedback-map]]: 17. Map every action to a sound.
- [[11-Audio/audio-design-rules]]: 17. Mix with SoundGroups and free audio only.
- [[12-Multiplayer-NPCs/multiplayer-patterns]]: 18. Add social and shared state if the genre needs it.

## Phase 6: Money and retention
- [[07-Monetization/product-catalog-template]]: 19. Choose gamepasses and developer products.
- [[07-Monetization/pricing-ladder]]: 19. Set price tiers.
- [[07-Monetization/processreceipt-template]]: 19. Write idempotent purchase handling.
- [[07-Monetization/paid-random-items-checklist]]: 19. If random paid items exist, show odds.
- [[06-Retention/day-1-day-7-day-30-playbook]]: 20. Plan the hooks for D1, D7 and D30.
- [[06-Retention/favorite-and-follow-prompts]]: 20. Ask for favorites at the right moment.

## Phase 7: Test and ship
- [[14-QA-Optimization/playtest-qa-checklist]]: 21. Run the playtest QA.
- [[14-QA-Optimization/exploit-testing]]: 21. Attack your own remotes.
- [[14-QA-Optimization/device-test-matrix]]: 21. Test on small phones and slow networks.
- [[13-Luau-Engineering/headless-test-gate]]: 21. Pass the build gate.
- [[01-Roblox-Fundamentals/pre-launch-policy-checklist]]: 22. One hour policy check.
- [[10-Art-Environment/thumbnail-pipeline]]: 23. Make the thumbnail and icon.
- [[08-UI-UX/thumbnail-icon-spec]]: 23. Check sizes and file limits.
- [[17-Development-Templates/launch-checklist]]: 24. Run the launch checklist.

## Phase 8: After launch
- [[15-LiveOps/post-launch-first-30-days]]: 25. Follow the day-by-day plan.
- [[06-Retention/analytics-reading-guide]]: 25. Read D1 and session first.
- [[06-Retention/decision-tree]]: 26. Decide what to fix first.
- [[15-LiveOps/update-cadence]]: 27. Set the update rhythm.
- [[17-Development-Templates/update-checklist]]: 27. Gate every update.
- [[15-LiveOps/weekly-review-template]]: 27. Review weekly.
- [[19-Project-Memory/how-project-memory-works]]: 28. Record decisions per game.

## For our Roblox games
- Copy the order: loop, then feel, then onboarding, then money. Never copy another game's name, art, logos or music.
- Zero spend: every tool and asset named here is free. If a step seems to need money, skip it.
- Stuck on a specific question? Use [[00-Start-Here/question-router]].
- Children play these games: no dark patterns, odds always shown, purchases only from a player tap.
