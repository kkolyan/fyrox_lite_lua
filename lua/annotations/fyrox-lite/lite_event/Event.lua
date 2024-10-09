
-----------------------------------------------------------
------ fyrox_lite::lite_event::Event
-----------------------------------------------------------

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

