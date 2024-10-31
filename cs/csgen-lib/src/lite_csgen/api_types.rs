use std::ops::Deref;
use lite_model::DataType;

pub fn type_rs2cs(ty: &DataType) -> TypeMarshalling {
    match ty {
        DataType::UnresolvedClass(it) => panic!("Unresolved class: {}", it),
        DataType::Unit => TypeMarshalling::Blittable(format!("void")),
        DataType::Bool => TypeMarshalling::Blittable(format!("bool")),
        DataType::Byte => TypeMarshalling::Blittable(format!("byte")),
        DataType::I32 => TypeMarshalling::Blittable(format!("int")),
        DataType::I64 => TypeMarshalling::Blittable(format!("long")),
        DataType::F32 => TypeMarshalling::Blittable(format!("float")),
        DataType::F64 => TypeMarshalling::Blittable(format!("dobule")),
        DataType::String => TypeMarshalling::Blittable(format!("string")),
        DataType::ClassName => TypeMarshalling::Blittable(format!("string")),
        DataType::Vec(it) => TypeMarshalling::templated(
            // TODO there is two options to design it;
            // 1. return iterator, that also contains seem hash to check the rust-side arena-allocated collection is alive every iteration
            // 2. add "fetch" method that recursively called on every returned value and fetch collection inside
            // 3. consider analog that accepts managed array pointer as argument and returns number of added items
            // combination of 2+3 seems Unity way.
            "List<{}>",
            "{}_slice",
            &type_rs2cs(it.deref()),
        ),
        DataType::UserScript => TypeMarshalling::Mapped {
            facade: "object".to_string(),
            facade_generic: "T".to_string(),
            blittable: "UserScript".to_string(),
        },
        DataType::UserScriptMessage => TypeMarshalling::Mapped {
            facade: "object".to_string(),
            facade_generic: "T".to_string(),
            blittable: "UserScriptMessage".to_string(),
        },
        DataType::UserScriptGenericStub => panic!("WTF, UserScriptGenericStub should be filtered out"),
        DataType::Buffer(it) => TypeMarshalling::templated(
            "List<{}>",
            "{}_slice",
            &type_rs2cs(it.deref()),
        ),
        DataType::Object(it) => TypeMarshalling::Blittable(it.to_string()),
        DataType::Option(it) => TypeMarshalling::templated(
            "{}?",
            "{}_optional",
            &type_rs2cs(it.deref()),
        ),
        // err will throw exception
        DataType::Result { ok, .. } => TypeMarshalling::templated(
            "{}",
            "{}_result",
            &type_rs2cs(ok.deref()),
        ),
    }
}

pub enum TypeMarshalling {
    Blittable(String),
    Mapped {
        facade: String,
        facade_generic: String,
        blittable: String,
    },
}
// pub fn conversion_to_blittable(ty: &DataType, var: &str) -> String {
//     match ty {
//         DataType::Vec(_) => render_string(r#""#, []),
//         DataType::UserScript => render_string(r#""#, []),
//         DataType::UserScriptMessage => render_string(r#""#, []),
//         DataType::Option(_) => render_string(r#""#, []),
//         DataType::Result { .. } => render_string(r#""#, []),
//         it => panic!("unexpected for {:?}", it)
//     }
// }
// pub fn conversion_to_facade(ty: &DataType, var: &str) -> String {
//     match ty {
//         DataType::Vec(_) => render_string(r#""#, []),
//         DataType::UserScript => render_string(r#""#, []),
//         DataType::UserScriptMessage => render_string(r#""#, []),
//         DataType::Option(_) => render_string(r#""#, []),
//         DataType::Result { .. } => render_string(r#""#, []),
//         it => panic!("unexpected for {:?}", it)
//     }
// }

impl TypeMarshalling {
    pub fn templated(facade_template: &str, blittable_template: &str, base_type: &TypeMarshalling) -> Self {
        Self::Mapped {
            facade: facade_template.replace("{}", &base_type.to_facade()),
            facade_generic: facade_template.replace("{}", &base_type.to_facade_generic()),
            blittable: blittable_template.replace("{}", &base_type.to_blittable()),
        }
    }

    pub fn to_blittable(&self) -> String {
        match self {
            TypeMarshalling::Blittable(it) => it.to_string(),
            TypeMarshalling::Mapped { blittable, .. } => blittable.to_string(),
        }
    }

    pub fn is_mapped(&self) -> bool {
        match self {
            TypeMarshalling::Blittable(_) => false,
            TypeMarshalling::Mapped { .. } => true,
        }
    }

    pub fn to_facade(&self) -> String {
        match self {
            TypeMarshalling::Blittable(it) => it.to_string(),
            TypeMarshalling::Mapped { facade, .. } => facade.to_string(),
        }
    }

    pub fn to_facade_generic(&self) -> String {
        match self {
            TypeMarshalling::Blittable(it) => it.to_string(),
            TypeMarshalling::Mapped { facade_generic, .. } => facade_generic.to_string(),
        }
    }
}