
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

---@class Script
---@field node Node
Script = {}

-- Used to 
function script_class() end

-----------------------------------------------------------
------ fyrox_lite::lite_event::Event
-----------------------------------------------------------
do

---@class Event_static

---@class Event_WindowEvent
---@field window_id integer
---@field event WindowEvent
Event_WindowEvent = {}

---@param state Event_WindowEvent
---@return Event
function Event:WindowEvent(state) end

---@class Event_DeviceEvent
---@field event DeviceEvent
Event_DeviceEvent = {}

---@param state Event_DeviceEvent
---@return Event
function Event:DeviceEvent(state) end

---@type Event
Event.Suspended = {}

---@type Event
Event.Resumed = {}

---@type Event
Event.AboutToWait = {}

---@type Event
Event.LoopExiting = {}

---@type Event
Event.MemoryWarning = {}
Event = {}

---@class Event
---@field WindowEvent Event_WindowEvent
---@field DeviceEvent Event_DeviceEvent
---@field Suspended boolean
---@field Resumed boolean
---@field AboutToWait boolean
---@field LoopExiting boolean
---@field MemoryWarning boolean
Event_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::StartCause
-----------------------------------------------------------
do

---@class StartCause_static

---@type StartCause
StartCause.ResumeTimeReached = {}

---@type StartCause
StartCause.WaitCancelled = {}

---@type StartCause
StartCause.Poll = {}

---@type StartCause
StartCause.Init = {}
StartCause = {}

---@class StartCause
---@field ResumeTimeReached boolean
---@field WaitCancelled boolean
---@field Poll boolean
---@field Init boolean
StartCause_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::WindowEvent
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::DeviceEvent
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::RawKeyEvent
-----------------------------------------------------------
do

---@class RawKeyEvent
---@field physical_key PhysicalKey
---@field state ElementState
RawKeyEvent_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::PhysicalKey
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::KeyCode
-----------------------------------------------------------
do

---@class KeyCode_static

---@type KeyCode
KeyCode.Backquote = {}

---@type KeyCode
KeyCode.Backslash = {}

---@type KeyCode
KeyCode.BracketLeft = {}

---@type KeyCode
KeyCode.BracketRight = {}

---@type KeyCode
KeyCode.Comma = {}

---@type KeyCode
KeyCode.Digit0 = {}

---@type KeyCode
KeyCode.Digit1 = {}

---@type KeyCode
KeyCode.Digit2 = {}

---@type KeyCode
KeyCode.Digit3 = {}

---@type KeyCode
KeyCode.Digit4 = {}

---@type KeyCode
KeyCode.Digit5 = {}

---@type KeyCode
KeyCode.Digit6 = {}

---@type KeyCode
KeyCode.Digit7 = {}

---@type KeyCode
KeyCode.Digit8 = {}

---@type KeyCode
KeyCode.Digit9 = {}

---@type KeyCode
KeyCode.Equal = {}

---@type KeyCode
KeyCode.IntlBackslash = {}

---@type KeyCode
KeyCode.IntlRo = {}

---@type KeyCode
KeyCode.IntlYen = {}

---@type KeyCode
KeyCode.KeyA = {}

---@type KeyCode
KeyCode.KeyB = {}

---@type KeyCode
KeyCode.KeyC = {}

---@type KeyCode
KeyCode.KeyD = {}

---@type KeyCode
KeyCode.KeyE = {}

---@type KeyCode
KeyCode.KeyF = {}

---@type KeyCode
KeyCode.KeyG = {}

---@type KeyCode
KeyCode.KeyH = {}

---@type KeyCode
KeyCode.KeyI = {}

---@type KeyCode
KeyCode.KeyJ = {}

---@type KeyCode
KeyCode.KeyK = {}

---@type KeyCode
KeyCode.KeyL = {}

---@type KeyCode
KeyCode.KeyM = {}

---@type KeyCode
KeyCode.KeyN = {}

---@type KeyCode
KeyCode.KeyO = {}

---@type KeyCode
KeyCode.KeyP = {}

---@type KeyCode
KeyCode.KeyQ = {}

---@type KeyCode
KeyCode.KeyR = {}

---@type KeyCode
KeyCode.KeyS = {}

---@type KeyCode
KeyCode.KeyT = {}

