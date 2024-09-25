---@uuid 12371d19-9f1a-4286-8486-add4ebaadaec
---@class Bullet : Script
---@field velocity Vector3
---@field remaining_sec number
---@field author_collider Node
Bullet = script_class()

Bullet.HitMessage = "BulletHit"


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
function Bullet:spawn(seed)
    local orientation = Quaternion:face_towards(seed.direction, Vector3.Y);
    local bullet = seed.prefab:instantiate_at(seed.origin, orientation);
    local script = bullet:add_script("Bullet")
    script.velocity = seed.direction:normalize():mul(seed.initial_velocity)
    script.remaining_sec = seed.range / seed.initial_velocity
    script.author_collider = seed.author_collider
end

---@param ctx ScriptContext
function Bullet:on_update(dt)
    self.remaining_sec = self.remaining_sec - dt
    if self.remaining_sec <= 0.0 then
        ctx.handle:remove_node()
        return;
    end
    local new_pos = self.node:local_position():add(self.velocity:mul(dt))

    ---@type RayCastOptions
    local opts = {
        ray_origin= self.node:local_position(),
        ray_direction= self.velocity:normalize(),
        max_len=self.velocity:magnitude() * dt,
        sort_results= true
    }
    local results = {}
    Physics:cast_ray(opts, results)

    for i, hit in ipairs(results) do
        if hit.collider ~= self.author_collider then
            hit.collider:send_hierarchical(RoutingStrategy.Up, Bullet.HitMessage)
			self.node:destroy()
            return;
        end
    end

    self.node:set_local_position(new_pos);
end