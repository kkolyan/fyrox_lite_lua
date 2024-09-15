//! Game project.
use fyrox::core::ComponentProvider;
use fyrox::{
    core::{reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*, TypeUuidProvider},
    script::{ScriptContext, ScriptTrait},
};
use fyrox_lite_api::lite_ctx::{LiteContext, LiteScript};

use crate::game::Game;

#[derive(Visit, Reflect, Debug, Clone, TypeUuidProvider, ComponentProvider, Default)]
#[type_uuid(id = "7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
#[visit(optional)]
pub struct Beacon {}

impl LiteScript for Beacon {
    fn on_update(&mut self, ctx: &mut LiteContext) {
        let pos = ctx.node.global_position();
        ctx.with_plugin::<Game, _>(|it| it.beacons.push(pos.into()));
        ctx.node.destroy();
        println!("beacon registered: {:?}", ctx.node);
    }
}

impl ScriptTrait for Beacon {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_update(ctx);
    }
}
