-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_event::NativeKeyCode
-----------------------------------------------------------

---@class NativeKeyCode_static

---@type NativeKeyCode
NativeKeyCode.Unidentified = {}

---@class NativeKeyCode_Android
---@field _1 integer
NativeKeyCode_Android = {}

---@param _1 integer
---@return NativeKeyCode
function NativeKeyCode:Android(_1) end

---@class NativeKeyCode_MacOS
---@field _1 integer
NativeKeyCode_MacOS = {}

---@param _1 integer
---@return NativeKeyCode
function NativeKeyCode:MacOS(_1) end

---@class NativeKeyCode_Windows
---@field _1 integer
NativeKeyCode_Windows = {}

---@param _1 integer
---@return NativeKeyCode
function NativeKeyCode:Windows(_1) end

---@class NativeKeyCode_Xkb
---@field _1 integer
NativeKeyCode_Xkb = {}

---@param _1 integer
---@return NativeKeyCode
function NativeKeyCode:Xkb(_1) end
NativeKeyCode = {}

---@class NativeKeyCode
---@field Unidentified boolean
---@field Android NativeKeyCode_Android
---@field MacOS NativeKeyCode_MacOS
---@field Windows NativeKeyCode_Windows
---@field Xkb NativeKeyCode_Xkb
NativeKeyCode_instance = {}

