
-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteFeatureId
-----------------------------------------------------------

---@class FeatureId_static

---@class FeatureId_Vertex
---@field _1 integer
FeatureId_Vertex = {}

---@param _1 integer
---@return FeatureId
function FeatureId:Vertex(_1) end

---@class FeatureId_Edge
---@field _1 integer
FeatureId_Edge = {}

---@param _1 integer
---@return FeatureId
function FeatureId:Edge(_1) end

---@class FeatureId_Face
---@field _1 integer
FeatureId_Face = {}

---@param _1 integer
---@return FeatureId
function FeatureId:Face(_1) end

---@type FeatureId
FeatureId.Unknown = {}
FeatureId = {}

---@class FeatureId
---@field Vertex FeatureId_Vertex
---@field Edge FeatureId_Edge
---@field Face FeatureId_Face
---@field Unknown boolean
FeatureId_instance = {}

