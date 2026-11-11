---
name: crowd
description: Crowd's job. Designs and builds multiplayer play, social features, and NPCs or bots, with the server owning every rule. Run by the Crowd agent only.
disable-model-invocation: true
argument-hint: <task>
---
# Crowd (NPC and Multiplayer Specialist)

Designs and builds multiplayer play, social features, and NPCs or bots, with the server owning every rule.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `12-Multiplayer-NPCs/index.md`, then `multiplayer-patterns`, `npc-ai`
3. `13-Luau-Engineering/remotes-and-security` before adding any remote

Use `brewix search <words> --agent crowd` to find a specific note fast. If a note you need is a stub, write "STUB: <note>" in your output and fall back to this skill's rules.

When the task touches these topics, read the note's Do this block first (stop there if it answers you): `03-Game-Psychology/play-with-friends`, `13-Luau-Engineering/networking-hardening`.

## Steps
1. Read the design. Decide what players do together and what one player alone does.
2. Write server-authoritative code: state on the server, remotes validated for type, range, rate, and permission.
3. NPCs or bots: a state machine module, pathfinding recomputed on events or a few times a second, low tick rate, slept when far from players. Bots are labelled as bots.
4. Run `brewix gate build projects/<game>`.

## Output
`<run folder>/multiplayer.md` with the sections your checks name, and any code changes in the project.
End every output with a Loaded section: each vault note you read, one line each, with why.

## Quality bar (game psychology, from 03-Game-Psychology/player-psychology)
- Time to first fun under 10 seconds; teach by doing, never a wall of text.
- Fixed reward schedules for progression, variable only for surprise. Never only random.
- Sawtooth difficulty with rest beats; retry under 2 seconds; never punish trying.
- A full loop inside one median session (9.8 min) plus a reason to return tomorrow.
- Social is the cheapest retention lever: friends, teams, trading, leaderboards.
- Roblox's order: D1 and session time first, then D7/D30, then payers, then acquisition.
- Anti-slop: if your output could describe any game, it is generic. Name this game's specifics or do not ship it.

## Gate
Write `<run folder>/gate.json` exactly as `{"passed": bool, "checks": [{"name": str, "passed": bool, "detail": str}]}`. `passed` is true only if every check passed. A check you could not run is a failed check with detail "NOT RUN: reason".
Checks:
- has a Server authority section listing what the server owns
- every new remote is validated (type, range, rate, permission)
- the game works with one player (solo goal or bots) and says how
- NPC logic runs at a low tick rate, not every frame
- the build gate passes after the change
