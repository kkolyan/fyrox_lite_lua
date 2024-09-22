---@uuid 12371d19-9f1a-4286-8486-add4ebaadaec
---@class Bullet : Script
---@field velocity Vector3
---@field remaining_sec number
---@field author_collider Node
Bullet = script_class()


---@class BulletSeed
---@field prefab Prefab
---@field origin Vector3
---@field direction Vector3
---@field initial_velocity number
---@field author_collider Node
---@field range number
local BulletSeed = {}

---@param scene Scene
---@param seed BulletSeed
function Bullet:spawn(scene, seed)
    local orientation = UnitQuaternion:face_towards(seed.direction, Vector3:y_axis());
    local bullet = seed.prefab:instantiate_at(scene, seed.origin, orientation);
    local script = bullet:add_script("Bullet")
    script.velocity = seed.direction:normalize() * seed.initial_velocity
    script.remaining_sec = seed.range / seed.initial_velocity
    script.author_collider = seed.author_collider
end

---@param ctx ScriptContext
function Bullet:on_update(ctx)
    self.remaining_sec = self.remaining_sec - ctx.dt
    if self.remaining_sec <= 0.0 then
        ctx.handle:remove_node()
        return;
    end
    local t = ctx.handle:local_transform_mut()
    local new_pos = t:position():add(self.velocity * ctx.dt)

    ---@type RayCastOptions
    local opts = {
        ray_origin= Point3:from(t:position():get_value_ref()),
        ray_direction= self.velocity:normalize(),
        max_len=self.velocity:magnitude() * ctx.dt,
        sort_results= true
    }
    local results = {}
    physics:cast_ray(opts, results)

    for hit in ipairs(results) do
        if hit.collider ~= self.author_collider then
            ctx.message_sender
                .send_hierarchical(hit.collider, RoutingStrategy.Up, BulletHit:new())
			ctx.handle:remove_node();
            return;
        end
    end

    ctx.handle
        :local_transform_mut()
        :set_position(new_pos);
end