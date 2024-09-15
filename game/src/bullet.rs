//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::{Point3, UnitQuaternion};
use fyrox::core::ComponentProvider;
use fyrox::graph::BaseSceneGraph;
use fyrox::resource::model::{Model, ModelResourceExtension};
use fyrox::scene::graph::physics::RayCastOptions;
use fyrox::script::RoutingStrategy;
use fyrox::{
    core::{
        algebra::Vector3, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*, TypeUuidProvider,
    },
    scene::{node::Node, Scene},
    script::{ScriptContext, ScriptTrait},
};
use fyrox_lite_api::lite_ctx::{LiteContext, LiteScript};
use fyrox_lite_api::lite_node::LiteNode;
use fyrox_lite_api::lite_physics::LitePhysics;
use fyrox_lite_api::lite_prefab::LitePrefab;
use std::ops::Add;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "12371d19-9f1a-4286-8486-add4ebaadaec")]
#[visit(optional)]
pub struct Bullet {
    pub params: BulletParams,
}

#[derive(Visit, Reflect, Debug, Clone, Default)]
pub struct BulletParams {
    pub velocity: Vector3<f32>,
    pub remaining_sec: f32,
    pub author_collider: Handle<Node>,
}

pub struct BulletSeed {
    pub prefab: Resource<Model>,
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub initial_velocity: f32,
    pub author_collider: Handle<Node>,
    pub range: f32,
}

impl Bullet {
    pub fn spawn(seed: BulletSeed) {
        let orientation = UnitQuaternion::face_towards(&seed.direction, &Vector3::y_axis());
        let bullet = LitePrefab::from(seed.prefab).instantiate_at( seed.origin, orientation);
        bullet.with_script::<Bullet>(|it| {
            it.params = BulletParams {
                velocity: seed.direction.normalize() * seed.initial_velocity,
                remaining_sec: seed.range / seed.initial_velocity,
                author_collider: seed.author_collider,
            };
        });
    }
}

#[derive(Debug)]
pub struct BulletHit {}

impl LiteScript for Bullet {
    fn on_update(&mut self, ctx: &mut LiteContext) {
        profiling::scope!("Bullet::on_update");
        self.params.remaining_sec -= ctx.dt;
        if self.params.remaining_sec <= 0.0 {
            ctx.node.destroy();
            return;
        }
        let new_pos = ctx.node.local_position().add(self.params.velocity * ctx.dt);

        let opts = RayCastOptions {
            ray_origin: Point3::from(ctx.node.local_position()),
            ray_direction: self.params.velocity.normalize(),
            max_len: self.params.velocity.magnitude() * ctx.dt,
            groups: Default::default(),
            sort_results: true,
        };
        let mut results = vec![];
        {
            profiling::scope!("cast_ray");
            LitePhysics::cast_ray(opts, &mut results);
        }

        for hit in results {
            if hit.collider == self.params.author_collider.into() {
                continue;
            }
            hit.collider.send_hierarchical(RoutingStrategy::Up, BulletHit {});
            ctx.node.destroy();
            return;
        }

        ctx.node.set_local_position(new_pos);
    }
}

impl ScriptTrait for Bullet {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_update(ctx);
    }
}
