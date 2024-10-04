use std::{fmt::Display, ops::Deref};

use crate::DataType;

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::UnresolvedClass(_) => write!(f, "UnresolvedClass"),
            DataType::Unit => write!(f, "Unit"),
            DataType::Bool => write!(f, "Bool"),
            DataType::Byte => write!(f, "Byte"),
            DataType::I32 => write!(f, "i32"),
            DataType::I64 => write!(f, "i64"),
            DataType::F32 => write!(f, "f32"),
            DataType::F64 => write!(f, "f64"),
            DataType::String => write!(f, "String"),
            DataType::Vec(it) => write!(f, "Vec<{}>", it.deref()),
            DataType::UserScript => write!(f, "UserScript"),
            DataType::UserScriptMessage => write!(f, "UserScriptMessage"),
            DataType::UserScriptGenericStub => write!(f, "UserScriptGenericStub"),
            DataType::Object(it) => write!(f, "{}", it.0),
            DataType::Option(it) => write!(f, "Option<{}>", it.deref()),
            DataType::Result { ok } => write!(f, "Result<{}>", ok.deref()),
        }
    }
}
