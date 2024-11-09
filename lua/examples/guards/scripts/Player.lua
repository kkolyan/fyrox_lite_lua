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
---@field private aim_y number
Player = script_class()


FRACTION_PLAYER = 0

---@param x number
function Player:turn(x)
    local rot_delta = Quaternion:from_axis_angle(Vector3.Y, self.sensitivity * x);

    self.node.local_rotation = self.node.local_rotation:mul_quat(rot_delta)
end

function Player:aim(y)
    self.aim_y = self.aim_y + y * self.sensitivity;

    self.aim_y = math.max(-math.pi / 2.0, math.min(self.aim_y, math.pi / 2.0))

    self.camera.local_rotation = Quaternion:from_axis_angle(
        Vector3.X,
        self.aim_y
    );
end

function Player:fire()
    local camera_pos = self.camera.global_position;

    local bullet_orientation = self.camera.global_rotation;

    Bullet:spawn(
        {
            prefab = self.bullet,
            origin = camera_pos,
            direction = bullet_orientation:mul_vec(Vector3.Z),
            initial_velocity = self.initial_bullet_velocity,
            author_collider = self.collider,
            range = self.shooting_range,
            fraction = FRACTION_PLAYER,
        }
    );
end


function Player:on_init()
    Window.cursor_grab = CursorGrabMode.Confined;

    self.collider = not_nil(self.node:find_collider_in_children(), "player collider missing")
end

function Player:on_start()
    self.node:subscribe_to()
end

function Player:on_message(message)
    if message.type == Bullet.HitMessage and message.fraction ~= FRACTION_PLAYER then
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

    if Input:is_mouse_button_pressed(Input.MouseLeft) then
        if self.reload_sec <= 0.0 then
            self.reload_sec = self.reload_delay_sec;
            self:fire();
        end
    end

    local move_delta = Vector3.ZERO;

    if Input:is_key_pressed(KeyCode.W) then
        move_delta.z = move_delta.z + 1.0
    end
    if Input:is_key_pressed(KeyCode.S) then
        move_delta.z = move_delta.z - 1.0
    end
    if Input:is_key_pressed(KeyCode.A) then
        move_delta.x = move_delta.x + 1.0
    end
    if Input:is_key_pressed(KeyCode.D) then
        move_delta.x = move_delta.x - 1.0
    end

    self:turn(-Input.mouse_move.x)
    self:aim(Input.mouse_move.y)

    if move_delta:magnitude() > 0.001 then
        move_delta:normalize_inplace();
    end

    local self_rotation = self.node.local_rotation
    local move_delta = self_rotation:mul_vec(move_delta);
    local force = move_delta:mul(self.power);
    self.node:as_rigid_body():apply_force(force);
end
