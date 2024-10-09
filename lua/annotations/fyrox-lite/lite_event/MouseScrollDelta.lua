
-----------------------------------------------------------
------ fyrox_lite::lite_event::MouseScrollDelta
-----------------------------------------------------------

---@class MouseScrollDelta_static

---@class MouseScrollDelta_LineDelta
---@field _1 Vector2
MouseScrollDelta_LineDelta = {}

---@param _1 Vector2
---@return MouseScrollDelta
function MouseScrollDelta:LineDelta(_1) end

---@class MouseScrollDelta_PixelDelta
---@field _1 Vector2
MouseScrollDelta_PixelDelta = {}

---@param _1 Vector2
---@return MouseScrollDelta
function MouseScrollDelta:PixelDelta(_1) end
MouseScrollDelta = {}

---@class MouseScrollDelta
---@field LineDelta MouseScrollDelta_LineDelta
---@field PixelDelta MouseScrollDelta_PixelDelta
MouseScrollDelta_instance = {}

