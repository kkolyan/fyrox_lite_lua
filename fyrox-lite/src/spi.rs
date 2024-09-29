

// pub trait Language: Sized {
//     type Vector3: Vector3Trait;
//     type Quaternion: QuaternionTrait;
// }

use std::any::Any;

use fyrox_lite_macro::{fyrox_lite_pod, fyrox_lite_user_class};


pub trait UserScript : LiteDataType {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait UserScriptMessage: LiteDataType {
	fn as_any(&self) -> &dyn Any;
}

/// implemented only by the types from `fyrox_lite_model::DataType` (mostly by proc macros)
pub trait LiteDataType {
	fn compiles_if_type_is_allowed() {}
}

impl <T> LiteDataType for Vec<T> {}
impl <T> LiteDataType for Option<T> {}

impl LiteDataType for String {}
impl LiteDataType for u8 {}
impl LiteDataType for i32 {}
impl LiteDataType for i64 {}
impl LiteDataType for f32 {}
impl LiteDataType for f64 {}
impl LiteDataType for bool {}
