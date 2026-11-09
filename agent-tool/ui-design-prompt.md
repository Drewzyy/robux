# CLAUDE CODE PROMPT: Apple-Style UI (Dark Mode, Red Accents)

You are Claude Code, working for Andrew Dieu. Design and build the UI for the internal agent tool (a Tauri desktop app that wraps Claude Code with a team of specialized agents and an Obsidian knowledge vault for making Roblox games). The app is a shell and a review queue: Andrew pitches an idea in plain words, the agents design it, build it, and playtest it, and the result lands in a review queue for Andrew to approve. Nothing ships until he approves it.

Read the build prompt first: `agent-tool/claude-code-build-prompt.md` (copy it into this folder if it is not here). It is authoritative for what the app does. This prompt is authoritative for how it looks.

## The design direction

Make the UI look like it was built by Apple. Dark mode only. Red accents. This is not optional polish, it is the design standard for the whole app. If a design choice does not look like Apple made it, it is wrong.

## Design tokens (use these exact values)

- **Dark mode only.** Background near-black `#0A0A0A`. Cards and panels dark gray `#1C1C1E` with subtle borders `#38383A` and large corner radii (12-16px). Secondary text gray `#8E8E93`. No light mode. Do not build a theme toggle.
- **Red accent.** Apple's system red `#FF3B30` for primary actions, active states, and highlights only. Approve buttons, active navigation, active agent, progress. Use it sparingly. Everything else is gray and white. Never use red for a destructive action without a confirmation.
- **Typography.** System font stack: `-apple-system, BlinkMacSystemFont, SF Pro, Inter, sans-serif`. Large titles 28-34px semibold, section headings 17-20px semibold, body 15-17px regular, secondary text 13-14px gray. Generous line height. No decorative fonts, no custom fonts.
- **Spacing.** Content padding 24-32px. Card padding 16-20px. Section gaps 24px. Element gaps 8-12px. Generous whitespace everywhere. No clutter.
- **Components.** Segmented controls for views, toggle switches for settings, pill badges for status (approved, pending, failed), progress bars in red, subtle dividers (`#38383A`). Buttons: primary red filled with white text, secondary gray filled (`#2C2C2E`) with white text, destructive red text only (no red fill).
- **Shadows and depth.** No gradients. No heavy shadows. Only subtle ones on cards. Flat surfaces, clear hierarchy through spacing and typography, not through decoration.
- **Motion.** Subtle and fast. No bouncy animations, no spinners that linger. Transitions under 200ms.

## Screens to build (in this order)

1. **Sidebar navigation (left, always visible).** macOS-style sidebar. Items: Review Queue, Agents, Vault, Runs, Settings. Active item highlighted in red. A small status line at the bottom showing the Claude connection state and the running cost estimate.
2. **Review queue (the main screen).** A list of items awaiting approval. Each item is a card with a thumbnail or icon, a title, a one-line summary, which agent produced it, and Approve (red filled) and Reject (gray filled) buttons. A header shows the app name, the current run, and the running cost estimate. Empty state: a clean centered message, not a blank screen.
3. **Agent team view.** A list of the 12 agents (Pulse, Lead, Architect, Builder, Scenic, Librarian, Frame, Playtest, Splash, Merchant, Liveops, Producer). Each row shows the agent name, what it does in one line, and its current status (idle, working, waiting for approval, blocked). The active agent is highlighted in red.
4. **Run detail view.** When Andrew taps a run, show its steps in order, each with a status, what was produced, and how it was verified. Include the cost estimate for the run before it starts, and the actual cost after it finishes.
5. **Settings.** Claude connection, vault folder path, cost budget (a number Andrew sets, after which agent runs stop and ask), and notification preferences. Toggles, not dropdowns, wherever possible.

## Rules

- The main screen is the review queue. It is the first thing Andrew sees. Everything else is one tap away in the sidebar.
- The running cost estimate must be visible on every screen (in the sidebar status line and in the review queue header). Never run agents without showing the cost first.
- Every button must do something. No dead buttons, no placeholder screens. If a screen is not built yet, do not show it in the sidebar.
- No emoji as UI icons. Use text labels or simple inline SVG marks.
- Verify by building the app and screenshotting each screen. Look at the screenshots. If a screen does not look like Apple made it, fix it and screenshot again. Report the screenshot results with every screen.

## How to work

1. Read the build prompt first. If a step in this prompt contradicts it, the build prompt wins for what the app does, and this prompt wins for how it looks. Say so if they conflict.
2. Build one screen at a time, in the order above. Verify each screen (build + screenshot) before moving to the next.
3. Report back in this format: DONE (what you built, with verification), BLOCKED (with exactly what is needed and from whom), NEXT (the next step).

## Additions from Andrew (same message)

- An auto-updater button that downloads and installs the update and restarts the app automatically.
- A Discord webhook that notifies Andrew when a game has finished building, when something needs approval, or anything like that.
- Claude is integrated into the app without any API key.
- Make sure it is done; if anything is blocked, find a free way around it.
