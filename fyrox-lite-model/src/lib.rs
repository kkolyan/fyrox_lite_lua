pub mod raw;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Domain {
    pub engine_classes: Vec<EngineClass>,
    pub pod_classes: Vec<PodClass>,
    pub enum_classes: Vec<EnumClass>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct EngineClassName(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct EngineClass {
    pub parent: Option<EngineClassName>,
    pub class_name: EngineClassName,
    pub methods: Vec<Method>,
    pub constants: Vec<Constant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Method {
    pub instance: bool,
    pub method_name: String,
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub field_name: String,
    pub ty: DataType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Constant {
    pub const_name: String,
    pub ty: DataType,
    pub value: ConstantValue,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct PodClassName(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct PodClass {
    pub parent: Option<PodClassName>,
    pub class_name: PodClassName,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct EnumClassName(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumClass {
    pub class_name: EnumClassName,
    pub variants: Vec<(String, EnumVariant)>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EnumVariant {
    Unit,
    Tuple {
        fields: Vec<DataType>
    },
    Struct {
        fields: Vec<Field>
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    pub params: Vec<DataType>,
    pub return_ty: Option<DataType>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataType {
    UnresolvedClass(String),
    Bool,
    Byte,
    I32,
    I64,
    F32,
    F64,
    String,
    Vec(Box<DataType>),
    /// trait in Lite, manual implementation on language binding side
    UserScript,
    /// fixed type (unknown for Lite) for send_hierarchical/subscribe_to/on_message (we can't make it generic outside of Rust)
    UserScriptMessage,
    Pod(PodClassName),
    EngineObject(EngineClassName),
    Option(Box<DataType>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConstantValue {
    Bool(bool),
    Integer(i32),
    Float(f32),
    String(String),
    Reference {
        owner: EngineClassName,
        constant_name: String,
    },
    BinaryOp {
        op: BinaryOp,
        left: Box<ConstantValue>,
        right: Box<ConstantValue>,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitXor,
    BitAnd,
    BitOr,
    Shl,
    Shr,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}
