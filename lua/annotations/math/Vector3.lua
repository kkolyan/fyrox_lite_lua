-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite_math::lite_math::LiteVector3
-----------------------------------------------------------

---@class Vector3_static
---@field X Vector3
---@field Y Vector3
---@field Z Vector3
---@field ZERO Vector3
Vector3 = {}

---@class Vector3
---@field x number
---@field y number
---@field z number
Vector3_instance = {}

---@param x number
---@param y number
---@param z number
---@return Vector3
function Vector3:new(x, y, z) end

---@param o number
---@return Vector3
function Vector3_instance:mul(o) end

---@param o Vector3
---@return Vector3
function Vector3_instance:add(o) end

---@return Vector3
function Vector3_instance:normalize() end

---@param o Vector3
---@return Vector3
function Vector3_instance:sub(o) end

---@return number
function Vector3_instance:magnitude() end

function Vector3_instance:normalize_inplace() end

