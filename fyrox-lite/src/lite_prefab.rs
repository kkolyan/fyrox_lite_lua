use std::fmt::Debug;

use fyrox::{
    asset::Resource,
    resource::model::{Model, ModelResourceExtension},
};
use lite_macro::lite_api;

use crate::{
    lite_math::{PodQuaternion, PodVector3},
    lite_node::LiteNode,
    script_context::with_script_context,
};

#[derive(Clone, Default)]
pub struct LitePrefab {
    resource: Resource<Model>,
}

impl Debug for LitePrefab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.resource.header().kind.path() {
            Some(it) => write!(f, "Prefab({:?})", it),
            None => write!(f, "Prefab()"),
        }
    }
}

impl LitePrefab {
    pub fn new(resource: Resource<Model>) -> Self {
        Self { resource }
    }

    pub fn inner(&self) -> Resource<Model> {
        self.resource.clone()
    }
}

#[lite_api(class=Prefab)]
impl LitePrefab {
    pub fn instantiate_at(&self, position: PodVector3, orientation: PodQuaternion) -> LiteNode {
        with_script_context(|ctx| {
            LiteNode::new(
                self.resource
                    .begin_instantiation(ctx.scene.as_mut().expect("scene unavailable"))
                    .with_rotation(orientation.into())
                    .with_position(position.into())
                    .finish(),
            )
        })
    }
}
