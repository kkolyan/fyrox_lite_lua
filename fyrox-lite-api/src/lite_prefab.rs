use fyrox::{
    asset::Resource,
    resource::model::{Model, ModelResourceExtension},
};

use crate::{
    lite_math::{LiteQuaternion, LiteVector3},
    lite_node::LiteNode,
    script_context::with_script_context,
};

#[derive(Debug, Clone, Default)]
pub struct LitePrefab {
    resource: Resource<Model>,
}

impl LitePrefab {
    pub fn new(resource: Resource<Model>) -> Self {
        Self { resource }
    }

    pub fn inner(&self) -> Resource<Model> {
        self.resource.clone()
    } 
}

impl LitePrefab {
    pub fn instantiate_at(&self, position: LiteVector3, orientation: LiteQuaternion) -> LiteNode {
        with_script_context(|ctx| {
            self.resource
                .begin_instantiation(ctx.scene.as_mut().expect("scene unavailable"))
                .with_rotation(orientation.into())
                .with_position(position.into())
                .finish()
                .into()
        })
    }
}
