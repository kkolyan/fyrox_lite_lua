//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::{Point3, Rotation3, Unit, UnitQuaternion, UnitVector3};
use fyrox::core::log::Log;
use fyrox::core::math::Matrix4Ext;
use fyrox::core::num_traits::Zero;
use fyrox::core::ComponentProvider;
use fyrox::event::{DeviceEvent, KeyEvent};
use fyrox::graph::BaseSceneGraph;
use fyrox::graph::SceneGraph;
use fyrox::resource::model::{Model, ModelResourceExtension};
use fyrox::scene::graph::physics::RayCastOptions;
use fyrox::scene::rigidbody::RigidBody;
use fyrox::script::RoutingStrategy;
use fyrox::{
    core::{
        algebra::{Vector2, Vector3},
        pool::Handle,
        reflect::prelude::*,
        type_traits::prelude::*,
        visitor::prelude::*,
        TypeUuidProvider,
    },
    engine::GraphicsContext,
    event::{ElementState, Event, WindowEvent},
    gui::{
        button::ButtonMessage,
        message::{MessageDirection, UiMessage},
        text::TextMessage,
        widget::WidgetMessage,
        UiNode, UserInterface,
    },
    keyboard::{KeyCode, PhysicalKey},
    plugin::{Plugin, PluginContext, PluginRegistrationContext},
    scene::{animation::spritesheet::SpriteSheetAnimation, node::Node, Scene},
    script::{ScriptContext, ScriptTrait},
};
use std::f32::consts::PI;
use std::ops::{Add, Mul};
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};

use crate::fyrox_utils::print_ancestors;
use crate::guard::Guard;
use crate::transient::Transient;

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
    pub fn spawn(scene: &mut Scene, seed: BulletSeed) {
        let orientation = UnitQuaternion::face_towards(&seed.direction, &Vector3::y_axis());
        let bullet = seed.prefab.instantiate_at(
            scene,
            seed.origin,
            orientation,
        );
        let bullet = &mut scene.graph[bullet];
        let bullet_script = bullet.try_get_script_mut::<Bullet>().unwrap();
        bullet_script.params = BulletParams {
            velocity: seed.direction.normalize()
                * seed.initial_velocity,
            remaining_sec: seed.range / seed.initial_velocity,
            author_collider: seed.author_collider,
        };
    }
}

#[derive(Debug)]
pub struct BulletHit {}

impl ScriptTrait for Bullet {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        profiling::scope!("Bullet::on_update");
        self.params.remaining_sec -= ctx.dt;
        if self.params.remaining_sec <= 0.0 {
            ctx.scene.graph.remove_node(ctx.handle);
            return;
        }
        let t = ctx.scene.graph[ctx.handle].local_transform_mut();
        let new_pos = t.position().add(self.params.velocity * ctx.dt);

        let opts = RayCastOptions {
            ray_origin: Point3::from(*t.position().get_value_ref()),
            ray_direction: self.params.velocity.normalize(),
            max_len: self.params.velocity.magnitude() * ctx.dt,
            groups: Default::default(),
            sort_results: true,
        };
        let mut results = vec![];
        {
            profiling::scope!("cast_ray");
            ctx.scene.graph.physics.cast_ray(opts, &mut results);
        }

        for hit in results {
            if hit.collider == self.params.author_collider {
                continue;
            }
            ctx.message_sender
                .send_hierarchical(hit.collider, RoutingStrategy::Up, BulletHit {});
            ctx.scene.graph.remove_node(ctx.handle);
            return;
        }

        ctx.scene.graph[ctx.handle]
            .local_transform_mut()
            .set_position(new_pos);
    }
}
