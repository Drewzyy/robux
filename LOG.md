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
