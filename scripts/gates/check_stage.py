#!/usr/bin/env python3
"""Studio-side checks on one agent's output. The agent's own gate.json is not
trusted here; this script reads the output file and, for Builder, rebuilds the
project itself.

  python3 scripts/gates/check_stage.py <agent> <output-file> [--project projects/<game>]

Prints a JSON list of {"name", "passed", "detail"}.
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_gate  # noqa: E402


def has(text, pattern):
    return re.search(pattern, text, re.I | re.M) is not None


def c(name, ok, detail=""):
    return {"name": name, "passed": bool(ok), "detail": detail}


def heading(text, words):
    return has(text, r"^#{1,4}\s*(\d+[.)]\s*)?.*(" + words + r")")


def common(text):
    out = [c("has a Loaded section listing vault notes read", has(text, r"^#{1,4}\s*(\d+[.)]\s*)?Loaded|^\*\*Loaded|^Loaded:"))]
    filler = [p for p in ["lorem ipsum", "as an ai", "in today's fast-paced", "game-changer", "unleash", "delve into"] if p in text.lower()]
    out.append(c("no filler phrases", not filler, ", ".join(filler) or "none found"))
    return out


def project_from(text):
    m = re.search(r"^\s*Project:\s*`?(projects/[A-Za-z0-9_-]+|none)`?", text, re.M)
    return m.group(1) if m else None


def check(agent, text, project):
    r = common(text)
    if agent == "pulse":
        r.append(c("names its data sources", heading(text, "data sources|sources")))
        r.append(c("ranked pitches present", heading(text, "rank")))
        r.append(c("numbers are sourced or marked UNVERIFIED", has(text, r"https?://") or "UNVERIFIED" in text))
    elif agent == "lead":
        rows = [l for l in text.splitlines() if l.strip().startswith("|")]
        r.append(c("plan table with stages", len(rows) >= 4, f"{len(rows)} table rows"))
        r.append(c("has a Blocked section", heading(text, "blocked")))
        r.append(c("ends in the review queue", has(text, r"review queue")))
    elif agent == "architect":
        r.append(c("core loop section", heading(text, "core loop")))
        r.append(c("first 60 seconds section", heading(text, "first 60")))
        windows = all(has(text, w) for w in [r"0\s*[-–]\s*5", r"5\s*[-–]\s*10", r"10\s*[-–]\s*30", r"30\s*[-–]\s*(45|60)"])
        r.append(c("first 60 seconds covers 0-5, 5-10, 10-30, 30-60", windows))
        r.append(c("says what we will not do", heading(text, "not do|will not|won't")))
    elif agent == "builder":
        declared = project or project_from(text)
        r.append(c("declares its project folder", declared is not None, declared or "no 'Project:' line"))
        if declared and declared != "none":
            pdir = ROOT / declared
            r.append(c("project folder exists", pdir.is_dir(), declared))
            if pdir.is_dir():
                n = len(list((pdir / "src").rglob("*.luau"))) if (pdir / "src").is_dir() else 0
                r.append(c("project has Luau source", n > 0, f"{n} .luau files under src/"))
                r.extend(build_gate.gate(Path(declared)))
        elif declared == "none":
            r.append(c("produced a buildable project", False, "Project: none (snippet only)"))
    elif agent == "playtest":
        r.append(c("states how it tested", heading(text, "how tested|method")))
        r.append(c("has a verdict", heading(text, "verdict")))
        r.append(c("problems have severities", has(text, r"\b(blocker|major|minor)\b") or has(text, r"no issue found")))
        live = has(text, r"played live|live in studio|studio mcp")
        r.append(c("does not claim live play without saying how", True, "claims live play" if live else "headless or code walk"))
    elif agent == "merchant":
        r.append(c("catalog present", heading(text, "catalog")))
        r.append(c("do-not-sell check present", heading(text, "do-not-sell|do not sell")))
    elif agent == "frame":
        r.append(c("mobile checklist results present", has(text, r"mobile|phone")))
    elif agent == "splash":
        r.append(c("icon and thumbnail briefs present", heading(text, "icon") and heading(text, "thumbnail")))
    elif agent == "liveops":
        r.append(c("numbers come with dates", has(text, r"\d{4}-\d{2}-\d{2}") or "BLOCKED" in text.upper()))
    elif agent == "producer":
        r.append(c("one named reason", heading(text, "reason")))
    return r


def main():
    a = sys.argv[1:]
    if len(a) < 2:
        print(__doc__)
        sys.exit(2)
    agent, out = a[0], Path(a[1])
    project = a[a.index("--project") + 1] if "--project" in a else None
    text = out.read_text(encoding="utf-8") if out.is_file() else ""
    if not text:
        print(json.dumps([c("output readable", False, f"{out.name} missing")]))
        return
    print(json.dumps(check(agent, text, project)))


if __name__ == "__main__":
    main()
