use lite_model::StructClass;

use crate::{annotations::type_to_lua::type_rust_to_lua, writelnu};

pub fn generate_struct(s: &mut String, class: &StructClass) {
    writelnu!(s, "");
    writelnu!(s, "---@class {}", class.class_name);
    fields(s, class);
    writelnu!(s, "{}_instance = {{}}", class.class_name);
}

pub fn fields(s: &mut String, class: &StructClass) {
    for field in class.fields.iter() {
        writelnu!(
            s,
            "---@field {} {}",
            &field.name,
            &type_rust_to_lua(&field.ty),
        );
    }
}
