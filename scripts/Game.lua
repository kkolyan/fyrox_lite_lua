---@class Game : Plugin
---@field player Node
---@field beacons Vector3[]
---@field frags number
---@field wounds number
---@field hud UiNode
Game = script_class()

---@param scene_path string
---@param ctx PluginContext
function Game:init(scene_path)
	if not scene_path then
		scene_path = "data/scene.rgs"
	end
	load_scene_async(scene_path)

	self.hud = TextBuilder:build(ctx.user_interfaces[0], {
		font_size = 40,
		widget = {
			foreground = Brush.Solid(Color.BLACK)
		}
	})
end

---@param ctx PluginContext
function Game:update()
	ctx.user_interfaces[0]:send_message(TextMessage:text(
		self.hud,
		MessageDirection.ToWidget,
		string.format("Wounds: %s\nKilled Guards: %s", self.wounds, self.frags)
	))
end

function Game:inc_frags()
	self.frags = self.frags + 1
end

function Game:inc_wounds()
	self.wounds = self.wounds + 1
end
