
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields


-----------------------------------------------------------
------ fyrox_lite::lite_event::DeviceEvent
-----------------------------------------------------------

---@class DeviceEvent_static

---@type DeviceEvent
DeviceEvent.Added = {}

---@type DeviceEvent
DeviceEvent.Removed = {}

---@class DeviceEvent_MouseMotion
---@field delta Vector2
DeviceEvent_MouseMotion = {}

---@param state DeviceEvent_MouseMotion
---@return DeviceEvent
function DeviceEvent:MouseMotion(state) end

---@class DeviceEvent_MouseWheel
---@field delta MouseScrollDelta
DeviceEvent_MouseWheel = {}

---@param state DeviceEvent_MouseWheel
---@return DeviceEvent
function DeviceEvent:MouseWheel(state) end

---@class DeviceEvent_Motion
---@field axis integer
---@field value number
DeviceEvent_Motion = {}

---@param state DeviceEvent_Motion
---@return DeviceEvent
function DeviceEvent:Motion(state) end

---@class DeviceEvent_Button
---@field button integer
---@field state ElementState
DeviceEvent_Button = {}

---@param state DeviceEvent_Button
---@return DeviceEvent
function DeviceEvent:Button(state) end

---@class DeviceEvent_Key
---@field _1 RawKeyEvent
DeviceEvent_Key = {}

---@param _1 RawKeyEvent
---@return DeviceEvent
function DeviceEvent:Key(_1) end
DeviceEvent = {}

---@class DeviceEvent
---@field Added boolean
---@field Removed boolean
---@field MouseMotion DeviceEvent_MouseMotion
---@field MouseWheel DeviceEvent_MouseWheel
---@field Motion DeviceEvent_Motion
---@field Button DeviceEvent_Button
---@field Key DeviceEvent_Key
DeviceEvent_instance = {}