---@type KeyCode
KeyCode.KeyU = {}

---@type KeyCode
KeyCode.KeyV = {}

---@type KeyCode
KeyCode.KeyW = {}

---@type KeyCode
KeyCode.KeyX = {}

---@type KeyCode
KeyCode.KeyY = {}

---@type KeyCode
KeyCode.KeyZ = {}

---@type KeyCode
KeyCode.Minus = {}

---@type KeyCode
KeyCode.Period = {}

---@type KeyCode
KeyCode.Quote = {}

---@type KeyCode
KeyCode.Semicolon = {}

---@type KeyCode
KeyCode.Slash = {}

---@type KeyCode
KeyCode.AltLeft = {}

---@type KeyCode
KeyCode.AltRight = {}

---@type KeyCode
KeyCode.Backspace = {}

---@type KeyCode
KeyCode.CapsLock = {}

---@type KeyCode
KeyCode.ContextMenu = {}

---@type KeyCode
KeyCode.ControlLeft = {}

---@type KeyCode
KeyCode.ControlRight = {}

---@type KeyCode
KeyCode.Enter = {}

---@type KeyCode
KeyCode.SuperLeft = {}

---@type KeyCode
KeyCode.SuperRight = {}

---@type KeyCode
KeyCode.ShiftLeft = {}

---@type KeyCode
KeyCode.ShiftRight = {}

---@type KeyCode
KeyCode.Space = {}

---@type KeyCode
KeyCode.Tab = {}

---@type KeyCode
KeyCode.Convert = {}

---@type KeyCode
KeyCode.KanaMode = {}

---@type KeyCode
KeyCode.Lang1 = {}

---@type KeyCode
KeyCode.Lang2 = {}

---@type KeyCode
KeyCode.Lang3 = {}

---@type KeyCode
KeyCode.Lang4 = {}

---@type KeyCode
KeyCode.Lang5 = {}

---@type KeyCode
KeyCode.NonConvert = {}

---@type KeyCode
KeyCode.Delete = {}

---@type KeyCode
KeyCode.End = {}

---@type KeyCode
KeyCode.Help = {}

---@type KeyCode
KeyCode.Home = {}

---@type KeyCode
KeyCode.Insert = {}

---@type KeyCode
KeyCode.PageDown = {}

---@type KeyCode
KeyCode.PageUp = {}

---@type KeyCode
KeyCode.ArrowDown = {}

---@type KeyCode
KeyCode.ArrowLeft = {}

---@type KeyCode
KeyCode.ArrowRight = {}

---@type KeyCode
KeyCode.ArrowUp = {}

---@type KeyCode
KeyCode.NumLock = {}

---@type KeyCode
KeyCode.Numpad0 = {}

---@type KeyCode
KeyCode.Numpad1 = {}

---@type KeyCode
KeyCode.Numpad2 = {}

---@type KeyCode
KeyCode.Numpad3 = {}

---@type KeyCode
KeyCode.Numpad4 = {}

---@type KeyCode
KeyCode.Numpad5 = {}

---@type KeyCode
KeyCode.Numpad6 = {}

---@type KeyCode
KeyCode.Numpad7 = {}

---@type KeyCode
KeyCode.Numpad8 = {}

---@type KeyCode
KeyCode.Numpad9 = {}

---@type KeyCode
KeyCode.NumpadAdd = {}

---@type KeyCode
KeyCode.NumpadBackspace = {}

---@type KeyCode
KeyCode.NumpadClear = {}

---@type KeyCode
KeyCode.NumpadClearEntry = {}

---@type KeyCode
KeyCode.NumpadComma = {}

---@type KeyCode
KeyCode.NumpadDecimal = {}

---@type KeyCode
KeyCode.NumpadDivide = {}

---@type KeyCode
KeyCode.NumpadEnter = {}

---@type KeyCode
KeyCode.NumpadEqual = {}

---@type KeyCode
KeyCode.NumpadHash = {}

---@type KeyCode
KeyCode.NumpadMemoryAdd = {}

---@type KeyCode
KeyCode.NumpadMemoryClear = {}

---@type KeyCode
KeyCode.NumpadMemoryRecall = {}

---@type KeyCode
KeyCode.NumpadMemoryStore = {}

