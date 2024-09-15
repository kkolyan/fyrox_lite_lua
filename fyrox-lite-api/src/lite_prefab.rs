use fyrox::{
    asset::Resource,
    core::algebra::{UnitQuaternion, Vector3},
    resource::model::{Model, ModelResourceExtension},
};

use crate::{lite_node::LiteNode, script_context::with_script_context};

pub struct LitePrefab {
    resource: Resource<Model>,
}

impl From<Resource<Model>> for LitePrefab {
    fn from(value: Resource<Model>) -> Self {
        Self { resource: value }
    }
}

impl LitePrefab {
    pub fn instantiate_at(
        &self,
        position: Vector3<f32>,
        orientation: UnitQuaternion<f32>,
    ) -> LiteNode {
        with_script_context(|ctx| {
            self.resource
                .begin_instantiation(ctx.scene)
                .with_rotation(orientation)
                .with_position(position)
                .finish()
                .into()
        })
    }
}
