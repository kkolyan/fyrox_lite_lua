
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global

---@class Script
---@field node Node
Script = {}

-- Used to 
function script_class() end

---@class Event_static
Event = {}

---@class Event
Event_instance = {}
---@class Event_static
---@field Suspended Event
---@field Resumed Event
---@field AboutToWait Event
---@field LoopExiting Event
---@field MemoryWarning Event
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

---@class Event_WindowEvent
Event_WindowEvent = {}

---@param state Event_WindowEvent
---@return Event
function Event:WindowEvent(state) end
---@class Event_DeviceEvent
Event_DeviceEvent = {}

---@param state Event_DeviceEvent
---@return Event
function Event:DeviceEvent(state) end

---@class StartCause_static
StartCause = {}

---@class StartCause
StartCause_instance = {}
---@class StartCause_static
---@field ResumeTimeReached StartCause
---@field WaitCancelled StartCause
---@field Poll StartCause
---@field Init StartCause
StartCause = {}
---@class StartCause
---@field ResumeTimeReached boolean
---@field WaitCancelled boolean
---@field Poll boolean
---@field Init boolean
StartCause_instance = {}


---@class WindowEvent_static
WindowEvent = {}

---@class WindowEvent
WindowEvent_instance = {}
---@class WindowEvent_static
---@field ActivationTokenDone WindowEvent
---@field CloseRequested WindowEvent
---@field Destroyed WindowEvent
---@field HoveredFileCancelled WindowEvent
---@field ModifiersChanged WindowEvent
---@field Ime WindowEvent
---@field CursorEntered WindowEvent
---@field CursorLeft WindowEvent
---@field SmartMagnify WindowEvent
---@field ThemeChanged WindowEvent
---@field RedrawRequested WindowEvent
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

---@class WindowEvent_Resized
WindowEvent_Resized = {}

---@param state WindowEvent_Resized
---@return WindowEvent
function WindowEvent:Resized(state) end
---@class WindowEvent_Moved
WindowEvent_Moved = {}

---@param state WindowEvent_Moved
---@return WindowEvent
function WindowEvent:Moved(state) end
---@class WindowEvent_DroppedFile
WindowEvent_DroppedFile = {}

---@param state WindowEvent_DroppedFile
---@return WindowEvent
function WindowEvent:DroppedFile(state) end
---@class WindowEvent_HoveredFile
WindowEvent_HoveredFile = {}

---@param state WindowEvent_HoveredFile
---@return WindowEvent
function WindowEvent:HoveredFile(state) end
---@class WindowEvent_Focused
WindowEvent_Focused = {}

---@param state WindowEvent_Focused
---@return WindowEvent
function WindowEvent:Focused(state) end
---@class WindowEvent_KeyboardInput
WindowEvent_KeyboardInput = {}

---@param state WindowEvent_KeyboardInput
---@return WindowEvent
function WindowEvent:KeyboardInput(state) end
---@class WindowEvent_CursorMoved
WindowEvent_CursorMoved = {}

---@param state WindowEvent_CursorMoved
---@return WindowEvent
function WindowEvent:CursorMoved(state) end
---@class WindowEvent_MouseWheel
WindowEvent_MouseWheel = {}

---@param state WindowEvent_MouseWheel
---@return WindowEvent
function WindowEvent:MouseWheel(state) end
---@class WindowEvent_MouseInput
WindowEvent_MouseInput = {}

---@param state WindowEvent_MouseInput
---@return WindowEvent
function WindowEvent:MouseInput(state) end
---@class WindowEvent_TouchpadMagnify
WindowEvent_TouchpadMagnify = {}

---@param state WindowEvent_TouchpadMagnify
---@return WindowEvent
function WindowEvent:TouchpadMagnify(state) end
---@class WindowEvent_TouchpadRotate
WindowEvent_TouchpadRotate = {}

---@param state WindowEvent_TouchpadRotate
---@return WindowEvent
function WindowEvent:TouchpadRotate(state) end
---@class WindowEvent_TouchpadPressure
WindowEvent_TouchpadPressure = {}

