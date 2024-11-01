use std::fmt::Display;
use std::ops::Deref;
use gen_common::context::GenerationContext;
use lite_model::{ClassName, DataType};

pub fn type_cs(ty: &DataType) -> CsType {
    match ty {
        DataType::UnresolvedClass(it) => panic!("Unresolved class: {}", it),
        DataType::Unit => CsType::Blittable(format!("void")),
        DataType::Bool => CsType::Blittable(format!("bool")),
        DataType::Byte => CsType::Blittable(format!("byte")),
        DataType::I32 => CsType::Blittable(format!("int")),
        DataType::I64 => CsType::Blittable(format!("long")),
        DataType::F32 => CsType::Blittable(format!("float")),
        DataType::F64 => CsType::Blittable(format!("double")),
        DataType::String => CsType::Mapped {
            facade: "string".to_string(),
            facade_generic: "string".to_string(),
            blittable: "NativeString".to_string(),
        },
        DataType::ClassName => type_cs(&DataType::String),
        DataType::Vec(it) => CsType::templated(
            // TODO there is two options to design it;
            // 1. return iterator, that also contains seem hash to check the rust-side arena-allocated collection is alive every iteration
            // 2. add "fetch" method that recursively called on every returned value and fetch collection inside
            // 3. consider analog that accepts managed array pointer as argument and returns number of added items
            // combination of 2+3 seems Unity way.
            "List<{}>",
            "{}_slice",
            &type_cs(it.deref()),
        ),
        DataType::UserScript => CsType::Mapped {
            facade: "object".to_string(),
            facade_generic: "T".to_string(),
            blittable: "UserScript".to_string(),
        },
        DataType::UserScriptMessage => CsType::Mapped {
            facade: "object".to_string(),
            facade_generic: "T".to_string(),
            blittable: "UserScriptMessage".to_string(),
        },
        DataType::UserScriptGenericStub => panic!("WTF, UserScriptGenericStub should be filtered out"),
        DataType::Object(it) => {
            if is_implemented_externally(it) {
                return CsType::templated("{}", "Native{}", &CsType::Blittable(it.to_string()))
            }
            CsType::Blittable(it.to_string())
        },
        DataType::Option(it) => CsType::templated(
            "{}?",
            "{}_optional",
            &type_cs(it.deref()),
        ),
        // err will throw exception
        DataType::Result { ok, .. } => CsType::templated(
            "{}",
            "{}_result",
            &type_cs(ok.deref()),
        ),
    }
}

pub fn is_implemented_externally(class_name: &ClassName) -> bool {
    class_name.0 == "Vector3" || class_name.0 == "Vector2" || class_name.0 == "Quaternion"
}

pub fn type_rs(ty: &DataType, ctx: &GenerationContext) -> RsType {
    match ty {
        DataType::UnresolvedClass(it) => panic!("Unresolved class: {}", it),
        DataType::Unit => RsType::Basic(format!("())")),
        DataType::Bool => RsType::Basic(format!("bool")),
        DataType::Byte => RsType::Basic(format!("u8")),
        DataType::I32 => RsType::Basic(format!("i32")),
        DataType::I64 => RsType::Basic(format!("i64")),
        DataType::F32 => RsType::Basic(format!("f32")),
        DataType::F64 => RsType::Basic(format!("f64")),
        DataType::String => RsType::Mapped {native: "NativeString".to_string(), lite: "String".to_string()},
        DataType::ClassName => type_rs(&DataType::String, ctx),
        DataType::Vec(it) => RsType::templated(
            // TODO there is two options to design it;
            // 1. return iterator, that also contains seem hash to check the rust-side arena-allocated collection is alive every iteration
            // 2. add "fetch" method that recursively called on every returned value and fetch collection inside
            // 3. consider analog that accepts managed array pointer as argument and returns number of added items
            // combination of 2+3 seems Unity way.
            "Vec<{}>",
            "{}_slice",
            &type_rs(it.deref(), ctx),
        ),
        DataType::UserScript => RsType::Mapped {
            lite: "crate::UserScriptImpl".to_string(),
            native: "NativeHandle".to_string(),
        },
        DataType::UserScriptMessage => RsType::Basic("UserScriptMessage".to_string()),
        DataType::UserScriptGenericStub => panic!("WTF, UserScriptGenericStub should be filtered out"),
        DataType::Object(it) => RsType::Mapped {
            lite: ctx.domain.get_class(it).unwrap().rust_name().to_string(),
            native: format!("Native{}", it),
        },
        DataType::Option(it) => RsType::templated(
            "Option<{}>",
            "{}_optional",
            &type_rs(it.deref(), ctx),
        ),
        // err will throw exception
        DataType::Result { ok, .. } => RsType::templated(
            "Result<{}, String>",
            "{}_result",
            &type_rs(ok.deref(), ctx),
        ),
    }
}

pub enum CsType {
    Blittable(String),
    Mapped {
        facade: String,
        facade_generic: String,
        blittable: String,
    },
}
pub enum RsType {
    Basic(String),
    Mapped {
        lite: String,
        native: String,
    },
}

impl RsType {
    pub fn templated(lite_template: &str, native_template: &str, base_type: &RsType) -> Self {
        Self::Mapped {
            lite: lite_template.replace("{}", &base_type.to_lite()),
            native: native_template.replace("{}", &base_type.to_native()),
        }
    }

    pub fn to_lite(&self) -> String {
        match self {
            RsType::Basic(it) => it.to_string(),
            RsType::Mapped { lite, .. } => lite.to_string(),
        }
    }

    pub fn to_native(&self) -> String {
        match self {
            RsType::Basic(it) => it.to_string(),
            RsType::Mapped { native, .. } => native.to_string(),
        }
    }
}

impl CsType {
    pub fn templated(facade_template: &str, blittable_template: &str, base_type: &CsType) -> Self {
        Self::Mapped {
            facade: facade_template.replace("{}", &base_type.to_facade()),
            facade_generic: facade_template.replace("{}", &base_type.to_facade_generic()),
            blittable: blittable_template.replace("{}", &base_type.to_blittable()),
        }
    }

    pub fn to_blittable(&self) -> String {
        match self {
            CsType::Blittable(it) => it.to_string(),
            CsType::Mapped { blittable, .. } => blittable.to_string(),
        }
    }

    pub fn is_mapped(&self) -> bool {
        match self {
            CsType::Blittable(_) => false,
            CsType::Mapped { .. } => true,
        }
    }

    pub fn to_facade(&self) -> String {
        match self {
            CsType::Blittable(it) => it.to_string(),
            CsType::Mapped { facade, .. } => facade.to_string(),
        }
    }

    pub fn to_facade_generic(&self) -> String {
        match self {
            CsType::Blittable(it) => it.to_string(),
            CsType::Mapped { facade_generic, .. } => facade_generic.to_string(),
        }
    }
}
