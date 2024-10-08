
---@class Event
Event = {}

---@class StartCause
StartCause = {}

---@class WindowEvent
WindowEvent = {}

---@class DeviceEvent
DeviceEvent = {}

---@class RawKeyEvent
---@field physical_key PhysicalKey
---@field state ElementState
RawKeyEvent = {}

---@class PhysicalKey
PhysicalKey = {}

---@class KeyCode
KeyCode = {}

---@class NativeKeyCode
NativeKeyCode = {}

---@class KeyEvent
---@field physical_key PhysicalKey
---@field state ElementState
---@field repeat boolean
KeyEvent = {}

---@class KeyLocation
KeyLocation = {}

---@class TouchPhase
TouchPhase = {}

---@class Touch
---@field phase TouchPhase
---@field location Vector2
---@field force Force?
---@field id integer
Touch = {}

---@class Force
Force = {}

---@class ElementState
ElementState = {}

---@class MouseButton
MouseButton = {}

---@class MouseScrollDelta
MouseScrollDelta = {}

---@class InnerSizeWriter
InnerSizeWriter = {}

---@param new_inner_size Vector2i
---@return boolean
function InnerSizeWriter:request_inner_size(new_inner_size) end

---@class Log
Log = {}

---@class Vector2
---@field x number
---@field y number
Vector2 = {}

---@class Vector2i
---@field x integer
---@field y integer
Vector2i = {}

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
Node = {}

---@return RigidBody?
function Node:as_rigid_body() end

function Node:destroy() end

---@param routing RoutingStrategy
---@param payload UserScriptMessage
function Node:send_hierarchical(routing, payload) end

---@param new_pos Vector3
function Node:set_local_position(new_pos) end

---@param value Quaternion
function Node:set_local_rotation(value) end

function Node:subscribe_to() end

---@return Node?
function Node:find_collider_in_children() end

---@param class_name string
---@return UserScript
function Node:add_script(class_name) end

---@param class_name string
---@return UserScript?
function Node:find_script(class_name) end

---@param tag string
---@return boolean
function Node:tag_is(tag) end

---@param tag string
function Node:set_tag(tag) end

---@class RoutingStrategy
RoutingStrategy = {}

---@class Physics
Physics = {}

---@class Intersection
---@field collider Node
---@field normal Vector3
---@field position Vector3
---@field feature FeatureId
---@field toi number
Intersection = {}

---@class FeatureId
FeatureId = {}

---@class RayCastOptions
---@field ray_origin Vector3
---@field ray_direction Vector3
---@field max_len number
---@field groups InteractionGroups?
---@field sort_results boolean
RayCastOptions = {}

---@class InteractionGroups
---@field memberships integer
---@field filter integer
InteractionGroups = {}

---@class RigidBody
RigidBody = {}

---@param force Vector3
function RigidBody:apply_force(force) end

---@class Plugin
Plugin = {}

---@class Prefab
Prefab = {}

---@param position Vector3
---@param orientation Quaternion
---@return Node
function Prefab:instantiate_at(position, orientation) end

---@class Scene
Scene = {}

---@class UiNode
UiNode = {}

---@class Text
---@field text_async string
Text = {}

---@param text string
function Text:set_text_async(text) end

---@class TextBuilder
---@field foregound Brush?
---@field font_size number?
TextBuilder = {}

---@class Brush
Brush = {}

---@class Color
Color = {}

---@class GradientPoint
---@field stop number
---@field color Color
GradientPoint = {}

---@class Window
Window = {}

---@class CursorGrabMode
CursorGrabMode = {}

---@class Quaternion
Quaternion = {}

---@param o Vector3
---@return Vector3
function Quaternion:mul_vec(o) end

---@param rot_delta Quaternion
---@return Quaternion
function Quaternion:mul_quat(rot_delta) end

---@class Vector3
---@field x number
---@field y number
---@field z number
Vector3 = {}

---@param value number
function Vector3:set_x(value) end

---@param value number
function Vector3:set_y(value) end

---@param value number
function Vector3:set_z(value) end

---@param o number
---@return Vector3
function Vector3:mul(o) end

---@param o Vector3
---@return Vector3
function Vector3:add(o) end

---@return Vector3
function Vector3:normalize() end

---@param o Vector3
---@return Vector3
function Vector3:sub(o) end

---@return number
function Vector3:magnitude() end

function Vector3:normalize_inplace() end
