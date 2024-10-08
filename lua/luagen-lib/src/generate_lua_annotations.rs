use std::collections::{HashMap, HashSet};

use lite_model::{Class, DataType, Domain, EngineClass, EnumClass, EnumValue, EnumVariant, Method, StructClass};
use to_vec::ToVec;

use crate::{
    bindings::generate_methods::{is_getter, is_setter},
    code_model::{HierarchicalCodeBase, ModContent, Module},
    templating::strExt,
};

use crate::annotations::generate_annotations

pub fn generate_lua_annotations(domain: &Domain) -> HierarchicalCodeBase {
    let mut s = String::new();
    s += "
			-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
			-- More about Lua annotations format: https://luals.github.io/wiki/annotations
			-- This file is auto-generated, do not edit it manually.
			
			---@diagnostic disable: missing-return, lowercase-global

			---@class Script
			---@field node Node
			Script = {}

			-- Used to 
			function script_class() end
		".deindent().as_str();
    for class in domain.classes.iter() {
        generate_class(&mut s, class);
    }
    HierarchicalCodeBase {
        mods: vec![Module {
            name: "fyrox-lite".into(),
            content: ModContent::Code(s),
        }],
    }
}

macro_rules! render {
    ($dst:expr, $($arg:tt)*) => {{
		use std::fmt::Write;
		writeln!($dst, $($arg)*).unwrap()
    }}
}

fn generate_class(s: &mut String, class: &Class) {
    render!(s, "");
    render!(s, "---@class {}_static", class.class_name());
    fields(s, class, false);
    render!(s, "{} = {{}}", class.class_name());

    render!(s, "");
    render!(s, "---@class {}", class.class_name());
    fields(s, class, true);
    render!(s, "{}_instance = {{}}", class.class_name());

    methods(s, class, false);
    methods(s, class, true);

	variants(s, class);
}

fn methods(s: &mut String, class: &Class, instance: bool) {
    if let Class::Engine(class) = class {
        for method in class.methods.iter() {
            if method.instance != instance {
                continue;
            }
            if is_getter(method) {
                continue;
            }
            *s += "\n";
            let params = method
                .signature
                .params
                .iter()
                .filter(|it| !matches!(it.ty, DataType::UserScriptGenericStub))
                .to_vec();
            let arg_names = params
                .iter()
                .map(|it| it.name.to_string())
                .to_vec()
                .join(", ");

            if params.iter().any(|it| matches!(it.ty, DataType::ClassName)) {
                render!(s, "---@generic T");
            }

            for param in params.iter() {
                render!(
                    s,
                    "---@param {} {}",
                    &param.name,
                    &type_rust_to_lua(&param.ty)
                );
            }

            if let Some(return_ty) = &method.signature.return_ty {
                render!(s, "---@return {}", type_rust_to_lua(return_ty),);
            }

            render!(
                s,
                "function {}{}:{}({}) end",
                &class.class_name,
                if instance { "_instance" } else { "" },
                &method.method_name,
                &arg_names,
            );
        }
    }
}

fn fields(s: &mut String, class: &Class, instance: bool) {
    if let Class::Struct(class) = class {
        if instance {
            for field in class.fields.iter() {
                render!(
                    s,
                    "---@field {} {}",
                    &field.name,
                    &type_rust_to_lua(&field.ty),
                );
            }
        }
    }
    if let Class::Engine(class) = class {
        if !instance {
            for c in class.constants.iter() {
                render!(s, "---@field {} {}", &c.const_name, type_rust_to_lua(&c.ty),);
            }
        }
        let mut prop_names: Vec<&str> = Default::default();
        let mut getters: HashMap<&str, &Method> = Default::default();
        let mut setters: HashMap<&str, &Method> = Default::default();
        for method in class.methods.iter() {
            if method.instance != instance {
                continue;
            }
            if is_setter(method) {
                let name = method.method_name.strip_prefix("set_").unwrap();
                if !prop_names.contains(&name) {
                    prop_names.push(name);
                }
                setters.insert(name, method);
            }
            if is_getter(method) {
                let name = method.method_name.strip_prefix("get_").unwrap();
                if !prop_names.contains(&name) {
                    prop_names.push(name);
                }
                getters.insert(name, method);
            }
        }
        for prop in prop_names {
            let get_ty = getters
                .get(prop)
                .map(|it| it.signature.return_ty.as_ref().unwrap());
            let set_ty = setters.get(prop).map(|it| &it.signature.params[0].ty);
            let mut types = HashSet::new();
            get_ty.map(|it| types.insert(it));
            set_ty.map(|it| types.insert(it));
            if types.len() > 1 {
                panic!("conflicting accessors for {}::{}", class.class_name, prop);
            }
            let ty = types.into_iter().next().unwrap();
            render!(s, "---@field {} {}", prop, type_rust_to_lua(ty));
        }
    }
}

fn enum_inner_type(class: &EnumClass, variant: &EnumVariant) -> String {
	match &variant.value {
		EnumValue::Unit => "boolean".to_string(),
		EnumValue::Tuple { fields: _ } | EnumValue::Struct { fields: _ } => format!("{}_{}", class.class_name, variant.tag),
	}
}

#[allow(clippy::useless_format)]
fn type_rust_to_lua(ty: &DataType) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => panic!("Unit is not allowed in this context"),
        DataType::Bool => format!("boolean"),
        DataType::Byte => format!("integer"),
        DataType::I32 => format!("integer"),
        DataType::I64 => format!("integer"),
        DataType::F32 => format!("number"),
        DataType::F64 => format!("number"),
        DataType::String => format!("string"),
        DataType::ClassName => format!("`T`"),
        DataType::Vec(item_ty) => format!("{}[]", type_rust_to_lua(item_ty)),
        DataType::UserScript => format!("T"),
        DataType::UserScriptMessage => format!("any"),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub is not allowed in this context")
        }
        DataType::Object(class_name) => class_name.to_string(),
        DataType::Option(item_ty) => format!("{}?", type_rust_to_lua(item_ty)),
        DataType::Result { ok } => type_rust_to_lua(ok),
    }
}
