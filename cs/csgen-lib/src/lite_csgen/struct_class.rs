use convert_case::{Case, Casing};
use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::StructClass;
use crate::lite_csgen::api_types;

pub(crate) fn generate_bindings(class: &StructClass, ctx: &GenerationContext) -> Module {
    let mut s = String::new();
    render(&mut s, r#"
            // ${rust_path}
            [StructLayout(LayoutKind.Sequential)]
            public struct ${class}
            {
    "#, [("class", &class.class_name), ("rust_path", &class.rust_struct_path)]);

    for field in class.fields.iter() {
        render(&mut s, r#"
                public ${type} ${name};
            "#, [
            ("type", &api_types::type_rs2cs(&field.ty)),
            ("name", &field.name.to_case(Case::Pascal)),
        ]);
    }

    render(&mut s, r#"
            }
            "#, []);
    Module::code(&class.class_name, s)
}