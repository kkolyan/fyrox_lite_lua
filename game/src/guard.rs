//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::Point3;
use fyrox::core::ComponentProvider;
use fyrox::graph::BaseSceneGraph;
use fyrox::rand::random;
use fyrox::resource::model::Model;
use fyrox::scene::graph::physics::RayCastOptions;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::{
    core::{
        algebra::Vector3, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*, TypeUuidProvider,
    },
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};
use fyrox_lite_api::lite_ctx::{LiteContext, LiteScript};
use fyrox_lite_api::lite_node::LiteNode;
use fyrox_lite_api::lite_physics::LitePhysics;
use std::ops::Sub;

use crate::bullet::{Bullet, BulletHit, BulletSeed};
use crate::fyrox_utils::HandleNodeExt;
use crate::game::Game;
use crate::player::Player;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e")]
#[visit(optional)]
pub struct Guard {
    #[reflect(hidden)]
    reloading_sec: f32,

    reload_delay_sec: f32,

    gun_height: f32,

    switch_waypoint_timeout_sec: f32,

    #[reflect(hidden)]
    waypoint_sec: f32,

    #[reflect(hidden)]
    current_waypoint: Option<Vector3<f32>>,

    #[reflect(hidden)]
    collider: Handle<Node>,

    bullet_prefab: Option<Resource<Model>>,
    initial_bullet_velocity: f32,
    attack_range: f32,

    beacon_reached_distance: f32,
    move_power: f32,
}

impl Guard {
    fn try_attack_player(&mut self, ctx: &mut LiteContext) -> bool {
        let player_pos = ctx.with_plugin::<Game, _>(|it| LiteNode::from(it.player)).global_position();
        let self_pos = ctx.node.global_position();
        let sight_vector = player_pos.sub(self_pos);
        println!(
            "try attack player. player_pos: {:?}, self_pos: {:?}",
            player_pos, self_pos
        );

        if self.can_see_player(ctx, player_pos, sight_vector) {
            Bullet::spawn(
                BulletSeed {
                    prefab: self.bullet_prefab.as_ref().unwrap().clone(),
                    origin: self_pos + Vector3::new(0.0, self.gun_height, 0.0),
                    direction: sight_vector,
                    initial_velocity: self.initial_bullet_velocity,
                    author_collider: self.collider,
                    range: self.attack_range,
                },
            );
            self.reloading_sec = self.reload_delay_sec;
            return true;
        }

        false
    }

    fn can_see_player(
        &self,
        ctx: &LiteContext,
        player_pos: Vector3<f32>,
        sight_vector: Vector3<f32>,
    ) -> bool {
        let opts = RayCastOptions {
            ray_origin: Point3::from(player_pos),
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
                if node.has_script::<Player>() {
                    return true;
                }
                handle = node.parent();
            }
            return false;
        }
        false
    }

    fn move_to_waypoint(&mut self, ctx: &mut LiteContext) {
        self.waypoint_sec += ctx.dt;
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
        let vector_to_beacon = self.current_waypoint.unwrap().sub(pos);
        if vector_to_beacon.magnitude() < self.beacon_reached_distance {
            self.current_waypoint = None;
        } else {
            let force = vector_to_beacon.normalize() * self.move_power;
            ctx.node.with_rigid_body(|it| {
                it.apply_force(force);
            });
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
            self.reloading_sec -= ctx.dt;
        }

        if self.reloading_sec > 0.0 || !self.try_attack_player(ctx) {
            self.move_to_waypoint(ctx);
        }
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox_lite_api::lite_node::LiteMessageContext,
    ) {
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
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        self.redispatch_message(message, ctx);
    }
}
