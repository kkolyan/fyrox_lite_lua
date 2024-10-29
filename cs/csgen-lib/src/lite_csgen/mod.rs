use gen_common::by_package::classes_by_package;
use gen_common::code_model::{HierarchicalCodeBase, ModContent, Module};
use gen_common::context::GenerationContext;
use lite_model::{Class, DataType, Domain};

pub mod engine_class;
pub mod struct_class;
pub mod enum_class;
pub mod api_types;
pub mod write_cs;
pub mod wrappers;

pub fn generate_cs_facade(domain: &Domain) -> HierarchicalCodeBase {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };

    let mut bindings = HierarchicalCodeBase::default();

    let by_package = classes_by_package(ctx.domain);

    for (package, class_names) in by_package {
        let mut mods = Vec::new();
        for class_name in class_names.iter() {
            let class = ctx.domain.get_class(class_name).unwrap();
            match class {
                Class::Engine(it) => {
                    mods.push(engine_class::generate_bindings(it, &ctx));
                }
                Class::Struct(it) => {
                    mods.push(struct_class::generate_bindings(it, &ctx));
                }
                Class::Enum(it) => {
                    mods.push(enum_class::generate_bindings(it, &ctx));
                }
            }
        }
        bindings.mods.push(Module {
            name: package.to_string(),
            content: ModContent::Children(mods),
        });
    }
    bindings.mods.push(generate_base());
    bindings
}

fn generate_base() -> Module {
    let mut s = String::new();
    wrappers::generate_optional(&mut s, &DataType::F32);
    wrappers::generate_result(&mut s, &DataType::String);
    wrappers::generate_optional(&mut s, &DataType::UserScript);
    wrappers::generate_result(&mut s, &DataType::UserScript);
    wrappers::generate_result(&mut s, &DataType::Option(Box::new(DataType::UserScript)));
    Module::code("LiteBase", s)
}