-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LitePhysics
-----------------------------------------------------------

---@class Physics_static
---@field EXCLUDE_FIXED number
---@field EXCLUDE_KINEMATIC number
---@field EXCLUDE_DYNAMIC number
---@field EXCLUDE_SENSORS number
---@field EXCLUDE_SOLIDS number
---@field ONLY_DYNAMIC number
---@field ONLY_KINEMATIC number
---@field ONLY_FIXED number
Physics = {}

---@class Physics
Physics_instance = {}

---@param opts RayCastOptions
---@return Intersection[]
function Physics:cast_ray(opts) end

