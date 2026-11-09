# Build log

Running log for the Robux Studio internal agent tool. Newest phase at the bottom.
Every entry says what was done, how it was verified, and what is next.

Build environment: Claude Code cloud session (Linux container), branch
`claude/beautiful-carson-h75brk`. Target platform is macOS 12+; the app was
compiled and driven on Linux because that is what this session runs on.

## Inputs

- Research report: pasted in full into the build prompt. It is NOT in this repo
  as `agent-tool/vault-research.md` yet (see BLOCKED in the Phase 0 report).
- Roblox god plan (`business-plans/roblox-god-plan.md`), `PIPELINE.md`, and
  `roblox-games/sky-tower-obby/` live on Andrew's Mac under `~/workspace/`.
  They are not in this repo. Phase 4 needs them.

## Phase 0: scaffold

What was built
- `app/` Tauri 2 desktop app (Rust core + plain HTML/JS, no web build step).
  - Review queue screen (default tab). Items are JSON files in `queue/items/`.
    Approve/Reject buttons; a failed verification gate cannot be approved;
    every decision is appended to `queue/decisions.jsonl`. Approving never
    publishes anything.
  - Run tab: pick an agent, type an instruction, "Estimate cost" shows the
    expected cost, range, and hard cap before anything runs. "Approve and run"
    restates the cap; the backend refuses if the cap does not match.
  - Run history tab with estimate vs actual cost and a list of files each run
    loaded.
- `studio-cli`: headless twin of the app using the same Rust core and the same
  approval rules. Used for scripted verification.
- CLI bridge (`app/src-tauri/src/bridge.rs`): runs
  `claude -p --agent <name> --model <m> --output-format stream-json
  --max-budget-usd <approved cap> --permission-mode dontAsk --allowedTools ...`
  with the vault as working folder. Streams output to the window.
- Cost estimator (`cost.rs`) with prices in `studio/pricing.json`.
- Vault hook `vault/.claude/hooks/log-load.py`: logs every Read/Skill/Glob/Grep
  an agent makes to `runs/<id>/loads.jsonl`. This is the Phase 1 proof tool.
- `bridge-test` agent (`vault/.claude/agents/bridge-test.md`).

How it was verified
1. Probe runs before writing code (tiny Haiku runs, $0.0036 and $0.0010):
   confirmed `claude -p` works here, `--agent` loads project subagents, the
   PostToolUse hook fires and receives `agent_type`, and the CLI needs
   stdin closed. Also found that a subagent's `skills:` frontmatter does NOT
   put the skill text into context, so agents must load their skill
   explicitly (design note for Phase 1).
2. Cost formula: recomputed both probe runs from their token counts using the
   price table; both match the CLI's `total_cost_usd` to the cent (unit tests
   `usage_formula_matches_real_run_1/2`). This confirmed Claude Code bills
   cache writes at the 1-hour rate (2x input).
3. `cargo build`: no warnings. `cargo test`: 7/7 pass (queue flow, failed gate
   cannot be approved, no double decisions, path traversal refused, cost
   formula, estimate range, bridge args always carry the cap and dontAsk).
4. `studio-cli`: empty queue lists empty; `run` without approval stops after
   the estimate; `run` with a wrong cap is refused; a real approved run
   returned `BRIDGE_OK hello from phase zero` at $0.00072 actual vs $0.0014
   expected (range $0.0007 to $0.0035, cap $0.05).
5. GUI: `scripts/gui_smoke.py --phase0` drives the real app window with
   WebDriver under a virtual display: queue renders with the empty state,
   estimate card shows before running, approve-and-run streams output into
   the window and shows `BRIDGE_OK hello from the app window`, actual $0.0005.
   Screenshots: `docs/screenshots/01..03`. Evidence: `docs/evidence/phase0/`.

Not verified
- The macOS build. Tauri cannot be compiled for macOS from Linux. Andrew needs
  to run the app once on his Mac (steps in README.md).

## Phase 1: vault skeleton and router

What was built
- `vault/00-router/index.md`: the router (52 lines). Rules, a task-to-section
  table, and each agent's home section. Pointers only.