---@type KeyCode
KeyCode.NumpadMemorySubtract = {}

---@type KeyCode
KeyCode.NumpadMultiply = {}

---@type KeyCode
KeyCode.NumpadParenLeft = {}

---@type KeyCode
KeyCode.NumpadParenRight = {}

---@type KeyCode
KeyCode.NumpadStar = {}

---@type KeyCode
KeyCode.NumpadSubtract = {}

---@type KeyCode
KeyCode.Escape = {}

---@type KeyCode
KeyCode.Fn = {}

---@type KeyCode
KeyCode.FnLock = {}

---@type KeyCode
KeyCode.PrintScreen = {}

---@type KeyCode
KeyCode.ScrollLock = {}

---@type KeyCode
KeyCode.Pause = {}

---@type KeyCode
KeyCode.BrowserBack = {}

---@type KeyCode
KeyCode.BrowserFavorites = {}

---@type KeyCode
KeyCode.BrowserForward = {}

---@type KeyCode
KeyCode.BrowserHome = {}

---@type KeyCode
KeyCode.BrowserRefresh = {}

---@type KeyCode
KeyCode.BrowserSearch = {}

---@type KeyCode
KeyCode.BrowserStop = {}

---@type KeyCode
KeyCode.Eject = {}

---@type KeyCode
KeyCode.LaunchApp1 = {}

---@type KeyCode
KeyCode.LaunchApp2 = {}

---@type KeyCode
KeyCode.LaunchMail = {}

---@type KeyCode
KeyCode.MediaPlayPause = {}

---@type KeyCode
KeyCode.MediaSelect = {}

---@type KeyCode
KeyCode.MediaStop = {}

---@type KeyCode
KeyCode.MediaTrackNext = {}

---@type KeyCode
KeyCode.MediaTrackPrevious = {}

---@type KeyCode
KeyCode.Power = {}

---@type KeyCode
KeyCode.Sleep = {}

---@type KeyCode
KeyCode.AudioVolumeDown = {}

---@type KeyCode
KeyCode.AudioVolumeMute = {}

---@type KeyCode
KeyCode.AudioVolumeUp = {}

---@type KeyCode
KeyCode.WakeUp = {}

---@type KeyCode
KeyCode.Meta = {}

---@type KeyCode
KeyCode.Hyper = {}

---@type KeyCode
KeyCode.Turbo = {}

---@type KeyCode
KeyCode.Abort = {}

---@type KeyCode
KeyCode.Resume = {}

---@type KeyCode
KeyCode.Suspend = {}

---@type KeyCode
KeyCode.Again = {}

---@type KeyCode
KeyCode.Copy = {}

---@type KeyCode
KeyCode.Cut = {}

---@type KeyCode
KeyCode.Find = {}

---@type KeyCode
KeyCode.Open = {}

---@type KeyCode
KeyCode.Paste = {}

---@type KeyCode
KeyCode.Props = {}

---@type KeyCode
KeyCode.Select = {}

---@type KeyCode
KeyCode.Undo = {}

---@type KeyCode
KeyCode.Hiragana = {}

---@type KeyCode
KeyCode.Katakana = {}

---@type KeyCode
KeyCode.F1 = {}

---@type KeyCode
KeyCode.F2 = {}

---@type KeyCode
KeyCode.F3 = {}

---@type KeyCode
KeyCode.F4 = {}

---@type KeyCode
KeyCode.F5 = {}

---@type KeyCode
KeyCode.F6 = {}

---@type KeyCode
KeyCode.F7 = {}

---@type KeyCode
KeyCode.F8 = {}

---@type KeyCode
KeyCode.F9 = {}

---@type KeyCode
KeyCode.F10 = {}

---@type KeyCode
KeyCode.F11 = {}

---@type KeyCode
KeyCode.F12 = {}

---@type KeyCode
KeyCode.F13 = {}

---@type KeyCode
KeyCode.F14 = {}

---@type KeyCode
KeyCode.F15 = {}

---@type KeyCode
KeyCode.F16 = {}

---@type KeyCode
KeyCode.F17 = {}

---@type KeyCode
KeyCode.F18 = {}

---@type KeyCode
KeyCode.F19 = {}

---@type KeyCode
KeyCode.F20 = {}

---@type KeyCode
KeyCode.F21 = {}

