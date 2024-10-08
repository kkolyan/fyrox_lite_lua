use core::panic;

use lite_model::{
    EnumClass, EnumValue,
};
use to_vec::ToVec;

use crate::{
    annotations::type_to_lua::type_rust_to_lua, writelnu
};

pub fn generate_enum(s: &mut String, class: &EnumClass) {
    writelnu!(s, "");
    writelnu!(s, "---@class {}_static", class.class_name);
    variant_constructors(s, class);
    writelnu!(s, "{} = {{}}", class.class_name);

    writelnu!(s, "");
    writelnu!(s, "---@class {}", class.class_name);
    variant_accessors(s, class);
    writelnu!(s, "{}_instance = {{}}", class.class_name);
}

pub fn variant_constructors(s: &mut String, class: &EnumClass) {
    for variant in class.variants.iter() {
        if matches!(variant.value, EnumValue::Unit) {
            writelnu!(s, "");
            writelnu!(s, "---@type {}", class.class_name);
            writelnu!(s, "{}.{} = {{}}", class.class_name, variant.tag)
        } else {
            writelnu!(s, "");
            let inner_data_class = format!("{}_{}", class.class_name, variant.tag);
            writelnu!(s, "---@class {}", inner_data_class);
            match &variant.value {
                EnumValue::Unit => panic!("WTF, we just checked!"),
                EnumValue::Tuple { fields } => {

					for (i, ty) in fields.iter().enumerate() {
						writelnu!(
							s,
							"---@field _{} {}",
							i + 1,
							&type_rust_to_lua(ty),
						);
					}
					writelnu!(s, "{} = {{}}", inner_data_class);
					writelnu!(s, "");

					for (i, ty) in fields.iter().enumerate() {
						writelnu!(
							s,
							"---@param _{} {}",
							i + 1,
							&type_rust_to_lua(ty),
						);
					}

					writelnu!(s, "---@return {}", class.class_name);
					writelnu!(
						s,
						"function {}:{}({}) end",
						&class.class_name,
						&variant.tag,
						fields.iter().enumerate().map(|(i, _)| format!("_{}", i + 1)).to_vec().join(", ")
					);
				}
                EnumValue::Struct { fields } => {
					for field in fields.iter() {
						writelnu!(
							s,
							"---@field {} {}",
							&field.name,
							&type_rust_to_lua(&field.ty),
						);
					}
					writelnu!(s, "{} = {{}}", inner_data_class);
					writelnu!(s, "");
					writelnu!(s, "---@param state {}", inner_data_class);
					writelnu!(s, "---@return {}", class.class_name);
					writelnu!(
						s,
						"function {}:{}(state) end",
						&class.class_name,
						&variant.tag
					);
				}
            }
        }
    }
}
pub fn variant_accessors(s: &mut String, class: &EnumClass) {
    for variant in class.variants.iter() {
		let ty = match &variant.value {
			EnumValue::Unit => "boolean".to_string(),
			EnumValue::Tuple { fields: _ } | EnumValue::Struct { fields: _ } => {
				format!("{}_{}", class.class_name, variant.tag)
			}
		};
		writelnu!(s, "---@field {} {}", &variant.tag, ty);
    }
}
