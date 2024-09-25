---@uuid c5671d19-9f1a-4286-8486-add4ebaadaec
---@class Player : Script
---@field sensitivity number
---@field camera Node
---@field power number
---@field bullet Prefab
---@field initial_bullet_velocity number
---@field shooting_range number
---@field reload_delay_sec number
---@field private reload_sec number
---@field private published boolean
---@field private collider Node
Player = script_class()

local aim_y = 0
local forward = false
local back = false
local left = false
local right = false
local fire = false


---@param x number
function Player:turn(x)
    local rot_delta = Quaternion:from_axis_angle(Vector3.Y, self.sensitivity * x);

    self.node:set_local_rotation(self.node:local_rotation():mul(rot_delta))
end

function Player:aim(y)
    aim_y = aim_y + y * self.sensitivity;

    aim_y = math.max(-math.pi / 2.0, math.min(aim_y, math.pi / 2.0))

    self.camera:set_local_rotation(Quaternion:from_axis_angle(
        Vector3.X,
        aim_y
    ));
end

function Player:fire()
    local camera_pos = self.camera:global_position();

    local bullet_orientation = self.camera:global_rotation();

    Bullet:spawn(
        {
            prefab = self.bullet,
            origin = camera_pos,
            direction = bullet_orientation:mul(Vector3.Z),
            initial_velocity = self.initial_bullet_velocity,
            author_collider = self.collider,
            range = self.shooting_range
        }
    );
end


function Player:on_init()
    Window:set_cursor_grab(CursorGrabMode.Confined);

    self.collider = self.node:find_collider_in_children()
    end

function Player:on_start(ctx)
    self.node:subscribe_to()
end

function Player:on_message(message, ctx)
    local _bullet = message.downcast_ref("BulletHit")
    if _bullet ~= nil then
        Plugin:get("Game"):inc_wounds()
        print("player wounded!")
    end
end

function Player:on_update(dt)
    if self.reload_sec > 0.0 then
        self.reload_sec = self.reload_sec - dt;
    end
    if not self.published then
        self.published = true;
        Plugin:get("Game").player = self.node;
    end

    if fire then
        if self.reload_sec <= 0.0 then
            self.reload_sec = self.reload_delay_sec;
            self:fire();
        end
    end

    local move_delta = Vector3.ZERO;
    if forward then
        move_delta.z = move_delta.z + 1.0
    end
    if back then
        move_delta.z = move_delta.z - 1.0
    end
    if left then
        move_delta.x = move_delta.x + 1.0
    end
    if right then
        move_delta.x = move_delta.x - 1.0
    end

    if move_delta:magnitude() > 0.001 then
        move_delta:normalize_inplace();
    end

    local self_rotation = self.node:local_rotation()
    local move_delta = self_rotation:mul(move_delta);
    local force = move_delta:mul(self.power);
    self.node:as_rigid_body():apply_force(force);
end

function Player:on_os_event(event)
    if event:is("WindowEvent") then
        local event = event.event
        if event:is("KeyboardInput") then
            local event = event.event
            local value = event.state:is("Pressed")
            if event.physical_key:is("Code") then
                local code = event.physical_key[1]
                if code:is("KeyW") then
                    forward = value
                end
                if code:is("KeyS") then
                    back = value
                end
                if code:is("KeyA") then
                    left = value
                end
                if code:is("KeyD") then
                    right = value
                end
            end
        end
        if event:is("MouseInput") then
            if event.button:is("Left") then
                fire = event.state:is("Pressed")
            end
        end
    end
    if event:is("DeviceEvent") then
        local event = event.event
        if event:is("MouseMotion") then
            local x, y = table.unpack(event.delta)
            self:turn(-x);
            self:aim(y);
        end
    end
end
