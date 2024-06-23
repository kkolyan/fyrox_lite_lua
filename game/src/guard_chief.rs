//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::{Rotation3, Unit, UnitQuaternion};
use fyrox::core::log::Log;
use fyrox::core::math::Matrix4Ext;
use fyrox::core::num_traits::Zero;
use fyrox::core::ComponentProvider;
use fyrox::event::{DeviceEvent, KeyEvent};
use fyrox::graph::BaseSceneGraph;
use fyrox::graph::SceneGraph;
use fyrox::rand::random;
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
use std::ops::{Add, Deref, Mul, Not, Sub};
use std::path::Path;
use std::sync::atomic::{AtomicI64, Ordering};

use crate::game::Game;
use crate::transient::Transient;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "c69ae5fa-de26-4ee5-b70c-113df285f6e2")]
#[visit(optional)]
pub struct GuardChief {
    gaurd_prefab: Option<Resource<Model>>,
    initial_count: usize,

    #[reflect(hidden)]
    initialized: bool,

    #[reflect(hidden)]
    frame_skipped_for_beacons: bool,
}

impl GuardChief {}

impl ScriptTrait for GuardChief {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        if !self.frame_skipped_for_beacons {
            self.frame_skipped_for_beacons = true;
			return;
        }
        if !self.initialized {
            self.initialized = true;
            for _ in 0..self.initial_count {
                let beacons = &ctx.plugins.get::<Game>().beacons;
                if !beacons.is_empty() {
                    let position = beacons[random::<usize>() % beacons.len()];

                    let orientation = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), random());

                    self.gaurd_prefab.as_ref().unwrap().instantiate_at(
                        ctx.scene,
                        position,
                        orientation,
                    );
                    Log::info(format!("guard spawned at {:?}", position));
                } else {
                    Log::err("cannot spawn guards: no beacons found");
                }
            }
        }
    }
}
