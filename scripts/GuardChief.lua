---@uuid c69ae5fa-de26-4ee5-b70c-113df285f6e2
---@class GuardChief : Script
---@field gaurd_prefab Prefab
---@field initial_count number
---@field private initialized boolean
---@field private frame_skipped_for_beacons boolean
GuardChief = script_class()


function GuardChief:on_update(ctx)
    if not self.frame_skipped_for_beacons then
        self.frame_skipped_for_beacons = true;
        return;
    end
    if not self.initialized then
        self.initialized = true;
        for _ in 0, self.initial_count do
            local beacons = ctx.plugins.get("Game").beacons;
            if not beacons.is_empty() then
                local position = beacons[math.random(#beacons)];

                local orientation = UnitQuaternion:from_axis_angle(Vector3:y_axis(), math.random() * 2 * math.pi);

                self.gaurd_prefab.as_ref().unwrap().instantiate_at(
                    ctx.scene,
                    position,
                    orientation
                )
                Log:info(string.format("guard spawned at %s", position));
            else
                Log:err("cannot spawn guards: no beacons found");
            end
        end
    end
end