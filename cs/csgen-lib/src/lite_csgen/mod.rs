use gen_common::by_package::classes_by_package;
use gen_common::code_model::{HierarchicalCodeBase, ModContent, Module};
use gen_common::context::GenerationContext;
use lite_model::{Class, DataType, Domain};
use crate::lite_csgen::gen_rs::RustEmitter;

pub mod engine_class;
pub mod struct_class;
pub mod enum_class;
pub mod api_types;
pub mod write_cs;
pub mod wrappers;
mod gen_rs;

pub fn generate_cs_facade(domain: &Domain) -> (HierarchicalCodeBase, RustEmitter) {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };
    
    let mut rust = RustEmitter::default();

    let mut bindings = HierarchicalCodeBase::default();

    let by_package = classes_by_package(ctx.domain);

    for (package, class_names) in by_package {
        let mut mods = Vec::new();
        for class_name in class_names.iter() {
            let class = ctx.domain.get_class(class_name).unwrap();
            match class {
                Class::Engine(it) => {
                    mods.push(engine_class::generate_bindings(it, &ctx, &mut rust));
                }
                Class::Struct(it) => {
                    mods.push(struct_class::generate_bindings(it, &ctx, &mut rust));
                }
                Class::Enum(it) => {
                    mods.push(enum_class::generate_bindings(it, &ctx, &mut rust));
                }
            }
        }
        bindings.mods.push(Module {
            name: package.to_string(),
            content: ModContent::Children(mods),
        });
    }
    bindings.mods.push(generate_base(&mut rust, &ctx));
    (bindings, rust)
}

fn generate_base(rust: &mut RustEmitter, ctx: &GenerationContext) -> Module {
    let mut s = String::new();
    wrappers::generate_optional(&mut s, rust, &DataType::F32, ctx);
    wrappers::generate_result(&mut s, rust, &DataType::String);
    wrappers::generate_optional(&mut s, rust, &DataType::UserScript, ctx);
    wrappers::generate_result(&mut s, rust, &DataType::UserScript);
    wrappers::generate_result(&mut s, rust, &DataType::Option(Box::new(DataType::UserScript)));
    Module::code("LiteBase", s)
}