--!nonstrict
-- BrewixGames UI kit: build every screen from these parts so the whole game
-- looks finished and consistent (rounded corners, outlines, gradients,
-- shadows, press feedback, open animations) on phones and PCs.
--
--   local UI = require(ReplicatedStorage.Shared.UI)
--   local screen = UI.Screen("Shop")
--   local win = UI.Window(screen, { Title = "Shop", Size = Vector2.new(760, 520) })
--   UI.Button(win.Body, { Text = "Buy", Color = UI.Theme.Colors.Primary, OnClick = buy })
--   win:Open()
--
-- Everything is laid out at a 1280x720 design size; Screen() adds a UIScale
-- that fits it to the device, so sizes here are design pixels.

local Players = game:GetService("Players")
local TweenService = game:GetService("TweenService")
local UserInputService = game:GetService("UserInputService")
local Workspace = game:GetService("Workspace")

local Theme = require(script.Theme)

local UI = {}
UI.Theme = Theme

local C = Theme.Colors

-- Create an instance with properties and children in one call.
function UI.New(className: string, props: { [string]: any }?, children: { Instance }?): Instance
	local inst = Instance.new(className)
	local parent = nil
	for k, v in pairs(props or {}) do
		if k == "Parent" then
			parent = v
		else
			inst[k] = v
		end
	end
	for _, child in ipairs(children or {}) do
		child.Parent = inst
	end
	inst.Parent = parent
	return inst
end
local New = UI.New

local function tween(inst: Instance, info: TweenInfo, goal: { [string]: any })
	local t = TweenService:Create(inst, info, goal)
	t:Play()
	return t
end
UI.tween = tween

local function corner(radius: number): UICorner
	if radius >= 999 then
		return New("UICorner", { CornerRadius = UDim.new(0.5, 0) })
	end
	return New("UICorner", { CornerRadius = UDim.new(0, radius) })
end

local function stroke(color: Color3, thickness: number): UIStroke
	return New("UIStroke", {
		Color = color,
		Thickness = thickness,
		ApplyStrokeMode = Enum.ApplyStrokeMode.Border,
	})
end

local function gradient(top: Color3, bottom: Color3): UIGradient
	return New("UIGradient", {
		Color = ColorSequence.new(top, bottom),
		Rotation = 90,
	})
end

-- 1.2K, 3.4M, 5.6B
function UI.abbreviate(n: number): string
	local abs = math.abs(n)
	local units = { { 1e12, "T" }, { 1e9, "B" }, { 1e6, "M" }, { 1e3, "K" } }
	for _, u in ipairs(units) do
		if abs >= u[1] then
			local v = n / u[1]
			local s = if math.abs(v) >= 100 then string.format("%d", v) else string.format("%.1f", v)
			return (s:gsub("%.0$", "")) .. u[2]
		end
	end
	return tostring(math.floor(n))
end

-- The device scale: the 1280x720 design fitted to the screen, a little
-- bigger on touch devices so buttons stay easy to tap.
function UI.scaleFor(viewport: Vector2, touch: boolean): number
	local s = math.min(viewport.X / 1280, viewport.Y / 720)
	if touch then
		s *= 1.25
	end
	return math.clamp(s, 0.55, 1.3)
end

-- A ScreenGui for one screen, kept out of the notch and Roblox's top bar,
-- with a UIScale that follows the window size.
function UI.Screen(name: string, props: { [string]: any }?): ScreenGui
	local player = Players.LocalPlayer
	local gui = New("ScreenGui", {
		Name = name,
		ResetOnSpawn = false,
		ZIndexBehavior = Enum.ZIndexBehavior.Sibling,
		ScreenInsets = Enum.ScreenInsets.CoreUISafeInsets,
		DisplayOrder = props and props.DisplayOrder or 0,
		Parent = player:WaitForChild("PlayerGui"),
	})
	local scale = New("UIScale", { Parent = gui })
	local function fit()
		local cam = Workspace.CurrentCamera
		if cam then
			scale.Scale = UI.scaleFor(cam.ViewportSize, UserInputService.TouchEnabled)
		end
	end
	fit()
	if Workspace.CurrentCamera then
		Workspace.CurrentCamera:GetPropertyChangedSignal("ViewportSize"):Connect(fit)
	end
	return gui
