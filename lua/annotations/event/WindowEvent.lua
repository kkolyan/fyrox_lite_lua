-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_event::WindowEvent
-----------------------------------------------------------

---@class WindowEvent_static

---@type WindowEvent
WindowEvent.ActivationTokenDone = {}

---@class WindowEvent_Resized
---@field _1 Vector2i
WindowEvent_Resized = {}

---@param _1 Vector2i
---@return WindowEvent
function WindowEvent:Resized(_1) end

---@class WindowEvent_Moved
---@field _1 Vector2i
WindowEvent_Moved = {}

---@param _1 Vector2i
---@return WindowEvent
function WindowEvent:Moved(_1) end

---@type WindowEvent
WindowEvent.CloseRequested = {}

---@type WindowEvent
WindowEvent.Destroyed = {}

---@class WindowEvent_DroppedFile
---@field _1 string
WindowEvent_DroppedFile = {}

---@param _1 string
---@return WindowEvent
function WindowEvent:DroppedFile(_1) end

---@class WindowEvent_HoveredFile
---@field _1 string
WindowEvent_HoveredFile = {}

---@param _1 string
---@return WindowEvent
function WindowEvent:HoveredFile(_1) end

---@type WindowEvent
WindowEvent.HoveredFileCancelled = {}

---@class WindowEvent_Focused
---@field _1 boolean
WindowEvent_Focused = {}

---@param _1 boolean
---@return WindowEvent
function WindowEvent:Focused(_1) end

---@class WindowEvent_KeyboardInput
---@field event KeyEvent
---@field is_synthetic boolean
WindowEvent_KeyboardInput = {}

---@param state WindowEvent_KeyboardInput
---@return WindowEvent
function WindowEvent:KeyboardInput(state) end

---@type WindowEvent
WindowEvent.ModifiersChanged = {}

---@type WindowEvent
WindowEvent.Ime = {}

---@class WindowEvent_CursorMoved
---@field position Vector2i
WindowEvent_CursorMoved = {}

---@param state WindowEvent_CursorMoved
---@return WindowEvent
function WindowEvent:CursorMoved(state) end

---@type WindowEvent
WindowEvent.CursorEntered = {}

---@type WindowEvent
WindowEvent.CursorLeft = {}

---@class WindowEvent_MouseWheel
---@field delta MouseScrollDelta
---@field phase TouchPhase
WindowEvent_MouseWheel = {}

---@param state WindowEvent_MouseWheel
---@return WindowEvent
function WindowEvent:MouseWheel(state) end

---@class WindowEvent_MouseInput
---@field state ElementState
---@field button MouseButton
WindowEvent_MouseInput = {}

---@param state WindowEvent_MouseInput
---@return WindowEvent
function WindowEvent:MouseInput(state) end

---@class WindowEvent_TouchpadMagnify
---@field delta number
---@field phase TouchPhase
WindowEvent_TouchpadMagnify = {}

---@param state WindowEvent_TouchpadMagnify
---@return WindowEvent
function WindowEvent:TouchpadMagnify(state) end

---@type WindowEvent
WindowEvent.SmartMagnify = {}

---@class WindowEvent_TouchpadRotate
---@field delta number
---@field phase TouchPhase
WindowEvent_TouchpadRotate = {}

---@param state WindowEvent_TouchpadRotate
---@return WindowEvent
function WindowEvent:TouchpadRotate(state) end

---@class WindowEvent_TouchpadPressure
---@field pressure number
---@field stage integer
WindowEvent_TouchpadPressure = {}

---@param state WindowEvent_TouchpadPressure
---@return WindowEvent
function WindowEvent:TouchpadPressure(state) end

---@class WindowEvent_AxisMotion
---@field axis integer
---@field value number
WindowEvent_AxisMotion = {}

---@param state WindowEvent_AxisMotion
---@return WindowEvent
function WindowEvent:AxisMotion(state) end

---@class WindowEvent_Touch
---@field _1 Touch
WindowEvent_Touch = {}

---@param _1 Touch
---@return WindowEvent
function WindowEvent:Touch(_1) end

---@class WindowEvent_ScaleFactorChanged
---@field scale_factor number
---@field inner_size_writer InnerSizeWriter
WindowEvent_ScaleFactorChanged = {}

---@param state WindowEvent_ScaleFactorChanged
---@return WindowEvent
function WindowEvent:ScaleFactorChanged(state) end

---@type WindowEvent
WindowEvent.ThemeChanged = {}

---@class WindowEvent_Occluded
---@field _1 boolean
WindowEvent_Occluded = {}

---@param _1 boolean
---@return WindowEvent
function WindowEvent:Occluded(_1) end

---@type WindowEvent
WindowEvent.RedrawRequested = {}
WindowEvent = {}

---@class WindowEvent
---@field ActivationTokenDone boolean
---@field Resized WindowEvent_Resized
---@field Moved WindowEvent_Moved
---@field CloseRequested boolean
---@field Destroyed boolean
---@field DroppedFile WindowEvent_DroppedFile
---@field HoveredFile WindowEvent_HoveredFile
---@field HoveredFileCancelled boolean
---@field Focused WindowEvent_Focused
---@field KeyboardInput WindowEvent_KeyboardInput
---@field ModifiersChanged boolean
---@field Ime boolean
---@field CursorMoved WindowEvent_CursorMoved
---@field CursorEntered boolean
---@field CursorLeft boolean
---@field MouseWheel WindowEvent_MouseWheel
---@field MouseInput WindowEvent_MouseInput
---@field TouchpadMagnify WindowEvent_TouchpadMagnify
---@field SmartMagnify boolean
---@field TouchpadRotate WindowEvent_TouchpadRotate
---@field TouchpadPressure WindowEvent_TouchpadPressure
---@field AxisMotion WindowEvent_AxisMotion
---@field Touch WindowEvent_Touch
---@field ScaleFactorChanged WindowEvent_ScaleFactorChanged
---@field ThemeChanged boolean
---@field Occluded WindowEvent_Occluded
---@field RedrawRequested boolean
WindowEvent_instance = {}