---@type KeyCode
KeyCode.F22 = {}

---@type KeyCode
KeyCode.F23 = {}

---@type KeyCode
KeyCode.F24 = {}

---@type KeyCode
KeyCode.F25 = {}

---@type KeyCode
KeyCode.F26 = {}

---@type KeyCode
KeyCode.F27 = {}

---@type KeyCode
KeyCode.F28 = {}

---@type KeyCode
KeyCode.F29 = {}

---@type KeyCode
KeyCode.F30 = {}

---@type KeyCode
KeyCode.F31 = {}

---@type KeyCode
KeyCode.F32 = {}

---@type KeyCode
KeyCode.F33 = {}

---@type KeyCode
KeyCode.F34 = {}

---@type KeyCode
KeyCode.F35 = {}
KeyCode = {}

---@class KeyCode
---@field Backquote boolean
---@field Backslash boolean
---@field BracketLeft boolean
---@field BracketRight boolean
---@field Comma boolean
---@field Digit0 boolean
---@field Digit1 boolean
---@field Digit2 boolean
---@field Digit3 boolean
---@field Digit4 boolean
---@field Digit5 boolean
---@field Digit6 boolean
---@field Digit7 boolean
---@field Digit8 boolean
---@field Digit9 boolean
---@field Equal boolean
---@field IntlBackslash boolean
---@field IntlRo boolean
---@field IntlYen boolean
---@field KeyA boolean
---@field KeyB boolean
---@field KeyC boolean
---@field KeyD boolean
---@field KeyE boolean
---@field KeyF boolean
---@field KeyG boolean
---@field KeyH boolean
---@field KeyI boolean
---@field KeyJ boolean
---@field KeyK boolean
---@field KeyL boolean
---@field KeyM boolean
---@field KeyN boolean
---@field KeyO boolean
---@field KeyP boolean
---@field KeyQ boolean
---@field KeyR boolean
---@field KeyS boolean
---@field KeyT boolean
---@field KeyU boolean
---@field KeyV boolean
---@field KeyW boolean
---@field KeyX boolean
---@field KeyY boolean
---@field KeyZ boolean
---@field Minus boolean
---@field Period boolean
---@field Quote boolean
---@field Semicolon boolean
---@field Slash boolean
---@field AltLeft boolean
---@field AltRight boolean
---@field Backspace boolean
---@field CapsLock boolean
---@field ContextMenu boolean
---@field ControlLeft boolean
---@field ControlRight boolean
---@field Enter boolean
---@field SuperLeft boolean
---@field SuperRight boolean
---@field ShiftLeft boolean
---@field ShiftRight boolean
---@field Space boolean
---@field Tab boolean
---@field Convert boolean
---@field KanaMode boolean
---@field Lang1 boolean
---@field Lang2 boolean
---@field Lang3 boolean
---@field Lang4 boolean
---@field Lang5 boolean
---@field NonConvert boolean
---@field Delete boolean
---@field End boolean
---@field Help boolean
---@field Home boolean
---@field Insert boolean
---@field PageDown boolean
---@field PageUp boolean
---@field ArrowDown boolean
---@field ArrowLeft boolean
---@field ArrowRight boolean
---@field ArrowUp boolean
---@field NumLock boolean
---@field Numpad0 boolean
---@field Numpad1 boolean
---@field Numpad2 boolean
---@field Numpad3 boolean
---@field Numpad4 boolean
---@field Numpad5 boolean
---@field Numpad6 boolean
---@field Numpad7 boolean
---@field Numpad8 boolean
---@field Numpad9 boolean
---@field NumpadAdd boolean
---@field NumpadBackspace boolean
---@field NumpadClear boolean
---@field NumpadClearEntry boolean
---@field NumpadComma boolean
---@field NumpadDecimal boolean
---@field NumpadDivide boolean
---@field NumpadEnter boolean
---@field NumpadEqual boolean
---@field NumpadHash boolean
---@field NumpadMemoryAdd boolean
---@field NumpadMemoryClear boolean
---@field NumpadMemoryRecall boolean
---@field NumpadMemoryStore boolean
---@field NumpadMemorySubtract boolean
---@field NumpadMultiply boolean
---@field NumpadParenLeft boolean
---@field NumpadParenRight boolean
---@field NumpadStar boolean
---@field NumpadSubtract boolean
---@field Escape boolean
---@field Fn boolean
---@field FnLock boolean
---@field PrintScreen boolean
---@field ScrollLock boolean
---@field Pause boolean
---@field BrowserBack boolean
---@field BrowserFavorites boolean
---@field BrowserForward boolean
---@field BrowserHome boolean
---@field BrowserRefresh boolean
---@field BrowserSearch boolean
---@field BrowserStop boolean
---@field Eject boolean
---@field LaunchApp1 boolean
---@field LaunchApp2 boolean
---@field LaunchMail boolean
---@field MediaPlayPause boolean
---@field MediaSelect boolean
---@field MediaStop boolean
---@field MediaTrackNext boolean
---@field MediaTrackPrevious boolean
---@field Power boolean
---@field Sleep boolean
---@field AudioVolumeDown boolean
---@field AudioVolumeMute boolean
---@field AudioVolumeUp boolean
---@field WakeUp boolean
---@field Meta boolean
---@field Hyper boolean
---@field Turbo boolean
---@field Abort boolean
---@field Resume boolean
---@field Suspend boolean
---@field Again boolean
---@field Copy boolean
---@field Cut boolean
---@field Find boolean
---@field Open boolean
---@field Paste boolean
---@field Props boolean
---@field Select boolean
---@field Undo boolean
---@field Hiragana boolean
---@field Katakana boolean
---@field F1 boolean
---@field F2 boolean
---@field F3 boolean
---@field F4 boolean
---@field F5 boolean
---@field F6 boolean
---@field F7 boolean
---@field F8 boolean
---@field F9 boolean
---@field F10 boolean
---@field F11 boolean
---@field F12 boolean
---@field F13 boolean
---@field F14 boolean
---@field F15 boolean
---@field F16 boolean
---@field F17 boolean
---@field F18 boolean
---@field F19 boolean
---@field F20 boolean
---@field F21 boolean
---@field F22 boolean
---@field F23 boolean
---@field F24 boolean
---@field F25 boolean
---@field F26 boolean
---@field F27 boolean
---@field F28 boolean
---@field F29 boolean
---@field F30 boolean
---@field F31 boolean
---@field F32 boolean
---@field F33 boolean
---@field F34 boolean
---@field F35 boolean
KeyCode_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::NativeKeyCode
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::KeyEvent
-----------------------------------------------------------
do

