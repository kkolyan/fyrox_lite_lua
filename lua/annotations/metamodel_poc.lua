---@class Class<MyClass>
---@field SOMETHING number
MyClass = {}

---@return MyClass
function MyClass:my_static_method() end

---@class MyClass
---@field that number
MyClass_instance = {}


function MyClass_instance:my_instance_method() end


local s = MyClass:my_static_method()
s:my_instance_method()

local i = MyClass.SOMETHING
local j = s.that