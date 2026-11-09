#!/usr/bin/env python3
"""Check which vault sections an agent run loaded.

  python3 scripts/check_loads.py <run_id> --allow 00-router 10-game-design

Reads runs/<run_id>/loads.jsonl (written by vault/.claude/hooks/log-load.py)
and fails if the run read any vault section not in --allow, or searched the
vault root with Glob/Grep. The agent's own skill folder, its run folder, and
projects/ are always allowed.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VAULT = ROOT / "vault"
SECTIONS = [
    "00-router", "10-game-design", "20-luau-rojo", "30-ui-ux", "40-monetization",
    "50-retention-analytics", "60-policy", "70-competitors", "80-assets",
    "90-templates-checklists",
]


def classify(target: str, run_id: str, agent: str):
    """Return (kind, section_or_detail)."""
    if not target:
        return "other", "(empty)"
    p = Path(target)
    if p.is_absolute():
        try:
            rel = p.resolve().relative_to(VAULT.resolve())
            target = str(rel)
        except ValueError:
            try:
                rel = p.resolve().relative_to(ROOT.resolve())
            except ValueError:
                return "outside", target
            parts = rel.parts
            if parts[:2] == ("runs", run_id):
                return "run-folder", str(rel)
            if parts and parts[0] == "projects":
                return "projects", str(rel)
            if parts and parts[0] == "runs":
                return "other-run", str(rel)
            return "repo", str(rel)
    first = Path(target).parts[0] if Path(target).parts else ""
    if first in SECTIONS:
        return "section", first
    if target.startswith(f".claude/skills/{agent}") or target == agent:
        return "own-skill", target
    if target in ("CLAUDE.md",):
        return "vault-claude-md", target
    return "vault-other", target


def main() -> int:
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        return 2
    run_id = args[0]
    allow = set(args[args.index("--allow") + 1 :]) if "--allow" in args else set()
    run_dir = ROOT / "runs" / run_id
    req = json.loads((run_dir / "request.json").read_text())
    agent = req["agent"]
    log = run_dir / "loads.jsonl"
    rows = [json.loads(l) for l in log.read_text().splitlines()] if log.exists() else []
    violations = []
    sections_seen = []
    print(f"run {run_id}  agent {agent}  status {req['status']}  cost {req.get('actual_cost_usd')}")
    print(f"allowed sections: {sorted(allow)}")
    for r in rows:
        tool, target = r.get("tool"), r.get("target") or ""
        inp = r.get("input") or {}
        if tool in ("Glob", "Grep"):
            # Classify a search by the folder searched (older logs stored the pattern).
            target = inp.get("path") or "(vault root)"
            pat = str(inp.get("pattern", ""))
            if not inp.get("path") and pat.startswith("/") and not pat.startswith(str(VAULT) + "/"):
                target = re.split(r"[*?{\[]", pat)[0].rstrip("/")
        flag = "ok"
        if tool in ("Glob", "Grep") and target in ("(vault root)", ".", str(VAULT)):
            pat = str(inp.get("pattern", ""))
            if pat.startswith(str(VAULT) + "/"):
                pat = pat[len(str(VAULT)) + 1:]
            first = pat.split("/")[0]
            kind, what = ("section", first) if first in SECTIONS else ("vault-root-search", target)
            if first not in allow:
                flag = "VIOLATION: search over the vault root"
        else:
            kind, what = classify(target, run_id, agent)
        if kind == "section":
            if what not in sections_seen:
                sections_seen.append(what)
            if what not in allow:
                flag = f"VIOLATION: section {what} not allowed"
        elif kind in ("outside", "other-run", "vault-other", "repo"):
            flag = f"VIOLATION: {kind}"
        if flag != "ok":
            violations.append((tool, target, flag))
        print(f"  {tool:6} {kind:12} {target}  [{flag}]")
    print(f"sections loaded, in order: {sections_seen}")
    if not rows:
        print("FAIL: nothing was logged")
        return 1
    if violations:
        print(f"FAIL: {len(violations)} violation(s)")
        return 1
    print("PASS: only allowed sections were loaded")
    return 0


if __name__ == "__main__":
    sys.exit(main())
