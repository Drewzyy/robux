---
description: How the vault works. Progressive disclosure, the note format, write rules, and how to keep it fresh. For Andrew and for agents.
type: reference
status: verified
last_checked: 2026-11-09
sources: Research report A3 and A5 (Anthropic Agent Skills progressive disclosure; LachlanCB "inject pointers, not content"; Slomka vault hygiene).
verify: Phase 1 load logs (docs/evidence/phase1/) show agents reading only the router, one section index, and that section's notes.
tags: [start-here, reference]
---
# How to use this vault

## Why it is built this way
Agents are not trained on this vault. They read it as context, and context is finite: a model gets measurably worse as its context fills. A 200,000-word playbook read in full would use most of the window before any work starts. So the vault is built to be read in small pieces.

## Progressive disclosure, three levels
1. Router: `00-Start-Here/index.md`. Short. Maps a task to one section.
2. Section index: `<section>/index.md`. Lists the notes in that section, one line each.
3. Notes: loaded one at a time, only when a step needs them.

Skills follow the same idea: Claude Code loads only each skill's name and description at start, and the full skill body only when the agent invokes it.

## The note format
Every note starts with flat YAML frontmatter:
- `description`: about 150 characters. Lets an agent judge relevance without opening the note.
- `type`: router, section-index, playbook, template, checklist, reference, teardown.
- `status`: stub (empty), draft (written from the research report, mostly secondary sources; re-check a number before relying on it), verified (checked against a primary source or by running it), unverified (no source yet).
- `last_checked`: date the facts were last checked against sources.
- `sources`: where the facts came from.
- `verify`: how to check the note, or the word UNVERIFIED and why.

A note without a `verify:` line is not allowed. A claim inside a note that could not be checked is written with `UNVERIFIED:` in front of it.

## Write rules
- Agents never edit `00-Start-Here/` to `17-Development-Templates/`. This is enforced by deny rules in `.claude/settings.json`, not just by asking.
- Agents write output to their run folder (`runs/<run_id>/`). Andrew decides in the review queue what becomes permanent.
- New knowledge goes in only after a successful, verified run. A note written from a guess is noise.
- Overwrite and synthesize. One maintained note beats ten fragments.

## Freshness
- Policy notes: re-check before every launch.
- Money notes: re-check before every cashout.
- Benchmarks: compare against similar-game benchmarks in Creator Analytics, not universal targets.
- Dated changes go in `00-Start-Here/policy-change-log.md`.
