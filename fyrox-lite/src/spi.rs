

// pub trait Language: Sized {
//     type Vector3: Vector3Trait;
//     type Quaternion: QuaternionTrait;
// }

use std::{any::Any, fmt::Debug};

use fyrox::script::{ScriptMessagePayload, ScriptTrait};
use fyrox_lite_macro::{fyrox_lite_pod, fyrox_lite_user_class};


pub trait ProxyScript : LiteDataType + ScriptTrait {
	// type UserScriptType: UserScript<ProxyScriptType=Self>;
	// fn as_any(&self) -> &dyn Any;
	// fn as_any_mut(&mut self) -> &mut dyn Any;
	// fn as_instance_of(&self, class_name: &str) -> Option<Self::UserScriptType>;
}

pub trait UserScript : Sized + LiteDataType {
	type ProxyScriptType: ProxyScript;

	fn extract_from(proxy: &Self::ProxyScriptType, class_name: &str) -> Option<Self>;

	fn into_proxy_script(self) -> Self::ProxyScriptType;
}

pub trait UserScriptMessage: ScriptMessagePayload + LiteDataType {
}

/// implemented only by the types from `fyrox_lite_model::DataType` (mostly by proc macros)
pub trait LiteDataType {
	fn compiles_if_type_is_allowed() {}
}

impl <T: LiteDataType> LiteDataType for Vec<T> {}
impl <T: LiteDataType> LiteDataType for Option<T> {}

impl LiteDataType for String {}
impl LiteDataType for u8 {}
impl LiteDataType for i32 {}
impl LiteDataType for i64 {}
impl LiteDataType for f32 {}
impl LiteDataType for f64 {}
impl LiteDataType for bool {}
