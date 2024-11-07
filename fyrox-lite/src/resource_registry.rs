//!
//! This module is created, because there is no centralized pool of resources in Fyrox.
//! 

use std::{cell::RefCell, collections::HashMap};

use fyrox::{
    asset::{
        untyped::{ResourceHeader, UntypedResource},
        Resource,
    },
    core::pool::{Handle, Pool},
    resource::model::Model,
};

// TODO replace with SendWrapper
thread_local! {
    static RESOURCES: RefCell<Option<ResourceRegistry>> = Default::default();
}

#[derive(Debug, Default)]
struct ResourceRegistry {
    pub resources: Pool<UntypedResource>,
    pub reverse_resources: HashMap<*mut ResourceHeader, Handle<UntypedResource>>,
}

pub(crate) fn retrieve_externalized(handle: u128) -> UntypedResource {
    println!("DEBUG: retrieve_externalized {}", handle);
    RESOURCES.with_borrow(|it| {
        let handle = Handle::decode_from_u128(handle);
        it.as_ref()
            .unwrap()
            .resources
            .try_borrow(handle)
            .unwrap()
            .clone()
    })
}

pub(crate) fn externalize_resource(resource: UntypedResource) -> u128 {
    RESOURCES.with_borrow_mut(|registry| {
        if registry.is_none() {
            *registry = Some(Default::default());
        }
        let registry = registry.as_mut().unwrap();
        let existing_handle = registry.reverse_resources.get(&resource.0.data_ptr());
        let handle = existing_handle
            .copied()
            .unwrap_or_else(|| registry.resources.spawn(resource.clone()));
        let id = handle.encode_to_u128();
        println!("DEBUG: externalize_resource {:?} with id {}", resource, id);
        id
    })
}