- `vault/00-router/`: how-to-use-this-vault (real content), glossary,
  policy-change-log, sources (stubs).
- One `index.md` per section 10 to 90, each a stub table naming the notes the
  section will hold (82 planned notes, from research report Part C).
- `vault/CLAUDE.md`: 8 lines, always loaded, points at the router.
- Skills `vault/.claude/skills/{lead,architect,builder}/SKILL.md`: job, what to
  load in order, steps, output file, quality bar (game psychology), and a
  verification gate written to `gate.json`. All are
  `disable-model-invocation: true`, so an agent cannot pull in another
  agent's skill on its own.
- Subagents `vault/.claude/agents/{lead,architect,builder}.md`.
- Bridge changes: the prompt is sent as `/<agent> <instruction>` so the agent's
  skill loads deterministically; the run folder is passed with
  `--append-system-prompt`; agents run with `--setting-sources project
  --strict-mcp-config` so they do not inherit plugins or MCP servers from the
  user's personal Claude Code setup.
- Vault write protection: deny rules in `vault/.claude/settings.json` for
  every section folder, `.claude/`, `.obsidian/`, and `CLAUDE.md`. Probe run
  confirmed a Write into a denied folder is blocked and a Write elsewhere goes
  through.
- `scripts/check_loads.py` (proves which sections a run loaded) and
  `scripts/check_vault.py` (structure checks).

How it was verified
1. Architect, asked for a design (beach lemonade tycoon): loaded
   `00-router/index.md` then `10-game-design/index.md`, nothing else.
   check_loads PASS. Cost $0.157 (estimate $0.25, cap $0.63).
2. Builder, asked for code (coin counter that saves): loaded
   `00-router/index.md` then `20-luau-rojo/index.md`, plus a listing of
   `20-luau-rojo/` and its own run folder. check_loads PASS. Cost $0.121
   (estimate $0.80, cap $2.02).
3. Negative test: Architect's log checked against a code-only allow list
   FAILS, so the checker does catch a wrong section.
4. Skill wiring: prompts went out as `/architect ...` and `/builder ...`
   (command.json). Outputs follow the skills exactly (file names, all
   required sections, gate.json shape).
5. `check_vault.py` PASS. `cargo test` 7/7.
Evidence: `docs/evidence/phase1/`.

Mistakes caught during verification
- The first Architect/Builder runs used a stale `studio-cli` binary
  (`cargo test` does not rebuild it), so the skill was never sent. Routing was
  correct but skill wiring was untested. Rebuilt and reran both; the evidence
  above is from the reruns.
- The load hook logged a Glob's pattern instead of its folder, which made two
  legitimate folder listings look like violations. Fixed the hook and checker.

Findings that shape Phase 2 and 3
- With stub notes, both agents said so and failed or flagged their gates
  instead of inventing. Builder followed the ProfileStore rule from its skill.
  The vault content (Phase 3) is what will let gates pass on substance.
- The first (skill-less) Architect run named its file freely and used a
  different gate.json shape. The pipeline runner must validate output file
  names and gate.json shape itself, not trust the agent.
- Builder's build gate cannot pass until rojo, selene, stylua, and lune are
  installed. Phase 2 installs them.

## Phase 2: agent team and review queue

What was built
- All 12 agents: `vault/.claude/agents/<name>.md` (subagent) plus
  `vault/.claude/skills/<name>/SKILL.md` (job, what to load in order, inputs,
  steps, output file, the game-psychology quality bar, and gate checks).
  Roster: `docs/AGENTS.md` (generated from config, so it cannot drift).
- `studio/agents.json`: per agent model, cost profile, tool allowlist,
  allowed vault sections, and required output file.
- Pipelines (`app/src-tauri/src/pipeline.rs`, `studio/pipelines.json`):
  `pitch-to-build` (Pulse, Lead, Architect, Builder) and `improve-build`
  (Architect, Builder, Playtest on an existing project, for Phase 4). One
  approval at the total cap; each stage runs under its own cap; a stage that
  writes nothing stops the pipeline; the result is one review queue item.
