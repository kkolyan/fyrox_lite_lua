use fyrox_lite_lua_generator::{
    code_model::SimpleRustCodeBase, context::GenerationContext,
    generate_engine_class_bindings::generate_engine_class_bindings,
};
use fyrox_lite_model::{Class, Domain, RustQualifiedName};
use fyrox_lite_parser::generate_domain::generate_domain;

fn main() {
    let fyrox: Domain = generate_domain("fyrox-lite/src");
    let math: Domain = generate_domain("fyrox-lite-math/src");
    let domain = Domain::merge_all([fyrox, math]);
    let mut context = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };

    context.internal_to_external.insert(
        RustQualifiedName("lite_math::PodVector3".to_string()),
        RustQualifiedName("vec::LiteVector3".to_string()),
    );

    context.internal_to_external.insert(
        RustQualifiedName("lite_math::PodQuaternion".to_string()),
        RustQualifiedName("quat::LiteQuaternion".to_string()),
    );

    let mut code_base = SimpleRustCodeBase::default();

    for item in context.domain.classes.iter() {
        match item {
            Class::Engine(it) => {
                code_base.mods.push(generate_engine_class_bindings(it, &context));
            }
            Class::Struct(it) => {}
            Class::Enum(it) => {}
        }
    }

    code_base.write("fyrox-lua/src/generated").unwrap();
}
