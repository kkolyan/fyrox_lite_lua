use std::{collections::HashSet, fmt::Display};

use gen_common::templating::render;
use lite_model::{ClassName, DataType, StructClass};
use to_vec::ToVec;

use super::{simple_from, types};

pub(crate) fn generate_struct(
    s: &mut String,
    class: &StructClass,
    client_replicated_types: &HashSet<ClassName>,
    generated_structs: &mut HashSet<String>,
) {
    generated_structs.insert(format!("Native{}", class.class_name));
    generated_structs.insert(format!("Native{}_array", class.class_name));
    generated_structs.insert(format!("Native{}_option", class.class_name));
    generated_structs.insert(format!("Native{}_result", class.class_name));
    render(
        s,
        r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Native${class} {
    "#,
        [("class", &class.class_name)],
    );

    for field in class.fields.iter() {
        render(
            s,
            r#"
                pub ${field}: ${type},
        "#,
            [
                ("field", &field.name),
                (
                    "type",
                    &types::generate_ffi_type(&field.ty, client_replicated_types),
                ),
            ],
        );
    }
    render(
        s,
        r#"
            }
    "#,
        [],
    );

    render(
        s,
        r#"
            native_utils!(Native${class}, Native${class}_array, Native${class}_option, Native${class}_result);
    "#,
        [("class", &class.class_name)],
    );

    generate_from_native(s, class, client_replicated_types);
    generate_to_native(s, class, client_replicated_types);
}

fn generate_to_native(
    s: &mut String,
    class: &StructClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    let mut field_names = vec![];

    render(
        s,
        r#"
            impl From<${from}> for ${to} {
                fn from(__value: ${from}) -> Self {
    "#,
        [("from", &class.rust_struct_path), ("to", &format!("Native{}", &class.class_name))],
    );

    for field in class.fields.iter() {
        let name = &field.name;
        let ty = &field.ty;
        field_names.push(name.to_string());

        render(s, "
                    let ${name} = __value.${name};
                    let ${name} = ${expr};
        ", [
            ("name", &name),
            ("expr", &types::generate_to_native(&ty, &name, client_replicated_types)),
        ]);
    }

    render(
        s,
        r#"
                    Self { ${field_names} }
                }
            }
    "#,
        [("field_names", &field_names.join(", "))],
    );
}

fn generate_from_native(
    s: &mut String,
    class: &StructClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    let mut field_names = vec![];

    render(
        s,
        r#"
            impl From<${from}> for ${to} {
                fn from(__value: ${from}) -> Self {
    "#,
        [("from", &format!("Native{}", &class.class_name)), ("to", &class.rust_struct_path)],
    );

    for field in class.fields.iter() {
        field_names.push(field.name.to_string());

        render(s, "
                    let ${name} = __value.${name};
                    let ${name} = ${expr};
        ", [
            ("name", &field.name),
            ("expr", &types::generate_from_native(&field.ty, &field.name, client_replicated_types)),
        ]);
    }

    render(
        s,
        r#"
                    Self { ${field_names} }
                }
            }
    "#,
        [("field_names", &field_names.join(", "))],
    );
}
