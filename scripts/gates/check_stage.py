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


def table_col(text, col):
    rows = [[c.strip() for c in l.strip().strip("|").split("|")] for l in text.splitlines() if l.strip().startswith("|")]
    for i, r in enumerate(rows):
        k = next((j for j, c in enumerate(r) if col.lower() in c.lower()), None)
        if k is not None:
            out = []
            for x in rows[i + 1:]:
                if len(x) <= k:
                    break
                if x[0].startswith("--") or x[0].startswith(":-"):
                    continue
                out.append(x[k])
            return out
    return []


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


def sounds_checks(text, project):
    """Validate projects/<game>/sounds.json (the ForgeGUI request sheet)."""
    proj = project or (re.search(r"projects/[A-Za-z0-9_-]+", text) or [None])[0]
    if not proj:
        return [c("sounds.json found", False, "no project named in the output")]
    f = ROOT / proj / "sounds.json"
    if not f.is_file():
        return [c("sounds.json found", False, f"{proj}/sounds.json missing")]
    try:
        data = json.loads(f.read_text())
    except Exception as e:
        return [c("sounds.json parses", False, str(e)[:120])]
    sounds = data.get("sounds") if isinstance(data, dict) else None
    out = [c("sounds.json parses with a sounds list", isinstance(sounds, list) and len(sounds) > 0, f"{len(sounds or [])} sounds")]
    if not isinstance(sounds, list):
        return out
    ids = [s.get("id") for s in sounds if isinstance(s, dict)]
    bad_shape = [str(s.get("id")) for s in sounds if not (isinstance(s, dict) and s.get("id") and s.get("event") and s.get("prompt"))]
    bad_id = [str(s.get("id")) for s in sounds if isinstance(s, dict) and s.get("asset_id") is not None and not re.fullmatch(r"rbxassetid://\d+", str(s.get("asset_id")))]
    dup = sorted({i for i in ids if ids.count(i) > 1})
    out.append(c("every sound has id, event, prompt", not bad_shape, ", ".join(bad_shape) or "all"))
    out.append(c("asset ids are null or rbxassetid://<number>", not bad_id, ", ".join(bad_id) or "all valid"))
    out.append(c("sound ids are unique", not dup, ", ".join(dup) or "unique"))
    out.append(c("first 60 seconds has sounds requested", any(s.get("first_60_seconds") for s in sounds if isinstance(s, dict))))
    return out


def simulation_checks(out_path):
    f = out_path.parent / "simulation.json"
    if not f.is_file():
        return [c("simulation.json written", False, "missing")]
    try:
        v = json.loads(f.read_text())
    except Exception as e:
        return [c("simulation.json parses", False, str(e)[:120])]
    s = lambda x, k: x.get(k) if isinstance(x.get(k), str) else ""
    def n(x, k):
        y = x.get(k)
        return float(y) if isinstance(y, (int, float)) and not isinstance(y, bool) else None
    personas = v.get("personas") if isinstance(v.get("personas"), list) else []
    bands = sorted({s(p, "age_band") for p in personas if s(p, "age_band")})
    complete = all(
        n(p, "session_minutes") is not None and 0 <= n(p, "session_minutes") <= 180
        and n(p, "fun") is not None and 1 <= n(p, "fun") <= 10
        and s(p, "quit_reason") and s(p, "evidence")
        and s(p, "would_return") in ("yes", "maybe", "no")
        for p in personas)
    rating = v.get("rating") if isinstance(v.get("rating"), dict) else {}
    score = n(rating, "score")
    imps = v.get("improvements") if isinstance(v.get("improvements"), list) else []
    imps_ok = len(imps) >= 3 and all(s(i, "title") and s(i, "how") and s(i, "why") for i in imps)
    method = s(v, "method").lower()
    return [
        c("simulation.json parses", True, f"{len(personas)} personas"),
        c("at least 5 personas across 4+ age bands", len(personas) >= 5 and len(bands) >= 4, ", ".join(bands)),
        c("every persona has minutes, fun, quit reason, evidence, return answer", bool(personas) and complete),
        c("rating between 0 and 100", score is not None and 0 <= score <= 100, "" if score is None else str(score)),
        c("3 or more improvements with why and how", imps_ok, f"{len(imps)} improvements"),
        c("says it is an estimate, not real player data", "not real" in method or "estimate" in method),
    ]


