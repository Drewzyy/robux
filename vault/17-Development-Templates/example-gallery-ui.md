---
description: Original SVG examples of good and bad phone HUD, shop, reward reveal, daily calendar, gift popup, favorite card, quests, leaderboard and settings, each with what it teaches.
type: reference
status: draft
last_checked: 2026-11-15
keywords: ui examples, hud, shop, reward reveal, daily reward, quest tracker, leaderboard, settings, phone ui, good vs bad
sources: Original SVG diagrams drawn for this vault, 2026-11-15; rules from the linked vault notes, read 2026-11-15
verify: Diagrams are our own illustrations of the vault rules. Numbers come from the linked notes; check any UNVERIFIED mark there. No real game art used.
tags: [development-templates, reference]
---
# Example gallery: UI screens
Original diagrams drawn for this vault so agents and the owner can see what the rules look like. Nothing here is copied from a real game.

## Do this
- Copy the layout logic, not the pixels: objective, currency, progress, next reward in the top third; joystick zones empty.
- Keep everything under the 58 px top bar clear and out of the bottom 30%.
- Show price and odds before any purchase, one accent button, a big close X.
- Run all four reward beats; scale length by rarity and add Skip over 2 s.
- Daily, playtime and favorite prompts are free, honest, dismissible and never shown in the first 5 minutes.
- Settings carry comfort toggles (shake, reduced motion) and state in words, not colour only.

## Rule notes these pictures follow
- [[08-UI-UX/hud-template]]
- [[08-UI-UX/shop-ui]]
- [[08-UI-UX/mobile-first-checklist]]
- [[08-UI-UX/reward-moments-and-juice]]
- [[08-UI-UX/color-palettes]]
- [[08-UI-UX/accessibility]]
- [[06-Retention/favorite-and-follow-prompts]]

## Good HUD
![[attachments/examples/good-hud.svg]]
*Four core elements in the top third, thumb zones empty.*

What it teaches:
- Top-left objective, top-right currency, progress bar and next reward grouped.
- Text stays 16 px or larger with high contrast.
- Compare with bad-hud: same screen, nothing blocks the joystick.

## Bad HUD
![[attachments/examples/bad-hud.svg]]
*Five common mistakes marked on the screen.*

What it teaches:
- Buttons in the top bar, tiny crammed stats, buy prompt during play.
- A fake-timer popup covers the action and hides a tiny close box.
- Use it as a review list: if your HUD matches any badge, fix it.

## Good shop
![[attachments/examples/good-shop.svg]]
*Prices on buttons, odds line, locked items grey, big close.*

What it teaches:
- Opened only by a player tap on a labelled shop button.
- Balance stays visible while shopping.
- Random items must show odds, always.

## Bad shop
![[attachments/examples/bad-shop.svg]]
*Pressure timer, hidden price, tiny close, shame copy.*

What it teaches:
- Never write copy that shames or rushes a child.
- A purchase starts only from a clear tap on a labelled button.
- Do not pop a shop on join, on death or in the first 5 minutes.

## Reward reveal in 4 beats
![[attachments/examples/reward-reveal-sequence.svg]]
*Anticipation, reveal, celebration, collection with timings.*

What it teaches:
- Rare reward is about 5 s total; small reward 0.8 s.
- Server grants first, client plays the show.
- Each beat gets one sound; pitch rises with rarity.

## Daily reward calendar
![[attachments/examples/daily-reward-calendar.svg]]
*Seven cards, today glows, day 7 shows the big prize.*

What it teaches:
- One Claim button on today only.
- Kind streak rule: a missed day softens, never shames.
- Visible day 7 reward is the open loop.

## Playtime gift popup
![[attachments/examples/playtime-gift-popup.svg]]
*Corner card with a progress bar to the next gift.*

What it teaches:
- Never blocks movement or the world.
- Gift is claimed by tap, not auto-spent.
- Rewards time played, not money spent.

## Favorite reminder
![[attachments/examples/favorite-reminder-card.svg]]
*One ask after a win, with Maybe later of equal size.*

What it teaches:
- No bribe and no guilt; no reward locked behind favoriting.
- Never repeated in the same session.
- Teaches where to tap instead of forcing the action.

## Quest tracker
![[attachments/examples/quest-tracker.svg]]
*One main quest pinned, two side quests, guide line to target.*

What it teaches:
- Progress as bar plus number.
- Max 3 visible entries, collapsible.
- Guide line removes aimless searching.

## Leaderboard
![[attachments/examples/leaderboard.svg]]
*Top 5 plus your row between neighbours.*

What it teaches:
- A near rank is a near goal.
- Weekly reset gives new players a chance.
- Rank effort, not spending.

## Settings menu
![[attachments/examples/settings-menu.svg]]
*Audio, comfort and controls groups, no scrolling.*

What it teaches:
- Toggles show ON or OFF in words.
- Screen shake and reduced motion are first-class settings.
- Saves instantly, no confirm step.

## How to use these pictures
- Open the note, read the badges on each picture, then compare your own screenshot side by side.
- Keep the SVG files as the reference; redraw in Figma or Studio with your own art.
- If a picture and a rule note disagree, the rule note wins; fix the picture.

## For our Roblox games
- Copy: the principle shown (layout zones, timings, counts, order of steps).
- Never copy: names, art, logos, characters or music from real games. Every picture here uses our own shapes.
- Zero spend: all of this is built with free tools; no paid assets or ads.
- Children play: no pressure timers, no shame copy, odds always shown, purchases only from a clear player tap.
