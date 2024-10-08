use std::collections::{HashMap, HashSet};

use lite_model::{Class, DataType, Domain, EngineClass, EnumClass, EnumValue, EnumVariant, Method, StructClass};
use to_vec::ToVec;

use crate::{
    annotations::type_to_lua::type_rust_to_lua, bindings::generate_methods::{is_getter, is_setter}, code_model::{HierarchicalCodeBase, ModContent, Module}, templating::strExt, writelnu
};

pub fn methods(s: &mut String, class: &Class, instance: bool) {
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
                writelnu!(s, "---@generic T");
            }

            for param in params.iter() {
                writelnu!(
                    s,
                    "---@param {} {}",
                    &param.name,
                    &type_rust_to_lua(&param.ty)
                );
            }

            if let Some(return_ty) = &method.signature.return_ty {
                writelnu!(s, "---@return {}", type_rust_to_lua(return_ty),);
            }

            writelnu!(
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
