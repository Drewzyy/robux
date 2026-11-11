from common import note
S = "08-UI-UX"
R = "Research report (Nov 9 2026) C3"

note(f"{S}/design-system", "One font, one palette, a spacing grid, one button template cloned everywhere, and 4.5:1 contrast on text and key elements.", "reference", "draft",
 R + "; Veronica Aalin, dev.to 'Designing Clean Roblox GUIs'; ashexplained roblox-game-ux",
 "Community guidance. The 4.5:1 contrast ratio is the WCAG AA text standard; check with any contrast checker on real colours.", r"""
# Design system

- **One font**, used everywhere. Weights for hierarchy, not new fonts.
- **One palette**: background, surface, text, accent, success, danger. Write them in one shared module so every screen uses the same values.
- **Grid**: build on a grid, not by eye. `UIAspectRatioConstraint` plus `UIGridLayout`, a base cell (example 80x80), every size a multiple of it.
- **Contrast** at least 4.5:1 on text and key elements.
- **One button template** (Frame + TextLabel + UIStroke + UICorner + a small LocalScript), cloned everywhere. Same press feedback on every button.
- **Readable at phone size** first; see [[08-UI-UX/mobile-first-checklist]].
- Before building any non-trivial screen, write a short markdown design brief (screens, what each shows, where on a phone) and get it approved. Verify the built result with a phone playtest.
""")

note(f"{S}/hud-template", "What the HUD must show during the loop (objective, currency, progress, health or energy, next reward) and where it goes on a phone.", "template", "draft",
 R + "; ashexplained roblox-game-ux; tabooharmony roblox-gui", "Template. Verify by a phone playtest screenshot: every HUD element readable and outside the top 58 px and the bottom thumb controls.", r"""
# HUD template

## Must be visible during the core loop
| Element | Why | Where on a phone |
|---|---|---|
| Current objective | the player always knows what to do | top third, below the 58 px top bar |
| Currency | feedback on every reward | top corner, inside safe insets |
| Progress (bar, stage, collection meter) | goal gradient: near goals pull | top third |
| Health / energy (if any) | failure is legible | near the objective |
| Next reward | open loop | close to progress |

## Rules
- Anchor critical HUD to the top 40% of the screen: the virtual joystick and jump button take roughly the bottom 30% on mobile.
- Nothing interactive in the top 58 px (Roblox top bar). Use `ScreenGui.ScreenInsets = Enum.ScreenInsets.CoreUISafeInsets`.
- Minimal HUD in the first minute; add action and meta HUD as the player meets them (progressive disclosure).
- `ResetOnSpawn = false` for persistent HUD.
""")

note(f"{S}/onboarding-flow", "Onboarding UI order: spawn facing the objective, prompt one action, instant feedback and reward, show the next short goal, reveal social later.", "template", "draft",
 R + "; create.roblox.com/docs/production/game-design/onboarding (as distilled in the report)", "Verify by stopwatch playtest against 04-Onboarding/first-60-seconds-checklist.", r"""
# Onboarding flow

1. Spawn facing the first interactable or objective.
2. Prompt **one** action, in context (an arrow or highlight at the thing), not a modal.
3. Immediate feedback and reward: sound, particles, number going up.
4. Show the next short goal and make it look close.
5. Reveal social and progression context only after the player has acted.

Avoid: long tutorials, text walls, unskippable cutscenes, menus on the first screen. Put the player into the core action within 10 to 30 seconds. Fade prompts after the player shows mastery.

Timing detail lives in [[04-Onboarding/first-60-seconds-checklist]].
""")

note(f"{S}/shop-ui", "Shop UI: one main HUD, one overlay panel per feature, 3-tap rule, prices in Robux up front, no pop-ups on join, on death, or in the first 5 minutes.", "reference", "draft",
 R + "; dev.to 3-click rule; god plan shop rules (god plan not in repo)", "Studio rules plus community guidance. Verify in a playtest: count taps to buy, and confirm no shop prompt fires in the first 5 minutes or on death.", r"""
# Shop UI

- Flat structure: one main HUD, one overlay panel per feature. Deep menus stay optional in the first session.
- **3-tap rule:** any core action (equip, buy, start) in 3 taps or fewer.
- Price in Robux visible before the purchase prompt. The item does exactly what it says.
- Show value before the prompt: let players use a limited version first.
- **Never** pop the shop on join, on death, on stage completion, or in the first 5 minutes. Death is never a sales pitch.
- Shop never covers the core action.
- Every product follows [[07-Monetization/do-not-sell-list]]; random items follow [[07-Monetization/paid-random-items-checklist]].
""")