---@param state WindowEvent_TouchpadPressure
---@return WindowEvent
function WindowEvent:TouchpadPressure(state) end
---@class WindowEvent_AxisMotion
WindowEvent_AxisMotion = {}

---@param state WindowEvent_AxisMotion
---@return WindowEvent
function WindowEvent:AxisMotion(state) end
---@class WindowEvent_Touch
WindowEvent_Touch = {}

---@param state WindowEvent_Touch
---@return WindowEvent
function WindowEvent:Touch(state) end
---@class WindowEvent_ScaleFactorChanged
WindowEvent_ScaleFactorChanged = {}

---@param state WindowEvent_ScaleFactorChanged
---@return WindowEvent
function WindowEvent:ScaleFactorChanged(state) end
---@class WindowEvent_Occluded
WindowEvent_Occluded = {}

---@param state WindowEvent_Occluded
---@return WindowEvent
function WindowEvent:Occluded(state) end

---@class DeviceEvent_static
DeviceEvent = {}

---@class DeviceEvent
DeviceEvent_instance = {}
---@class DeviceEvent_static
---@field Added DeviceEvent
---@field Removed DeviceEvent
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

---@class DeviceEvent_MouseMotion
DeviceEvent_MouseMotion = {}

---@param state DeviceEvent_MouseMotion
---@return DeviceEvent
function DeviceEvent:MouseMotion(state) end
---@class DeviceEvent_MouseWheel
DeviceEvent_MouseWheel = {}

---@param state DeviceEvent_MouseWheel
---@return DeviceEvent
function DeviceEvent:MouseWheel(state) end
---@class DeviceEvent_Motion
DeviceEvent_Motion = {}

---@param state DeviceEvent_Motion
---@return DeviceEvent
function DeviceEvent:Motion(state) end
---@class DeviceEvent_Button
DeviceEvent_Button = {}

---@param state DeviceEvent_Button
---@return DeviceEvent
function DeviceEvent:Button(state) end
---@class DeviceEvent_Key
DeviceEvent_Key = {}

---@param state DeviceEvent_Key
---@return DeviceEvent
function DeviceEvent:Key(state) end

---@class RawKeyEvent_static
RawKeyEvent = {}

---@class RawKeyEvent
---@field physical_key PhysicalKey
---@field state ElementState
RawKeyEvent_instance = {}

---@class PhysicalKey_static
PhysicalKey = {}

---@class PhysicalKey
PhysicalKey_instance = {}
---@class PhysicalKey_static
PhysicalKey = {}
---@class PhysicalKey
---@field Code PhysicalKey_Code
---@field Unidentified PhysicalKey_Unidentified
PhysicalKey_instance = {}

---@class PhysicalKey_Code
PhysicalKey_Code = {}

---@param state PhysicalKey_Code
---@return PhysicalKey
function PhysicalKey:Code(state) end
---@class PhysicalKey_Unidentified
PhysicalKey_Unidentified = {}

---@param state PhysicalKey_Unidentified
---@return PhysicalKey
function PhysicalKey:Unidentified(state) end

---@class KeyCode_static
KeyCode = {}

