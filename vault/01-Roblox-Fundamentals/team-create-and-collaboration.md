---
description: Working in a team on a Roblox game: Team Create sessions, edit access and groups, early testers, Team Test, MCP permissions per person, plus workflow rules for AI agents and humans.
type: playbook
status: draft
last_checked: 2026-11-15
keywords: team create, collaboration, edit access, group, roles, team test, early testers, collaborators, studio, place size, source control, permissions, rojo, merge conflicts
sources: creator-docs content/en-us/studio/testing-modes.md (read 2026-11-15); creator-docs content/en-us/production/publishing/publish-games-and-places.md (read 2026-11-15); creator-docs content/en-us/reference/engine/enums/CollaboratorStatus.yaml (read 2026-11-15); creator-docs content/en-us/reference/engine/enums/TeamCreateErrorState.yaml (read 2026-11-15); creator-docs content/en-us/studio/mcp-open-cloud.md (read 2026-11-15); creator-docs content/en-us/production/promotion/content-ratings.md (read 2026-11-15)
verify: The dedicated Team Create guide (projects/collaboration) is NOT in the cloned docs, so session setup steps and permission levels are UNVERIFIED. What is listed below is only what other pages and enum references state. Check the Creator Hub docs before relying on it.
tags: [roblox-fundamentals, playbook]
---
# Team Create and collaboration

How several people (and AI agents) work on one Roblox game safely. The dedicated Team Create page was not in our docs clone, so this note keeps to what related pages confirm.

## Do this
- Give each teammate the lowest access that works: Edit access for builders, Playtest access for testers, early tester invites for outside friends. Group games use roles and permissions, adjustable per game.
- Split ownership by area (map, UI, scripts, audio) and tell the team before touching someone else's area.
- Keep code in version control through our toolchain ([[13-Luau-Engineering/toolchain]], [[13-Luau-Engineering/project-structure]]), not only inside the live Team Create place.
- Watch place size: Studio reports save status (size approaching or over the limit, repeated upload failures). Fix big meshes and duplicated assets when you see a warning.
- Test together with Team Test, and end the session when done: only one team test can run at a time.
- Review every AI agent change like a teammate's change: small commits, a headless test gate ([[13-Luau-Engineering/headless-test-gate]]).
- Remove access when someone leaves the project.

## Access and people (from the docs)
- New games are accessible to the owner, users with Edit access and group members with the right role. Group owners or members with enough rights can change access per game.
- Early testers: up to 10 similar-age friends before public release; manage them with Manage Collaborators in Studio. Under-9 testers need parent approval.
- If a game gets 18+ only content, Studio removes every collaborator who is not age-verified 18+ from active sessions. Plan staffing for that.
- Analytics and alerts need analytics permission or specific permissions such as View all analytics and Create and configure alerts. Group editors get alert creation by default.
- Early Access Program accounts may collaborate only with friends in the same program.

## Presence and status
- Collaborators have a status: not active, editing in the 3D viewport, scripting a server-replicated script, or scripting a local or private script. Use it to avoid two people editing one script.
- CollaboratorsService enforces server-side access checks. Do not set statuses from client code.

## Team Test
- In Studio choose Team Test in the testing dropdown and press Play; collaborators press Play to join with their characters. End Session kicks all testers.
- For solo multi-client checks use Server and Clients (up to 8 simulated clients in StudioTestService) before inviting people.
- Network Simulator adds delay on top of real internet latency during Team Test, so results vary by geography.

## Working with MCP and agents
- Studio MCP permissions are per person: each collaborator sets their own, and enabling it for yourself does not enable it for others. See [[19-Project-Memory/how-project-memory-works]].
- Keep agents inside one area per task and require a diff summary and a playtest note.

## Habits that prevent lost work
1. Pull the latest, make one change, save, push.
2. Name places and scripts clearly; avoid renaming in the middle of a team session.
3. Publish from one person at a time. Announce before publishing or restarting servers.
4. Keep a short changelog for each release ([[17-Development-Templates/update-checklist]]).

## Roles on release day
- Publisher: one person who clicks Publish and restarts servers.
- Tester: runs the release checklist ([[14-QA-Optimization/playtest-qa-checklist]]) on phone and desktop.
- Watcher: reads Error Report and Performance for the first hour after launch.
- Writer: updates the changelog and the patch note players see.

## Common problems
- Two people edit the same script: check the collaborator status first, or move the work into source files.
- A save fails because the place is too big: remove duplicate meshes and unused assets, then retry.
- A teammate cannot join a test: check access level, age limits and any Early Access restriction.

## For our Roblox games
Copy the principle: least access, one owner per area, one publisher at a time, and a test pass before every release.

Never copy: other teams' private place files, models or scripts, or access to anyone's account. Do not share passwords or let outsiders edit without invites.
