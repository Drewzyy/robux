# Phase 3 spot checks (2026-11-09)

Five vault notes checked against primary sources (Roblox's own creator-docs on
GitHub, fetched 2026-11-09) or by running the code. GameAnalytics could not be
reached from this session (host not in the allowed network), so
`50-retention-analytics/benchmarks` was not spot-checked and stays `draft`.

| # | Note | Source | Result | Change made |
|---|---|---|---|---|
| 1 | 50-retention-analytics/discovery-signals | creator-docs `content/en-us/discovery.md` | MATCH: two stages; most important = play-through, first-play bounce (<60 s, 61-180 s), play days (Day 1, 2-7, 8-28), playtime (60 min cap per user per game per day); important = co-play days, qualified play sessions, spend days, Robux spent; non-RfY traffic not counted in ranking. Page does not mention QPTR or a 28-day window. Page says a custom thumbnail is required for homepage recommendation impressions. | June 2026 change labeled secondary; custom-thumbnail rule promoted from UNVERIFIED to verified here and in 30-ui-ux/thumbnail-icon-spec |
| 2 | 60-policy/content-maturity-guide | creator-docs `production/promotion/content-maturity.md` | PARTIAL MISMATCH: label texts differ from the research. Restricted label text = strong violence, heavy realistic blood, moderate crude humor, romantic themes, unplayable gambling content, alcohol, strong language; social hangouts with private spaces (18+ age-verified) and AI interactions are separate sections. No-label restriction, resubmission, and avatar-item exclusion MATCH. | Minimal and Mild filled with quoted text; Restricted corrected; note set to verified; 60-policy/ai-content-rules 18+ chatbot claim downgraded to UNVERIFIED |
| 3 | 40-monetization/money-math | creator-docs `production/monetization/developer-exchange.md` | MATCH: $0.0038 standard ($114 per 30,000); $0.0054 US 18+ (no start date on page); 30,000 minimum; 13+, verified email, W-9/W-8, DevEx portal, good standing; earned vs bought Robux lists. MISMATCH: research said US 18+ Robux cash out first; page says pre-Sep-5-2025 balances ($0.0035) cash out before current-rate balances. | DevEx section marked verified; cashout order corrected; eligible/ineligible Robux lists from the page |
| 4 | 10-game-design/first-60-seconds-checklist | creator-docs `production/game-design/onboarding.md` | PARTIAL: Roblox's three goals, low early XP thresholds, starter items, Player Funnel and A/B testing tutorial steps MATCH. Roblox gives NO seconds-level targets ("within minutes"); the 0-5/5-10/10-30/30-60 windows are community guidance. Roblox says showing controls in a tutorial "can be beneficial". | Added a Roblox-guidance section; timeline labeled community guidance |
| 5 | 20-luau-rojo/service-controller-template, project-structure, 40-monetization/processreceipt-template | executed: `scripts/verify_vault_code.py` | PASS: both templates extracted into temp Rojo projects using the project-structure mapping; rojo build, selene (offline std), stylua --check, and Lune tests (4 and 5 assertions) all pass | none needed |

Fetches were summarized by WebFetch's reader model; quotes above are as it
returned them. Re-read the pages directly before quoting them externally.
