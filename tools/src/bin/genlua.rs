use std::collections::HashMap;

use fyrox_lite_lua_generator::{
    code_model::{Mod, ModContent, SimpleRustCodeBase},
    context::GenerationContext,
    generate_engine_class_bindings::generate_engine_class_bindings,
    generate_enum_class_bindings::generate_enum_class_bindings,
    generate_struct_class_bindings::generate_struct_class_bindings,
    templating::render,
};
use lite_model::{Class, ClassName, Domain, RustQualifiedName};
use lite_parser::generate_domain::generate_domain;
use itertools::Itertools;

fn main() {
    let mut fyrox: Domain = generate_domain("fyrox-lite");
    let math: Domain = generate_domain("fyrox-lite-math");

    // math "overrides" classes in fyrox by name
    fyrox.classes.retain_mut(|fyrox_class| {
        let override_class = math.get_class(fyrox_class.class_name());
        if let Some(override_class) = override_class {
            println!(
                "overriding {} by {}",
                fyrox_class.rust_name(),
                override_class.rust_name()
            );
        }
        override_class.is_none()
    });

    let domain = Domain::merge_all([fyrox, math]);
    let mut ctx = GenerationContext {
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
        for class_name in class_names {
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

    code_base.write("fyrox-lua/src/generated");
}

fn generate_registry(ctx: &GenerationContext) -> Mod {
    let mut s = String::new();
    render(
        &mut s,
        r#"
        pub fn register_classes(lua: &mlua::Lua) {
            use crate::fyrox_lite_class::FyroxUserData;
    "#,
        [],
    );
    for class in ctx.domain.classes.iter() {
        let provides_class = match class {
            Class::Engine(_) => true,
            Class::Struct(_) => false,
            Class::Enum(_) => true,
        };
        if provides_class {
            s += format!(
                "
            {}::register_class(lua);
            ",
                class.rust_name()
            )
            .as_str();
        }
    }
    s += r#"
        }
    "#;
    Mod {
        name: "registry".into(),
        content: ModContent::Code(s),
    }
}
