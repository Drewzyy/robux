--!nonstrict
-- One place for every colour, font, size and timing the UI uses.
-- Change the look of the whole game here; never inline values in screens.
-- Sizes are at the 1280x720 design size; UI.Screen scales them per device.

local Theme = {}

Theme.Colors = {
	-- Surfaces (dark, slightly blue: reads well over bright 3D worlds)
	Panel = Color3.fromRGB(30, 34, 51),
	PanelTop = Color3.fromRGB(44, 50, 74),
	PanelStroke = Color3.fromRGB(12, 14, 24),
	Inset = Color3.fromRGB(20, 23, 36),
	Shadow = Color3.fromRGB(0, 0, 0),
	-- Text
	Text = Color3.fromRGB(255, 255, 255),
	TextDim = Color3.fromRGB(176, 184, 214),
	TextStroke = Color3.fromRGB(14, 16, 28),
	-- One accent per kind of action
	Primary = Color3.fromRGB(61, 220, 132), -- confirm, play, buy
	Danger = Color3.fromRGB(255, 77, 77), -- close, cancel
	Gold = Color3.fromRGB(255, 200, 61), -- currency, rewards
	Info = Color3.fromRGB(61, 155, 255), -- secondary actions
	Neutral = Color3.fromRGB(92, 100, 140), -- muted buttons
	-- Rarity
	Common = Color3.fromRGB(190, 196, 214),
	Rare = Color3.fromRGB(61, 155, 255),
	Epic = Color3.fromRGB(176, 92, 255),
	Legendary = Color3.fromRGB(255, 176, 32),
}

Theme.Font = Enum.Font.FredokaOne
Theme.FontBody = Enum.Font.GothamBold

Theme.TextSize = {
	Small = 18,
	Body = 22,
	Button = 26,
	Title = 36,
	Huge = 56,
}

Theme.Radius = {
	Panel = 16,
	Button = 12,
	Pill = 999,
}

Theme.Stroke = {
	Panel = 3,
	Button = 3,
	Text = 2,
}

Theme.Space = {
	Gap = 10, -- between items in a list
	Pad = 16, -- inside panels
	Edge = 16, -- from the screen edge
}

Theme.Button = {
	Height = 72, -- 48+ px on a phone after scaling
	MinWidth = 160,
}

Theme.Motion = {
	Hover = TweenInfo.new(0.12, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
	Press = TweenInfo.new(0.08, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
	Open = TweenInfo.new(0.25, Enum.EasingStyle.Back, Enum.EasingDirection.Out),
	Close = TweenInfo.new(0.15, Enum.EasingStyle.Quad, Enum.EasingDirection.In),
	Bar = TweenInfo.new(0.3, Enum.EasingStyle.Quad, Enum.EasingDirection.Out),
	Pop = TweenInfo.new(0.18, Enum.EasingStyle.Back, Enum.EasingDirection.Out),
}

-- A lighter and a darker shade of a colour (for gradients and strokes).
function Theme.lighter(c: Color3, amount: number?): Color3
	return c:Lerp(Color3.new(1, 1, 1), amount or 0.25)
end

function Theme.darker(c: Color3, amount: number?): Color3
	return c:Lerp(Color3.new(0, 0, 0), amount or 0.45)
end

return Theme
