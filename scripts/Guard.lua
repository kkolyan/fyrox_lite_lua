---@uuid 9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e
---@class Guard : Script
---@field private reloading_sec number
---@field reload_delay_sec number
---@field gun_height number
---@field switch_waypoint_timeout_sec number
---@field private waypoint_sec number
---@field private current_waypoint Vector3
---@field private collider Node
---@field bullet_prefab Prefab
---@field initial_bullet_velocity number
---@field attack_range number
---@field beacon_reached_distance number
---@field move_power number
Guard = script_class()

---@param ctx ScriptContext
---@return boolean
function Guard:try_attack_player(ctx)
    local player_pos = ctx.plugins:get("Game").player:global_position()
    local self_pos = ctx.handle:global_position()
    local sight_vector = player_pos:sub(self_pos)
    print(string.format(
        "try attack player. player_pos: %s, self_pos: %s",
        player_pos, self_pos
    ))

    if self:can_see_player(ctx, player_pos, sight_vector) then
        Bullet:spawn(
            ctx.scene,
            {
                prefab = self.bullet_prefab,
                origin = self_pos + Vector3:new(0.0, self.gun_height, 0.0),
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

---@param ctx ScriptContext
---@param player_pos Vector3
---@param sight_vector Vector3
---@return boolean
function Guard:can_see_player(ctx, player_pos, sight_vector)
    ---@type RayCastOptions
    local opts = {
        ray_origin = Point3:from(player_pos),
        ray_direction = sight_vector:normalize(),
        max_len = sight_vector:magnitude(),
        sort_results = true
    }
    local results = {}
    physics:cast_ray(ctx, opts, results)
    for i, hit in ipairs(results) do
        local node = hit.collider
        if node ~= self.collider then
            while node:is_some() do
                if node:has_script("Player") then
                    return true
                end
                node = node:parent()
            end
            return false
        end
    end
    return false
end

---@param ctx ScriptContext
function Guard:move_to_waypoint(ctx)
    self.waypoint_sec = self.waypoint_sec + ctx.dt
    if self.waypoint_sec > self.switch_waypoint_timeout_sec then
        self.current_waypoint = nil;
        self.waypoint_sec = 0.0;

        -- println!("guard {:?end switched waypoint", ctx.handle);
    end
    if self.current_waypoint ~= nil then
        local beacons = ctx.plugins:get("Game").beacons;
        self.current_waypoint = beacons[math.random(#beacons)]
    end
    local pos = ctx.handle
        :local_transform()
        :position()
        :get_value_ref();
    local vector_to_beacon = self.current_waypoint.unwrap().sub(pos)
    if vector_to_beacon:magnitude() < self.beacon_reached_distance then
        self.current_waypoint = nil;
    else
        local force = vector_to_beacon.normalize() * self.move_power;
        ctx.handle
            :as_rigid_body_mut()
            :unwrap()
            :apply_force(force);
    end
end


---@param ctx ScriptContext
function Guard:on_init(ctx)
    self.collider = ctx
        .handle
        .try_get_collider(ctx.scene)
        :expect("Collider not found under Guard node");
end

function Guard:on_start()
    ctx.message_dispatcher.subscribe_to("BulletHit", ctx.handle);
end

---@param ctx ScriptContext
function Guard:on_update(ctx)
    if self.reloading_sec > 0.0 then
        self.reloading_sec = self.reloading_sec - ctx.dt;
    end

    if self.reloading_sec > 0.0 or not self:try_attack_player(ctx) then
        self:move_to_waypoint(ctx)
    end
end

---@param ctx ScriptContext
---@param message ScriptMessagePayload
function Guard:on_message(ctx, message)
    local bullet = message:downcast_ref("BulletHit")
    if bullet then
        ctx.handle:remove_node();
        ctx.plugins:get_mut("Game"):inc_frags()
        print("guard killed!")
    end
end
