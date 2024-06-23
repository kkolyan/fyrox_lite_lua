//! Game project.
use fyrox::asset::Resource;
use fyrox::core::algebra::UnitQuaternion;
use fyrox::core::log::Log;
use fyrox::core::ComponentProvider;
use fyrox::rand::random;
use fyrox::resource::model::{Model, ModelResourceExtension};
use fyrox::{
    core::{
        algebra::Vector3, reflect::prelude::*, type_traits::prelude::*, visitor::prelude::*,
        TypeUuidProvider,
    },
    script::{ScriptContext, ScriptTrait},
};

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
