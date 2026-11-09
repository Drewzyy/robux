from common import note
S = "60-policy"
R = "Research report (Nov 9 2026) B8d, B8e, C6"

note(f"{S}/policy-index", "Every governing Roblox document with its URL and last-checked date. Re-read before every launch and every monetization change.", "reference", "draft",
 R, "Links listed as given in the report. Each must be opened and its date recorded before a launch; until then each row is UNVERIFIED for currency.", r"""
# Policy index

Re-read the relevant documents before every launch and every monetization change. Record the date in the table.

| Document | URL | What it covers | Last read |
|---|---|---|---|
| Roblox Terms of Use | https://en.help.roblox.com/hc/en-us/articles/115004647846 | the platform contract | not yet |
| Creator Terms (incl. DevEx Terms) | Creator Hub (find from create.roblox.com) | creator obligations, DevEx | not yet |
| Community Standards | create.roblox.com/docs (policy section) | content and conduct | not yet |
| Creator Rewards framework | Creator Hub | Daily Engagement, Audience Expansion | not yet |
| Paid Random Items policy | Creator Hub (live May 26, 2026) | odds, value, region gating | not yet |
| Advertising Standards | Creator Hub (updated June 2026) | ads inside and for games | not yet |
| Content Maturity | https://github.com/Roblox/creator-docs/blob/main/content/en-us/production/promotion/content-maturity.md | labels and questionnaire | 2026-11-09 (via report) |
| Discovery | https://github.com/Roblox/creator-docs/blob/main/content/en-us/discovery.md | how ranking works | 2026-11-09 (via report) |
| Onboarding guidance | https://create.roblox.com/docs/production/game-design/onboarding | first session design | 2026-11-09 (via report) |

Where a URL says "Creator Hub", the exact page was not given in the research. Andrew or an agent with web access finds it and fills the URL before relying on it.
""")

note(f"{S}/pre-launch-policy-checklist", "One hour per launch: maturity questionnaire, paid random items, text filter, no data collection, no off-platform payments, IP, thumbnails, products.", "checklist", "draft",
 R, "Checklist built from the report. Each item is checked against the live policy pages listed in policy-index on launch day.", r"""
# Pre-launch policy checklist (one hour per launch)

- [ ] Re-read the documents in [[60-policy/policy-index]]; record the date.
- [ ] Content Maturity questionnaire filled and accurate ([[60-policy/content-maturity-guide]]). A game without a label is restricted.
- [ ] No paid random items (game 1), or the full [[40-monetization/paid-random-items-checklist]].
- [ ] Nothing that crosses the [[60-policy/gambling-line]].
- [ ] All player text input goes through TextService filtering.
- [ ] No collection of real names, emails, socials, or off-platform contact.
- [ ] No off-platform payments, no external shops, no Robux or item trading for money.
- [ ] No fake engagement: no bought visits or likes, no bots, no misleading thumbnails or titles.
- [ ] No trademarked characters, logos, or copied assets; no Toolbox free models ([[60-policy/ip-and-toolbox-rules]]).
- [ ] AI-generated content checked like any other content; AI chat interactions disclosed if present ([[60-policy/ai-content-rules]]).
- [ ] Every product does exactly what it says; ProcessReceipt tested.
- [ ] Thumbnails and icon moderated-safe ([[30-ui-ux/thumbnail-icon-spec]]).
- [ ] PolicyService used where needed (paid random items, social links for under-13s).
- [ ] Andrew approves the exact build in the review queue. Publish defaults to Saved, never live.
""")

note(f"{S}/gambling-line", "Gambling is banned: no real money, Robux, or valuable items wagered. Decorative references allowed. Paid random items with disclosed odds are the allowed side.", "reference", "draft",
 R + "; antimedia.co/news/roblox-gambling-2026", "Secondary reporting of Roblox policy. Read Community Standards (gambling section) before any chance-based feature.", r"""
# The gambling line

## Banned
- Simulated and actual gambling.
- Exchanging real money, Robux, or in-game items of value in connection with gambling.
- Casinos, slots, coin pushers, crash games, case opening for profit.

## Allowed
- Unplayable references (a casino sign as decoration), except where local law forbids it.
- Paid random items that follow every rule in [[40-monetization/paid-random-items-checklist]]. Example of the allowed side: Case Opening Simulator prints odds in its description ("4 in 5 for a blue (79.92%), 1 in 6 for a purple (15.98%), 1 in 31 for a pink (3.2%)").
- Free random rewards.

## Studio rule
If unsure, do not build it. Read the policy, pick the conservative option.
""")

note(f"{S}/ai-content-rules", "AI-made assets are allowed and the uploader is responsible. AI interactions need maturity disclosure; extended chatbots need Restricted (18+). Ranking, not filtering.", "reference", "draft",
 R + "; umatechnology.org AI moderation article; Roblox Q3 2026 shareholder letter as cited by the god plan",
 "Secondary reporting. The Q3 2026 shareholder letter claim (40% of top-100 RfY games under three months old) is UNVERIFIED here; read the letter before quoting.", r"""
# AI content rules

## Assets
- AI-generated clothing, models, and code are allowed. No disclosure is required for AI-made assets.
- They must follow Community Standards, Marketplace Policy, and IP rules. The uploader is responsible, not the tool. "The AI wrote it" is not a defence.
- All inputs and outputs go through moderation (text filter, image review, asset review).

## AI interactions (different rules)
- Games where player interaction triggers generative responses must disclose it in the Content Maturity questionnaire.
- Extended chatbot-like interaction (continuous conversation, memory across sessions) requires a **Restricted (18+)** label and a visible notice such as "This is an AI-powered conversation, not human. It may make mistakes."
- AI output must match the game's maturity level.

## How Roblox treats AI slop
Roblox does not filter AI content or employ curators for it. From Q2 2026 it optimised Recommended for You for long-term retention. Reported result: by end of Q3 2026 about 40% of the weekly top 100 by impressions were made in the previous three months (UNVERIFIED). Translation: new games can break in fast; generic games get buried fast. This is why the studio's anti-slop gate exists ([[90-templates-checklists/anti-slop-checklist]]).
""")

