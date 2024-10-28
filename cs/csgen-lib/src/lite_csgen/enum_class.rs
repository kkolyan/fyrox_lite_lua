use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::EnumClass;

pub(crate) fn generate_bindings(class: &EnumClass, ctx: &GenerationContext) -> Module {
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
    Module::code(&class.class_name, s)
}