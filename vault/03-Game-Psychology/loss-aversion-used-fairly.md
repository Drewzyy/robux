---
description: How loss aversion shapes player behavior and how to use it fairly: stakes without punishment, streak softening, FOMO limits, refunds and protection, and ethical lines for young audiences.
type: checklist
status: draft
last_checked: 2026-11-14
keywords: loss aversion, fomo, scarcity, streak, stakes, ethics, dark patterns, risk, protection, refund, countdown, limited time
sources: Kahneman and Tversky prospect theory (1979, from memory, not re-read 2026-11-14); Author working knowledge of game psychology (no pages opened 2026-11-14); Roblox Creator Hub docs and Roblox safety and policy pages (titles only, not opened 2026-11-14); Bartle player types and Kahneman and Tversky prospect theory (known from memory, not re-read)
verify: Prospect theory is established; app of it to Roblox is judgment. UNVERIFIED: effect sizes. Check current Roblox policy on manipulative monetization before shipping offers.
tags: [game-psychology, checklist]
---
# Loss aversion used fairly

People feel losses roughly twice as strongly as equal gains. That makes risk exciting but also easy to abuse. Use stakes to create tension, never to trap players, especially children. See [[03-Game-Psychology/streaks-and-daily-rewards]] and [[03-Game-Psychology/reward-schedules]].

## Do this
- Add stakes only where the player chose the risk (a bet, a raid, a hard mode), not by default.
- Cap the loss: lose a share of a temporary resource, not permanent progress or paid items.
- Soften streaks: allow 1 missed day per week (a "rest day" or freeze earned by play, not bought).
- Use FOMO honestly: limited items have clear end dates and a stated return plan; no fake timers or resetting countdowns.
- Give recovery: death drops a recoverable item pile for 2-5 minutes; stolen items can be bought back. See [[02-Game-Design/genre-playbooks/steal-and-defend]].
- Show the risk before the choice with plain numbers ("You may lose 20 coins").
- Protect new players for 5-10 minutes from any loss mechanic.
- Never hold paid items hostage to a streak or timer.

## Why it works
- Loss framing raises engagement with protection, insurance, and defense mechanics.
- Choice-based risk creates stories and feels earned.
- Fair rules maintain trust, which supports long-term retention.

## Fair versus unfair
- Fair: "Defend your base for 30 s or lose 10 coins." Unfair: "Your streak of 60 days will vanish in 2 minutes unless you pay."
- Fair: item rotation with a calendar and a return later. Unfair: invented scarcity that restocks silently.
- Fair: loss that can be recovered by play. Unfair: loss that can only be fixed with money.
- Fair: opt-in hardcore mode. Unfair: forced hardcore for all.

## How to build it in Roblox
- Death penalty module with a single config: percent lost, pile lifetime, protected slots.
- Streak system storing a freeze count; freeze earned every 7 days of play or quests.
- Event calendar UI showing the start and end of limited items.
- Risk prompts using a clear confirm dialog with the numbers and a cancel option.
- Log penalties per player; if someone loses more than a threshold, trigger a gentle assist (free consumable).
- Server-side only for all loss calculations to avoid exploits.

## Numbers (starting points)
- Loss on death 10-30 percent of carried soft currency; never 100 percent.
- Pile lifetime 120-300 s.
- Streak freeze 1 per week.
- Limited item window 7-28 days, with a published return schedule.
- Protection window 5-10 min.

## Mistakes
- Timer pressure on purchases for kids.
- Resetting a long streak with no warning.
- Loss on core progress items.
- Stacking multiple loss mechanics; players feel punished.
- Hiding the true cost.

## Playtest check
- Ask a new tester if any loss felt unfair and what they would have changed.
- Check that a player who loses everything carried can recover within 10 minutes.
- Review all timers for honesty: does the stated end date match the code?

## For our Roblox games
Copy the principle: chosen, capped, recoverable stakes with honest time limits. Never copy manipulative designs, fake scarcity, or pay-to-undo loss. Follow Roblox policy and be kind to young players.
