//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::{Rotation3, Unit, UnitQuaternion};
use fyrox::core::futures::executor::block_on;
use fyrox::core::log::Log;
use fyrox::core::num_traits::Zero;
use fyrox::core::ComponentProvider;
use fyrox::event::{DeviceEvent, KeyEvent, MouseButton};
use fyrox::graph::SceneGraph;
use fyrox::resource::model::{Model, ModelResourceExtension};
use fyrox::scene::rigidbody::RigidBody;
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
use std::ops::Mul;
use std::path::Path;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "c5671d19-9f1a-4286-8486-add4ebaadaec")]
#[visit(optional)]
pub struct Player {
    pub sensitivity: f32,
    pub camera: Handle<Node>,
    pub power: f32,
    pub bullet: Option<Resource<Model>>,

    #[reflect(hidden)]
    #[visit(skip)]
    temp: TempState,
}

#[derive(Debug, Default, Clone)]
pub struct TempState {
    pub aim_y: f32,
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
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
}

impl ScriptTrait for Player {
    fn on_init(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
        let _ = ctx.graphics_context.as_initialized_mut().window
            .set_cursor_grab(fyrox::window::CursorGrabMode::Confined);
    }

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        let self_rotation = ctx.scene.graph[ctx.handle]
            .local_transform()
            .rotation()
            .clone();

        let rb = ctx.scene.graph[ctx.handle].cast_mut::<RigidBody>().unwrap();
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


        let move_delta = self_rotation.transform_vector(&move_delta);
        let force = move_delta * self.power * ctx.dt;
        rb.apply_force(force);
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
                if *state == ElementState::Pressed && *button == MouseButton::Left {
                    let camera_rot = ctx.scene.graph[self.camera].global_transform();
                    let camera_pos = ctx.scene.graph[self.camera].global_position();

                    let bullet = self.bullet.as_ref().unwrap().instantiate(ctx.scene);
                    let bullet = &mut ctx.scene.graph[bullet];
                    bullet.local_transform_mut().set_position(camera_pos);
                    bullet.cast_mut::<RigidBody>().unwrap()
                        .set_lin_vel(camera_rot.transform_vector(&Vector3::z_axis()));
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
