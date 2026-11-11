---
description: How BrewixGames remembers decisions per game: every approve, reject, and change request is appended to that game's memory note.
type: reference
status: verified
last_checked: 2026-11-11
verify: app/src-tauri/src/queue.rs writes these notes; cargo test covers it.
tags: [project-memory, reference]
---
# How project memory works

When Andrew approves, rejects, or requests changes on a review item, the app appends one line to `19-Project-Memory/<game>.md`: the date, the decision, what it was about, and the note Andrew wrote.

Agents read their game's memory note before planning so they do not repeat rejected ideas or undo approved ones. They never edit it.