---@class KeyEvent
---@field physical_key PhysicalKey
---@field state ElementState
---@field repeat boolean
KeyEvent_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::KeyLocation
-----------------------------------------------------------
do

---@class KeyLocation_static

---@type KeyLocation
KeyLocation.Standard = {}

---@type KeyLocation
KeyLocation.Left = {}

---@type KeyLocation
KeyLocation.Right = {}

---@type KeyLocation
KeyLocation.Numpad = {}
KeyLocation = {}

---@class KeyLocation
---@field Standard boolean
---@field Left boolean
---@field Right boolean
---@field Numpad boolean
KeyLocation_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::TouchPhase
-----------------------------------------------------------
do

---@class TouchPhase_static

---@type TouchPhase
TouchPhase.Started = {}

---@type TouchPhase
TouchPhase.Moved = {}

---@type TouchPhase
TouchPhase.Ended = {}

---@type TouchPhase
TouchPhase.Cancelled = {}
TouchPhase = {}

---@class TouchPhase
---@field Started boolean
---@field Moved boolean
---@field Ended boolean
---@field Cancelled boolean
TouchPhase_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::Touch
-----------------------------------------------------------
do

---@class Touch
---@field phase TouchPhase
---@field location Vector2
---@field force Force?
---@field id integer
Touch_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::Force
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::ElementState
-----------------------------------------------------------
do

---@class ElementState_static

---@type ElementState
ElementState.Pressed = {}

---@type ElementState
ElementState.Released = {}
ElementState = {}

---@class ElementState
---@field Pressed boolean
---@field Released boolean
ElementState_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::MouseButton
-----------------------------------------------------------
do

---@class MouseButton_static

---@type MouseButton
MouseButton.Left = {}

---@type MouseButton
MouseButton.Right = {}

---@type MouseButton
MouseButton.Middle = {}

