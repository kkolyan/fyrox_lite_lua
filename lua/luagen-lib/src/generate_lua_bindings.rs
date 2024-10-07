use std::collections::HashMap;

use lite_model::{Class, ClassName, Domain};

use crate::{code_model::{Mod, ModContent, SimpleRustCodeBase}, context::GenerationContext, generate_engine_class_bindings::generate_engine_class_bindings, generate_enum_class_bindings::generate_enum_class_bindings, generate_registry::generate_registry, generate_struct_class_bindings::generate_struct_class_bindings};



pub fn generate_lua_bindings(domain: Domain) -> SimpleRustCodeBase {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };


    // context.internal_to_external.insert(
    //     RustQualifiedName("lite_math::PodVector3".to_string()),
    //     RustQualifiedName("vec::LiteVector3".to_string()),
    // );

    // context.internal_to_external.insert(
    //     RustQualifiedName("lite_math::PodQuaternion".to_string()),
    //     RustQualifiedName("quat::LiteQuaternion".to_string()),
    // );

    let mut code_base = SimpleRustCodeBase::default();

    code_base.mods.push(generate_registry(&ctx));

    let mut by_package: HashMap<&str, Vec<ClassName>> = Default::default();
    for class in ctx.domain.classes.iter() {
        let rust_name_without_crate = class.rust_name().0.split_once("::").unwrap().1;
        let package = rust_name_without_crate.split_once("::").unwrap().0;
        by_package
            .entry(package)
            .or_default()
            .push(class.class_name().clone());
    }

    for (package, class_names) in by_package {
        let mut mods = Vec::new();
        for class_name in class_names.iter() {
            let class = ctx.domain.get_class(&class_name).unwrap();
            match class {
                Class::Engine(it) => {
                    mods.push(generate_engine_class_bindings(it, &ctx));
                }
                Class::Struct(it) => {
                    mods.push(generate_struct_class_bindings(it, &ctx));
                }
                Class::Enum(it) => {
                    mods.push(generate_enum_class_bindings(it, &ctx));
                }
            }
        }
        code_base.mods.push(Mod {
            name: package.to_string(),
            content: ModContent::Children(mods),
        });
    }
    code_base
}