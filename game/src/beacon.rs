//! Game project.
use fyrox::core::ComponentProvider;
use fyrox::graph::BaseSceneGraph;
use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*, TypeUuidProvider},
    script::{ScriptContext, ScriptTrait},
};

use crate::game::Game;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
#[visit(optional)]
pub struct Beacon {}

impl ScriptTrait for Beacon {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        let pos = ctx.scene.graph[ctx.handle].global_position();
        ctx.plugins.get_mut::<Game>().beacons.push(pos);
        ctx.scene.graph.remove_node(ctx.handle);
        println!("beacon registered: {:?}", ctx.handle);
    }
}
