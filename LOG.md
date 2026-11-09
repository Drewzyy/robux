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
