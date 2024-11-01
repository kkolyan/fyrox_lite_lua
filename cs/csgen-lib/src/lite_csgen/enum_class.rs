use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::{DataType, EnumClass, StructClass};
use crate::lite_csgen::api_types;
use crate::lite_csgen::gen_rs::RustEmitter;

pub(crate) fn generate_bindings(class: &EnumClass, ctx: &GenerationContext, rust: &mut RustEmitter) -> Module {
    let mut s = String::new();

    render(&mut s, r#"
            // ${rust_path}
            public enum ${class}
            {
            "#, [("class", &class.class_name), ("rust_path", &class.rust_struct_path)]);

    for variant in class.variants.iter() {
        render(&mut s, r#"
                ${name},
        "#, [("name", &variant.tag)]);
    }

    render(&mut s, r#"
            }
            "#, []);


    let mut rs = String::new();

    render(&mut rs, r#"
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq, Eq)]
            pub enum ${class} {
    "#, [("class", &api_types::type_rs(&DataType::Object(class.class_name.clone()), ctx).to_native())]);

    for variant in class.variants.iter() {
        render(&mut rs, r#"
                ${name},
        "#, [("name", &variant.tag)]);
    }

    render(&mut rs, r#"
            }
            "#, []);

    generate_rust_conversions(&mut rs, class, ctx);

    rust.emit_statement(rs);

    Module::code(&class.class_name, s)
}

fn generate_rust_conversions(rs: &mut String, class: &EnumClass, ctx: &GenerationContext) {
    let ty = DataType::Object(class.class_name.clone());
    let mut clauses_from_lite = String::new();
    let mut clauses_from_native = String::new();

    let class_native = api_types::type_rs(&ty, ctx).to_native();
    let class_lite = api_types::type_rs(&ty, ctx).to_lite();

    for variant in class.variants.iter() {
        render(&mut clauses_from_lite, r#"
                        ${class_lite}::${name} => ${class_native}::${name},
        "#, [("name", &variant.tag), ("class_lite", &class_lite), ("class_native", &class_native), ("class_lite", &class_lite)]);
        render(&mut clauses_from_native, r#"
                        ${class_native}::${name} => ${class_lite}::${name},
        "#, [("name", &variant.tag), ("class_lite", &class_lite), ("class_native", &class_native), ("class_lite", &class_lite)]);
    }

    render(rs, r#"

            impl From<${class_lite}> for ${class_native} {
                fn from(value: ${class_lite}) -> Self {
                    match value {
                        ${clauses_from_lite}
                    }
                }
            }

            impl From<${class_native}> for ${class_lite} {
                fn from(value: ${class_native}) -> Self {
                    match value {
                        ${clauses_from_native}
                    }
                }
            }
    "#, [
        ("class_native", &class_native),
        ("class_lite", &class_lite),
        ("clauses_from_lite", &clauses_from_lite),
        ("clauses_from_native", &clauses_from_native),
    ]);
}
