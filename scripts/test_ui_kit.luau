local roblox = require("@lune/roblox")
local fs = require("@lune/fs")
local luau = require("@lune/luau")
local Instance = roblox.Instance
local KIT = "studio/kits/ui/UI/" -- run from the repo root: lune run scripts/test_ui_kit.luau

local env = {}
for _, k in {"Color3","Vector2","UDim","UDim2","ColorSequence","Enum","Instance"} do env[k] = roblox[k] end
env.TweenInfo = roblox.TweenInfo or { new = function(...) return {...} end }
-- Real instances (property names and types are checked) behind a thin
-- proxy that adds the events Lune does not have.
local EVENTS = { MouseEnter = 1, MouseLeave = 1, MouseButton1Down = 1, MouseButton1Up = 1, Activated = 1 }
local real = setmetatable({}, { __mode = "k" })
local function unwrap(v) return real[v] or v end
local stub = { Connect = function() end }
env.Instance = { new = function(cls)
	local inst = Instance.new(cls)
	local p = setmetatable({}, {
		__index = function(_, k)
			if EVENTS[k] then return stub end
			local v = inst[k]
			if type(v) == "function" then return function(_, ...) return v(inst, ...) end end
			return v
		end,
		__newindex = function(_, k, v) inst[k] = unwrap(v) end,
	})
	real[p] = inst
	return p
end }
local fakeTween = { Completed = { Once = function(_, f) f() end }, Play = function() end }
local services = {
	Players = { LocalPlayer = { WaitForChild = function() return Instance.new("Folder") end } },
	TweenService = { Create = function(_, inst, info, goal)
		for k, v in goal do inst[k] = v end
		return fakeTween
	end },
	UserInputService = { TouchEnabled = true },
	Workspace = { CurrentCamera = { ViewportSize = roblox.Vector2.new(844, 390), GetPropertyChangedSignal = function() return { Connect = function() end } end } },
}
env.game = { GetService = function(_, n) return services[n] end }
env.task = { delay = function() end }
env.math, env.string, env.table, env.pairs, env.ipairs, env.setmetatable, env.tostring, env.type, env.print, env.error = math, string, table, pairs, ipairs, setmetatable, tostring, type, print, error

local function load(name, extra)
	local src = fs.readFile(KIT .. name)
	local e = {}
	for k, v in env do e[k] = v end
	for k, v in extra or {} do e[k] = v end
	return luau.load(src, { debugName = name, environment = e })()
end
local Theme = load("Theme.luau")
local UI = load("init.luau", { require = function() return Theme end, script = { Theme = true } })

-- every component, with real Roblox instances (bad property names error)
local screen = UI.Screen("Test")
local rawScreen = real[screen]
assert(rawScreen:FindFirstChildOfClass("UIScale").Scale > 0.55, "phone scale")
local win = UI.Window(screen, { Title = "Shop", Size = roblox.Vector2.new(760, 520) })
UI.Button(win.Body, { Text = "Buy", OnClick = function() end, Icon = "rbxassetid://1" })
local bar = UI.Bar(win.Body, { Value = 0.3, Text = "3/10" }); bar:Set(0.6, "6/10")
local c = UI.Counter(win.Body, { Value = 1250, Icon = "rbxassetid://2" }); c:Set(2500000)
local grid = UI.Grid(win.Body)
UI.Card(grid, { Title = "Dragon", Image = "rbxassetid://3", ButtonText = "99", RarityColor = Theme.Colors.Legendary })
UI.Toast(screen, "Saved!")
win:Open(); win:Close(); win:Toggle()
assert(UI.abbreviate(1250) == "1.3K" or UI.abbreviate(1250) == "1.2K", UI.abbreviate(1250))
assert(UI.abbreviate(2500000) == "2.5M", UI.abbreviate(2500000))
assert(UI.abbreviate(1000) == "1K", UI.abbreviate(1000))
assert(UI.abbreviate(150000) == "150K", UI.abbreviate(150000))
assert(UI.abbreviate(42) == "42")
local s = UI.scaleFor(roblox.Vector2.new(844, 390), true)
print("phone scale", s, "-> button height px", 72 * s)
print("pc scale", UI.scaleFor(roblox.Vector2.new(1920, 1080), false))
print("instances", #rawScreen:GetDescendants())
print("ALL OK")
