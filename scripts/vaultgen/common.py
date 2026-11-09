"""Helper for writing vault notes with the required frontmatter."""
from pathlib import Path

VAULT = Path(__file__).resolve().parents[2] / "vault"


def note(path, description, type_, status, sources, verify, body):
    assert len(description) <= 200, (path, len(description))
    assert verify.strip(), path
    p = VAULT / f"{path}.md"
    p.parent.mkdir(parents=True, exist_ok=True)
    fm = (
        "---\n"
        f"description: {description}\n"
        f"type: {type_}\n"
        f"status: {status}\n"
        "last_checked: 2026-11-09\n"
        f"sources: {sources}\n"
        f"verify: {verify}\n"
        "---\n"
    )
    p.write_text(fm + body.strip() + "\n", encoding="utf-8")
    return p
