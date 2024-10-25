use std::collections::HashSet;

use gen_common::templating::render;
use lite_model::{ClassName, StructClass};

use super::types;

pub(crate) fn generate_struct(
    s: &mut String,
    class: &StructClass,
    client_replicated_types: &HashSet<ClassName>,
) {
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

fn generate_to_native(s: &mut String, class: &StructClass, client_replicated_types: &HashSet<ClassName>) {

    let mut field_names = vec![];

    render(
        s,
        r#"
            impl From<${rust_class}> for Native${class} {
                fn from(__value: ${rust_class}) -> Self {
    "#,
        [("rust_class", &class.rust_struct_path), ("class", &class.class_name)],
    );

    for field in class.fields.iter() {
        field_names.push(field.name.to_string());

        render(s, "
                    let ${name} = __value.${name};
                    let ${name} = ${expr};
        ", [
            ("name", &field.name),
            ("expr", &types::generate_to_native(&field.ty, &field.name, client_replicated_types)),
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

fn generate_from_native(s: &mut String, class: &StructClass, client_replicated_types: &HashSet<ClassName>) {

    let mut field_names = vec![];

    render(
        s,
        r#"
            impl From<Native${class}> for ${rust_class} {
                fn from(__value: Native${class}) -> Self {
    "#,
        [("rust_class", &class.rust_struct_path), ("class", &class.class_name)],
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