---@type MouseButton
MouseButton.Back = {}

---@type MouseButton
MouseButton.Forward = {}

---@class MouseButton_Other
---@field _1 integer
MouseButton_Other = {}

---@param _1 integer
---@return MouseButton
function MouseButton:Other(_1) end
MouseButton = {}

---@class MouseButton
---@field Left boolean
---@field Right boolean
---@field Middle boolean
---@field Back boolean
---@field Forward boolean
---@field Other MouseButton_Other
MouseButton_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::MouseScrollDelta
-----------------------------------------------------------
do

---@class MouseScrollDelta_static

---@class MouseScrollDelta_LineDelta
---@field _1 Vector2
MouseScrollDelta_LineDelta = {}

---@param _1 Vector2
---@return MouseScrollDelta
function MouseScrollDelta:LineDelta(_1) end

---@class MouseScrollDelta_PixelDelta
---@field _1 Vector2
MouseScrollDelta_PixelDelta = {}

---@param _1 Vector2
---@return MouseScrollDelta
function MouseScrollDelta:PixelDelta(_1) end
MouseScrollDelta = {}

---@class MouseScrollDelta
---@field LineDelta MouseScrollDelta_LineDelta
---@field PixelDelta MouseScrollDelta_PixelDelta
MouseScrollDelta_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_event::InnerSizeWriter
-----------------------------------------------------------
do

---@class InnerSizeWriter_static
InnerSizeWriter = {}

---@class InnerSizeWriter
InnerSizeWriter_instance = {}

---@param new_inner_size Vector2i
---@return boolean
function InnerSizeWriter_instance:request_inner_size(new_inner_size) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_log::LiteLog
-----------------------------------------------------------
do

---@class Log_static
Log = {}

---@class Log
Log_instance = {}

---@param msg string
function Log:info(msg) end

---@param msg string
function Log:warn(msg) end

---@param msg string
function Log:err(msg) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_math::PodVector2
-----------------------------------------------------------
do

---@class Vector2
---@field x number
---@field y number
Vector2_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_math::PodVector2i
-----------------------------------------------------------
do

---@class Vector2i
---@field x integer
---@field y integer
Vector2i_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_node::LiteNode
-----------------------------------------------------------
do

---@class Node_static
Node = {}

---@class Node
---@field name string?
---@field alive boolean
---@field global_position Vector3
---@field local_position Vector3
---@field local_rotation Quaternion
---@field valid boolean
---@field parent Node
---@field global_rotation Quaternion
---@field tag string
Node_instance = {}

---@return RigidBody?
function Node_instance:as_rigid_body() end

function Node_instance:destroy() end

---@param routing RoutingStrategy
---@param payload any
function Node_instance:send_hierarchical(routing, payload) end

function Node_instance:subscribe_to() end

---@return Node?
function Node_instance:find_collider_in_children() end

---@generic T
---@param class_name `T`
---@return T
function Node_instance:add_script(class_name) end

---@generic T
---@param class_name `T`
---@return T?
function Node_instance:find_script(class_name) end

---@param tag string
---@return boolean
function Node_instance:tag_is(tag) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_node::LiteRoutingStrategy
-----------------------------------------------------------
do

---@class RoutingStrategy_static

---@type RoutingStrategy
RoutingStrategy.Up = {}

---@type RoutingStrategy
RoutingStrategy.Down = {}
RoutingStrategy = {}

---@class RoutingStrategy
---@field Up boolean
---@field Down boolean
RoutingStrategy_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LitePhysics
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteIntersection
-----------------------------------------------------------
do

---@class Intersection
---@field collider Node
---@field normal Vector3
---@field position Vector3
---@field feature FeatureId
---@field toi number
Intersection_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteFeatureId
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteRayCastOptions
-----------------------------------------------------------
do

---@class RayCastOptions
---@field ray_origin Vector3
---@field ray_direction Vector3
---@field max_len number
---@field groups InteractionGroups?
---@field sort_results boolean
RayCastOptions_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteInteractionGroups
-----------------------------------------------------------
do

---@class InteractionGroups
---@field memberships integer
---@field filter integer
InteractionGroups_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteRigidBody
-----------------------------------------------------------
do

---@class RigidBody_static
RigidBody = {}

