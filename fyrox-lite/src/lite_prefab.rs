use std::fmt::Debug;

use fyrox::{
    asset::Resource,
    resource::model::{Model, ModelResourceExtension},
};
use fyrox::core::reflect::Reflect;
use lite_macro::lite_api;

use crate::{
    externalizable::Externalizable,
    lite_math::{PodQuaternion, PodVector3},
    lite_node::LiteNode,
    resource_registry,
    script_context::with_script_context,
};
use crate::spi::UserScript;

#[derive(Clone, Default)]
pub struct LitePrefab {
    resource: Option<Resource<Model>>,
}

impl Debug for LitePrefab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(resource) = &self.resource else {
            return write!(f, "None")
        };
        match resource.header().kind.path() {
            Some(it) => write!(f, "Prefab({:?})", it),
            None => write!(f, "Prefab()"),
        }
    }
}

impl LitePrefab {
    pub fn new(resource: Resource<Model>) -> Self {
        Self { resource: Some(resource) }
    }

    pub fn inner(&self) -> Option<Resource<Model>> {
        self.resource.clone()
    }
}

#[lite_api(class=Prefab)]
impl LitePrefab {
    pub fn instantiate_at<T: UserScript>(&self, position: PodVector3, orientation: PodQuaternion, _stub: T::UserScriptGenericStub) -> Result<LiteNode, T::LangSpecificError> {
        with_script_context(|ctx| {
            let handle = self.resource
                .as_ref()
                .ok_or_else(|| T::create_error("this prefab is null reference"))?
                .begin_instantiation(ctx.scene.as_mut().expect("scene unavailable"))
                .with_rotation(orientation.into())
                .with_position(position.into())
                .finish();
            let node = &mut ctx.scene.as_mut().unwrap().graph[handle];
            println!("loaded: ({:?}) [{}]", node, node.scripts().map(|it| it.type_name().to_string()).collect::<Vec<_>>().join(", "));
            Ok(LiteNode::new(
                handle,
            ))
        })
    }
}

impl Externalizable for LitePrefab {
    fn to_external(&self) -> u128 {
        let it = self.resource.as_ref();
        let Some(it) = it else {
            return 0;
        };
        resource_registry::externalize_resource(it.clone().into_untyped())
    }

    fn from_external(handle: u128) -> Self {
        if handle == 0 {
            return LitePrefab::default();
        }
        LitePrefab {
            resource: Some(resource_registry::retrieve_externalized(handle).into()),
        }
    }
}