---@class KeyCode
KeyCode_instance = {}
---@class KeyCode_static
---@field Backquote KeyCode
---@field Backslash KeyCode
---@field BracketLeft KeyCode
---@field BracketRight KeyCode
---@field Comma KeyCode
---@field Digit0 KeyCode
---@field Digit1 KeyCode
---@field Digit2 KeyCode
---@field Digit3 KeyCode
---@field Digit4 KeyCode
---@field Digit5 KeyCode
---@field Digit6 KeyCode
---@field Digit7 KeyCode
---@field Digit8 KeyCode
---@field Digit9 KeyCode
---@field Equal KeyCode
---@field IntlBackslash KeyCode
---@field IntlRo KeyCode
---@field IntlYen KeyCode
---@field KeyA KeyCode
---@field KeyB KeyCode
---@field KeyC KeyCode
---@field KeyD KeyCode
---@field KeyE KeyCode
---@field KeyF KeyCode
---@field KeyG KeyCode
---@field KeyH KeyCode
---@field KeyI KeyCode
---@field KeyJ KeyCode
---@field KeyK KeyCode
---@field KeyL KeyCode
---@field KeyM KeyCode
---@field KeyN KeyCode
---@field KeyO KeyCode
---@field KeyP KeyCode
---@field KeyQ KeyCode
---@field KeyR KeyCode
---@field KeyS KeyCode
---@field KeyT KeyCode
---@field KeyU KeyCode
---@field KeyV KeyCode
---@field KeyW KeyCode
---@field KeyX KeyCode
---@field KeyY KeyCode
---@field KeyZ KeyCode
---@field Minus KeyCode
---@field Period KeyCode
---@field Quote KeyCode
---@field Semicolon KeyCode
---@field Slash KeyCode
---@field AltLeft KeyCode
---@field AltRight KeyCode
---@field Backspace KeyCode
---@field CapsLock KeyCode
---@field ContextMenu KeyCode
---@field ControlLeft KeyCode
---@field ControlRight KeyCode
---@field Enter KeyCode
---@field SuperLeft KeyCode
---@field SuperRight KeyCode
---@field ShiftLeft KeyCode
---@field ShiftRight KeyCode
---@field Space KeyCode
---@field Tab KeyCode
---@field Convert KeyCode
---@field KanaMode KeyCode
---@field Lang1 KeyCode
---@field Lang2 KeyCode
---@field Lang3 KeyCode
---@field Lang4 KeyCode
---@field Lang5 KeyCode
---@field NonConvert KeyCode
---@field Delete KeyCode
---@field End KeyCode
---@field Help KeyCode
---@field Home KeyCode
---@field Insert KeyCode
---@field PageDown KeyCode
---@field PageUp KeyCode
---@field ArrowDown KeyCode
---@field ArrowLeft KeyCode
---@field ArrowRight KeyCode
---@field ArrowUp KeyCode
---@field NumLock KeyCode
---@field Numpad0 KeyCode
---@field Numpad1 KeyCode
---@field Numpad2 KeyCode
---@field Numpad3 KeyCode
---@field Numpad4 KeyCode
---@field Numpad5 KeyCode
---@field Numpad6 KeyCode
---@field Numpad7 KeyCode
---@field Numpad8 KeyCode
---@field Numpad9 KeyCode
---@field NumpadAdd KeyCode
---@field NumpadBackspace KeyCode
---@field NumpadClear KeyCode
---@field NumpadClearEntry KeyCode
---@field NumpadComma KeyCode
---@field NumpadDecimal KeyCode
---@field NumpadDivide KeyCode
---@field NumpadEnter KeyCode
---@field NumpadEqual KeyCode
---@field NumpadHash KeyCode
---@field NumpadMemoryAdd KeyCode
---@field NumpadMemoryClear KeyCode
---@field NumpadMemoryRecall KeyCode
---@field NumpadMemoryStore KeyCode
---@field NumpadMemorySubtract KeyCode
---@field NumpadMultiply KeyCode
---@field NumpadParenLeft KeyCode
---@field NumpadParenRight KeyCode
---@field NumpadStar KeyCode
---@field NumpadSubtract KeyCode
---@field Escape KeyCode
---@field Fn KeyCode
---@field FnLock KeyCode
---@field PrintScreen KeyCode
---@field ScrollLock KeyCode
---@field Pause KeyCode
---@field BrowserBack KeyCode
---@field BrowserFavorites KeyCode
---@field BrowserForward KeyCode
---@field BrowserHome KeyCode
---@field BrowserRefresh KeyCode
---@field BrowserSearch KeyCode
---@field BrowserStop KeyCode
---@field Eject KeyCode
---@field LaunchApp1 KeyCode
---@field LaunchApp2 KeyCode
---@field LaunchMail KeyCode
---@field MediaPlayPause KeyCode
---@field MediaSelect KeyCode
---@field MediaStop KeyCode
---@field MediaTrackNext KeyCode
---@field MediaTrackPrevious KeyCode
---@field Power KeyCode
---@field Sleep KeyCode
---@field AudioVolumeDown KeyCode
---@field AudioVolumeMute KeyCode
---@field AudioVolumeUp KeyCode
---@field WakeUp KeyCode
---@field Meta KeyCode
---@field Hyper KeyCode
---@field Turbo KeyCode
---@field Abort KeyCode
---@field Resume KeyCode
---@field Suspend KeyCode
---@field Again KeyCode
---@field Copy KeyCode
---@field Cut KeyCode
---@field Find KeyCode
---@field Open KeyCode
---@field Paste KeyCode
---@field Props KeyCode
---@field Select KeyCode
---@field Undo KeyCode
---@field Hiragana KeyCode
---@field Katakana KeyCode
---@field F1 KeyCode
---@field F2 KeyCode
---@field F3 KeyCode
---@field F4 KeyCode
---@field F5 KeyCode
---@field F6 KeyCode
---@field F7 KeyCode
---@field F8 KeyCode
---@field F9 KeyCode
---@field F10 KeyCode
---@field F11 KeyCode
---@field F12 KeyCode
---@field F13 KeyCode
---@field F14 KeyCode
---@field F15 KeyCode
---@field F16 KeyCode
---@field F17 KeyCode
---@field F18 KeyCode
---@field F19 KeyCode
---@field F20 KeyCode
---@field F21 KeyCode
---@field F22 KeyCode
---@field F23 KeyCode
---@field F24 KeyCode
---@field F25 KeyCode
---@field F26 KeyCode
---@field F27 KeyCode
---@field F28 KeyCode
---@field F29 KeyCode
---@field F30 KeyCode
---@field F31 KeyCode
---@field F32 KeyCode
---@field F33 KeyCode
---@field F34 KeyCode
---@field F35 KeyCode
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