---@class RigidBody
RigidBody_instance = {}

---@param force Vector3
function RigidBody_instance:apply_force(force) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_plugin::LitePlugin
-----------------------------------------------------------
do

---@class Plugin_static
Plugin = {}

---@class Plugin
Plugin_instance = {}

---@generic T
---@param class_name `T`
---@return T
function Plugin:get(class_name) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_prefab::LitePrefab
-----------------------------------------------------------
do

---@class Prefab_static
Prefab = {}

---@class Prefab
Prefab_instance = {}

---@param position Vector3
---@param orientation Quaternion
---@return Node
function Prefab_instance:instantiate_at(position, orientation) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_scene::LiteScene
-----------------------------------------------------------
do

---@class Scene_static
Scene = {}

---@class Scene
Scene_instance = {}

---@param scene_path string
function Scene:load_async(scene_path) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_ui::LiteUiNode
-----------------------------------------------------------
do

---@class UiNode_static
UiNode = {}

---@class UiNode
UiNode_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_ui::LiteText
-----------------------------------------------------------
do

---@class Text_static
Text = {}

---@class Text
---@field text_async string
Text_instance = {}

---@param state TextBuilder
---@return Text
function Text:new(state) end

end

-----------------------------------------------------------
------ fyrox_lite::lite_ui::TextBuilder
-----------------------------------------------------------
do

---@class TextBuilder
---@field foregound Brush?
---@field font_size number?
TextBuilder_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_ui::Brush
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite::lite_ui::Color
-----------------------------------------------------------
do

---@class Color_static
---@field WHITE Color
---@field BLACK Color
---@field RED Color
---@field GREEN Color
---@field BLUE Color
---@field TRANSPARENT Color
---@field MAROON Color
---@field DARK_RED Color
---@field BROWN Color
---@field FIREBRICK Color
---@field CRIMSON Color
---@field TOMATO Color
---@field CORAL Color
---@field INDIAN_RED Color
---@field LIGHT_CORAL Color
---@field DARK_SALMON Color
---@field SALMON Color
---@field LIGHT_SALMON Color
---@field ORANGE_RED Color
---@field DARK_ORANGE Color
---@field ORANGE Color
---@field GOLD Color
---@field DARK_GOLDEN_ROD Color
---@field GOLDEN_ROD Color
---@field PALE_GOLDEN_ROD Color
---@field DARK_KHAKI Color
---@field KHAKI Color
---@field OLIVE Color
---@field YELLOW Color
---@field YELLOW_GREEN Color
---@field DARK_OLIVE_GREEN Color
---@field OLIVE_DRAB Color
---@field LAWN_GREEN Color
---@field CHARTREUSE Color
---@field GREEN_YELLOW Color
---@field DARK_GREEN Color
---@field FOREST_GREEN Color
---@field LIME Color
---@field LIME_GREEN Color
---@field LIGHT_GREEN Color
---@field PALE_GREEN Color
---@field DARK_SEA_GREEN Color
---@field MEDIUM_SPRING_GREEN Color
---@field SPRING_GREEN Color
---@field SEA_GREEN Color
---@field MEDIUM_AQUA_MARINE Color
---@field MEDIUM_SEA_GREEN Color
---@field LIGHT_SEA_GREEN Color
---@field DARK_SLATE_GRAY Color
---@field TEAL Color
---@field DARK_CYAN Color
---@field AQUA Color
---@field CYAN Color
---@field LIGHT_CYAN Color
---@field DARK_TURQUOISE Color
---@field TURQUOISE Color
---@field MEDIUM_TURQUOISE Color
---@field PALE_TURQUOISE Color
---@field AQUA_MARINE Color
---@field POWDER_BLUE Color
---@field CADET_BLUE Color
---@field STEEL_BLUE Color
---@field CORN_FLOWER_BLUE Color
---@field DEEP_SKY_BLUE Color
---@field DODGER_BLUE Color
---@field LIGHT_BLUE Color
---@field SKY_BLUE Color
---@field LIGHT_SKY_BLUE Color
---@field MIDNIGHT_BLUE Color
---@field NAVY Color
---@field DARK_BLUE Color
---@field MEDIUM_BLUE Color
---@field ROYAL_BLUE Color
---@field BLUE_VIOLET Color
---@field INDIGO Color
---@field DARK_SLATE_BLUE Color
---@field SLATE_BLUE Color
---@field MEDIUM_SLATE_BLUE Color
---@field MEDIUM_PURPLE Color
---@field DARK_MAGENTA Color
---@field DARK_VIOLET Color
---@field DARK_ORCHID Color
---@field MEDIUM_ORCHID Color
---@field PURPLE Color
---@field THISTLE Color
---@field PLUM Color
---@field VIOLET Color
---@field MAGENTA Color
---@field ORCHID Color
---@field MEDIUM_VIOLET_RED Color
---@field PALE_VIOLET_RED Color
---@field DEEP_PINK Color
---@field HOT_PINK Color
---@field LIGHT_PINK Color
---@field PINK Color
---@field ANTIQUE_WHITE Color
---@field BEIGE Color
---@field BISQUE Color
---@field BLANCHED_ALMOND Color
---@field WHEAT Color
---@field CORN_SILK Color
---@field LEMON_CHIFFON Color
---@field LIGHT_GOLDEN_ROD_YELLOW Color
---@field LIGHT_YELLOW Color
---@field SADDLE_BROWN Color
---@field SIENNA Color
---@field CHOCOLATE Color
---@field PERU Color
---@field SANDY_BROWN Color
---@field BURLY_WOOD Color
---@field TAN Color
---@field ROSY_BROWN Color
---@field MOCCASIN Color
---@field NAVAJO_WHITE Color
---@field PEACH_PUFF Color
---@field MISTY_ROSE Color
---@field LAVENDER_BLUSH Color
---@field LINEN Color
---@field OLD_LACE Color
---@field PAPAYA_WHIP Color
---@field SEA_SHELL Color
---@field MINT_CREAM Color
---@field SLATE_GRAY Color
---@field LIGHT_SLATE_GRAY Color
---@field LIGHT_STEEL_BLUE Color
---@field LAVENDER Color
---@field FLORAL_WHITE Color
---@field ALICE_BLUE Color
---@field GHOST_WHITE Color
---@field HONEYDEW Color
---@field IVORY Color
---@field AZURE Color
---@field SNOW Color
---@field DIM_GRAY Color
---@field GRAY Color
---@field DARK_GRAY Color
---@field SILVER Color
---@field LIGHT_GRAY Color
---@field GAINSBORO Color
---@field WHITE_SMOKE Color
Color = {}

