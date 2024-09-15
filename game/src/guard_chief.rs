//! Game project.
use fyrox::asset::Resource;
use fyrox::core::log::Log;
use fyrox::core::ComponentProvider;
use fyrox::rand::random;
use fyrox::resource::model::Model;
use fyrox::{
    core::{
        reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*,
        TypeUuidProvider,
    },
    script::{ScriptContext, ScriptTrait},
};
use fyrox_lite_api::lite_ctx::LiteScript;
use fyrox_lite_api::lite_math::{LiteQuaternion, LiteVector3};
use fyrox_lite_api::lite_prefab::LitePrefab;

use crate::game::Game;

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

impl LiteScript for GuardChief {
    fn on_update(&mut self, ctx: &mut fyrox_lite_api::lite_ctx::LiteContext) {
        if !self.frame_skipped_for_beacons {
            self.frame_skipped_for_beacons = true;
            return;
        }
        if !self.initialized {
            self.initialized = true;
            for _ in 0..self.initial_count {
                let beacons = ctx.with_plugin::<Game, _>(|it| it.beacons.clone());
                if !beacons.is_empty() {
                    let position = LiteVector3::from(beacons[random::<usize>() % beacons.len()]);

                    let orientation = LiteQuaternion::from_axis_angle(LiteVector3::y_axis(), random());

                    LitePrefab::from(self.gaurd_prefab.as_ref().unwrap().clone())
                        .instantiate_at(position, orientation);
                    Log::info(format!("guard spawned at {:?}", position));
                } else {
                    Log::err("cannot spawn guards: no beacons found");
                }
            }
        }
    }
}

impl ScriptTrait for GuardChief {
    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.redispatch_update(ctx);
    }
}
