-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

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

