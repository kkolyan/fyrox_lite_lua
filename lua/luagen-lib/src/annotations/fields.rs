use std::collections::{HashMap, HashSet};

use lite_model::{Class, Method};

use crate::{annotations::type_to_lua::type_rust_to_lua, bindings::generate_methods::{is_getter, is_setter}, writelnu};



pub fn fields(s: &mut String, class: &Class, instance: bool) {
    if let Class::Struct(class) = class {
        if instance {
            for field in class.fields.iter() {
                writelnu!(
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
                writelnu!(s, "---@field {} {}", &c.const_name, type_rust_to_lua(&c.ty),);
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
            writelnu!(s, "---@field {} {}", prop, type_rust_to_lua(ty));
        }
    }
	if let Class::Enum(class) = class {
		
	}
}