- Studio-side gates, so the queue does not trust an agent's word:
  - every stage: run finished, output file written, `gate.json` in the
    required shape and passed, loaded only its allowed vault sections and read
    the router first (from the hook log), has a Loaded section, no filler.
  - per agent: required sections (`scripts/gates/check_stage.py`).
  - Builder: the studio rebuilds the project itself
    (`scripts/gates/build_gate.py`: `rojo build`, Selene, StyLua, Lune tests).
- Free toolchain installed from crates.io: Rojo 7.7.1, Selene 0.31.0, StyLua
  2.6.0 (Luau), Lune 0.11.1. GitHub is blocked in this sandbox, so Selene's
  full Roblox std cannot download; the gate falls back to
  `scripts/gates/roblox-offline.yml` (Roblox globals, no API member checks)
  and says so in the check detail. On Andrew's Mac the full std will load.
- App: Run screen has a pipeline mode (pipeline, project, total estimate,
  approve all stages, live stage chips). `studio-cli pipeline ...` mirrors it.

How it was verified
1. Build gate on a hand-made good project: all 4 checks PASS. Same project
   with a lint error, bad formatting, and a wrong test: Selene, StyLua, and
   Lune all FAIL. So the gate catches real problems.
2. Acceptance run, `pitch-to-build`, instruction "Find a rising Roblox trend
   that a solo developer could make a small, original game for, and pitch
   three ideas." Estimate $1.54, cap $3.87, actual $1.52.
   - Pulse used web search, wrote 3 ranked original pitches (top: "Dawn
     Watch", co-op survive-the-night), every number marked UNVERIFIED
     (secondhand search summaries). Loaded 00-router, 70-competitors,
     10-game-design.
   - Lead wrote the plan (Architect then Builder, ends in the review queue).
     Loaded 00-router, 90-templates-checklists.
   - Architect wrote the Dawn Watch design doc. Loaded 00-router,
     10-game-design.
   - Builder built `projects/dawn-watch` (17 Luau files, 8 Lune specs).
     Loaded 00-router, 20-luau-rojo. Studio gate: rojo build PASS, Selene PASS
     (offline std), Lune 8/8 PASS, StyLua FAIL.
   - One queue item landed: `q-pipe-pitch-to-build-20261109-105542-ca9c`,
     status pending, gate failed (41 checks, 5 failed), 5 artifacts linked.
   - Nothing published: no publish, Open Cloud, or HttpService calls in the
     project; agents had no tool that could publish.
3. GUI (`scripts/gui_smoke.py --phase2`, real window via WebDriver): the
   pipeline item renders with all 41 checks and 5 artifacts; a failed-gate
   item's Approve button is disabled; approving a passing fixture in the
   window recorded `approved by Andrew` with the note; the pipeline estimate
   screen shows the $3.87 total cap and cancels without running. Fixtures are
   removed afterwards. Screenshots 04 and 05.
4. Negative tests: pipeline with a wrong cap is refused and all 4 stages end
   `cancelled`; `improve-build` without a project is refused; a project path
   of `../etc` is refused.
5. `cargo test` 9/9.
Evidence: `docs/evidence/phase2/`.

Problems found and fixed
- Builder could not run its tools: it tried one compound shell loop, which
  does not match the per-tool allow rules, was denied, and gave up (its gate
  said NOT RUN, which was honest). Fix: Builder may now run the studio's own
  gate script as one command, and its skill says one tool per command. Rerun
  on `projects/dawn-watch` (cost $0.07): Builder ran the gate, ran StyLua,
  reran the gate; studio gate now 4/4 PASS. Evidence in
  `docs/evidence/phase2/builder-fix-rerun/`.

Why every stage's self-gate failed (expected at this phase)
- Pulse: required vault notes are stubs; live chart numbers not verified at
  the source.
- Lead: could not verify toolchain claims (no notes yet).
- Architect: could not check the do-not-sell list because that note is a stub.
- Builder: tools not runnable (fixed above).
These are the gates working: with an empty vault the agents refuse to claim
what they cannot back up. Phase 3 fills the vault.

Open item for Andrew
- The Dawn Watch item in the review queue should be rejected (its gate
  failed). It is test output from the acceptance run, not a game to ship.
