---
description: Concrete UI patterns from hit Roblox and other games for menus, HUD, shop, reward reveals and notifications, with sizes in px and timings in seconds.
type: reference
status: draft
last_checked: 2026-11-14
keywords: ui patterns, hud, menu, shop, reward reveal, notification, toast, button size, touch target, animation timing, popup, cooldown, currency counter
sources: Apple Human Interface Guidelines touch target guidance (44 pt minimum, from model knowledge); Google Material touch target guidance (48 dp, model knowledge); general UI animation practice (100-400 ms); recalled UI of Adopt Me, Blox Fruits, Pet Simulator style games, Clash Royale, Genshin Impact (not re-viewed 2026-11-14)
verify: UNVERIFIED: numbers are common practice ranges and the examples are recalled, not measured. Test on a 5.5 inch phone in Studio's device emulator. Roblox UI APIs not checked.
tags: [08-ui-ux, reference]
---
# Game UI patterns from hits

Practical numbers for agents building Roblox UI. Companion to [[08-UI-UX/hud-template]], [[08-UI-UX/shop-ui]], [[08-UI-UX/ui-animation]], [[08-UI-UX/ui-sound-pairing]].

## Do this
- Touch targets: at least 44x44 px (about 8-10% of phone height for primary buttons); gap between buttons at least 8 px. Primary action 56-72 px high.
- HUD: keep the centre 60% of the screen empty. Put currencies top-left or top-right, action buttons bottom-right (thumb zone), menu buttons on the left edge in a vertical column of 4-6.
- Show at most 3 numbers at all times (main currency, one progress, one timer). Everything else on demand.
- Currency counters tick up over 0.3-0.6 s with a scale pop to 1.15 and back in 0.15 s; floating "+N" text rises 40-60 px over 0.8 s and fades.
- Menus open in 0.15-0.25 s (scale 0.9 to 1.0 plus fade, easing out); close in 0.1-0.15 s. Never block gameplay for more than needed.
- Shop: one featured item large (about 40% width), a grid of 2-3 columns below, price on the button, the real price shown (no hidden totals), a clear close button top-right at least 44 px.
- Reward reveal: 1.0-2.0 s total for common, 3-4 s for rare, skippable by tap after 0.5 s. Sequence: anticipation 0.4-0.8 s, burst, item scale-in 0.3 s, rarity colour glow, name and stats, Collect button.
- Notifications: toasts 2.5-4 s on screen, enter 0.2 s, stack max 3, newest on top, never cover action buttons, never use full-screen popups for minor events.

## Patterns by area
### Main menu and lobby
- One big Play button, 60-70% of the width of the panel, high contrast. Secondary buttons smaller and quieter.
- Daily reward, shop and settings as icons with red dot badges (8-12 px circle) when something is new.
- Show the player's avatar, name and top currency; avoid more than 6 menu entries on one screen.

### HUD (in game)
- Health or stamina bar: 12-20 px high, 150-220 px wide on phone, colour shifts at 30% (to red) and a pulse below 20%.
- Cooldown on action buttons: a radial or bottom-up fill, number shown only when above 1 s.
- Quest or goal tracker: one line, top centre or left, 14-16 pt minimum (phone), collapse after 5 s of no change.
- Minimap or compass only when navigation needs it; 20% of screen height max.

### Shop and offers
- Tabs at top: 3-5 (Featured, Pets, Boosts, Cosmetics). Remember last tab.
- Limited-time offers show a countdown in plain words and a real end time; do not reset it.
- Confirm purchases with Roblox's prompt only; no fake countdown pressure. See [[08-UI-UX/shop-ui]].
- Owned items show "Owned" or "Equipped", never a buy button.

### Reward reveals (egg or crate openings)
- Shake or build-up 0.5-1.0 s, then reveal; rarer tiers get a longer build-up up to 2.5 s.
- Rarity feedback by colour (grey, green, blue, purple, gold) plus particles and sound; do not rely on colour alone.
- Auto-open and skip options after the first few opens, for players who open hundreds. Skip must keep the result and log it.
- Show odds where the game requires it (paid random items policy; check the roblox-monetization skill).

### Notifications and tutorials
- Tutorials: one prompt at a time, an arrow or highlight on the exact object, 3-6 words, auto-dismiss when the action is done.
- Achievement banner 3 s, top centre, 300x70 px or less.
- Errors: red text near the cause, 2 s, with a short sound; no modal.

## Timing table
| Element | In | Hold | Out |
|---|---|---|---|
| Button press feedback | 0.05-0.1 s scale 0.95 | - | 0.1 s |
| Menu open | 0.2 s | - | 0.12 s |
| Toast | 0.2 s | 2.5-4 s | 0.2 s |
| Floating +N | 0 | 0.8 s rise | 0.2 s fade |
| Currency pop | 0.15 s | - | 0.15 s |
| Common reward reveal | 0.4 s | 1 s | 0.2 s |
| Rare reward reveal | 1-2.5 s build | 2 s | 0.3 s |
| Level up banner | 0.3 s | 2 s | 0.3 s |

## Why these patterns work
- Thumb reach and clear centre space keep players seeing the world.
- Short, snappy animations feel responsive; slower ones are reserved for rare moments so they feel special.
- Badges and one big button reduce choice at the moment of entry.

## Mistakes
- Tiny close buttons, text under 14 pt on phone.
- Stacked popups at join (daily reward, offer, update notes all at once). Queue them; max 1 per 10 s in the first 3 minutes.
- Reward reveals that cannot be skipped.
- UI that covers the jump or action buttons on phones; check the safe area.
- Offers whose countdown resets.

## For our Roblox games
Copy: the layouts, sizes, timings and feedback principles above. Never copy: any game's UI art, icons, button designs, names, fonts, sound or exact layouts; draw our own from [[08-UI-UX/design-system]]. Related: [[11-Audio/sound-feedback-map]].
