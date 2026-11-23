--!nonstrict
-- Study a Roblox place file without running anything in it.
--   lune run scripts/rbxl_study.luau <place.rbxl> <out folder>
-- Writes: every script's source under <out>/scripts/<path>.luau, and
-- <out>/summary.md (what the game is made of, biggest scripts, UI, and a
-- scan for backdoors: require(<asset id>), loadstring, getfenv, outside
-- web calls, obfuscated text).

local fs = require("@lune/fs")
local process = require("@lune/process")
local roblox = require("@lune/roblox")

local file, out = process.args[1], process.args[2]
assert(file and out, "usage: lune run scripts/rbxl_study.luau <place.rbxl> <out folder>")

local game = roblox.deserializePlace(fs.readFile(file))
fs.writeDir(out .. "/scripts")

local classes: { [string]: number } = {}
local scripts = {}
local guis, parts, meshes, sounds, anims = 0, 0, 0, 0, 0
local flags = {}

local SCRIPT = { Script = true, LocalScript = true, ModuleScript = true }
local RISK = {
	{ "require%(%s*%d%d%d%d+%s*%)", "require(<asset id>): loads outside code (classic backdoor)" },
	{ "loadstring", "loadstring: runs text as code" },
	{ "getfenv", "getfenv: environment tricks (often obfuscation)" },
	{ "setfenv", "setfenv: environment tricks" },
	{ "HttpService.-PostAsync", "sends data to a web server" },
	{ "HttpService.-GetAsync", "downloads from a web server" },
	{ "\\%d%d%d\\%d%d%d\\%d%d%d", "escaped byte strings (obfuscated text)" },
	{ "string%.reverse", "reversed strings (often hides asset ids)" },
	{ "MarketplaceService.-PromptPurchase", "prompts a purchase (check it is the game's own)" },
}

local function pathOf(inst)
	local parts_ = {}
	local cur = inst
	while cur and cur ~= game do
		table.insert(parts_, 1, (cur.Name:gsub("[^%w%-_ ]", "_")))
		cur = cur.Parent
	end
	return table.concat(parts_, "/")
end

for _, inst in game:GetDescendants() do
	local c = inst.ClassName
	classes[c] = (classes[c] or 0) + 1
	if inst:IsA("GuiObject") then
		guis += 1
	elseif inst:IsA("BasePart") then
		parts += 1
	end
	if c == "MeshPart" or c == "SpecialMesh" then
		meshes += 1
	elseif c == "Sound" then
		sounds += 1
	elseif c == "Animation" then
		anims += 1
	end
	if SCRIPT[c] then
		local ok, src = pcall(function()
			return inst.Source
		end)
		src = ok and src or ""
		local p = pathOf(inst)
		local lines = select(2, src:gsub("\n", "")) + 1
		table.insert(scripts, { path = p, class = c, lines = lines, disabled = (c ~= "ModuleScript" and inst.Disabled) or false })
		local dir = out .. "/scripts/" .. p:match("^(.*)/[^/]*$")
		pcall(fs.writeDir, dir)
		pcall(fs.writeFile, out .. "/scripts/" .. p .. ".luau", "-- " .. c .. "\n" .. src)
		for _, r in RISK do
			if src:find(r[1]) then
				table.insert(flags, string.format("- `%s` (%s): %s", p, c, r[2]))
			end
		end
	end
end

table.sort(scripts, function(a, b)
	return a.lines > b.lines
end)
local classList = {}
for k, v in classes do
	table.insert(classList, { k, v })
end
table.sort(classList, function(a, b)
	return a[2] > b[2]
end)

local total = 0
for _, s in scripts do
	total += s.lines
end
local md = { "# " .. file:match("[^/]*$"), "" }
table.insert(md, string.format("Scripts: %d (%d lines). Parts: %d. Meshes: %d. UI objects: %d. Sounds: %d. Animations: %d.", #scripts, total, parts, meshes, guis, sounds, anims))
table.insert(md, "\n## Possible backdoors and risks (" .. #flags .. ")")
if #flags == 0 then
	table.insert(md, "None found by the scan.")
end
for _, f in flags do
	table.insert(md, f)
end
table.insert(md, "\n## Biggest scripts")
for i = 1, math.min(25, #scripts) do
	local s = scripts[i]
	table.insert(md, string.format("- %s `%s` %d lines%s", s.class, s.path, s.lines, s.disabled and " (disabled)" or ""))
end
table.insert(md, "\n## Most used classes")
for i = 1, math.min(25, #classList) do
	table.insert(md, string.format("- %s: %d", classList[i][1], classList[i][2]))
end
fs.writeFile(out .. "/summary.md", table.concat(md, "\n") .. "\n")
print(string.format("%s: %d scripts, %d lines, %d risk flags", file:match("[^/]*$"), #scripts, total, #flags))
