-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format: https://luals.github.io/wiki/annotations
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_node::LiteNode
-----------------------------------------------------------

---@class Node_static
Node = {}

---@class Node
---@field name string
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
---@param class_id `T`
---@return T
function Node_instance:add_script(class_id) end

---@generic T
---@param class_id `T`
---@return T?
function Node_instance:find_script(class_id) end

---@param tag string
---@return boolean
function Node_instance:tag_is(tag) end

