from common import note
S = "90-templates-checklists"
R = "Research report (Nov 9 2026) C9"

note(f"{S}/launch-checklist", "Launch checklist: policy re-check, maturity, thumbnail, icon, description, products, ProcessReceipt test, playtest pass, Andrew's approval of the exact build, publish to Saved.", "checklist", "draft",
 R, "Checklist; each line points to the note that defines it. Andrew ticks the final two lines himself.", r"""
# Launch checklist

- [ ] [[60-policy/pre-launch-policy-checklist]] done today.
- [ ] Content Maturity questionnaire accurate.
- [ ] Thumbnail(s) and icon per [[30-ui-ux/thumbnail-icon-spec]]; they show what the first 10 seconds deliver.
- [ ] Title and description accurate; no keyword stuffing, no giveaways as the lead.
- [ ] Products created in Creator Dashboard by Andrew, matching the approved catalog ([[40-monetization/product-catalog-template]]).
- [ ] Every developer product tested through ProcessReceipt in Studio.
- [ ] [[90-templates-checklists/playtest-qa-checklist]] passed on PC, phone, and gamepad.
- [ ] Build gate passes ([[20-luau-rojo/headless-test-gate]]).
- [ ] Analytics funnels and economy events in place ([[50-retention-analytics/event-taxonomy]]).
- [ ] Andrew approved this exact build in the review queue.
- [ ] Publish script run by Andrew, defaulting to **Saved, never live**. Going live is a separate, explicit decision.
""")

note(f"{S}/update-checklist", "Update checklist: one named reason, the metric it should move, analytics snapshot before, 72-hour wait after, short update notes, Andrew's approval.", "checklist", "draft",
 R, "Checklist; no facts to verify.", r"""
# Update checklist

- [ ] One named reason for this update, and the one metric it should move.
- [ ] Snapshot of current metrics with the date ([[50-retention-analytics/weekly-review-template]]).
- [ ] Scope fits one update; out-of-scope list written.
- [ ] Build gate passes; Playtest report has no blockers or majors.
- [ ] Content Maturity answers still true (resubmit if not).
- [ ] Update notes: one short paragraph.
- [ ] Andrew approves the exact build. Publish to Saved by default.
- [ ] Change time logged; read results after 72 hours, not before.
""")

note(f"{S}/playtest-qa-checklist", "Playtest QA: how it was tested, spawn, first 60 seconds, every stage readable, checkpoints, coins, shop, no early pop-ups, PC, phone, gamepad, performance.", "checklist", "draft",
 R + " (C9); 10-game-design and 30-ui-ux checklists", "Checklist used by the Playtest agent; its report is checked by scripts/gates/check_stage.py (method stated, verdict, severities).", r"""
# Playtest QA checklist

## State the method first
Live in Studio (Studio MCP or Andrew), headless (Lune tests, rojo build), or code walk only. Never imply live play that did not happen.

## First 60 seconds (per platform)
- [ ] Spawn faces the objective; no wall of text.
- [ ] Core verb within 5-10 s.
- [ ] First reward within 10-30 s with feedback.
- [ ] Next goal visible by 30-60 s.

## Core systems
- [ ] Every stage or level readable before the jump or action.
- [ ] Checkpoints save and respawn under 2 seconds.
- [ ] Currency earned matches the design numbers.
- [ ] Shop opens, shows Robux prices, never pops up on join, on death, or in the first 5 minutes.
- [ ] Progress saves and loads (rejoin test).

## Platforms
- [ ] PC: keyboard and mouse.
- [ ] Phone: touch targets reachable, nothing under the joystick or top bar.
- [ ] Gamepad: every action reachable.

## Performance
- [ ] Loads to playable under 10 s; no stutter on a low-end phone profile.

## Report each problem as
id, platform, where (file and line or place), what happens, what should happen, severity (blocker, major, minor). Verdict PASS only with no blockers or majors. A generic pass is a failed report.
""")

note(f"{S}/anti-slop-checklist", "Anti-slop gate for the tool's own output and for games: specific not generic, verified not asserted, sourced or marked UNVERIFIED, would a player tell a friend.", "checklist", "draft",
 R + " (A5 good vs useless vault; C9); god plan Section 7.3 (not in repo)",
 "The studio's own list is below. The god plan's 16-point hand-tuned checklist is NOT in this repo and is not reproduced: BLOCKED until Andrew adds the god plan.", r"""
# Anti-slop checklist

Applies to every agent output and every game. If an output fails, it does not ship.

## Agent output
- [ ] **Specific:** names this game's mechanics, numbers, files. If it could describe any game, it is generic.
- [ ] **Verified, not asserted:** every check in gate.json was actually run; anything not run is a failed check.
- [ ] **Sourced:** facts come from a vault note with a verify line, or are marked UNVERIFIED.
- [ ] **No invented APIs, paths, or credentials.**
- [ ] **No filler:** no "delve", "game-changer", "in today's fast-paced", empty praise.
- [ ] **Loaded section** lists what was read.

## Game
- [ ] A new mechanic or twist, not a reskin of a top game.
- [ ] Original names, art, and thumbnails; no look-alike characters.
- [ ] The thumbnail's promise is kept in the first 10 seconds.
- [ ] Juice on every action ([[80-assets/vfx-sfx-library]]).
- [ ] Hand-tuned numbers: difficulty and economy were playtested, not left at first guesses.
- [ ] **Would a player recommend this to a friend?** If the honest answer is no, it does not ship.

## God plan Section 7.3 (16-point checklist)
BLOCKED: the god plan is on Andrew's Mac, not in this repo. Paste the 16 points here after it is added; do not reconstruct them from memory.
""")

