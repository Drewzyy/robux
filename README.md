# Robux Studio (internal)

Andrew's internal agent studio for making Roblox games. A local desktop app
that runs Claude Code agents against an Obsidian knowledge vault and puts
everything they make into a review queue. Nothing ships, publishes, or posts
without Andrew approving that exact thing.

Not a product. No accounts, no pricing, no server.

## Layout

| Path | What it is |
|---|---|
| `app/` | Tauri 2 desktop app (Rust core in `src-tauri/src`, UI in `ui/`) |
| `vault/` | The Obsidian vault. Also the agents' working folder. |
| `vault/.claude/agents/` | One Claude Code subagent per team member |
| `vault/.claude/skills/` | One skill per agent: job, inputs, outputs, verification gate |
| `studio/` | App config: `agents.json` (models, cost profiles, tool allowlists, allowed vault sections), `pipelines.json`, `pricing.json` |
| `queue/items/` | Review queue items (JSON) |
| `runs/` | One folder per agent run: estimate, approval, raw output, files loaded |
| `projects/` | Game projects the agents build (Rojo) |
| `scripts/` | Verification scripts |
| `LOG.md` | Build log: what was done and how it was verified |
| `docs/AGENTS.md` | The 12 agents: job, output, model, sections, tools (generated) |

## Run it on the Mac (first time)

Needs Rust (`rustup`), Node 18+, Xcode command line tools, and Claude Code
logged in (`claude` works in Terminal).

```
cd app
npm install            # installs the Tauri CLI only
npm run dev            # opens the app window
npm run build          # makes Robux Studio.app and a .dmg in src-tauri/target/release/bundle
```

If the app says it cannot find `claude`, set `CLAUDE_BIN` to the output of
`which claude`. If it cannot find the studio folder, set `ROBUX_STUDIO_ROOT`
to this repo's path.

Headless twin, same rules: `app/src-tauri/target/debug/studio-cli info`.

## Using the app

- **Review Queue** (opens first): approve or reject what the agents made. A failed check can only be rejected. Approving records your yes; it never publishes.
- **Agents**: the 12 agents and what each is doing right now.
- **Vault**: read any note; see which are verified.
- **Runs**: start a run (you always see the cost and a hard cap first) and open any run to see its steps, what it produced, how it was checked, and estimated vs actual cost.
- **Settings**:
  - **Claude**: shows whether Claude Code is signed in with your Claude account. Agents run on your subscription. The app never asks for an API key and strips `ANTHROPIC_API_KEY` from agent runs. "Sign In to Claude" opens Terminal running `claude auth login`.
  - **Budget**: a monthly limit. Runs stop and ask once this month's spend would pass it.
  - **Notifications**: paste a Discord webhook (Server Settings, Integrations, Webhooks, New Webhook, Copy URL). You get a message when something needs approval, a build finishes, a run fails, or the budget stops a run. Stored on your Mac only (`~/Library/Application Support/com.andrewdieu.robuxstudio/settings.json`), never in the repo.
  - **Import a game project**: copies a Rojo game (for example `~/workspace/roblox-games/sky-tower-obby`) into `projects/`.
  - **Updates**: "Check for Updates" fetches this repo's branch from GitHub; "Update and Restart" pulls, rebuilds the app the way it is running (dev, release, or the .app bundle), and restarts it. It refuses while an agent is running or if files were edited by hand. No release server and nothing public: it uses your existing git access.

## Cost

Agent runs cost Claude usage. The app shows an estimate and a hard cap before
every run and will not start without approval. On a Claude Max plan
($100 to $200 a month) runs count against plan limits; the dollar figures are
the API-equivalent cost.

## Pipelines

`studio/pipelines.json` chains agents. You approve the whole chain once, at
the total cap shown; each stage then runs under its own cap. After each stage
the studio checks the work itself (`scripts/gates/check_stage.py`, and for
Builder a real `rojo build`, Selene, StyLua, and Lune test run), then puts one
item in the review queue. A failed gate can only be rejected.

- `pitch-to-build`: Pulse, Lead, Architect, Builder.
- `improve-build`: Architect, Builder, Playtest on an existing project.

Headless: `studio-cli pipeline pitch-to-build "<instruction>"` prints the
estimate and stops; add `--approve-cap <the cap it printed>` to run.

## CI (free)

`.github/workflows/build.yml` runs on every push: Rust tests, the full vault
check, the vault's Luau templates through Rojo/Selene/StyLua/Lune, and a real
macOS build of `Robux Studio.app` (kept as a 7-day workflow artifact, not
published).

## Toolchain for the build gate (free)

`cargo install rojo --version 7.7.1 --locked`, `cargo install selene --locked`,
`cargo install stylua --features luau --locked`, `cargo install lune --version 0.11.1 --locked`.
On a machine that can reach GitHub, Rokit or Aftman installs the same tools faster.
