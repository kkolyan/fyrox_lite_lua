use lite_model::DataType;

#[allow(clippy::useless_format)]
pub fn type_rust_to_lua(ty: &DataType) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => panic!("Unit is not allowed in this context"),
        DataType::Bool => format!("boolean"),
        DataType::Byte => format!("number"),
        DataType::I32 => format!("number"),
        DataType::I64 => format!("number"),
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
        DataType::Buffer(item_ty) => format!("{}[]", type_rust_to_lua(item_ty)),
        DataType::Object(class_name) => class_name.to_string(),
        DataType::Option(item_ty) => format!("{}?", type_rust_to_lua(item_ty)),
        DataType::Result { ok } => type_rust_to_lua(ok),
    }
}