end

-- Text with an outline, in the game font.
function UI.Label(parent: Instance, props: { [string]: any }?): TextLabel
	props = props or {}
	local label = New("TextLabel", {
		Name = props.Name or "Label",
		BackgroundTransparency = 1,
		Size = props.Size or UDim2.new(1, 0, 0, (props.TextSize or Theme.TextSize.Body) + 8),
		Position = props.Position or UDim2.new(),
		AnchorPoint = props.AnchorPoint or Vector2.new(),
		Font = props.Font or Theme.Font,
		Text = props.Text or "",
		TextSize = props.TextSize or Theme.TextSize.Body,
		TextColor3 = props.Color or C.Text,
		TextXAlignment = props.Align or Enum.TextXAlignment.Center,
		TextWrapped = true,
		RichText = true,
		LayoutOrder = props.LayoutOrder or 0,
		Parent = parent,
	}, {
		stroke(C.TextStroke, Theme.Stroke.Text),
	})
	label:FindFirstChildOfClass("UIStroke").ApplyStrokeMode = Enum.ApplyStrokeMode.Contextual
	return label
end

-- A rounded panel with an outline, a soft top-lit gradient and a shadow.
-- Returns the panel; put content in panel.Body (padded).
function UI.Panel(parent: Instance, props: { [string]: any }?)
	props = props or {}
	local fill = props.Color or C.Panel
	local holder = New("Frame", {
		Name = props.Name or "Panel",
		BackgroundTransparency = 1,
		Size = props.Size or UDim2.fromOffset(400, 300),
		Position = props.Position or UDim2.fromScale(0.5, 0.5),
		AnchorPoint = props.AnchorPoint or Vector2.new(0.5, 0.5),
		LayoutOrder = props.LayoutOrder or 0,
		Parent = parent,
	})
	New("Frame", {
		Name = "Shadow",
		BackgroundColor3 = C.Shadow,
		BackgroundTransparency = 0.6,
		Size = UDim2.fromScale(1, 1),
		Position = UDim2.fromOffset(0, 6),
		ZIndex = 0,
		Parent = holder,
	}, { corner(props.Radius or Theme.Radius.Panel) })
	local panel = New("Frame", {
		Name = "Fill",
		BackgroundColor3 = Color3.new(1, 1, 1),
		Size = UDim2.fromScale(1, 1),
		ZIndex = 1,
		Parent = holder,
	}, {
		corner(props.Radius or Theme.Radius.Panel),
		stroke(Theme.darker(fill, 0.55), Theme.Stroke.Panel),
		gradient(Theme.lighter(fill, 0.12), fill),
	})
	local body = New("Frame", {
		Name = "Body",
		BackgroundTransparency = 1,
		Size = UDim2.fromScale(1, 1),
		ZIndex = 2,
		Parent = panel,
	}, {
		New("UIPadding", {
			PaddingTop = UDim.new(0, Theme.Space.Pad),
			PaddingBottom = UDim.new(0, Theme.Space.Pad),
			PaddingLeft = UDim.new(0, Theme.Space.Pad),
			PaddingRight = UDim.new(0, Theme.Space.Pad),
		}),
	})
	return setmetatable({ Instance = holder, Fill = panel, Body = body }, { __index = holder })
end

-- Lay children out in a row or column with even gaps.
function UI.List(parent: Instance, direction: "row" | "column"?, gap: number?, align: Enum.HorizontalAlignment?)
	return New("UIListLayout", {
		FillDirection = if direction == "row" then Enum.FillDirection.Horizontal else Enum.FillDirection.Vertical,
		Padding = UDim.new(0, gap or Theme.Space.Gap),
		SortOrder = Enum.SortOrder.LayoutOrder,
		HorizontalAlignment = align or Enum.HorizontalAlignment.Center,
		VerticalAlignment = Enum.VerticalAlignment.Center,
		Parent = parent,
	})
end

