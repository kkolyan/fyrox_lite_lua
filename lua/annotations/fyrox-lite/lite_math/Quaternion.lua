
-----------------------------------------------------------
------ fyrox_lite_math::lite_math::LiteQuaternion
-----------------------------------------------------------

---@class Quaternion_static
Quaternion = {}

---@class Quaternion
Quaternion_instance = {}

---@param dir Vector3
---@param up Vector3
---@return Quaternion
function Quaternion:face_towards(dir, up) end

---@param axis Vector3
---@param angle number
---@return Quaternion
function Quaternion:from_axis_angle(axis, angle) end

---@param o Vector3
---@return Vector3
function Quaternion_instance:mul_vec(o) end

---@param rot_delta Quaternion
---@return Quaternion
function Quaternion_instance:mul_quat(rot_delta) end

