use std::{fmt::Display, ops::Deref};

use crate::{Class, ClassName, DataType, Domain, EnumValue, Method, RustQualifiedName};

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
            DataType::ClassName => write!(f, "ClassName"),
            DataType::Vec(it) => write!(f, "Vec<{}>", it.deref()),
            DataType::UserScript => write!(f, "UserScript"),
            DataType::UserScriptMessage => write!(f, "UserScriptMessage"),
            DataType::UserScriptGenericStub => write!(f, "UserScriptGenericStub"),
            DataType::Object(it) => write!(f, "{}", it.0),
            DataType::Option(it) => write!(f, "Option<{}>", it.deref()),
            DataType::Result { ok } => write!(f, "Result<{}>", ok.deref()),
            DataType::Buffer(it) => write!(f, "Buffer<{}>", it.deref()),
        }
    }
}


impl Domain {
    pub fn get_class(&self, name: &ClassName) -> Option<&Class> {
        self.classes.iter().find(|it| it.class_name() == name)
    }
}

impl Domain {
    pub fn merge_all(domains: impl IntoIterator<Item=Domain>) -> Self {
        let mut classes = Vec::new();
        for domain in domains {
            classes.extend(domain.classes);
        }
        Self {classes}
    }
}

impl Class {
    pub fn class_name(&self) -> &ClassName {
        match self {
            Class::Engine(it) => &it.class_name,
            Class::Struct(it) => &it.class_name,
            Class::Enum(it) => &it.class_name,
        }
    }

    pub fn rust_name(&self) -> &RustQualifiedName {
        match self {
            Class::Engine(it) => &it.rust_struct_path,
            Class::Struct(it) => &it.rust_struct_path,
            Class::Enum(it) => &it.rust_struct_path,
        }
    }
}

pub(crate) fn is_false(value: &bool) -> bool {
    *value
}

impl EnumValue {
    pub(crate) fn is_unit(&self) -> bool {
        matches!(self, EnumValue::Unit)
    }

    pub(crate) fn unit() -> EnumValue {
        EnumValue::Unit
    }
}

impl Display for RustQualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Display for ClassName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Method {
    pub fn is_generic(&self) -> bool {
        self
            .signature
            .params
            .iter()
            .any(|it| {
                matches!(
                    it.ty,
                    DataType::UserScript
                        | DataType::UserScriptMessage
                        | DataType::UserScriptGenericStub
                        | DataType::Buffer(_)
                )
            })
    }
}