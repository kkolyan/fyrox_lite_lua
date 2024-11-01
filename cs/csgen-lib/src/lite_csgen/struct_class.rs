use std::fmt::Display;
use convert_case::{Case, Casing};
use itertools::Itertools;
use to_vec::ToVec;
use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use gen_common::templating::render;
use lite_model::{DataType, Field, StructClass};
use crate::lite_csgen::{api_types, wrappers};
use crate::lite_csgen::api_types::CsType;
use crate::lite_csgen::gen_rs::RustEmitter;

pub(crate) fn generate_bindings(class: &StructClass, ctx: &GenerationContext, rust: &mut RustEmitter) -> Module {
    let mut s = String::new();
    let class_name = api_types::type_cs(&DataType::Object(class.class_name.clone())).to_blittable();
    render(&mut s, r#"
            // ${rust_path}
            [StructLayout(LayoutKind.Sequential)]
            ${visibility} partial struct ${class}
            {
    "#, [
        ("visibility", &if api_types::is_implemented_externally(&class.class_name) {"internal"} else {"public"}),
        ("class", &class_name),
        ("rust_path", &class.rust_struct_path)
    ]);

    if !api_types::is_implemented_externally(&class.class_name) {
        for field in class.fields.iter() {
            generate_property(&mut s, class, field, ctx);
        }
        s += "
            //===============================================================
            // private fields for all properties (not only mapped),
            // because it makes ABI much more readable.
            // I hope, NativeAOT will optimize out this.
            //===============================================================";
    }

    let mut rs = String::new();

    render(&mut rs, r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct ${class} {"#, [("class", &api_types::type_rs(&DataType::Object(class.class_name.clone()), ctx).to_native())]);

    for field in class.fields.iter() {
        generate_field(&mut s, &mut rs, class, field, ctx);
    }

    render(&mut rs, r#"
            }"#, []);

    generate_rust_conversions(&mut rs, class, ctx);

    rust.emit_statement(rs);

    render(&mut s, r#"
            }
            "#, []);

    wrappers::generate_optional(&mut s, rust, &DataType::Object(class.class_name.clone()), ctx);

    wrappers::generate_slice(&mut s, rust, &DataType::Object(class.class_name.clone()), ctx);

    wrappers::generate_result(&mut s, rust, &DataType::Object(class.class_name.clone()), ctx);

    Module::code(&class_name, s)
}

fn generate_property(s: &mut String, class: &StructClass, field: &Field, ctx: &GenerationContext) {
    let ty = api_types::type_cs(&field.ty);
    let facade_name = field.name.to_case(Case::Pascal);
    let private_name = format!("_{}", field.name);
    match &ty {
        CsType::Blittable(ty) => {
            render(s, r#"
                public ${type} ${facade_name} {
                    get => ${private_name};
                    set => ${private_name} = value;
                }
                "#, [
                ("type", &ty),
                ("private_name", &private_name),
                ("facade_name", &facade_name),
            ]);
        }
        CsType::Mapped { facade, blittable, .. } => {
            render(s, r#"
                public ${facade_ty} ${facade_name} {
                    get => ${blittable_ty}.ToFacade(${private_name});
                    set => ${private_name} = ${blittable_ty}.FromFacade(value);
                }
                "#, [
                ("blittable_ty", &blittable),
                ("facade_ty", &facade),
                ("private_name", &private_name),
                ("facade_name", &facade_name),
            ]);
        }
    }
}
fn generate_field(s: &mut String, rs: &mut String, class: &StructClass, field: &Field, ctx: &GenerationContext) {
    let ty = api_types::type_cs(&field.ty);
    let private_name = format!("_{}", field.name);
    render(s, r#"
                private ${type} ${private_name};
                "#, [
        ("type", &ty.to_blittable()),
        ("private_name", &private_name),
    ]);
    render(rs, r#"
                pub ${private_name}: ${type},
            "#, [
        ("type", &api_types::type_rs(&field.ty, ctx).to_native()),
        ("private_name", &field.name),
    ]);
}

fn generate_rust_conversions(rs: &mut String, class: &StructClass, ctx: &GenerationContext) {
    let ty = DataType::Object(class.class_name.clone());
    let mut conversions = String::new();

    for field in class.fields.iter() {
        render(&mut conversions, r#"
                    let ${name} = __value.${name}.into();
        "#, [("name", &field.name)]);
    }
    
    let output_fields = class.fields.iter().map(|it| it.name.clone()).to_vec();


    render(rs, r#"

            impl From<${class_lite}> for ${class_native} {
                fn from(__value: ${class_lite}) -> Self {
                    ${conversions}
                    Self { ${output_fields} }
                }
            }

            impl From<${class_native}> for ${class_lite} {
                fn from(__value: ${class_native}) -> Self {
                    ${conversions}
                    Self { ${output_fields} }
                }
            }
    "#, [
        ("conversions", &conversions),
        ("class_native", &api_types::type_rs(&ty, ctx).to_native()),
        ("class_lite", &api_types::type_rs(&ty, ctx).to_lite()),
        ("output_fields", &output_fields.join(", ")),
    ]);
}
