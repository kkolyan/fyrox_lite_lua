-- This document follows this specification: https://luals.github.io/wiki/annotations

---@diagnostic disable: missing-return, lowercase-global

-- That file is just annotations for VSCode, are not part of source code and will be provided by bindings

---@class Script
Script = {}


---@type Node
Script.node = {}

---@class Plugin
Plugin = {}

---@generic T
---@param class `T`
---@return T
function Plugin:get(class) end

function Plugin:update() end

---@class Vector3
---@field x number
---@field y number
---@field z number
Vector3 = {}

---@type Vector3
Vector3.X = nil

---@type Vector3
Vector3.Y = nil

---@type Vector3
Vector3.Z = nil


---@return Vector3
function Vector3:normalize() end

---@param f number
---@return Vector3
function Vector3:mul(f) end

---@param x number
---@param y number
---@param z number
---@return Vector3
function Vector3:new(x, y, z) end

---@return number
function Vector3:magnitude() end

---@param other Vector3
---@return Vector3
function Vector3:add(other) end

---@param other Vector3
---@return Vector3
function Vector3:sub(other) end

---@class Quaternion
Quaternion = {}

---@param direction Vector3
---@param up Vector3
---@return Quaternion
function Quaternion:face_towards(direction, up) end

---@param axis Vector3
---@param angle number
---@return Quaternion
function Quaternion:from_axis_angle(axis, angle) end

---@class Node
Node = {}

---@generic T
---@param class `T`
---@initial_state T
---@return T
function Node:add_script(class) end

---@return Vector3
function Node:global_position() end

---@return Vector3
function Node:local_position() end

---@param v Vector3
function Node:set_local_position(v) end

function Node:destroy() end

function Node:subscribe_to() end

---@return RigidBody
function Node:as_rigid_body() end

---@return Node
function Node:find_collider_in_children() end

---@param routing RoutingStrategy
---@param message any
function Node:send_hierarchical(routing, message) end

---@class RigidBody
RigidBody = {}

---@param force Vector3
function RigidBody:apply_force(force) end

---@class Prefab
Prefab = {}

---@param position Vector3
---@param orientation Quaternion
---@return Node
function Prefab:instantiate_at(position, orientation) end

function script_class() end

---@class UiNode
UiNode = {}


---@class Brush
Brush = {}

---@param color Color
function Brush:solid(color) end

---@class Color
Color = {}

---@type Color
Color.BLACK = nil


---@class Widget : UiNode
---@field foreground Brush?
Widget = {}

---@class Text : Widget
---@field font_size number
Text = {}

---@static
---@param state Text
---@return Text
function Text:new(state) end

---@param text string
function Text:set_text_async(text) end

---@class Scene
Scene = {}

---@param scene_path string
function Scene:load_async(scene_path) end

---@class CursorGrabMode
CursorGrabMode = {}

---@type CursorGrabMode
CursorGrabMode.None = {}

---@type CursorGrabMode
CursorGrabMode.Locked = {}

---@type CursorGrabMode
CursorGrabMode.Confined = {}


---@class Window
Window = {}

---@param mode CursorGrabMode
function Window:set_cursor_grab(mode) end

---@class Physics
Physics = {}

---@param opts RayCastOptions
---@param results Intersection[]
function Physics:cast_ray(opts, results) end

---@class RayCastOptions
---@field ray_origin Vector3
---@field ray_direction Vector3
---@field max_len number
---@field sort_results boolean
RayCastOptions = {}

---@class Intersection
---@field collider Node
Intersection = {}


---@param o any
function var_dump(o) end

---@class RoutingStrategy
RoutingStrategy = {}

---@type RoutingStrategy
RoutingStrategy.Up = {}

---@type RoutingStrategy
RoutingStrategy.Down = {}


---@class Log
Log = {}

---@param message string
function Log:err(message, ...) end

---@param message string
function Log:info(message, ...) end
