
-----------------------------------------------------------
------ fyrox_lite::lite_event::Force
-----------------------------------------------------------

---@class Force_static

---@class Force_Calibrated
---@field force number
---@field max_possible_force number
---@field altitude_angle number?
Force_Calibrated = {}

---@param state Force_Calibrated
---@return Force
function Force:Calibrated(state) end

---@class Force_Normalized
---@field _1 number
Force_Normalized = {}

---@param _1 number
---@return Force
function Force:Normalized(_1) end
Force = {}

---@class Force
---@field Calibrated Force_Calibrated
---@field Normalized Force_Normalized
Force_instance = {}

