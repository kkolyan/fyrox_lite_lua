

// pub trait Language: Sized {
//     type Vector3: Vector3Trait;
//     type Quaternion: QuaternionTrait;
// }

use std::{any::Any, fmt::Debug};

use fyrox::script::{ScriptMessagePayload, ScriptTrait};


pub trait UserScript : Sized + LiteDataType {
	type ProxyScript: ScriptTrait;
	type LangSpecificError;
	type UserScriptMessage : ScriptMessagePayload + LiteDataType;
	type UserScriptGenericStub: LiteDataType;

	fn extract_from(proxy: &Self::ProxyScript, class_name: &str) -> Option<Self>;

	fn into_proxy_script(self) -> Result<Self::ProxyScript, Self::LangSpecificError>;
}

/// implemented only by the types from `fyrox_lite_model::DataType` (mostly by proc macros)
pub trait LiteDataType {
	fn compiles_if_type_is_allowed() {}
}

impl <T: LiteDataType> LiteDataType for Vec<T> {}
impl <T: LiteDataType> LiteDataType for Option<T> {}
impl <T: LiteDataType, E> LiteDataType for Result<T, E> {}

impl LiteDataType for String {}
impl LiteDataType for u8 {}
impl LiteDataType for i32 {}
impl LiteDataType for i64 {}
impl LiteDataType for f32 {}
impl LiteDataType for f64 {}
impl LiteDataType for bool {}
impl LiteDataType for () {}
