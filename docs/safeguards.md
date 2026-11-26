# Safeguards: every failure and loop seen so far, and what stops it

Keep all of these when changing the app. Each one exists because it really happened.

| Problem we saw | What stops it now | Where |
|---|---|---|
| The same job retried, rescued and re-queued all day | Loop guard: the same job 8 times, the same work by the same agent 6 times in 24 h, or more than 40 jobs an hour for one game, is refused and the job is closed as skipped | autopilot.rs `loop_guard` (tested) |
| Lead rescuing Lead, rescues stacking | One rescue at a time per game; a rescue re-queues its job once; at most 2 rescues, then skip | autopilot.rs `rescue_done`, `MAX_RESCUES` |
| Status messages ("is it going?") run as work | Never queued, never started; old ones closed | `is_ping`, `changes::take_next`, `tidy` |
| Two workers on the same thing | Duplicates cancelled before start and every Warden round | `changes::cancel_duplicates` (tested) |
| Code audits 30 times with nothing to fix | Only after code changed, at most every 12 h, one try, no rescue | `code_audit`, `side_work` |
| Side work (audits, Warden checks) holding a step open | Never counts toward finishing a step | `side_work` |
| Andrew's notes failing over and over | One try, no rescue | on_finish, `/note/` keys |
| Lead's "decide the next work" failing every round | Backs off 6 h after two failures | `warden_lead` |
| Agent's own checklist failing good work ("game reaches 80+") | Advisory only; the app's checks decide | pipeline.rs `evaluate_stage` |
| Missing gate.json failing good work | Passes when the real output exists | pipeline.rs |
| Lint warnings and formatting failing jobs | Code is formatted automatically; only lint errors fail | gates.rs |
| Vault check failing on Windows paths and skills | Backslashes normalised; skills allowed | pipeline.rs `loads_within` |
| "Did not start X: 6 jobs started by agents" forever | Cap counts only open jobs | on_finish requests |
| Score bar (80) on steps with no code | Plan, design, UI and world go ahead on their own; the bar applies from build on | `advance_inner` |
| Improve rounds every minute | At most one an hour, plus the loop guard | `advance_inner` |
| Questions for Andrew nobody could answer | Agents decide and list "Decisions made" | `prompt` |
| Agents padding runs with re-checks | Stay-on-track rules in every job and in vault/CLAUDE.md | `ON_TRACK` |
| Jobs "working" with no agent behind them | Warden fails them so they retry | warden.rs |
| "6 jobs about to start" for 30 min | Retry timers cleared on open; Warden revives stale jobs | `wake_all`, warden.rs |
| Local agents had no Studio or Blender tools | Tools attached for every engine | bridge.rs |
| Playtest on an empty Baseplate | The game is built and opened in Studio first | bridge.rs |
| Local model never got the task | Short prompts, a bigger context copy, lost-task detection | `idea_for_prompt`, ollama.rs, bridge.rs |
| PC unusable (memory, CPU, disk) | 24k context on 16 GB, half the cores, model unloaded after jobs, ffmpeg one at a time on 2 cores, low priority | ollama.rs, video.rs, paths.rs |
| Discord spam | Pings only for reviews at 80+, final publish, and Claude signed out | autopilot.rs notify calls |
| A step never finishing although every listed job was Done (a leftover failed improve job or rescue) | A step finishes when its own planned jobs are done; failed extra work never holds it | autopilot.rs `planned_key` |
| The hourly job ceiling delaying a step's real work | A step's planned jobs are exempt from the ceiling | `loop_guard` |
| "Nothing to do:" runs failing for a missing output file | They count as a normal finish | pipeline.rs `evaluate_stage` |
| A step stuck on "Starting the next step" with every job Done | Only planned jobs, or work actually running, hold a step; waiting improve rounds, rescues, paused, delayed or 6-hour-old extras never do (tested) | `advance_inner` `holds` |
| Waiting for Claude's usage to reset | Switches to the local model within about 2 minutes of hitting the limit (paused or failed on the limit), and back when it resets | main.rs `auto_local` |
