#!/usr/bin/env python3
"""Check the app's built-in gates (`brewix gate ...`) give the same results as
the reference Python scripts on real agent outputs."""
import json, subprocess, sys, glob
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
BIN = ROOT / "app/src-tauri/target/debug/studio-cli"
cases = []
for out in sorted(glob.glob(str(ROOT / "runs/*/*.md"))):
    req = Path(out).parent / "request.json"
    if not req.exists():
        continue
    agent = json.loads(req.read_text())["agent"]
    cases.append((agent, out))
ok = True
for agent, out in cases:
    py = json.loads(subprocess.run([sys.executable, str(ROOT / "scripts/gates/check_stage.py"), agent, out], capture_output=True, text=True).stdout)
    rs = json.loads(subprocess.run([str(BIN), "gate", "stage", agent, out], capture_output=True, text=True, cwd=ROOT).stdout)
    a = [(c["name"], c["passed"]) for c in py]
    b = [(c["name"], c["passed"]) for c in rs]
    same = a == b
    ok &= same
    print(("SAME " if same else "DIFF ") + agent.ljust(10) + Path(out).parent.name[:48] + "/" + Path(out).name, f"({len(b)} checks)")
    if not same:
        print("   python:", a); print("   rust:  ", b)
print("PARITY:", "PASS" if ok else "FAIL", f"({len(cases)} outputs)")
sys.exit(0 if ok else 1)
