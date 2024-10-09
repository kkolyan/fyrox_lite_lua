
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

