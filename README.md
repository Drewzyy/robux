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
| `studio/` | App config: `agents.json` (models, cost profiles, tool allowlists), `pricing.json` |
| `queue/items/` | Review queue items (JSON) |
| `runs/` | One folder per agent run: estimate, approval, raw output, files loaded |
| `projects/` | Game projects the agents build (Rojo) |
| `scripts/` | Verification scripts |
| `LOG.md` | Build log: what was done and how it was verified |

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

## Cost

Agent runs cost Claude usage. The app shows an estimate and a hard cap before
every run and will not start without approval. On a Claude Max plan
($100 to $200 a month) runs count against plan limits; the dollar figures are
the API-equivalent cost.
