
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields


-----------------------------------------------------------
------ fyrox_lite::lite_physics::LitePhysics
-----------------------------------------------------------

---@class Physics_static
---@field EXCLUDE_FIXED integer
---@field EXCLUDE_KINEMATIC integer
---@field EXCLUDE_DYNAMIC integer
---@field EXCLUDE_SENSORS integer
---@field EXCLUDE_SOLIDS integer
---@field ONLY_DYNAMIC integer
---@field ONLY_KINEMATIC integer
---@field ONLY_FIXED integer
Physics = {}

---@class Physics
Physics_instance = {}

---@param opts RayCastOptions
---@return Intersection[]
function Physics:cast_ray(opts) end

