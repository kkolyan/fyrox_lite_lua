
-----------------------------------------------------------
------ fyrox_lite::lite_event::MouseButton
-----------------------------------------------------------

---@class MouseButton_static

---@type MouseButton
MouseButton.Left = {}

---@type MouseButton
MouseButton.Right = {}

---@type MouseButton
MouseButton.Middle = {}

---@type MouseButton
MouseButton.Back = {}

---@type MouseButton
MouseButton.Forward = {}

---@class MouseButton_Other
---@field _1 integer
MouseButton_Other = {}

---@param _1 integer
---@return MouseButton
function MouseButton:Other(_1) end
MouseButton = {}

---@class MouseButton
---@field Left boolean
---@field Right boolean
---@field Middle boolean
---@field Back boolean
---@field Forward boolean
---@field Other MouseButton_Other
MouseButton_instance = {}

