use std::{collections::HashSet, fmt::Display};

use gen_common::templating::render;
use lite_model::{ClassName, DataType, StructClass};
use to_vec::ToVec;

use super::{simple_from, types};

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

fn generate_to_native(
    s: &mut String,
    class: &StructClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    simple_from::generate_from(
        s,
        &class.rust_struct_path,
        &format!("Native{}", &class.class_name),
        client_replicated_types,
        class.fields.iter().map(|it| (it.name.as_str(), &it.ty)),
    );
}

fn generate_from_native(
    s: &mut String,
    class: &StructClass,
    client_replicated_types: &HashSet<ClassName>,
) {
    simple_from::generate_from(
        s,
        &format!("Native{}", &class.class_name),
        &class.rust_struct_path,
        client_replicated_types,
        class.fields.iter().map(|it| (it.name.as_str(), &it.ty)),
    );
}
