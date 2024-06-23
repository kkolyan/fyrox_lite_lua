use std::{fmt::Debug, ops::{Deref, DerefMut}};

use fyrox::{asset::Resource, core::blank_reflect};
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
use std::any::Any;

/// syntax sugar for `#[visit(skip)]` and `#[reflect(hidden)]`
#[derive(Debug, Default, Clone, Copy)]
pub struct Transient<T>(T);

impl <T> From<T> for Transient<T> {
	fn from(value: T) -> Self {
		Self(value)
	}
}

impl<T> Deref for Transient<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Transient<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <T> Visit for Transient<T> {
	fn visit(&mut self, _name: &str, _visitor: &mut fyrox::core::visitor::Visitor) -> fyrox::core::visitor::VisitResult {
		Ok(())
	}
}

impl <T: Debug + 'static> Reflect for Transient<T> {
	blank_reflect!();
}
