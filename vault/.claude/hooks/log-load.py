#!/usr/bin/env python3
"""PostToolUse hook: log every file or search an agent touches.

Writes one JSON line per Read/Skill/Glob/Grep call to
$ROBUX_RUN_DIR/loads.jsonl when the run was started by BrewixGames,
otherwise to <vault>/.claude/logs/loads.jsonl. This is how we prove which
parts of the vault each agent loaded. It never blocks the tool call.
"""
import json
import os
import sys
import time


def main() -> None:
    try:
        data = json.load(sys.stdin)
    except Exception:
        return
    tool = data.get("tool_name", "")
    tin = data.get("tool_input") or {}
    if tool in ("Glob", "Grep"):
        # The folder searched is what matters; the pattern is kept in "input".
        target = tin.get("path") or "(vault root)"
    else:
        target = tin.get("file_path") or tin.get("skill") or tin.get("path") or ""
    vault = os.environ.get("CLAUDE_PROJECT_DIR") or data.get("cwd") or ""
    rel = target
    if vault and isinstance(target, str) and target.startswith(vault.rstrip("/") + "/"):
        rel = target[len(vault.rstrip("/")) + 1 :]
    record = {
        "ts": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "run_id": os.environ.get("ROBUX_RUN_ID"),
        "agent": os.environ.get("ROBUX_AGENT") or data.get("agent_type"),
        "subagent": data.get("agent_type"),
        "tool": tool,
        "target": rel,
        "input": tin if tool in ("Grep", "Glob") else None,
    }
    run_dir = os.environ.get("ROBUX_RUN_DIR")
    out_dir = run_dir if run_dir else os.path.join(vault or ".", ".claude", "logs")
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, "loads.jsonl"), "a", encoding="utf-8") as f:
        f.write(json.dumps(record) + "\n")


if __name__ == "__main__":
    main()