note(f"{S}/content-maturity-guide", "Content Maturity labels Minimal, Mild, Moderate, Restricted: what each allows, the questionnaire, resubmit after changes, unlabeled games restricted.", "reference", "draft",
 R + "; github.com/Roblox/creator-docs content-maturity.md; webwise.ie", "From Roblox's Creator Docs as summarized in the report. Re-read content-maturity.md before filling the questionnaire.", r"""
# Content maturity guide

Every experience completes the Content Maturity questionnaire in Creator Dashboard.

| Label | May contain (summary) |
|---|---|
| Minimal | occasional mild content |
| Mild | mild violence, light unrealistic blood, mild crude humour (summary) |
| Moderate | moderate violence, light realistic blood, moderate fear, moderate crude humour, unplayable gambling content |
| Restricted (18+, age-checked only) | strong violence, heavy realistic blood, romantic themes, unplayable gambling content, alcohol, strong language, social hangouts and private spaces, extended AI interactions, moderate fear |

## Rules
- No questionnaire: Roblox restricts playability for all players.
- After an update that changes answers, update and resubmit.
- Inaccurate answers can lead to moderation of the game and the account.
- Maturity does not cover what players bring (avatar clothing).

Studio default: aim for Minimal or Mild. The audience is mostly minors (about 35-36% under 13 and 38% aged 13-17 among age-checked users, per secondary reporting of Roblox's 2026 figures).
""")

note(f"{S}/ip-and-toolbox-rules", "No trademarked characters, logos, or copies of other creators' work; no Toolbox free models; build from primitives, code, or assets the team made.", "reference", "draft",
 R + " (C6 IP)", "Studio rules aligned with Roblox IP policy as summarized. Librarian's and Splash's gates check these per asset.", r"""
# IP and Toolbox rules

- Do not prompt for, generate, or ship trademarked characters, brand logos, or copies of other creators' work. A Totoro-like creature, a Pokemon, or a Marvel character is a takedown risk no matter who made it.
- **No Toolbox free models.** They can contain hidden malicious scripts ([[80-assets/no-toolbox-rule]]).
- Build from primitives and code, from assets the team made, or from licensed sources with the license recorded ([[80-assets/asset-sources-and-licenses]]).
- Thumbnails: no platform logos, catalog items, or recognisable characters.
- Copy mechanics, never art, names, or levels. A teardown names mechanics to learn from ([[70-competitors/copy-vs-avoid]]).
""")

note(f"{S}/moderation-and-appeals", "How moderation works (automated plus human), what a strike can cost, and how to respond to a flag the same day: replace, resubmit, appeal politely with facts.", "playbook", "draft",
 R + " (C6)", "Secondary reporting. The claim that one strike can cost DevEx eligibility is UNVERIFIED; check the DevEx terms.", r"""
# Moderation and appeals

- Moderation is automated plus human. Text can be blocked in milliseconds; assets are reviewed before sale; decisions can be appealed.
- A game without a content maturity label is private or restricted.
- Keep a clean record. UNVERIFIED: one strike can cost DevEx eligibility.

## When something is flagged
1. Same day: replace the thumbnail, name, or asset and resubmit.
2. Appeal immediately and politely: state facts, link the policy, do not argue.
3. Log what happened and the fix in [[00-router/policy-change-log]] if it reveals a rule we did not know.

Appeals need Andrew's account. Agents draft the appeal text only.
""")

note(f"{S}/policy-change-watch", "Where policy changes appear and the dated changes that already matter: cross-game pass sales off May 30 2026, QPTR retired June 2026, Wallet Dec 2026.", "reference", "draft",
 R + " (Part D)", "Each dated change cites its report source; re-check on Creator Hub and DevForum before each launch.", r"""
# Policy change watch

## Where to watch
Creator Hub announcements, the DevForum, Roblox shareholder letters, RDC announcements. Re-read before each launch and each monetization change.

## Changes that already matter (newest first)
| Date | Change | Effect on us |
|---|---|---|
| Dec 2026 (announced) | Roblox Wallet opt-in for US 18+ sole proprietors; DevEx US transition mid-2027 | [[40-monetization/roblox-wallet]] |
| Nov 1, 2026 | US DevEx payouts treated as royalties for tax | W-9 needed to avoid 24% backup withholding |
| June 2026 | Discovery window 7 to 28 days; QPTR retired for play-through plus first-play bounce | [[50-retention-analytics/discovery-signals]] |
| June 8, 2026 | US 18+ DevEx rate about $0.0054 | [[40-monetization/money-math]] |
| June 2026 | Advertising Standards updated | re-read before any ad |
| May 30, 2026 | Cross-game pass sales disabled | sell passes only in their own game |
| May 26, 2026 | Paid Random Items policy live | [[40-monetization/paid-random-items-checklist]] |
| Apr 30, 2026 | Roblox Plus launched; new Premium sign-ups stopped | [[40-monetization/roblox-plus-and-subscriptions]] |
| Jan 5, 2026 | Paid ads for less healthy food and drink banned | no such ads |
| Jan 2026 | Age Check to Chat live globally (reported Jan 7) | chat features assume age gates |
| Sep 5, 2025 | DevEx standard rate $0.0038 | money math |
| July 24, 2025 | Creator Rewards replaced Premium Payouts | money math |
""")
print("60 done")
