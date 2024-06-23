//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::{Rotation3, UnitQuaternion};
use fyrox::core::num_traits::Zero;
use fyrox::core::ComponentProvider;
use fyrox::event::{DeviceEvent, MouseButton};
use fyrox::resource::model::Model;
use fyrox::{
    core::{
        algebra::Vector3, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*, TypeUuidProvider,
    },
    event::{ElementState, Event, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    scene::node::Node,
    script::{ScriptContext, ScriptTrait},
};
use std::f32::consts::PI;
use std::ops::Mul;

use crate::bullet::{Bullet, BulletHit, BulletSeed};
use crate::fyrox_utils::HandleNodeExt;
use crate::game::Game;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "c5671d19-9f1a-4286-8486-add4ebaadaec")]
#[visit(optional)]
pub struct Player {
    sensitivity: f32,
    camera: Handle<Node>,
    power: f32,
    bullet: Option<Resource<Model>>,
    initial_bullet_velocity: f32,
    shooting_range: f32,
    reload_delay_sec: f32,

    #[reflect(hidden)]
    reload_sec: f32,

    #[reflect(hidden)]
    published: bool,

    #[reflect(hidden)]
    collider: Handle<Node>,

    #[visit(skip)]
    #[reflect(hidden)]
    temp: TempState,
}

#[derive(Debug, Default, Clone)]
pub struct TempState {
    pub aim_y: f32,
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub fire: bool,
}

impl Player {
    fn turn(&self, x: f32, ctx: &mut ScriptContext) {
        let self_transform = ctx.scene.graph[ctx.handle].local_transform_mut();
        let rot_delta = Rotation3::from_axis_angle(&Vector3::y_axis(), self.sensitivity * x);
        self_transform.set_rotation(self_transform.rotation().mul(&rot_delta));
    }

    fn aim(&mut self, y: f32, ctx: &mut ScriptContext) {
        self.temp.aim_y += y * self.sensitivity;

        self.temp.aim_y = self.temp.aim_y.clamp(-PI / 2.0, PI / 2.0);

        let camera_transform = ctx.scene.graph[self.camera].local_transform_mut();
        camera_transform.set_rotation(UnitQuaternion::from_axis_angle(
            &Vector3::x_axis(),
            self.temp.aim_y,
        ));
    }

    fn fire(&mut self, ctx: &mut ScriptContext) {
        let camera_global_transform = ctx.scene.graph[self.camera].global_transform();
        let camera_pos = ctx.scene.graph[self.camera].global_position();

        let rot = camera_global_transform.fixed_view::<3, 3>(0, 0);
        let bullet_orientation = UnitQuaternion::from_matrix(&rot.into());

        let prefab = self.bullet.as_ref().unwrap().clone();
        Bullet::spawn(
            ctx.scene,
            BulletSeed {
                prefab,
                origin: camera_pos,
                direction: bullet_orientation.transform_vector(&Vector3::z_axis()),
                initial_velocity: self.initial_bullet_velocity,
                author_collider: self.collider,
                range: self.shooting_range,
            },
        );
        println!("bullet spawned");
    }
}

impl ScriptTrait for Player {
    fn on_init(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
        let _ = ctx
            .graphics_context
            .as_initialized_mut()
            .window
            .set_cursor_grab(fyrox::window::CursorGrabMode::Confined);

        self.collider = ctx
            .handle
            .try_get_collider(ctx.scene)
            .expect("Collider not found under Player node");
    }

    fn on_start(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
        ctx.message_dispatcher.subscribe_to::<BulletHit>(ctx.handle);
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(_bullet) = message.downcast_ref::<BulletHit>() {
            ctx.plugins.get_mut::<Game>().wounds += 1;
            println!("player wounded!");
        }
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        profiling::scope!("Player::on_update");
        if self.reload_sec > 0.0 {
            self.reload_sec -= ctx.dt;
        }
        if !self.published {
            self.published = true;
            ctx.plugins.get_mut::<Game>().player = ctx.handle;
        }

        if self.temp.fire {
            if self.reload_sec <= 0.0 {
                self.reload_sec = self.reload_delay_sec;
                self.fire(ctx);
            }
        }

        let mut move_delta = Vector3::<f32>::zero();
        if self.temp.forward {
            move_delta.z += 1.0
        }
        if self.temp.back {
            move_delta.z -= 1.0
        }
        if self.temp.left {
            move_delta.x += 1.0
        }
        if self.temp.right {
            move_delta.x -= 1.0
        }

        if move_delta.magnitude() > 0.001 {
            move_delta.normalize_mut();
        }

        let self_rotation = ctx.scene.graph[ctx.handle]
            .local_transform()
            .rotation()
            .clone();
        let move_delta = self_rotation.transform_vector(&move_delta);
        let force = move_delta * self.power;
        ctx.scene.graph[ctx.handle]
            .as_rigid_body_mut()
            .apply_force(force);
    }

    fn on_os_event(&mut self, event: &Event<()>, ctx: &mut ScriptContext) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } = event
            {
                let value = match event.state {
                    ElementState::Pressed => true,
                    ElementState::Released => false,
                };
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => self.temp.forward = value,
                    PhysicalKey::Code(KeyCode::KeyS) => self.temp.back = value,
                    PhysicalKey::Code(KeyCode::KeyA) => self.temp.left = value,
                    PhysicalKey::Code(KeyCode::KeyD) => self.temp.right = value,
                    _ => (),
                }
            }
            if let WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } = event
            {
                if *button == MouseButton::Left {
                    self.temp.fire = match state {
                        ElementState::Pressed => true,
                        ElementState::Released => false,
                    };
                }
            }
        }
        if let Event::DeviceEvent {
            device_id: _,
            event,
        } = event
        {
            if let DeviceEvent::MouseMotion { delta: (x, y) } = event {
                self.turn(-*x as f32, ctx);
                self.aim(*y as f32, ctx);
            }
        }
    }
}