note(f"{S}/mobile-first-checklist", "Mobile first: Scale not Offset, constraints, CoreUISafeInsets and the 58 px top bar, thumb zones, Activated not mouse events, real-device test.", "checklist", "draft",
 R + "; ashexplained, tabooharmony roblox-gui, simplified.media, johnygorsky roblox-ui",
 "Community guidance. Mobile share of sessions differs by source (50% to 78%): no single figure is verified. The 44-48 px tap size is Apple/Material guidance, not a Roblox rule. Verify on a real phone.", r"""
# Mobile-first checklist

Mobile is the majority of Roblox sessions (sources say 50% to 78%; no single number is verified). Design for the phone, then expand.

## Layout
- [ ] Position and size with **Scale**, not Offset. Offset only for thin borders and padding.
- [ ] `UIAspectRatioConstraint` and `UISizeConstraint` (MinSize/MaxSize) lock proportions.
- [ ] Text: `TextScaled` plus `UITextSizeConstraint`. Minimum readable size about 16 px on mobile (one guide).
- [ ] One global `UIScale` knob.
- [ ] Prefer vertical layouts on phones.

## Safe areas
- [ ] Nothing interactive in the top 58 px (Roblox top bar). `ScreenInsets = CoreUISafeInsets` for HUD and menus; `None` only for full-bleed non-interactive backgrounds.
- [ ] Nothing critical in the bottom ~30% (joystick and jump button) or at screen edges and gesture zones.
- [ ] `IgnoreGuiInset = true` only for fullscreen art, never with critical content behind the top bar.

## Input
- [ ] `GuiButton.Activated` for core actions (fires for click and tap). Never mouse-only events.
- [ ] Prefer the Input Action System for cross-platform input; `UserInputService.TouchEnabled` to adapt layout.
- [ ] Tap targets: 44x44 (Apple) to 48x48 (Material) as an accessibility guide. There is no official Roblox tap-size rule.

## Test
- [ ] Studio Device Emulator for layout.
- [ ] A real phone for touch size, thumb reach, and low-end performance. The emulator does not show these.
""")

note(f"{S}/thumbnail-icon-spec", "Thumbnail 1920x1080 under 3 MB (up to 10), icon 512x512 as a separate crop, game pass 150x150, badge 512 circle; packaging rules.", "reference", "draft",
 R + "; thumbblox.com roblox-thumbnail-size; umatechnology upload guide; obby.fun icon/thumbnail maker",
 "Sizes from secondary guides. Confirm in the Creator Dashboard upload screen before first upload. 'Custom thumbnail required for Home recommendations' is UNVERIFIED.", r"""
# Thumbnail and icon spec

| Asset | Size | Notes |
|---|---|---|
| Experience thumbnail | 1920x1080, 16:9 | under 3 MB each, up to 10 images or videos; smaller uploads are stretched |
| Game icon | 512x512, 1:1 | a separate crop, not a resized thumbnail; readable at about 150x150 |
| Game pass icon | 150x150 | |
| Badge icon | 512x512 | circular crop |
| YouTube cover / trailer | 1280x720 | |

Formats: .png, .jpg, .gif, .tga, .bmp. Avoid .webp (community reports of silent failures).

## Packaging rules
- Avatar clearly visible; one focal point.
- Text under 4 words, readable on a phone; high contrast.
- Show something that exists in the game. The thumbnail is a promise the first 10 seconds must keep.
- No platform logos, catalog items, trademarks, or recognisable characters. Every image is moderated.
- UNVERIFIED (one analytics guide): a custom thumbnail is required to be eligible for Home recommendations. Check Creator Docs before treating as a rule.

Capture workflow: [[10-Art-Environment/thumbnail-pipeline]].
""")
print("30 done")