def pitches_checks(out_path):
    f = out_path.parent / "pitches.json"
    try:
        v = json.loads(f.read_text())
    except Exception as e:
        return [c("pitches.json parses", False, str(e)[:120])]
    arr = lambda k: v.get(k) if isinstance(v.get(k), list) else []
    s = lambda x, k: x.get(k).strip() if isinstance(x.get(k), str) else ""
    sourced = sum(1 for x in arr("sources") if s(x, "url").startswith("http"))
    pitches = arr("pitches")
    fields = ["title", "one_liner", "trend", "core_verb", "twist", "why_now", "first_60_seconds", "originality"]
    incomplete = [s(p, "title") for p in pitches if any(not s(p, f) for f in fields) or s(p, "policy_risk") not in ("low", "medium", "high")]
    n = v.get("top_games_studied")
    status = s(v, "data_status")
    n_ok = isinstance(n, int) and not isinstance(n, bool) and n >= 0
    status_ok = n_ok and ((status == "live" and n > 0) or status == "partial" or (status == "none" and n == 0))
    return [
        c("pitches.json parses", True, f"{len(pitches)} pitches"),
        c("3 or more sources with links", sourced >= 3, f"{sourced} with links"),
        c("3 or more current trends", len(arr("trends")) >= 3, f"{len(arr('trends'))}"),
        c("5 pitches, each complete with a policy risk", len(pitches) == 5 and not incomplete, ", ".join(incomplete)),
        c("data status matches how many top games it saw", status_ok, f"{status}, {n if n_ok else '?'} seen"),
    ]


def check(agent, text, project, out_path=None):
    r = common(text)
    if agent == "pulse":
        r.append(c("names its data sources", heading(text, "data sources|sources")))
        r.append(c("ranked pitches present", heading(text, "rank")))
        r.append(c("numbers are sourced or marked UNVERIFIED", has(text, r"https?://") or "UNVERIFIED" in text))
        if out_path and (Path(out_path).parent / "pitches.json").is_file():
            r.extend(pitches_checks(Path(out_path)))
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
        denies = has(text, r"nothing was played live|not played live|no live play|code walk only|studio mcp was not used")
        claims = has(text, r"\bplayed (it )?live\b|\blive in studio\b|\bvia (the )?studio mcp\b") and not denies
        # Agent runs get no Studio MCP server (--strict-mcp-config), so a claim of
        # live play cannot be true until Studio is wired in. Fail it.
        r.append(c("does not claim live play it could not do", not claims, "claims live play in Studio" if claims else "headless or code walk (says so)"))
    elif agent == "merchant":
        r.append(c("catalog present", heading(text, "catalog")))
        r.append(c("do-not-sell check present", heading(text, "do-not-sell|do not sell")))
    elif agent == "librarian":
        r.extend(sounds_checks(text, project))
    elif agent == "frame":
        r.append(c("mobile checklist results present", has(text, r"mobile|phone")))
    elif agent == "splash":
        r.extend(splash_checks(text, Path(out_path)))
    elif agent == "liveops":
        r.append(c("numbers come with dates", has(text, r"\d{4}-\d{2}-\d{2}") or "BLOCKED" in text.upper()))
    elif agent == "modeler":
        r.append(c("models table with file, triangles, license", has(text, r"^\|.*file.*\|.*triangles.*\|.*license.*\|")))
        cells = table_col(text, "file")
        files = [m.group(0) for m in (re.search(r"projects/[A-Za-z0-9_-]+/assets/models/[A-Za-z0-9_.-]+\.(?:fbx|obj|py)", x) for x in cells) if m]
        missing = [f for f in files if not (ROOT / f).is_file()]
        rows = len(cells)
        ok = rows > 0 and len(files) == rows and not missing
        detail = "no Models table" if rows == 0 else (f"{rows - len(files)} of {rows} rows name no file path under projects/<game>/assets/models/" if len(files) < rows else (f"{len(files)} file(s)" if not missing else "missing: " + ", ".join(missing)))
        r.append(c("every model file in the table exists", ok, detail))
        lic = table_col(text, "license")
        bad = [l for l in lic if not has(l, r"\bcc0\b|made (in blender )?for this game") or has(l, r"sketchfab|poly ?pizza|cc[- ]by|paid|purchased")]
        r.append(c("licenses are CC0 or made for this game", bool(lic) and not bad, "; ".join(bad)))
        r.append(c("import steps for Andrew", heading(text, "import")))
    elif agent == "motion":
        r.append(c("action, trigger, effect table", has(text, r"^\|.*action.*\|.*trigger.*\|")))
        r.append(c("effects are cleaned up", has(text, r"debris|:destroy|destroy\(")))
    elif agent == "crowd":
        r.append(c("server authority section", heading(text, "server authority")))
        r.append(c("works with one player", has(text, r"one player|solo|single player|1 player")))
    elif agent == "tuner":
        r.append(c("budget section", heading(text, "budget")))
        r.append(c("findings cite file:line or none found", has(text, r"\.luau:\d+") or has(text, "no issue found")))
        r.append(c("says measured or estimated", has(text, r"measured|estimated")))
    elif agent == "director":
        r.append(c("identity section", heading(text, "identity")))
        r.append(c("cut list section", heading(text, "cut")))
        r.append(c("verdict section", heading(text, "verdict")))
    elif agent == "release":
        r.append(c("checklist section", heading(text, "checklist")))
        r.append(c("blockers section", heading(text, "blocker")))
        r.append(c("says nothing was published", has(text, r"nothing (was|has been) published|not published|andrew publishes")))
        r.append(c("no paid promotion", not has(text, r"\b(paid ads?|sponsor(ed)? ads?|ad credits?|buy ads?)\b") or has(text, r"no (paid )?ads")))
    elif agent == "simulator":
        r.extend(simulation_checks(Path(out_path)))
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
