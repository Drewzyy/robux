---
description: Why some sounds feel satisfying and how to make them: layering, pitch variation, rising pitch combos, and timing.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: satisfying sound, sfx, sound design, pitch, combo, coin sound, juice, feedback
sources: Game feel practice (practitioner sources); devforum audio balance thread
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
---
# Satisfying sound effects

## Do this
- Layer: a sharp attack (click, tick) + a body (thump, chime) + a short tail. Three short layers beat one long sound.
- Vary pitch 0.9 to 1.1 every play so repeats never sound robotic.
- Combos and streaks: raise pitch about a semitone per step (PlaybackSpeed 1, 1.06, 1.12...), reset after a pause. Collecting coins in a row should climb.
- Timing: sound on the exact frame of impact or press, never late.
- Big moments get a sound and a silence before them (duck music 0.3 s before a rare reveal).
- Keep UI sounds short (under 0.15 s) and soft; save loud, long sounds for rare rewards.

Where sounds come from (free): [[11-Audio/audio-pipeline]], [[11-Audio/forgegui]].
