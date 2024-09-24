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
Vector3 = {}


---@class Node
Node = {}

---@generic T
---@param class `T`
---@initial_state T
---@return T
function Node:add_script(class) end

---@class Prefab
Prefab = {}

---@return Node
function Prefab:instantiate_at() end


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