-- A chunky button: gradient fill, dark outline, outlined text, optional
-- icon, grows on hover and squashes on press. OnClick uses Activated, so it
-- works with mouse, touch and gamepad.
function UI.Button(parent: Instance, props: { [string]: any }): TextButton
	local color = props.Color or C.Primary
	local h = props.Height or Theme.Button.Height
	local button = New("TextButton", {
		Name = props.Name or (props.Text or "Button"),
		AutoButtonColor = false,
		Text = "",
		BackgroundColor3 = Color3.new(1, 1, 1),
		Size = props.Size or UDim2.fromOffset(math.max(Theme.Button.MinWidth, props.Width or 220), h),
		Position = props.Position or UDim2.new(),
		AnchorPoint = props.AnchorPoint or Vector2.new(),
		LayoutOrder = props.LayoutOrder or 0,
		Parent = parent,
	}, {
		corner(props.Radius or Theme.Radius.Button),
		stroke(Theme.darker(color, 0.5), Theme.Stroke.Button),
		gradient(Theme.lighter(color, 0.2), color),
	})
	local scale = New("UIScale", { Parent = button })
	local row = New("Frame", {
		BackgroundTransparency = 1,
		Size = UDim2.fromScale(1, 1),
		Parent = button,
	})
	UI.List(row, "row", 8)
	if props.Icon then
		New("ImageLabel", {
			Name = "Icon",
			BackgroundTransparency = 1,
			Image = props.Icon,
			ScaleType = Enum.ScaleType.Fit,
			Size = UDim2.fromOffset(h * 0.55, h * 0.55),
			LayoutOrder = 1,
			Parent = row,
		})
	end
	local label = UI.Label(row, {
		Text = props.Text or "",
		TextSize = props.TextSize or Theme.TextSize.Button,
		Size = UDim2.fromScale(0, 1),
		LayoutOrder = 2,
	})
	label.AutomaticSize = Enum.AutomaticSize.X
	label.TextWrapped = false

	button.MouseEnter:Connect(function()
		tween(scale, Theme.Motion.Hover, { Scale = 1.06 })
	end)
	button.MouseLeave:Connect(function()
		tween(scale, Theme.Motion.Hover, { Scale = 1 })
	end)
	button.MouseButton1Down:Connect(function()
		tween(scale, Theme.Motion.Press, { Scale = 0.94 })
	end)
	button.MouseButton1Up:Connect(function()
		tween(scale, Theme.Motion.Hover, { Scale = 1 })
	end)
	if props.OnClick then
		button.Activated:Connect(props.OnClick)
	end
	return button
end

-- A round icon button (close, settings, menu). At least 56 design px.
function UI.IconButton(parent: Instance, props: { [string]: any }): TextButton
	local size = math.max(56, props.Size or 64)
	return UI.Button(parent, {
		Name = props.Name or "IconButton",
		Text = props.Text or "",
		Icon = props.Icon,
		Color = props.Color or C.Info,
		Radius = Theme.Radius.Pill,
		Size = UDim2.fromOffset(size, size),
		Height = size,
		Position = props.Position,
		AnchorPoint = props.AnchorPoint,
		LayoutOrder = props.LayoutOrder,
		TextSize = props.TextSize or Theme.TextSize.Title,
		OnClick = props.OnClick,
	})
end

-- A progress bar. bar:Set(0.6) animates to 60%.
function UI.Bar(parent: Instance, props: { [string]: any }?)
	props = props or {}
	local color = props.Color or C.Primary
	local back = New("Frame", {
		Name = props.Name or "Bar",
		BackgroundColor3 = C.Inset,
		Size = props.Size or UDim2.fromOffset(360, 28),
		Position = props.Position or UDim2.new(),
		AnchorPoint = props.AnchorPoint or Vector2.new(),
		LayoutOrder = props.LayoutOrder or 0,
		Parent = parent,
	}, { corner(Theme.Radius.Pill), stroke(C.PanelStroke, Theme.Stroke.Panel) })
	local fill = New("Frame", {
		Name = "Fill",
		BackgroundColor3 = Color3.new(1, 1, 1),
		Size = UDim2.fromScale(math.clamp(props.Value or 0, 0, 1), 1),
		Parent = back,
	}, { corner(Theme.Radius.Pill), gradient(Theme.lighter(color, 0.25), color) })
	local text =
		UI.Label(back, { Text = props.Text or "", TextSize = Theme.TextSize.Small, Size = UDim2.fromScale(1, 1) })
	text.ZIndex = 3
	local bar = { Instance = back, Label = text }
	function bar:Set(value: number, label: string?)
		tween(fill, Theme.Motion.Bar, { Size = UDim2.fromScale(math.clamp(value, 0, 1), 1) })
		if label then
			text.Text = label
		end
	end
	return bar
