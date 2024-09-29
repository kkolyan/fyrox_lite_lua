use std::fmt::Debug;

use fyrox::core::algebra::UnitQuaternion;
use fyrox_lite_macro::{fyrox_lite_pod, fyrox_lite_user_class};

// #[fyrox_lite_user_class(Vector3)]
// pub trait Vector3Trait: Clone + Debug {
//     fn x(&self) -> f32;
//     fn y(&self) -> f32;
//     fn z(&self) -> f32;
//     fn set_x(&self, v: f32);
//     fn set_y(&self, v: f32);
//     fn set_z(&self, v: f32);
//     fn axes(&self) -> [f32; 3];
//     fn from_axes(v: [f32; 3]) -> Self;
// }


// #[fyrox_lite_user_class(Quaternion)]
// pub trait QuaternionTrait: Clone + Debug {
//     fn to_engine(&self) -> UnitQuaternion<f32>;
//     fn from_engine(v: UnitQuaternion<f32>) -> Self;
// }

// #[fyrox_lite_pod]
// pub struct LiteRayCastOptionsTrait {}
