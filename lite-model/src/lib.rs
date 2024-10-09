pub mod impls;

use serde::{Deserialize, Serialize};
use impls::is_false;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Domain {
    pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Class {
    Engine(EngineClass),
    Struct(StructClass),
    Enum(EnumClass),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EngineClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent: Option<ClassName>,

    pub class_name: ClassName,

    pub rust_struct_path: RustQualifiedName,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub methods: Vec<Method>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub constants: Vec<Constant>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    Eq
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Method {
    pub instance: bool,
    pub method_name: String,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Field {
    pub name: String,

    pub ty: DataType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Param {
    pub name: String,

    pub ty: DataType,

    #[serde(skip_serializing_if = "is_false")]
    pub variadic: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Constant {
    pub const_name: String,
    pub ty: DataType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct StructClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent: Option<ClassName>,

    pub class_name: ClassName,

    pub rust_struct_path: RustQualifiedName,

    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct ClassName(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct RustQualifiedName(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EnumClass {
    pub class_name: ClassName,

    pub rust_struct_path: RustQualifiedName,

    pub variants: Vec<EnumVariant>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EnumVariant {
    pub tag: String,

    #[serde(skip_serializing_if = "EnumValue::is_unit")]
    #[serde(default = "EnumValue::unit")]
    pub value: EnumValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum EnumValue {
    Unit,
    Tuple { fields: Vec<DataType> },
    Struct { fields: Vec<Field> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Signature {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub params: Vec<Param>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub return_ty: Option<DataType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    UnresolvedClass(String),
    Unit,
    Bool,
    Byte,
    I32,
    I64,
    F32,
    F64,
    String,
    ClassName,
    Vec(Box<DataType>),
    /// trait in Lite, manual implementation on language binding side
    UserScript,
    /// fixed type (unknown for Lite) for send_hierarchical/subscribe_to/on_message (we can't make it generic outside of Rust)
    UserScriptMessage,
    /// stub argument to call UserScript-type parameterized methods
    UserScriptGenericStub,
    Object(ClassName),
    Option(Box<DataType>),
    /// Error should be universal scripting language specific type, so it is not presented here
    Result {
        ok: Box<DataType>,
    },
}