---@class Color
Color_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_ui::GradientPoint
-----------------------------------------------------------
do

---@class GradientPoint
---@field stop number
---@field color Color
GradientPoint_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_window::LiteWindow
-----------------------------------------------------------
do

---@class Window_static
---@field cursor_grab CursorGrabMode
Window = {}

---@class Window
Window_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite::lite_window::LiteCursorGrabMode
-----------------------------------------------------------
do

---@class CursorGrabMode_static

---@type CursorGrabMode
CursorGrabMode.None = {}

---@type CursorGrabMode
CursorGrabMode.Confined = {}

---@type CursorGrabMode
CursorGrabMode.Locked = {}
CursorGrabMode = {}

---@class CursorGrabMode
---@field None boolean
---@field Confined boolean
---@field Locked boolean
CursorGrabMode_instance = {}

end

-----------------------------------------------------------
------ fyrox_lite_math::lite_math::LiteQuaternion
-----------------------------------------------------------
do

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

end

-----------------------------------------------------------
------ fyrox_lite_math::lite_math::LiteVector3
-----------------------------------------------------------
do

---@class Vector3_static
---@field X Vector3
---@field Y Vector3
---@field Z Vector3
---@field ZERO Vector3
Vector3 = {}

---@class Vector3
---@field x number
---@field y number
---@field z number
Vector3_instance = {}

---@param x number
---@param y number
---@param z number
---@return Vector3
function Vector3:new(x, y, z) end

---@param o number
---@return Vector3
function Vector3_instance:mul(o) end

---@param o Vector3
---@return Vector3
function Vector3_instance:add(o) end

---@return Vector3
function Vector3_instance:normalize() end

---@param o Vector3
---@return Vector3
function Vector3_instance:sub(o) end

---@return number
function Vector3_instance:magnitude() end

function Vector3_instance:normalize_inplace() end

end