---@class NativeKeyCode_static
NativeKeyCode = {}

---@class NativeKeyCode
NativeKeyCode_instance = {}
---@class NativeKeyCode_static
---@field Unidentified NativeKeyCode
NativeKeyCode = {}
---@class NativeKeyCode
---@field Unidentified boolean
---@field Android NativeKeyCode_Android
---@field MacOS NativeKeyCode_MacOS
---@field Windows NativeKeyCode_Windows
---@field Xkb NativeKeyCode_Xkb
NativeKeyCode_instance = {}

---@class NativeKeyCode_Android
NativeKeyCode_Android = {}

---@param state NativeKeyCode_Android
---@return NativeKeyCode
function NativeKeyCode:Android(state) end
---@class NativeKeyCode_MacOS
NativeKeyCode_MacOS = {}

---@param state NativeKeyCode_MacOS
---@return NativeKeyCode
function NativeKeyCode:MacOS(state) end
---@class NativeKeyCode_Windows
NativeKeyCode_Windows = {}

---@param state NativeKeyCode_Windows
---@return NativeKeyCode
function NativeKeyCode:Windows(state) end
---@class NativeKeyCode_Xkb
NativeKeyCode_Xkb = {}

---@param state NativeKeyCode_Xkb
---@return NativeKeyCode
function NativeKeyCode:Xkb(state) end

---@class KeyEvent_static
KeyEvent = {}

---@class KeyEvent
---@field physical_key PhysicalKey
---@field state ElementState
---@field repeat boolean
KeyEvent_instance = {}

---@class KeyLocation_static
KeyLocation = {}

---@class KeyLocation
KeyLocation_instance = {}
---@class KeyLocation_static
---@field Standard KeyLocation
---@field Left KeyLocation
---@field Right KeyLocation
---@field Numpad KeyLocation
KeyLocation = {}
---@class KeyLocation
---@field Standard boolean
---@field Left boolean
---@field Right boolean
---@field Numpad boolean
KeyLocation_instance = {}


---@class TouchPhase_static
TouchPhase = {}

---@class TouchPhase
TouchPhase_instance = {}
---@class TouchPhase_static
---@field Started TouchPhase
---@field Moved TouchPhase
---@field Ended TouchPhase
---@field Cancelled TouchPhase
TouchPhase = {}
---@class TouchPhase
---@field Started boolean
---@field Moved boolean
---@field Ended boolean
---@field Cancelled boolean
TouchPhase_instance = {}


