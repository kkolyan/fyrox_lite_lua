use std::{collections::HashSet, ops::Deref};

use gen_common::templating::render_string;
use lite_model::{ClassName, DataType};

pub(crate) fn generate_ffi_type(
    ty: &DataType,
    client_replicated_types: &HashSet<ClassName>,
) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => format!("void"),
        DataType::Bool => format!("bool"),
        DataType::Byte => format!("u8"),
        DataType::I32 => format!("i32"),
        DataType::I64 => format!("i64"),
        DataType::F32 => format!("f32"),
        DataType::F64 => format!("f64"),
        DataType::String => format!("NativeString"),
        DataType::ClassName => format!("NativeString"),
        DataType::Vec(it) => format!("{}_array", generate_ffi_type(it, client_replicated_types)),
        DataType::UserScript => format!("NativeHandle"),
        DataType::UserScriptMessage => format!("NativeInstanceId"),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub should not be exposed in bindings")
        }
        DataType::Buffer(_) => {
            panic!("Buffer should not be exposed in bindings")
        }
        DataType::Object(it) => match client_replicated_types.contains(it) {
            false => format!("NativeHandle"),
            true => format!("Native{}", it),
        },
        DataType::Option(it) => {
            format!("{}_option", generate_ffi_type(it, client_replicated_types))
        }
        DataType::Result { ok } => {
            format!("{}_result", generate_ffi_type(ok, client_replicated_types))
        }
    }
}

pub(crate) fn generate_to_native(
    ty: &DataType,
    var: &str,
    client_replicated_types: &HashSet<ClassName>,
) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => panic!("`void` should be handled before"),
        DataType::Bool => format!("{}", var),
        DataType::Byte => format!("{}", var),
        DataType::I32 => format!("{}", var),
        DataType::I64 => format!("{}", var),
        DataType::F32 => format!("{}", var),
        DataType::F64 => format!("{}", var),
        DataType::String => format!("<u8 as NativeType>::to_native_array({}.into_bytes())", var),
        DataType::ClassName => format!("<u8 as NativeType>::to_native_array({}.into_bytes())", var),
        DataType::Vec(it) => render_string(
            "<${element} as NativeType>::to_native_array(${var}.into_iter().map(|${var}| ${expr}).collect::<Vec<_>>())",
            [
                ("element", &generate_ffi_type(it.deref(), client_replicated_types)),
                ("var", &var),
                ("expr", &generate_to_native(it.deref(), var, client_replicated_types))
            ],
        ),
        DataType::UserScript => format!("{}.into()", var),
        DataType::UserScriptMessage => format!("{}", var),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub should not be exposed in bindings")
        }
        DataType::Buffer(_) => {
            panic!("Buffer should not be exposed in bindings")
        }
        DataType::Object(it) => match client_replicated_types.contains(it) {
            true => format!("{}.into()", var),
            false => format!("NativeHandle::from_u128(Externalizable::to_external(&{}))", var),
        },
        DataType::Option(it) => {
            render_string(
                "<${class} as NativeType>::to_native_option(if let Some(${var}) = ${var} { Some(${expr} ) } else { None })", 
                [
                    ("class", &generate_ffi_type(it.deref(), client_replicated_types)), 
                    ("var", &var),
                    ("expr", &generate_to_native(it.deref(), var, client_replicated_types)),
                ]
            )
        }
        DataType::Result { ok } => {
            render_string(
                "<${class} as NativeType>::to_native_result( match ${var} { Ok(${var}) => Ok(${expr}), Err(err) => Err(err) } )", 
                [
                    ("class", &generate_ffi_type(ok.deref(), client_replicated_types)),
                    ("var", &var),
                    ("expr", &generate_to_native(ok.deref(), var, client_replicated_types)),
                ]
            )
        },
    }
}

pub(crate) fn generate_from_native(
    ty: &DataType,
    var: &str,
    client_replicated_types: &HashSet<ClassName>,
) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => format!("()"),
        DataType::Bool => format!("{}", var),
        DataType::Byte => format!("{}", var),
        DataType::I32 => format!("{}", var),
        DataType::I64 => format!("{}", var),
        DataType::F32 => format!("{}", var),
        DataType::F64 => format!("{}", var),
        DataType::String => format!(
            "String::from_utf8(<u8 as NativeType>::from_native_array({})).unwrap()",
            var
        ),
        DataType::ClassName => format!(
            "String::from_utf8(<u8 as NativeType>::from_native_array({})).unwrap()",
            var
        ),
        DataType::Vec(it) => render_string(
            "<${element} as NativeType>::from_native_array(${var}).into_iter().map(|${var}| ${expr}).collect::<Vec<_>>()",
            [
                ("element", &generate_ffi_type(it.deref(), client_replicated_types)),
                ("var", &var),
                ("expr", &generate_from_native(it.deref(), var, client_replicated_types))
            ],
        ),
        DataType::UserScript => format!("{}", var),
        DataType::UserScriptMessage => format!("{}", var),
        DataType::UserScriptGenericStub => format!("()"),
        DataType::Buffer(_) => panic!("Buffer should not be here"),
        DataType::Object(it) => match client_replicated_types.contains(it) {
            true => format!("{}.into()", var),
            false => format!("Externalizable::from_external({}.as_u128())", var),
        },
        DataType::Option(it) => render_string(
            "if ${var}.present { Some( { let ${var} = ${var}.value; ${expr} } ) } else { None }",
            [
                ("var", &var),
                (
                    "expr",
                    &generate_from_native(it.deref(), var, client_replicated_types),
                ),
            ],
        ),
        DataType::Result { ok } => render_string(
            "if ${var}.ok { Ok((${expr}).value) } else { Err(${var}.err) }",
            [
                ("var", &var),
                (
                    "expr",
                    &generate_from_native(ok.deref(), var, client_replicated_types),
                ),
            ],
        ),
    }
}
