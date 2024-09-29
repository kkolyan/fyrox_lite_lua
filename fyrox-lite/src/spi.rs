

// pub trait Language: Sized {
//     type Vector3: Vector3Trait;
//     type Quaternion: QuaternionTrait;
// }

use fyrox_lite_macro::fyrox_lite_user_class;

pub trait DynamicArray<T: LiteDataType> {
	fn add(&mut self, item: T);
	fn sort(&mut self, cmp: &mut dyn FnMut(&T, &T) -> LiteOrdering);
	fn clear(&mut self);
}

pub enum LiteOrdering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}


/// implemented only by the types from `fyrox_lite_model::DataType` (mostly by proc macros)
pub trait LiteDataType {
	fn compiles_if_type_is_allowed() {}
}

impl <T> LiteDataType for &mut dyn DynamicArray<T> {}
impl <T> LiteDataType for Vec<T> {}
impl <T> LiteDataType for Option<T> {}

impl LiteDataType for String {}
impl LiteDataType for i32 {}
impl LiteDataType for i64 {}
impl LiteDataType for f32 {}
impl LiteDataType for f64 {}
impl LiteDataType for bool {}
