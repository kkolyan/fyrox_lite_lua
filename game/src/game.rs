//! Game project.
use fyrox::core::ComponentProvider;
use fyrox::graph::SceneGraph;
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
    plugin::{ PluginContext, PluginRegistrationContext},
    scene::{
        animation::spritesheet::SpriteSheetAnimation,
        dim2::{rectangle::Rectangle, rigidbody::RigidBody},
        node::Node,
        Scene,
    },
    script::{ScriptContext, ScriptTrait},
};
use std::path::Path;
pub use fyrox::plugin::Plugin;

use crate::bullet::Bullet;
use crate::player::Player;

#[derive(Visit, Reflect, Debug, Default)]
pub struct Game {
    scene: Handle<Scene>,
    debug_text: Handle<UiNode>,
    new_game: Handle<UiNode>,
    exit: Handle<UiNode>,
}
impl Plugin for Game {
    fn register(&self, ctx: PluginRegistrationContext) {
        ctx.serialization_context
            .script_constructors
            .add::<Player>("Player");
        ctx.serialization_context
            .script_constructors
            .add::<Bullet>("Bullet");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
    }
}
