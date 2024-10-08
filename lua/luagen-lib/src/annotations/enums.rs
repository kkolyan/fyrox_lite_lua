use std::collections::{HashMap, HashSet};

use lite_model::{Class, DataType, Domain, EngineClass, EnumClass, EnumValue, EnumVariant, Method, StructClass};
use to_vec::ToVec;

use crate::{
    bindings::generate_methods::{is_getter, is_setter},
    code_model::{HierarchicalCodeBase, ModContent, Module},
    templating::strExt,
};

pub fn variants(s: &mut String, class: &Class) {
	let Class::Enum(class) = class else {
		return;
	};

	render!(s, "---@class {}_static", class.class_name);
	for variant in class.variants.iter() {
		if matches!(variant.value, EnumValue::Unit) {
			let ty = enum_inner_type(class, variant);
			render!(s, "---@field {} {}", &variant.tag, class.class_name);
		}
	}
	render!(s, "{} = {{}}", class.class_name);

	render!(s, "---@class {}", class.class_name);
	for variant in class.variants.iter() {
		let ty = enum_inner_type(class, variant);
		render!(s, "---@field {} {}", &variant.tag, ty,);
	}
	render!(s, "{}_instance = {{}}", class.class_name);
	render!(s, "");

	for variant in class.variants.iter() {
		if matches!(variant.value, EnumValue::Unit) {
			continue;
		}
		let inner_type = enum_inner_type(class, variant);
		render!(s, "---@class {}", inner_type);
		render!(s, "{} = {{}}", inner_type);
		render!(s, "");
		render!(s, "---@param state {}", inner_type);
		render!(s, "---@return {}", class.class_name);
		render!(
			s,
			"function {}:{}(state) end",
			&class.class_name,
			&variant.tag
		);
	}
}