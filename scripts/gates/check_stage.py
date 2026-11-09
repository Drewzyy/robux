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


def svg_words(svg):
    words = []
    for m in re.finditer(r"<text\b[^>]*>(.*?)</text>", svg, re.S):
        inner = re.sub(r"<[^>]+>", " ", m.group(1))
        words += [w for w in re.split(r"\s+", inner) if w.strip()]
    return words


def svg_colors(svg):
    cols = set()
    for v in re.findall(r"(?:fill|stroke|stop-color)\s*[:=]\s*\"?(#[0-9a-fA-F]{3,6})", svg):
        v = v.lower()
        if len(v) == 4:
            v = "#" + "".join(ch * 2 for ch in v[1:])
        r_, g_, b_ = (int(v[i:i + 2], 16) for i in (1, 3, 5))
        if max(r_, g_, b_) - min(r_, g_, b_) < 24:  # black, white, and greys are not palette colours
            continue
        cols.add(v)
    return cols


def splash_checks(text, out_path):
    r = []
    tdir = out_path.parent / "thumbnails"
    thumbs = sorted(p for p in tdir.glob("t*.svg")) if tdir.is_dir() else []
    icon = tdir / "icon.svg"
    r.append(c("10 to 20 thumbnail comps", 10 <= len(thumbs) <= 20, f"{len(thumbs)} found in thumbnails/"))
    bad_size, bad_words, bad_cols, real = [], [], [], 0
    for t in thumbs:
        svg = t.read_text(encoding="utf-8", errors="replace")
        if not re.search(r'width="1920"', svg) or not re.search(r'height="1080"', svg):
            bad_size.append(t.name)
        if len(svg_words(svg)) > 3:
            bad_words.append(f"{t.name} ({len(svg_words(svg))} words)")
        if len(svg_colors(svg)) > 3:
            bad_cols.append(f"{t.name} ({len(svg_colors(svg))} colours)")
        for href in re.findall(r'href="([^"]+\.(?:png|jpe?g))"', svg):
            if not href.startswith(("http", "data:")) and (t.parent / href).resolve().is_file():
                real += 1
                break
    r.append(c("thumbnails are 1920x1080", thumbs and not bad_size, ", ".join(bad_size) or "all"))
    r.append(c("0 to 3 words of text each", thumbs and not bad_words, ", ".join(bad_words) or "all within limit"))
    r.append(c("max 3 saturated colours each", thumbs and not bad_cols, ", ".join(bad_cols) or "all within limit"))
    isvg = icon.read_text(encoding="utf-8", errors="replace") if icon.is_file() else ""
    r.append(c("512x512 icon", 'width="512"' in isvg and 'height="512"' in isvg, "thumbnails/icon.svg" if isvg else "missing"))
    r.append(c("uses the game's own visuals (real captures embedded)", thumbs and real == len(thumbs), f"{real}/{len(thumbs)} embed a capture that exists"))
    r.append(c("ranking names a launch pick", heading(text, "rank") and has(text, r"launch")))
    return r


def check(agent, text, project, out_path=None):
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
        r.extend(splash_checks(text, Path(out_path)))
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
    print(json.dumps(check(agent, text, project, out)))


if __name__ == "__main__":
    main()
