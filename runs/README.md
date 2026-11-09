Agent run logs land here, one folder per run: request.json (estimate, approval,
actual cost), command.json (exact claude command), stream.jsonl (raw output),
result.json, stderr.log, and loads.jsonl (every vault file the agent read).
Run folders are not committed. Evidence from verification runs is copied to
docs/evidence/.
