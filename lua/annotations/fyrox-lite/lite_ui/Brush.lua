
-----------------------------------------------------------
------ fyrox_lite::lite_ui::Brush
-----------------------------------------------------------

---@class Brush_static

---@class Brush_Solid
---@field _1 Color
Brush_Solid = {}

---@param _1 Color
---@return Brush
function Brush:Solid(_1) end

---@class Brush_LinearGradient
---@field from Vector2
---@field to Vector2
---@field stops GradientPoint[]
Brush_LinearGradient = {}

---@param state Brush_LinearGradient
---@return Brush
function Brush:LinearGradient(state) end

---@class Brush_RadialGradient
---@field center Vector2
---@field stops GradientPoint[]
Brush_RadialGradient = {}

---@param state Brush_RadialGradient
---@return Brush
function Brush:RadialGradient(state) end
Brush = {}

---@class Brush
---@field Solid Brush_Solid
---@field LinearGradient Brush_LinearGradient
---@field RadialGradient Brush_RadialGradient
Brush_instance = {}

