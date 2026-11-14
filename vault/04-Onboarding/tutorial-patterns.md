---
description: Tutorial patterns for Roblox games that teach by doing: one verb at a time, contextual prompts, no text walls, fade-out hints, and skip paths. Use when designing the first 2 minutes.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: tutorial, learn by doing, contextual prompt, text wall, onboarding, first session, hints, skip, guided quest, arrow, beam, mobile
sources: https://create.roblox.com/docs/production/game-design/onboarding (not opened this session, principles from model knowledge); vault note 04-Onboarding/onboarding-flow, consulted 2026-11-14
verify: Principles are standard game-design practice and consistent with existing vault notes. All timings are heuristics, UNVERIFIED against Roblox data. Playtest with stopwatch.
tags: [onboarding, playbook]
---
# Tutorial patterns (learn by doing)

Roblox players skip text and arrive on phones. Teach with action, not reading. Extends [[04-Onboarding/onboarding-flow]] and [[04-Onboarding/first-60-seconds-checklist]].

## Do this
- Teach one verb per step (move, collect, jump, buy, equip). Never introduce two new verbs in the same prompt.
- Max 8 words per prompt. If it needs more, the mechanic is too complex or the level should demonstrate it.
- Show the prompt at the object (BillboardGui, beam, arrow, highlight), not in a modal. Remove it the moment the player does the action.
- Step completes on the action, not on a "Next" button. Reward every step with sound plus a number change within 0.3 s.
- Gate the next verb behind a safe space: first failure costs nothing (no death penalty, no currency loss) for the first 60 to 90 seconds.
- Fade hints by mastery: after the player has done the action 2 to 3 times unprompted, stop showing the prompt (UNVERIFIED number, tune by playtest).
- Offer no skip button if the tutorial is under 90 seconds and is the game. Offer skip only for tutorials longer than that, and persist the "done" flag in DataStore so returning players never see it again.
- Total tutorial under 2 minutes of required input; everything else is optional quests in the world.

## Patterns that work
| Pattern | What it is | Use when |
|---|---|---|
| Level-as-tutorial | Layout forces the verb (gap forces jump) | Obby, platformer |
| Single glowing target | One highlighted object, everything else dimmed or locked | Simulator, tycoon |
| Guided first quest | 3 to 5 steps, each 10 to 20 s, ending in a visible reward | Any game with menus |
| Show then do | Short 3 s animation of the action, then player repeats | Combat, building |
| Fake-easy first enemy | Enemy cannot kill the player yet | Combat, survival |
| NPC line, one sentence | Speech bubble, auto-advances in 3 s, never blocks input | Story-lite games |

## Anti-patterns
- Wall of text, multi-page "How to play" panel, or a video.
- Unskippable cutscene before the first input (see [[04-Onboarding/onboarding-mistakes]]).
- Locking the camera or movement while a prompt is open.
- Teaching the shop, pets, trading and rebirth in the first session. Reveal systems when the player has a reason to need them (see [[05-Progression/goal-gradient-and-session-goals]]).

## How to build it in Roblox
- A small state machine in a ModuleScript on the server: step index per player, saved in the player's profile. Client only renders the prompt.
- Prompts: BillboardGui with AlwaysOnTop for world labels, Beam between attachments for a path, Highlight instance for the target.
- Use ProximityPrompt only when the interaction is a press; for walking, trigger on a Touched part or a distance check.
- Track each step in analytics (see [[06-Retention/event-taxonomy]]): step_started, step_completed, with time. A step where more than 20 percent of players quit is broken (heuristic, UNVERIFIED).
- Mobile first: prompts at least 44 px tall, no keyboard-only wording ("tap", not "press E"); read the platform with UserInputService and switch the text. See [[08-UI-UX/mobile-first-checklist]].

## How to test
- Hand the game to someone who has never seen it, say nothing, time to first reward and time to first quit.
- Count text on screen: any frame with more than 2 lines is a flag.
- Watch whether players read the prompt or just act. If they act without reading, the world is teaching well; delete the prompt.

## Worked example (generic obby-simulator hybrid)
1. Spawn on a pad facing a glowing button. Prompt: "Tap to jump". Step done on the first jump, coin sound.
2. A single gap. Prompt fades after the first success. No text if the player jumps unprompted within 5 s.
3. First checkpoint flag with a sparkle burst. Prompt: "Checkpoint saved". Teaches the safety net.
4. A coin trail leads to the first shop-free upgrade. Prompt: "Grab it". Reward: speed boost for 30 s.
5. Door opens to the main zone. No more prompts. Optional quest board appears later.
Total required reading: 14 words. Total time: about 60 to 90 s.

## Questions to ask each step
- Can the player fail here without losing anything?
- Is the reward visible before the player finishes reading?
- If the player ignores the prompt, does the world still guide them?
- Does this step work with touch only, with a gamepad, and with keyboard?
- Would a 9-year-old and a 15-year-old both understand it with no reading? (Roblox audience is young and international, so skip idioms.)

## Localization note
Keep prompt strings in one table so they can be run through Roblox's built-in localization later (zero cost). Prefer icons and arrows over words.

## For our Roblox games
Copy the principle: one verb, one prompt, action completes the step, reward instantly, fade hints. Never copy another game's tutorial text, NPC names or characters, art, music or logos; write original prompts and build original guide characters.
