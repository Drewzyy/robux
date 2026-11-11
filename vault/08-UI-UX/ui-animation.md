---
description: UI motion that feels clean: durations, easing, press feedback, panels, toasts, colour tweens, and reduced motion.
type: playbook
status: draft
last_checked: 2026-11-11
keywords: ui animation, tween, easing, button press, juice, microinteraction, popup, reduced motion, color animation, colour tween
sources: Motion design references (Emil Kowalski, impeccable motion-design); create.roblox.com accessibility (search summary)
verify: UNVERIFIED: researched through web search summaries on 2026-11-11; Roblox docs could not be opened directly. Test numbers in Studio.
tags: [ui-ux, playbook]
---
# UI animation

## Do this
| Moment | Duration | Easing (TweenInfo) |
|---|---|---|
| Button press | 0.08 to 0.12 s scale to 0.95 | Quad Out |
| Button release | 0.15 to 0.2 s back to 1.0 | Back Out (slight overshoot) |
| Hover / select highlight | 0.1 to 0.15 s | Quad Out |
| Panel or shop open | 0.2 to 0.3 s scale 0.9 to 1 + fade | Back Out or Quint Out |
| Panel close | 0.15 to 0.2 s | Quad In |
| Toast / notification | in 0.25 s, hold 2 to 3 s, out 0.2 s | Quint Out / Quad In |
| Reward pop | 0.35 to 0.5 s scale 0 to 1.15 to 1 | Back Out |
| Colour change (state) | 0.15 to 0.25 s | Linear is fine for colour |
- Animate UIScale, not Size, so layouts do not jump.
- Everything under 0.3 s except reward celebrations. Slow UI feels broken.
- Ease-out for things appearing, ease-in for things leaving. Never Linear for movement.
- Reduced motion: if GuiService.ReducedMotionEnabled, set tween time to 0 or use a short fade instead of scale and shake.

## Make it feel premium
- Every press = visual change + sound ([[08-UI-UX/ui-sound-pairing]]).
- Stagger list items 0.03 s apart when a panel opens.
- Idle shimmer or gentle bob (2 to 3 s loop) only on the one thing you want pressed next.
- Colour pulse for "ready" states: tween accent brightness up and down every 1.2 s.