---@class Touch_static
Touch = {}

---@class Touch
---@field phase TouchPhase
---@field location Vector2
---@field force Force?
---@field id integer
Touch_instance = {}

---@class Force_static
Force = {}

---@class Force
Force_instance = {}
---@class Force_static
Force = {}
---@class Force
---@field Calibrated Force_Calibrated
---@field Normalized Force_Normalized
Force_instance = {}

---@class Force_Calibrated
Force_Calibrated = {}

---@param state Force_Calibrated
---@return Force
function Force:Calibrated(state) end
---@class Force_Normalized
Force_Normalized = {}

---@param state Force_Normalized
---@return Force
function Force:Normalized(state) end

---@class ElementState_static
ElementState = {}

---@class ElementState
ElementState_instance = {}
---@class ElementState_static
---@field Pressed ElementState
---@field Released ElementState
ElementState = {}
---@class ElementState
---@field Pressed boolean
---@field Released boolean
ElementState_instance = {}


---@class MouseButton_static
MouseButton = {}

---@class MouseButton
MouseButton_instance = {}
---@class MouseButton_static
---@field Left MouseButton
---@field Right MouseButton
---@field Middle MouseButton
---@field Back MouseButton
---@field Forward MouseButton
MouseButton = {}
---@class MouseButton
---@field Left boolean
---@field Right boolean
---@field Middle boolean
---@field Back boolean
---@field Forward boolean
---@field Other MouseButton_Other
MouseButton_instance = {}

---@class MouseButton_Other
MouseButton_Other = {}

---@param state MouseButton_Other
---@return MouseButton
function MouseButton:Other(state) end

---@class MouseScrollDelta_static
MouseScrollDelta = {}

---@class MouseScrollDelta
MouseScrollDelta_instance = {}
---@class MouseScrollDelta_static
MouseScrollDelta = {}
---@class MouseScrollDelta
---@field LineDelta MouseScrollDelta_LineDelta
---@field PixelDelta MouseScrollDelta_PixelDelta
MouseScrollDelta_instance = {}

---@class MouseScrollDelta_LineDelta
MouseScrollDelta_LineDelta = {}

---@param state MouseScrollDelta_LineDelta
---@return MouseScrollDelta
function MouseScrollDelta:LineDelta(state) end
---@class MouseScrollDelta_PixelDelta
MouseScrollDelta_PixelDelta = {}

---@param state MouseScrollDelta_PixelDelta
---@return MouseScrollDelta
function MouseScrollDelta:PixelDelta(state) end

---@class InnerSizeWriter_static
InnerSizeWriter = {}

---@class InnerSizeWriter
InnerSizeWriter_instance = {}

---@param new_inner_size Vector2i
---@return boolean
function InnerSizeWriter_instance:request_inner_size(new_inner_size) end

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

---@class Vector2_static
Vector2 = {}

---@class Vector2
---@field x number
---@field y number
Vector2_instance = {}

---@class Vector2i_static
Vector2i = {}

---@class Vector2i
---@field x integer
---@field y integer
Vector2i_instance = {}

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

---@param new_pos Vector3
function Node_instance:set_local_position(new_pos) end

---@param value Quaternion
function Node_instance:set_local_rotation(value) end

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

---@param tag string
function Node_instance:set_tag(tag) end

---@class RoutingStrategy_static
RoutingStrategy = {}

---@class RoutingStrategy
RoutingStrategy_instance = {}
---@class RoutingStrategy_static
---@field Up RoutingStrategy
---@field Down RoutingStrategy
RoutingStrategy = {}
---@class RoutingStrategy
---@field Up boolean
---@field Down boolean
RoutingStrategy_instance = {}


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

---@class Intersection_static
Intersection = {}

---@class Intersection
---@field collider Node
---@field normal Vector3
---@field position Vector3
---@field feature FeatureId
---@field toi number
Intersection_instance = {}

---@class FeatureId_static
FeatureId = {}

