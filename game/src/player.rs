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
use fyrox_lite_api::lite_ctx::{LiteContext, LiteScript};
use fyrox_lite_api::lite_node::LiteNode;
use fyrox_lite_api::lite_window::LiteWindow;
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
    fn turn(&self, x: f32, ctx: &mut LiteContext) {
        let rot_delta = Rotation3::from_axis_angle(&Vector3::y_axis(), self.sensitivity * x);
        ctx.node
            .set_local_rotation(ctx.node.local_rotation().mul(&rot_delta));
    }

    fn aim(&mut self, y: f32) {
        self.temp.aim_y += y * self.sensitivity;

        self.temp.aim_y = self.temp.aim_y.clamp(-PI / 2.0, PI / 2.0);

        LiteNode::from(self.camera).set_local_rotation(UnitQuaternion::from_axis_angle(
            &Vector3::x_axis(),
            self.temp.aim_y,
        ));
    }

    fn fire(&mut self) {
        let camera_pos = LiteNode::from(self.camera).global_position();
        let bullet_orientation = LiteNode::from(self.camera).global_rotation();

        let prefab = self.bullet.as_ref().unwrap().clone();
        Bullet::spawn(BulletSeed {
            prefab,
            origin: camera_pos,
            direction: bullet_orientation.transform_vector(&Vector3::z_axis()),
            initial_velocity: self.initial_bullet_velocity,
            author_collider: self.collider,
            range: self.shooting_range,
        });
        println!("bullet spawned");
    }
}

impl LiteScript for Player {
    fn on_init(&mut self, ctx: &mut LiteContext) {
        let _ = LiteWindow::set_cursor_grab(fyrox::window::CursorGrabMode::Confined);

        self.collider = ctx
            .node
            .try_get_collider()
            .expect("Collider not found under Player node")
            .into();
    }

    fn on_start(&mut self, ctx: &mut LiteContext) {
        ctx.node.subscribe_to::<BulletHit>();
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox_lite_api::lite_node::LiteMessageContext,
    ) {
        if let Some(_bullet) = message.downcast_ref::<BulletHit>() {
            ctx.with_plugin::<Game, _>(|it| it.wounds += 1);
            println!("player wounded!");
        }
    }

    fn on_update(&mut self, ctx: &mut LiteContext) {
        profiling::scope!("Player::on_update");
        if self.reload_sec > 0.0 {
            self.reload_sec -= ctx.dt;
        }
        if !self.published {
            self.published = true;
            let player = ctx.node;
            ctx.with_plugin::<Game, _>(|it| it.player = player.into());
        }

        if self.temp.fire {
            if self.reload_sec <= 0.0 {
                self.reload_sec = self.reload_delay_sec;
                self.fire();
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

        let self_rotation = ctx.node.local_rotation().clone();
        let move_delta = self_rotation.transform_vector(&move_delta);
        let force = move_delta * self.power;
        ctx.node.with_rigid_body(|it| it.apply_force(force));
    }

    fn on_os_event(&mut self, event: &Event<()>, ctx: &mut LiteContext) {
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
                self.aim(*y as f32);
            }
        }
    }
}

impl ScriptTrait for Player {
    fn on_init(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
        self.redispatch_init(ctx);
    }

    fn on_start(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
        self.redispatch_start(ctx);
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        self.redispatch_message(message, ctx);
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_update(ctx);
    }

    fn on_os_event(&mut self, event: &Event<()>, ctx: &mut ScriptContext) {
        self.redispatch_os_event(event, ctx);
    }
}
