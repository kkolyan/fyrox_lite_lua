//! Game project.
use fyrox::core::color::Color;
use fyrox::graph::SceneGraph;
use fyrox::gui::text::{Text, TextBuilder};
use fyrox::gui::widget::WidgetBuilder;
pub use fyrox::plugin::Plugin;
use fyrox::{core::ComponentProvider, gui::brush::Brush};
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
    plugin::{PluginContext, PluginRegistrationContext},
    scene::{
        animation::spritesheet::SpriteSheetAnimation,
        dim2::{rectangle::Rectangle, rigidbody::RigidBody},
        node::Node,
        Scene,
    },
    script::{ScriptContext, ScriptTrait},
};
use std::path::Path;

use crate::beacon::Beacon;
use crate::bullet::Bullet;
use crate::guard::Guard;
use crate::guard_chief::GuardChief;
use crate::player::Player;
use crate::transient::Transient;

#[derive(Visit, Reflect, Debug, Default)]
pub struct Game {
    pub player: Handle<Node>,
    pub beacons: Vec<Vector3<f32>>,
    pub frags: usize,
    pub wounds: usize,
    hud: Handle<UiNode>,
}
impl Plugin for Game {
    fn register(&self, ctx: PluginRegistrationContext) {
        register_script::<Player>(&ctx, "Player");
        register_script::<Bullet>(&ctx, "Bullet");
        register_script::<Guard>(&ctx, "Guard");
        register_script::<GuardChief>(&ctx, "GuardChief");
        register_script::<Beacon>(&ctx, "Beacon");
    }

    fn init(&mut self, scene_path: Option<&str>, context: PluginContext) {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        self.hud =
            TextBuilder::new(WidgetBuilder::new().with_foreground(Brush::Solid(Color::BLACK)))
                .with_font_size(40.0)
                .build(&mut context.user_interfaces.first_mut().build_ctx());
    }

    fn update(&mut self, #[allow(unused_variables)] context: &mut PluginContext) {
        context
            .user_interfaces
            .first_mut()
            .send_message(TextMessage::text(
                self.hud,
                MessageDirection::ToWidget,
                format!("Wounds: {}\nKilled Guards: {}", self.wounds, self.frags),
            ));
        profiling::finish_frame!();
    }
}

fn register_script<T: TypeUuidProvider + ScriptTrait + Default>(
    ctx: &PluginRegistrationContext,
    name: &str,
) {
    ctx.serialization_context.script_constructors.add::<T>(name);
}
