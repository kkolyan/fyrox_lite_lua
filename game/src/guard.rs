//! Game project.
use fyrox::asset::Resource;
use fyrox::core::ComponentProvider;
use fyrox::rand::random;
use fyrox::resource::model::Model;
use fyrox::script::ScriptMessagePayload;
use fyrox::{
    core::{
        algebra::Vector3, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*, TypeUuidProvider,
    },
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};
use fyrox_lite_api::lite_ctx::{LiteContext, LiteScript};
use fyrox_lite_api::lite_math::LiteVector3;
use fyrox_lite_api::lite_node::LiteNode;
use fyrox_lite_api::lite_physics::{LitePhysics, LiteRayCastOptions};

use crate::bullet::{Bullet, BulletHit, BulletSeed};
use crate::game::Game;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e")]
#[visit(optional)]
pub struct Guard {
    #[reflect(hidden)]
    reloading_sec: f64,

    reload_delay_sec: f64,

    gun_height: f64,

    switch_waypoint_timeout_sec: f64,

    #[reflect(hidden)]
    waypoint_sec: f64,

    #[reflect(hidden)]
    current_waypoint: Option<Vector3<f32>>,

    #[reflect(hidden)]
    collider: Handle<Node>,

    bullet_prefab: Option<Resource<Model>>,
    initial_bullet_velocity: f64,
    attack_range: f64,

    beacon_reached_distance: f64,
    move_power: f64,
}

impl Guard {
    fn try_attack_player(&mut self, ctx: &mut LiteContext) -> bool {
        let player_pos = ctx
            .with_plugin::<Game, _>(|it| LiteNode::from(it.player))
            .global_position();
        let self_pos = ctx.node.global_position();
        let sight_vector = player_pos.sub(self_pos);
        println!(
            "try attack player. player_pos: {:?}, self_pos: {:?}",
            player_pos, self_pos
        );

        if self.can_see_player(player_pos, sight_vector) {
            Bullet::spawn(BulletSeed {
                prefab: self.bullet_prefab.as_ref().unwrap().clone(),
                origin: self_pos.add(LiteVector3::new(0.0, self.gun_height as f32, 0.0)),
                direction: sight_vector,
                initial_velocity: self.initial_bullet_velocity as f32,
                author_collider: self.collider,
                range: self.attack_range as f32,
            });
            self.reloading_sec = self.reload_delay_sec;
            return true;
        }

        false
    }

    fn can_see_player(&self, player_pos: LiteVector3, sight_vector: LiteVector3) -> bool {
        let opts = LiteRayCastOptions {
            ray_origin: player_pos,
            ray_direction: sight_vector.normalize(),
            max_len: sight_vector.magnitude(),
            groups: Default::default(),
            sort_results: true,
        };
        let mut results = vec![];
        LitePhysics::cast_ray(opts, &mut results);
        for hit in results {
            let mut handle = hit.collider;
            if handle == self.collider.into() {
                continue;
            }
            while handle.is_valid() {
                let node = &handle;
                if node.tag_is("Player") {
                    return true;
                }
                handle = node.parent();
            }
            return false;
        }
        false
    }

    fn move_to_waypoint(&mut self, ctx: &mut LiteContext) {
        self.waypoint_sec += ctx.dt as f64;
        if self.waypoint_sec > self.switch_waypoint_timeout_sec {
            self.current_waypoint = None;
            self.waypoint_sec = 0.0;

            // println!("guard {:?} switched waypoint", ctx.handle);
        }
        if self.current_waypoint.is_none() {
            ctx.with_plugin::<Game, _>(|it| {
                let beacons = &it.beacons;
                self.current_waypoint = Some(beacons[random::<usize>() % beacons.len()]);
            });
        }
        let pos = ctx.node.local_position();
        let vector_to_beacon = LiteVector3(self.current_waypoint.unwrap()).sub(pos);
        if vector_to_beacon.magnitude() < self.beacon_reached_distance as f32 {
            self.current_waypoint = None;
        } else {
            let force = vector_to_beacon.normalize().mul(self.move_power as f32);
            ctx.node.as_rigid_body().unwrap().apply_force(force);
        }
    }
}

impl LiteScript for Guard {
    fn on_init(&mut self, ctx: &mut LiteContext) {
        self.collider = ctx
            .node
            .try_get_collider()
            .expect("Collider not found under Guard node")
            .into();
    }

    fn on_start(&mut self, ctx: &mut LiteContext) {
        ctx.node.subscribe_to::<BulletHit>();
    }

    fn on_update(&mut self, ctx: &mut LiteContext) {
        if self.reloading_sec > 0.0 {
            self.reloading_sec -= ctx.dt as f64;
        }

        if self.reloading_sec > 0.0 || !self.try_attack_player(ctx) {
            self.move_to_waypoint(ctx);
        }
    }

    fn on_message(&mut self, message: &mut dyn ScriptMessagePayload, ctx: &mut LiteContext) {
        if let Some(_bullet) = message.downcast_ref::<BulletHit>() {
            ctx.node.destroy();
            ctx.with_plugin::<Game, _>(|it| {
                it.frags += 1;
            });
            println!("guard killed!");
        }
    }
}

impl ScriptTrait for Guard {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_init(ctx);
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_start(ctx);
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_update(ctx);
    }

    fn on_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        self.redispatch_message(message, ctx);
    }
}
