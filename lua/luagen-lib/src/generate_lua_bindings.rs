use lite_model::{Class, Domain};

use crate::{by_package::classes_by_package, code_model::{HierarchicalCodeBase, ModContent, Module}, context::GenerationContext };
use crate::bindings::{generate_engine_class_bindings::generate_engine_class_bindings, generate_enum_class_bindings::generate_enum_class_bindings, generate_registry::generate_registry, generate_struct_class_bindings::generate_struct_class_bindings};



pub fn generate_lua_bindings(domain: &Domain) -> HierarchicalCodeBase {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };

    let mut bindings = HierarchicalCodeBase::default();

    bindings.mods.push(generate_registry(&ctx));

    let by_package = classes_by_package(ctx.domain);

    for (package, class_names) in by_package {
        let mut mods = Vec::new();
        for class_name in class_names.iter() {
            let class = ctx.domain.get_class(class_name).unwrap();
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
        bindings.mods.push(Module {
            name: package.to_string(),
            content: ModContent::Children(mods),
        });
    }
    bindings
}