//! Game project.
use fyrox::core::algebra::{Rotation3, Unit, UnitQuaternion};
use fyrox::core::log::Log;
use fyrox::core::num_traits::Zero;
use fyrox::core::ComponentProvider;
use fyrox::event::{DeviceEvent, KeyEvent};
use fyrox::graph::SceneGraph;
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
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "12371d19-9f1a-4286-8486-add4ebaadaec")]
#[visit(optional)]
pub struct Bullet {
    #[reflect(hidden)]
    #[visit(skip)]
	id: Option<i64>
}

static NEXT_ID: AtomicI64 = AtomicI64::new(1);

impl ScriptTrait for Bullet {

	fn on_update(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
		if self.id.is_none() {
			self.id =Some( NEXT_ID.fetch_add(1, Ordering::Relaxed));
			println!("bullet {:003} created", self.id.unwrap());
		}
	}
}