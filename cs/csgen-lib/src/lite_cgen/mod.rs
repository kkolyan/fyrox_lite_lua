use std::collections::HashSet;

use engine_class::generate_engine_class;
use enum_class::generate_enum;
use lite_model::{Class, Domain};
use struct_class::generate_struct;

pub mod engine_class;
pub mod enum_class;
pub mod struct_class;
pub mod types;

pub fn generate_c_bindings_lite(domain: &Domain) -> String {
    let mut client_replicated_types: HashSet<lite_model::ClassName> = Default::default();
    for class in domain.classes.iter() {
        if let Class::Struct(class) = class {
            client_replicated_types.insert(class.class_name.clone());
        }
    }
    let mut s = String::new();
    s += "
            use crate::{bindings_manual::*, native_utils};
    ";
    for class in domain.classes.iter() {
        match class {
            Class::Engine(class) => generate_engine_class(&mut s, class, &client_replicated_types),
            Class::Struct(class) => generate_struct(&mut s, class, &client_replicated_types),
            Class::Enum(class) => generate_enum(&mut s, class),
        }
    }
    s
}
