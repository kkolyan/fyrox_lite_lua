
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields


-----------------------------------------------------------
------ fyrox_lite::lite_event::PhysicalKey
-----------------------------------------------------------

---@class PhysicalKey_static

---@class PhysicalKey_Code
---@field _1 KeyCode
PhysicalKey_Code = {}

---@param _1 KeyCode
---@return PhysicalKey
function PhysicalKey:Code(_1) end

---@class PhysicalKey_Unidentified
---@field _1 NativeKeyCode
PhysicalKey_Unidentified = {}

---@param _1 NativeKeyCode
---@return PhysicalKey
function PhysicalKey:Unidentified(_1) end
PhysicalKey = {}

---@class PhysicalKey
---@field Code PhysicalKey_Code
---@field Unidentified PhysicalKey_Unidentified
PhysicalKey_instance = {}