end

-- A currency pill: icon + abbreviated amount that pops when it changes.
-- counter:Set(1250) shows "1.3K".
function UI.Counter(parent: Instance, props: { [string]: any }?)
	props = props or {}
	local pill = New("Frame", {
		Name = props.Name or "Counter",
		BackgroundColor3 = C.Inset,
		BackgroundTransparency = 0.15,
		Size = props.Size or UDim2.fromOffset(200, 56),
		Position = props.Position or UDim2.new(),
		AnchorPoint = props.AnchorPoint or Vector2.new(),
		LayoutOrder = props.LayoutOrder or 0,
		Parent = parent,
	}, {
		corner(Theme.Radius.Pill),
		stroke(C.PanelStroke, Theme.Stroke.Panel),
		New("UIPadding", { PaddingLeft = UDim.new(0, 8), PaddingRight = UDim.new(0, 16) }),
	})
	UI.List(pill, "row", 8, Enum.HorizontalAlignment.Left)
	local scale = New("UIScale", { Parent = pill })
	if props.Icon then
		New("ImageLabel", {
			BackgroundTransparency = 1,
			Image = props.Icon,
			ScaleType = Enum.ScaleType.Fit,
			Size = UDim2.fromOffset(40, 40),
			LayoutOrder = 1,
			Parent = pill,
		})
	end
	local amount = UI.Label(pill, {
		Text = UI.abbreviate(props.Value or 0),
		Color = props.Color or C.Gold,
		TextSize = Theme.TextSize.Button,
		Size = UDim2.fromScale(0, 1),
		Align = Enum.TextXAlignment.Left,
		LayoutOrder = 2,
	})
	amount.AutomaticSize = Enum.AutomaticSize.X
	amount.TextWrapped = false
	local counter = { Instance = pill, Value = props.Value or 0 }
	function counter:Set(value: number)
		if value == self.Value then
			return
		end
		self.Value = value
		amount.Text = UI.abbreviate(value)
		scale.Scale = 1.15
		tween(scale, Theme.Motion.Pop, { Scale = 1 })
	end
	return counter
end

-- A window: panel + title + close button, hidden until :Open().
-- Content goes in window.Body (laid out as a column by default).
function UI.Window(screen: Instance, props: { [string]: any })
	local size = props.Size or Vector2.new(720, 480)
	local panel = UI.Panel(screen, {
		Name = props.Name or props.Title or "Window",
		Size = UDim2.fromOffset(size.X, size.Y),
		Position = UDim2.fromScale(0.5, 0.5),
		AnchorPoint = Vector2.new(0.5, 0.5),
		Color = props.Color,
	})
	local holder = panel.Instance
	holder.Visible = false
	local scale = New("UIScale", { Parent = holder })
	UI.Label(panel.Body, {
		Name = "Title",
		Text = props.Title or "",
		TextSize = Theme.TextSize.Title,
		Size = UDim2.new(1, -80, 0, Theme.TextSize.Title + 10),
		Position = UDim2.fromOffset(0, -4),
	})
	local window = { Instance = holder, IsOpen = false }
	local content = New("Frame", {
		Name = "Content",
		BackgroundTransparency = 1,
		Size = UDim2.new(1, 0, 1, -(Theme.TextSize.Title + 18)),
		Position = UDim2.fromOffset(0, Theme.TextSize.Title + 18),
		Parent = panel.Body,
	})
	window.Body = content
	if props.Layout ~= false then
		UI.List(content, "column")
	end
	UI.IconButton(holder, {
		Name = "Close",
		Text = "X",
		Color = C.Danger,
		Size = 56,
		Position = UDim2.new(1, 12, 0, -12),
		AnchorPoint = Vector2.new(1, 0),
		OnClick = function()
			window:Close()
		end,
	}).ZIndex =
		5
	function window:Open()
		if self.IsOpen then
			return
		end
		self.IsOpen = true
		holder.Visible = true
		scale.Scale = 0.8
		tween(scale, Theme.Motion.Open, { Scale = 1 })
		if props.OnOpen then
			props.OnOpen()
		end
	end
	function window:Close()
		if not self.IsOpen then
			return
		end
		self.IsOpen = false
		local t = tween(scale, Theme.Motion.Close, { Scale = 0.85 })
		t.Completed:Once(function()
			if not self.IsOpen then
				holder.Visible = false
			end
		end)
		if props.OnClose then
			props.OnClose()
		end
	end
	function window:Toggle()
		if self.IsOpen then
			self:Close()
		else
			self:Open()
		end
	end
	return window
