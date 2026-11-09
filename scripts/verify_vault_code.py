#!/usr/bin/env python3
"""Extract the Luau files from code-template notes into temporary Rojo
projects and run the studio build gate on each. This is how notes marked
`status: verified` for code are kept honest.

  python3 scripts/verify_vault_code.py
"""
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
V = ROOT / "vault"
FILE_BLOCK = re.compile(r"`([^`]+\.luau)`[^\n]*\n```lua\n(.*?)```", re.S)


def project_json():
    ps = (V / "20-luau-rojo/project-structure.md").read_text()
    pj = json.loads(re.search(r"```json\n(.*?)```", ps, re.S).group(1))
    pj["tree"]["ReplicatedStorage"].pop("Packages", None)
    return pj


def check(note: str) -> bool:
    text = (V / note).read_text()
    files = FILE_BLOCK.findall(text)
    with tempfile.TemporaryDirectory() as td:
        t = Path(td)
        for path, code in files:
            f = t / path
            f.parent.mkdir(parents=True, exist_ok=True)
            f.write_text(code)
        for d in ("src/server", "src/client", "src/shared"):
            (t / d).mkdir(parents=True, exist_ok=True)
        (t / "default.project.json").write_text(json.dumps(project_json()))
        (t / "selene.toml").write_text('std = "roblox"\n')
        (t / "stylua.toml").write_text('indent_type = "Tabs"\ncolumn_width = 120\n')
        r = subprocess.run([sys.executable, str(ROOT / "scripts/gates/build_gate.py"), str(t)], capture_output=True, text=True)
        checks = json.loads(r.stdout)
    ok = all(c["passed"] for c in checks)
    print(f"{'PASS' if ok else 'FAIL'} {note} ({len(files)} files)")
    for c in checks:
        print(f"   {'ok  ' if c['passed'] else 'FAIL'} {c['name']}: {c['detail'][:120]}")
    return ok


if __name__ == "__main__":
    notes = ["20-luau-rojo/service-controller-template.md", "40-monetization/processreceipt-template.md"]
    results = [check(n) for n in notes]
    sys.exit(0 if all(results) else 1)