note(f"{S}/game-idea-template", "Fill-in idea template Andrew or Pulse fills; Lead turns it into a plan and Architect into a design doc.", "template", "draft",
 R + " (C9; roblox-games/game-idea-template.md is on Andrew's Mac)", "Template. The original game-idea-template.md is not in this repo; this is the studio's version. Reconcile when Andrew adds the original.", r"""
# Game idea template

- **Working title:**
- **One sentence:** what the player does and why it is fun.
- **Genre:** (link the playbook in 10-game-design/genre-playbooks)
- **Core verb:**
- **The twist** (what is new compared with the top game in this genre):
- **Comps** (games it is like, with dates and numbers):
- **First 10 seconds:**
- **Reason to come back tomorrow:**
- **Social hook:**
- **What we will not sell:**
- **Risks** (policy, scope, IP):
- **Size:** can a solo studio ship a first playable in two weeks?

The original `roblox-games/game-idea-template.md` (on Andrew's Mac) feeds this; it is not in the repo yet.
""")

note(f"{S}/pitch-template", "Pulse's pitch format: data sources with dates, candidates table with scores, top 3 ranked pitches with core verb, twist, comps, risks.", "template", "draft",
 "Pulse skill (vault/.claude/skills/pulse)", "Checked by scripts/gates/check_stage.py (pulse): data sources, ranked pitches, numbers sourced or UNVERIFIED.", r"""
# Pitch template (Pulse)

## Data sources
Each source with URL and the date read. Secondhand numbers (search summaries) are UNVERIFIED.

## Candidates
| Trend | Example games | Numbers (with date) | Demand 1-5 | Solo fit 1-5 | Retention 1-5 | Policy risk 1-5 (low good) | Difference from top game 1-5 |
|---|---|---|---|---|---|---|---|

## Ranked pitches (top 3, best first)
For each: one-sentence pitch, core verb, the twist, comps, risks, what Architect needs to know.

## Risks
## Open questions for Andrew
## Loaded
""")

note(f"{S}/agent-output-format", "Every agent's output: header, the work, Open questions and UNVERIFIED, Loaded section, plus gate.json in the exact required shape.", "template", "verified",
 "vault/.claude/skills/*; app/src-tauri/src/pipeline.rs (evaluate_stage)",
 "Verified 2026-11-09 against the studio code: pipeline.rs parses gate.json with this exact shape and check_stage.py requires the Loaded section (Phase 2 runs).", r"""
# Agent output format

## The output file (name set by the skill: pitch.md, plan.md, design.md, build.md, ...)
1. Title line.
2. The work, in the sections the skill lists.
3. **Open questions and UNVERIFIED** claims.
4. **Loaded:** each vault note read, one line each, with why.

## gate.json (exact shape)
```json
{"passed": false, "checks": [{"name": "rojo build", "passed": false, "detail": "NOT RUN: reason"}]}
```
- `passed` is true only if every check passed.
- A check that was not run is a failed check with "NOT RUN: reason".

## What the studio does with it
The studio does not take gate.json as the verdict. It also checks: the run finished, the output file exists, the agent loaded only its allowed vault sections and read the router first, required sections exist, and (Builder) the project really builds and passes tests. All of it goes to the review queue.
""")

note(f"{S}/mvp-build-order", "Build order for a new game (core loop first, monetization last) and the order the vault itself was filled.", "reference", "draft",
 R + " (C1 build order, C9 minimum viable vault)", "Order from the report; no facts to verify.", r"""
# MVP build order

## A new game
1. Core loop on a baseplate (the 5-second verb is fun).
2. First 60 seconds ([[10-game-design/first-60-seconds-checklist]]).
3. Progression.
4. Saving ([[20-luau-rojo/datastore-playbook]]).
5. UI ([[30-ui-ux/hud-template]]).
6. Analytics events ([[50-retention-analytics/event-taxonomy]]).
7. Monetization last ([[40-monetization/do-not-sell-list]]).
8. Playtest, then the review queue.

## The vault (minimum viable order, per the research)
1. Router, glossary, policy change log.
2. 20-luau-rojo. 3. 60-policy. 4. 40-monetization. 5. 10-game-design. 6. 30-ui-ux. 7. 50-retention-analytics. 8. 70-competitors. 9. 80-assets. 10. 90-templates-checklists.

New notes go in only after a successful, verified run ([[00-router/how-to-use-this-vault]]).
""")
print("90 done")
