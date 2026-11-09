#!/usr/bin/env python3
"""Structural checks on the vault.

  python3 scripts/check_vault.py            # skeleton checks (Phase 1+)
  python3 scripts/check_vault.py --full     # also require every indexed note to exist and no stubs (Phase 3+)

Checks:
- router exists, is under 80 lines, and every [[link]] in it resolves
- every section has index.md
- every note has flat frontmatter with description, type, status, last_checked, verify
- descriptions are at most 200 characters
- index rows marked "planned" do not exist yet; rows not marked planned do exist
- (--full) no planned rows, no notes with status stub, every note has a verify line
  that is either a real check or starts with UNVERIFIED
"""
import re
import sys
from pathlib import Path

VAULT = Path(__file__).resolve().parent.parent / "vault"
SECTIONS = [
    "10-game-design", "20-luau-rojo", "30-ui-ux", "40-monetization",
    "50-retention-analytics", "60-policy", "70-competitors", "80-assets",
    "90-templates-checklists",
]
REQUIRED = ["description", "type", "status", "last_checked", "verify"]
LINK = re.compile(r"\[\[([^\]|#]+)(?:[|#][^\]]*)?\]\]")


def frontmatter(p: Path):
    t = p.read_text(encoding="utf-8")
    if not t.startswith("---\n"):
        return None, t
    end = t.find("\n---\n", 4)
    if end < 0:
        return None, t
    fm = {}
    for line in t[4:end].splitlines():
        if ":" in line and not line.startswith(" "):
            k, v = line.split(":", 1)
            fm[k.strip()] = v.strip()
    return fm, t[end + 5 :]


def resolves(link: str) -> bool:
    return (VAULT / f"{link}.md").is_file()


def main() -> int:
    full = "--full" in sys.argv
    errs, warns = [], []
    router = VAULT / "00-router/index.md"
    if not router.is_file():
        errs.append("missing 00-router/index.md")
    else:
        n = len(router.read_text().splitlines())
        if n >= 80:
            errs.append(f"router is {n} lines (must stay under 80)")
        for l in LINK.findall(router.read_text()):
            if not resolves(l):
                errs.append(f"router link does not resolve: [[{l}]]")
    notes = sorted(p for p in VAULT.rglob("*.md") if ".claude" not in p.parts and ".obsidian" not in p.parts and p.name != "CLAUDE.md")
    stubs = 0
    for p in notes:
        rel = p.relative_to(VAULT)
        fm, body = frontmatter(p)
        if fm is None:
            errs.append(f"{rel}: no frontmatter")
            continue
        for k in REQUIRED:
            if not fm.get(k):
                errs.append(f"{rel}: frontmatter missing {k}")
        if len(fm.get("description", "")) > 200:
            errs.append(f"{rel}: description over 200 chars")
        if fm.get("status") == "stub":
            stubs += 1
            if full:
                errs.append(f"{rel}: still a stub")
        for l in LINK.findall(body):
            if not resolves(l) and not (rel.name == "index.md" and not full):
                (errs if full else warns).append(f"{rel}: link does not resolve: [[{l}]]")
    planned = 0
    for s in SECTIONS:
        idx = VAULT / s / "index.md"
        if not idx.is_file():
            errs.append(f"{s}: missing index.md")
            continue
        for line in idx.read_text().splitlines():
            m = LINK.search(line)
            if not m or not line.startswith("|"):
                continue
            exists = resolves(m.group(1))
            is_planned = line.rstrip().endswith("| planned |")
            if is_planned:
                planned += 1
                if exists:
                    errs.append(f"{s}/index.md: [[{m.group(1)}]] exists but is marked planned")
                if full:
                    errs.append(f"{s}/index.md: [[{m.group(1)}]] still planned")
            elif not exists:
                errs.append(f"{s}/index.md: [[{m.group(1)}]] is listed but missing")
    print(f"notes: {len(notes)}  stubs: {stubs}  planned index rows: {planned}")
    for w in warns:
        print("warn:", w)
    for e in errs:
        print("ERROR:", e)
    print("PASS" if not errs else f"FAIL ({len(errs)} errors)")
    return 0 if not errs else 1


if __name__ == "__main__":
    sys.exit(main())
