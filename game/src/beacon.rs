//! Game project.
use fyrox::core::algebra::{Rotation3, Unit, UnitQuaternion};
use fyrox::core::log::Log;
use fyrox::core::math::Matrix4Ext;
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
use std::ops::{Add, Deref, Mul, Not};
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};
use fyrox::graph::BaseSceneGraph;

use crate::game::Game;
use crate::transient::Transient;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
#[visit(optional)]
pub struct Beacon {
}

impl ScriptTrait for Beacon {

	fn on_update(&mut self, ctx: &mut ScriptContext) {
		let pos = ctx.scene.graph[ctx.handle].global_position();
		ctx.plugins.get_mut::<Game>().beacons.push(pos);
		ctx.scene.graph.remove_node(ctx.handle);
		println!("beacon registered: {:?}", ctx.handle);
	}
}