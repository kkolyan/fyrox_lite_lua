use std::collections::HashSet;

use lite_model::{Class, DataType, Domain, EngineClass, EnumClass, StructClass};
use to_vec::ToVec;

use crate::{
    bindings::generate_methods::{is_getter, is_setter},
    code_model::{HierarchicalCodeBase, ModContent, Module},
    templating::render,
};

pub fn generate_lua_annotations(domain: &Domain) -> HierarchicalCodeBase {
    let mut s = String::new();
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

fn generate_class(s: &mut String, class: &Class) {
    *s += "\n";
    render(
        s,
        "---@class ${class_name}",
        [("class_name", class.class_name())],
    );
    fields(s, class);
    render(
        s,
        "${class_name} = {}",
        [("class_name", class.class_name())],
    );

    if let Class::Engine(class) = class {
        for method in class.methods.iter() {
            if !method.instance {
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

            for param in params.iter() {
                render(
                    s,
                    "---@param ${name} ${type}",
                    [
                        ("name", &param.name),
                        ("type", &type_rust_to_lua(&param.ty)),
                    ],
                );
            }

            if let Some(return_ty) = &method.signature.return_ty {
                render(
                    s,
                    "---@return ${type}",
                    [("type", &type_rust_to_lua(return_ty))],
                );
            }

            render(
                s,
                "function ${class_name}:${name}(${arg_names}) end",
                [
                    ("class_name", &class.class_name),
                    ("name", &method.method_name),
                    ("arg_names", &arg_names),
                ],
            );
        }
    }

    match class {
        Class::Engine(class) => engine_class(s, class),
        Class::Struct(class) => struct_class(s, class),
        Class::Enum(class) => enum_class(s, class),
    }
}

fn fields(s: &mut String, class: &Class) {
    if let Class::Struct(class) = class {
        for field in class.fields.iter() {
            render(
                s,
                "---@field ${name} ${type}",
                [
                    ("name", &field.name),
                    ("type", &type_rust_to_lua(&field.ty)),
                ],
            );
        }
    }
    if let Class::Engine(class) = class {
        let mut defined_fields = HashSet::new();
        for method in class.methods.iter() {
            if !method.instance {
                continue;
            }
            let mut property = None;
            if is_getter(method) {
                property = Some((
                    method.method_name.strip_prefix("get_").unwrap(),
                    match method.signature.return_ty.as_ref() {
                        Some(it) => it,
                        None => panic!(
                            "getter without return value: {}::{}",
                            class.class_name, method.method_name
                        ),
                    },
                ));
            } else if is_setter(method) {
                property = Some((
                    method.method_name.strip_prefix("set_").unwrap(),
                    &method.signature.params[0].ty,
                ));
            }
            if let Some((prop_name, ty)) = property {
                if defined_fields.insert(prop_name) {
                    render(
                        s,
                        "---@field ${name} ${type}",
                        [("name", &prop_name), ("type", &type_rust_to_lua(ty))],
                    );
                }
            }
        }
    }
}

fn enum_class(s: &mut String, class: &EnumClass) {}

fn struct_class(s: &mut String, class: &StructClass) {}

fn engine_class(s: &mut String, class: &EngineClass) {}

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
        DataType::Vec(item_ty) => format!("{}[]", type_rust_to_lua(item_ty)),
        DataType::UserScript => format!("UserScript"),
        DataType::UserScriptMessage => format!("UserScriptMessage"),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub is not allowed in this context")
        }
        DataType::Object(class_name) => class_name.to_string(),
        DataType::Option(item_ty) => format!("{}?", type_rust_to_lua(item_ty)),
        DataType::Result { ok } => type_rust_to_lua(ok),
    }
}
