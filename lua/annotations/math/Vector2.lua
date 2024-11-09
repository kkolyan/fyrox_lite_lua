-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite_math::lite_math::LiteVector2
-----------------------------------------------------------

---@class Vector2_static
---@field ZERO Vector2
Vector2 = {}

---@class Vector2
---@field x number
---@field y number
Vector2_instance = {}

---@param x number
---@param y number
---@return Vector2
function Vector2:new(x, y) end

---@param o number
---@return Vector2
function Vector2_instance:mul(o) end

---@param o Vector2
---@return Vector2
function Vector2_instance:add(o) end

---@return Vector2
function Vector2_instance:normalize() end

---@param o Vector2
---@return Vector2
function Vector2_instance:sub(o) end

---@return number
function Vector2_instance:magnitude() end

function Vector2_instance:normalize_inplace() end

