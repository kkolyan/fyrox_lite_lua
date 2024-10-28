use std::ops::Deref;
use lite_model::DataType;

pub fn type_rs2cs(ty: &DataType) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("Unresolved class: {}", it),
        DataType::Unit => format!("void"),
        DataType::Bool => format!("bool"),
        DataType::Byte => format!("byte"),
        DataType::I32 => format!("int"),
        DataType::I64 => format!("long"),
        DataType::F32 => format!("float"),
        DataType::F64 => format!("dobule"),
        DataType::String => format!("string"),
        DataType::ClassName => format!("string"),
        DataType::Vec(it) => format!("List<{}>", type_rs2cs(it.deref())),
        DataType::UserScript => format!("T"),
        DataType::UserScriptMessage => format!("T"),
        DataType::UserScriptGenericStub => panic!("WTF, UserScriptGenericStub should be filtered out"),
        DataType::Object(it) => format!("{}", it),
        DataType::Option(it) => format!("{}?", type_rs2cs(it.deref())),
        // err will throw exception
        DataType::Result { ok,.. } => format!("{}", type_rs2cs(ok.deref())),
    }
}