---@class FeatureId
FeatureId_instance = {}
---@class FeatureId_static
---@field Unknown FeatureId
FeatureId = {}
---@class FeatureId
---@field Vertex FeatureId_Vertex
---@field Edge FeatureId_Edge
---@field Face FeatureId_Face
---@field Unknown boolean
FeatureId_instance = {}

---@class FeatureId_Vertex
FeatureId_Vertex = {}

---@param state FeatureId_Vertex
---@return FeatureId
function FeatureId:Vertex(state) end
---@class FeatureId_Edge
FeatureId_Edge = {}

---@param state FeatureId_Edge
---@return FeatureId
function FeatureId:Edge(state) end
---@class FeatureId_Face
FeatureId_Face = {}

---@param state FeatureId_Face
---@return FeatureId
function FeatureId:Face(state) end

---@class RayCastOptions_static
RayCastOptions = {}

---@class RayCastOptions
---@field ray_origin Vector3
---@field ray_direction Vector3
---@field max_len number
---@field groups InteractionGroups?
---@field sort_results boolean
RayCastOptions_instance = {}

---@class InteractionGroups_static
InteractionGroups = {}

---@class InteractionGroups
---@field memberships integer
---@field filter integer
InteractionGroups_instance = {}

---@class RigidBody_static
RigidBody = {}

---@class RigidBody
RigidBody_instance = {}

---@param force Vector3
function RigidBody_instance:apply_force(force) end

---@class Plugin_static
Plugin = {}

---@class Plugin
Plugin_instance = {}

---@generic T
---@param class_name `T`
---@return T
function Plugin:get(class_name) end

---@class Prefab_static
Prefab = {}

---@class Prefab
Prefab_instance = {}

---@param position Vector3
---@param orientation Quaternion
---@return Node
function Prefab_instance:instantiate_at(position, orientation) end

---@class Scene_static
Scene = {}

---@class Scene
Scene_instance = {}

---@param scene_path string
function Scene:load_async(scene_path) end

---@class UiNode_static
UiNode = {}

---@class UiNode
UiNode_instance = {}

---@class Text_static
Text = {}

---@class Text
---@field text_async string
Text_instance = {}

---@param state TextBuilder
---@return Text
function Text:new(state) end

---@param text string
function Text_instance:set_text_async(text) end

---@class TextBuilder_static
TextBuilder = {}

---@class TextBuilder
---@field foregound Brush?
---@field font_size number?
TextBuilder_instance = {}

---@class Brush_static
Brush = {}

---@class Brush
Brush_instance = {}
---@class Brush_static
Brush = {}
---@class Brush
---@field Solid Brush_Solid
---@field LinearGradient Brush_LinearGradient
---@field RadialGradient Brush_RadialGradient
Brush_instance = {}

---@class Brush_Solid
Brush_Solid = {}

---@param state Brush_Solid
---@return Brush
function Brush:Solid(state) end
---@class Brush_LinearGradient
Brush_LinearGradient = {}

---@param state Brush_LinearGradient
---@return Brush
function Brush:LinearGradient(state) end
---@class Brush_RadialGradient
Brush_RadialGradient = {}

---@param state Brush_RadialGradient
---@return Brush
function Brush:RadialGradient(state) end

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

---@class GradientPoint_static
GradientPoint = {}

---@class GradientPoint
---@field stop number
---@field color Color
GradientPoint_instance = {}

---@class Window_static
---@field cursor_grab CursorGrabMode
Window = {}

---@class Window
Window_instance = {}

---@param mode CursorGrabMode
function Window:set_cursor_grab(mode) end

---@class CursorGrabMode_static
CursorGrabMode = {}

---@class CursorGrabMode
CursorGrabMode_instance = {}
---@class CursorGrabMode_static
---@field None CursorGrabMode
---@field Confined CursorGrabMode
---@field Locked CursorGrabMode
CursorGrabMode = {}
---@class CursorGrabMode
---@field None boolean
---@field Confined boolean
---@field Locked boolean
CursorGrabMode_instance = {}


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

---@param value number
function Vector3_instance:set_x(value) end

---@param value number
function Vector3_instance:set_y(value) end

---@param value number
function Vector3_instance:set_z(value) end

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