end

-- A short message that slides in at the top and fades out.
function UI.Toast(screen: Instance, text: string, color: Color3?)
	local panel = UI.Panel(screen, {
		Name = "Toast",
		Size = UDim2.fromOffset(520, 64),
		Position = UDim2.new(0.5, 0, 0, -80),
		AnchorPoint = Vector2.new(0.5, 0),
		Color = color or C.Info,
		Radius = Theme.Radius.Pill,
	})
	UI.Label(panel.Body, { Text = text, Size = UDim2.fromScale(1, 1) })
	local holder = panel.Instance
	tween(holder, Theme.Motion.Open, { Position = UDim2.new(0.5, 0, 0, Theme.Space.Edge) })
	task.delay(2.2, function()
		local t = tween(holder, Theme.Motion.Close, { Position = UDim2.new(0.5, 0, 0, -80) })
		t.Completed:Once(function()
			holder:Destroy()
		end)
	end)
	return holder
end

-- A grid of item cards (shop, inventory, pets). Returns the grid frame;
-- add cards with UI.Card(grid, ...).
function UI.Grid(parent: Instance, cell: Vector2?, gap: number?): ScrollingFrame
	local grid = New("ScrollingFrame", {
		Name = "Grid",
		BackgroundTransparency = 1,
		BorderSizePixel = 0,
		Size = UDim2.fromScale(1, 1),
		CanvasSize = UDim2.new(),
		AutomaticCanvasSize = Enum.AutomaticSize.Y,
		ScrollBarThickness = 6,
		ScrollBarImageColor3 = C.TextDim,
		Parent = parent,
	}, {
		New("UIGridLayout", {
			CellSize = UDim2.fromOffset((cell or Vector2.new(150, 180)).X, (cell or Vector2.new(150, 180)).Y),
			CellPadding = UDim2.fromOffset(gap or Theme.Space.Gap, gap or Theme.Space.Gap),
			SortOrder = Enum.SortOrder.LayoutOrder,
			HorizontalAlignment = Enum.HorizontalAlignment.Center,
		}),
		New("UIPadding", { PaddingTop = UDim.new(0, 6), PaddingBottom = UDim.new(0, 6) }),
	})
	return grid
end

-- One item card: picture, name, a price or a rarity colour, and a button.
function UI.Card(grid: Instance, props: { [string]: any }): Frame
	local rarity = props.RarityColor or C.Common
	local card = UI.Panel(grid, {
		Name = props.Name or "Card",
		Color = C.PanelTop,
		LayoutOrder = props.LayoutOrder or 0,
	})
	card.Fill:FindFirstChildOfClass("UIStroke").Color = rarity
	UI.List(card.Body, "column", 4)
	if props.Image then
		New("ImageLabel", {
			BackgroundTransparency = 1,
			Image = props.Image,
			ScaleType = Enum.ScaleType.Fit,
			Size = UDim2.fromScale(1, 0.5),
			LayoutOrder = 1,
			Parent = card.Body,
		})
	end
	UI.Label(card.Body, { Text = props.Title or "", TextSize = Theme.TextSize.Small, LayoutOrder = 2 })
	if props.ButtonText then
		UI.Button(card.Body, {
			Text = props.ButtonText,
			Color = props.ButtonColor or C.Primary,
			Size = UDim2.new(1, 0, 0, 44),
			Height = 44,
			TextSize = Theme.TextSize.Small,
			LayoutOrder = 3,
			OnClick = props.OnClick,
		})
	end
	return card.Instance
end

return UI
