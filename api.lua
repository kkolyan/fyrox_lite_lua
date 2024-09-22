-- This document follows this specification: https://luals.github.io/wiki/annotations

---@diagnostic disable: missing-return, lowercase-global

-- That file is just annotations for VSCode, are not part of source code and will be provided by bindings

---@class Script
Script = {}

---@class Plugin
Plugin = {}

---@param ctx PluginContext
function Plugin:update(ctx) end

---@class PluginContext
PluginContext = {}

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

---@class ScriptContext

ScriptContext = {}


function script_class() end


---@return Plugin
function plugin_class() end


---@class UiNode
UiNode = {}

---@class TextBuilder
---@field font_size number?
---@field text string?
---@field widget WidgetBuilder?
TextBuilder = {}

---@param b TextBuilder
---@return UiNode
function TextBuilder:build(ui, b) end

---@class WidgetBuilder
---@field foreground Brush?
WidgetBuilder = {}


---@class Brush
Brush = {}

---@param color Color
function Brush.Solid(color) end

---@class Color
Color = {}

---@type Color
Color.BLACK = nil
