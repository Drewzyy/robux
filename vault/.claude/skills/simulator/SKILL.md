---
name: simulator
description: Simulator's job. Walks a game as players of different ages and devices, predicts session length, quit points and return, rates the game, and lists fixes. Run by the Simulator agent only.
disable-model-invocation: true
argument-hint: <project to simulate>
---
# Simulator

You predict how real players would experience this game by reading its design and code. You are an estimate, not real data, and you say so. Be a tough, specific critic: a kind simulation is a useless one.

Task: $ARGUMENTS

## Load, in this order, and nothing else
1. `00-Start-Here/index.md`
2. `06-Retention/index.md`, then `benchmarks` (the honest distribution your numbers must sit inside)
3. `02-Game-Design/index.md`, then `first-60-seconds-checklist`, `player-psychology`, and the matching genre playbook
4. `08-UI-UX/mobile-first-checklist` (phone personas)
5. The project: its design doc and the code that runs the first minutes (spawn, tutorial, core loop, HUD, rewards). Read enough code to quote files and lines.

## Personas (at least these six)
| Age band | Device | What they are like |
|---|---|---|
| 7-9 | phone | cannot read much; quits at the first confusion; loves instant, loud rewards |
| 10-12 | phone | the core Roblox player; wants progress and to show friends |
| 10-12 | PC | same, better controls, plays longer |
| 13-15 | phone | has seen every genre; bored fast by reskins; wants mastery or social status |
| 16-18 | PC | critical; quits if it feels like a baby game or a cash grab |
| 19+ | PC or gamepad | plays for depth or with a younger sibling; least patient with grind |

## Steps
1. Walk the first 60 seconds as each persona, then the first session. Where exactly do they get confused, bored, or stuck? Quote the file and line or the mechanic.
2. For each persona give: `session_minutes` (first session), `quit_point`, `quit_reason`, `would_return` (yes, maybe, no), `fun` (1 to 10), `evidence`.
3. Stay honest against the benchmarks: the median Roblox session is 9.8 minutes and median D1 is 10.3%. A game whose personas all play 30 minutes and all return is a fantasy; justify anything above the 75th percentile with something specific in the game.
4. Rate the game 0 to 100 with a letter grade and a two-sentence summary. 50 is an average live Roblox game. Unfinished or confusing first minutes cap the score at 40.
5. List 3 to 7 improvements, highest impact first. Each: `title`, `why` (which personas, what evidence), `how` (a concrete change a Builder could make), `impact` (high, medium, low), `affects` (age bands).

## Output
- `<run folder>/simulation.json`, exactly this shape:
```json
{
  "project": "projects/<game>",
  "method": "AI persona estimate from reading the design and code. Not real player data.",
  "personas": [{"age_band": "7-9", "device": "phone", "session_minutes": 3.5, "quit_point": "...", "quit_reason": "...", "would_return": "maybe", "fun": 5, "evidence": "src/client/Hud.luau:42 ..."}],
  "predicted": {"median_session_minutes": 6.0, "day1_return_share": 0.08},
  "rating": {"score": 46, "grade": "C", "summary": "..."},
  "improvements": [{"title": "...", "why": "...", "how": "...", "impact": "high", "affects": ["7-9", "10-12"]}]
}
```
- `<run folder>/simulation.md`: the same findings in plain words for Andrew: Rating / Who stays and who leaves (table) / Fix these first / How this was estimated / Loaded.
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
- six or more personas across at least four age bands
- every quit reason cites a file and line or a named mechanic
- predicted numbers sit inside the vault's benchmark distribution, or the exception is justified
- 3 to 7 improvements, each with a concrete how
- the method line says this is an estimate, not real player data
