use std::collections::HashSet;

use gen_common::templating::render;
use lite_model::{ClassName, StructClass};

use super::types;

pub(crate) fn generate_struct(s: &mut String, class: &StructClass, client_replicated_types: &HashSet<ClassName>) {
    render(s, r#"
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Native${class} {
    "#, [("class", &class.class_name)]);

    for field in class.fields.iter() {
        render(s, r#"
                pub ${field}: ${type},
        "#, [
            ("field", &field.name),
            ("type", &types::generate_ffi_type(&field.ty, client_replicated_types)),
        ]);
    }
    render(s, r#"
            }
    "#, []);

    render(s, r#"
            native_utils!(Native${class}, Native${class}_array, Native${class}_option, Native${class}_result);
    "#, [("class", &class.class_name)]);
}