---@uuid 9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e
---@class Guard : Script
---@field private reloading_sec number
---@field reload_delay_sec number
---@field gun_height number
---@field switch_waypoint_timeout_sec number
---@field private waypoint_sec number
---@field private current_waypoint Vector3?
---@field private collider Node
---@field bullet_prefab Prefab
---@field initial_bullet_velocity number
---@field attack_range number
---@field beacon_reached_distance number
---@field move_power number
---@field private id number
Guard = script_class()


---@return boolean
function Guard:try_attack_player()
    local player_pos = Plugin:get("Game").player.global_position
    local self_pos = self.node.global_position
    local sight_vector = player_pos:sub(self_pos)

    if self:can_see_player(player_pos, sight_vector) then
        Bullet:spawn(
            {
                prefab = self.bullet_prefab,
                origin = self_pos:add(Vector3:new(0.0, self.gun_height, 0.0)),
                direction = sight_vector,
                initial_velocity = self.initial_bullet_velocity,
                author_collider = self.collider,
                range = self.attack_range,
            }
        )
        self.reloading_sec = self.reload_delay_sec
        return true
    end

    return false
end

---@param player_pos Vector3
---@param sight_vector Vector3
---@return boolean
function Guard:can_see_player(player_pos, sight_vector)
    ---@type RayCastOptions
    local opts = {
        ray_origin = player_pos,
        ray_direction = sight_vector:normalize(),
        max_len = sight_vector:magnitude(),
        sort_results = true
    }
    local results = Physics:cast_ray(opts)
    for i, hit in ipairs(results) do
        local node = hit.collider
        if node ~= self.collider then
            while node.alive do
                if node:find_script("Player") then
                    return true
                end
                node = node.parent
            end
            return false
        end
    end
    return false
end

function Guard:move_to_waypoint(dt)
    self.waypoint_sec = self.waypoint_sec + dt
    if self.waypoint_sec > self.switch_waypoint_timeout_sec then
        self.current_waypoint = nil;
        self.waypoint_sec = 0.0;

    end
    if self.current_waypoint == nil then
        local beacons = Plugin:get("Game").beacons;
        self.current_waypoint = beacons[math.random(#beacons)]
    end
    local pos = self.node.local_position
    local vector_to_beacon = self.current_waypoint:sub(pos)
    if vector_to_beacon:magnitude() < self.beacon_reached_distance then
        self.current_waypoint = nil;
    else
        local force = vector_to_beacon:normalize():mul(self.move_power)
        self.node:as_rigid_body():apply_force(force);
    end
end


function Guard:on_init()
    self.collider = self.node:find_collider_in_children()
    if not self.collider then
        Log:err("Collider not found under Guard node")
    end
end

function Guard:on_start()
    self.node:subscribe_to();
end

function Guard:on_update(dt)
    if self.reloading_sec > 0.0 then
        self.reloading_sec = self.reloading_sec - dt;
    end

    if self.reloading_sec > 0.0 or not self:try_attack_player() then
        self:move_to_waypoint(dt)
    end
end

function Guard:on_message(message)
    print("on_message", message)
    local bullet = message == Bullet.HitMessage
    if bullet then
        self.node:destroy()
        Plugin:get("Game"):inc_frags()
        print("guard killed!")
    end
end
