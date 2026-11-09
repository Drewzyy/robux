#!/usr/bin/env python3
"""Headless build gate for a Rojo project. Run by the studio, not the agent.

  python3 scripts/gates/build_gate.py projects/<game>

Prints a JSON list of checks: rojo build, selene, stylua --check, Lune tests.
A tool that is not installed is a FAILED check ("NOT RUN"), never a pass.

Selene: uses the project's selene.toml with the full Roblox standard library
when it can download the API dump. If that download fails (offline), it
falls back to scripts/gates/roblox-offline.yml, which knows Roblox globals
but cannot check API members, and says so in the detail.
"""
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent


def run(cmd, cwd, timeout=300):
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout)
        return p.returncode, (p.stdout + p.stderr)
    except FileNotFoundError:
        return None, f"NOT RUN: {cmd[0]} not installed"
    except subprocess.TimeoutExpired:
        return 1, f"timed out after {timeout}s"


def tail(s, n=600):
    s = s.strip()
    return s if len(s) <= n else "..." + s[-n:]


def gate(project: Path):
    checks = []
    proj = project if project.is_absolute() else ROOT / project
    pj = proj / "default.project.json"
    if not pj.is_file():
        return [{"name": "rojo project exists", "passed": False, "detail": f"no default.project.json in {project}"}]

    with tempfile.TemporaryDirectory() as td:
        code, out = run(["rojo", "build", str(pj), "-o", str(Path(td) / "build.rbxl")], cwd=proj)
        checks.append({"name": "rojo build", "passed": code == 0, "detail": "built place file" if code == 0 else tail(out)})

    src = proj / "src"
    # selene
    with tempfile.TemporaryDirectory() as td:
        cfg = (proj / "selene.toml").read_text() if (proj / "selene.toml").is_file() else 'std = "roblox"\n'
        Path(td, "selene.toml").write_text(cfg)
        code, out = run(["selene", "--no-summary", str(src)], cwd=td)
        note = ""
        if code is not None and "Could not collect standard library" in out:
            shutil.copy(ROOT / "scripts/gates/roblox-offline.yml", Path(td) / "roblox-offline.yml")
            lines = [l for l in cfg.splitlines() if not l.strip().startswith("std")]
            Path(td, "selene.toml").write_text('std = "roblox-offline"\n' + "\n".join(lines) + "\n")
            code, out = run(["selene", "--no-summary", str(src)], cwd=td)
            note = " (offline std: Roblox globals only, API members not checked)"
        passed = code == 0
        checks.append({"name": "selene lint", "passed": passed, "detail": ("clean" if passed else tail(out)) + note})

    code, out = run(["stylua", "--check", str(src)], cwd=proj)
    checks.append({"name": "stylua --check", "passed": code == 0, "detail": "formatted" if code == 0 else tail(out)})

    runner = proj / "tests" / "run.luau"
    if not runner.is_file():
        checks.append({"name": "lune unit tests", "passed": False, "detail": "no tests/run.luau in the project"})
    else:
        code, out = run(["lune", "run", "tests/run.luau"], cwd=proj)
        checks.append({"name": "lune unit tests", "passed": code == 0, "detail": tail(out, 400) or "passed"})
    return checks


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    print(json.dumps(gate(Path(sys.argv[1])), indent=2))
