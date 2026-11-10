#!/bin/sh
# Pack the studio files that ship inside the downloaded app: the vault (with
# its agents, skills, hook settings and Obsidian config) and the studio config.
# Never includes runs, projects, the review queue, or anything personal.
set -e
cd "$(dirname "$0")/.."
tar -czf app/src-tauri/studio-template.tar.gz \
  --exclude 'vault/.claude/logs' --exclude 'vault/.obsidian/workspace*' \
  vault studio runs/README.md queue/items/.gitkeep
echo "wrote app/src-tauri/studio-template.tar.gz"
