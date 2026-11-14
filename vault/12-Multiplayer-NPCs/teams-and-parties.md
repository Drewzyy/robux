---
description: Implement team assignment, team balance and friend parties: Teams service, auto-balance rules, friendly-fire checks, party invites and keeping friends in the same server.
type: playbook
status: draft
last_checked: 2026-11-14
keywords: teams, team balance, party, friends, join friend, invite, friendly fire, SpawnLocation, TeamColor, Team service, SocialService, group queue
sources: creator-docs content/en-us/production/promotion/invite-prompts.md (invite prompts, read 2026-11-14); creator-docs content/en-us/resources/modules/friends-locator.md (read 2026-11-14); creator-docs content/en-us/reference/engine/enums/MatchmakingType.yaml (read 2026-11-14)
verify: Teams, Team, SpawnLocation and SocialService API names are from model knowledge; class pages not in the local docs clone, UNVERIFIED. Check invite-prompts.md for current invite behaviour.
tags: [multiplayer-npcs, playbook]
---
# Teams and parties

Teams split players into sides. Parties keep friends together. Build them separately: a party is a social group that may span teams.

## Do this
- Create `Team` objects in the `Teams` service from the server: name, `TeamColor`, `AutoAssignable = false`. You assign manually so balance is under your control.
- Balance by count first, then by a skill proxy (level or win rate). Assign each new player to the smaller team; if equal, to the team with lower total skill.
- Keep parties together: put all members of a party on the same team unless the party is larger than half the match. Then split by order of join.
- Rebalance only between rounds. Never move a player mid-round unless a team is empty (then move one, with a message).
- Friendly fire off by default. Check team in the damage function on the server: `if attacker.Team == victim.Team then return end`.
- Use `SpawnLocation` parts with `Neutral = false` and `TeamColor` matching, so respawn goes to the right base.
- Send party invites with the built-in social prompt (SocialService) rather than a custom chat system. The prompt is Roblox UI, so it is safe and free.
- Show teammates: name tag color, minimap dots. Seeing allies is the whole point.

## Auto-balance

```lua
--!strict
local Players = game:GetService("Players")
local Teams = game:GetService("Teams")

local function teamCounts(): { [Team]: number }
	local counts: { [Team]: number } = {}
	for _, team in Teams:GetTeams() do
		counts[team] = 0
	end
	for _, player in Players:GetPlayers() do
		local team = player.Team
		if team and counts[team] ~= nil then
			counts[team] += 1
		end
	end
	return counts
end

local function assignSmallest(player: Player)
	local counts = teamCounts()
	local best: Team? = nil
	local bestCount = math.huge
	for team, count in counts do
		if count < bestCount then
			best = team
			bestCount = count
		end
	end
	if best then
		player.Team = best
	end
end

Players.PlayerAdded:Connect(assignSmallest)
```

`Teams:GetTeams()` returns the Team children. Setting `player.Team` also sets `TeamColor`.

## Party model
- Party lives on the server: `{ leader: Player, members: { Player } }`. Max size 4 for casual modes (UNVERIFIED default).
- Invite: server checks both players are in the same server, neither is in another party, target has not blocked the leader (use Roblox friends only; do not build your own block list).
- If the leader leaves, promote the next member; if only one remains, disband.
- To join a friend in another server, use the join-friend flow from Roblox itself (friends list) or `TeleportAsync` to the friend's `JobId` via the friend-locator module pattern. See friends-locator.md in the docs.

## Edge cases
- Odd player count: the extra player goes to the team with lower total skill.
- Team empties (all leave): end the round with a win to the other team, do not wait.
- Player rejoins after crash: put them back on their old team for the current round if the round is still `Playing`.
- Spectators have no team (`player.Neutral = true`).

## Mistakes
- Relying on `AutoAssignable` and then fighting it.
- Checking team on the client for damage.
- Splitting a party mid-round because of rebalance.
- Hard-coding team count; read from `Teams`.

## Related
[[12-Multiplayer-NPCs/matchmaking-and-lobbies]], [[12-Multiplayer-NPCs/round-system-pattern]], [[12-Multiplayer-NPCs/multiplayer-patterns]], [[13-Luau-Engineering/remote-validation-cookbook]]

## For our Roblox games
Copy the principle: server-owned teams, count-first balance, parties stay together, no friendly fire. Never copy another game's team names, colors as brand marks, emblem art or party UI. Zero spend: Teams and SocialService are free built-ins.
