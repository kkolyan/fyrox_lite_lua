use fyrox::{
    asset::Resource,
    core::algebra::{UnitQuaternion, Vector3},
    resource::model::{Model, ModelResourceExtension},
};

use crate::{lite_math::{LiteQuaternion, LiteVector3}, lite_node::LiteNode, script_context::with_script_context};

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
        position: LiteVector3,
        orientation: LiteQuaternion,
    ) -> LiteNode {
        with_script_context(|ctx| {
            self.resource
                .begin_instantiation(ctx.scene)
                .with_rotation(orientation.into())
                .with_position(position.into())
                .finish()
                .into()
        })
    }
}
