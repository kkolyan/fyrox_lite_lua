
use std::fmt::{Debug, Display};

use fyrox_core::{
    algebra::{Unit, UnitQuaternion, Vector3},
    num_traits::Zero,
};
use fyrox_lite::lite_math::{PodQuaternion, PodVector3};
use fyrox_lite_macro::fyrox_lite;

use std::ops::Mul;

use crate::vec::LiteVector3;

#[derive(Clone, Copy)]
pub struct LiteQuaternion(UnitQuaternion<f32>);

#[fyrox_lite(Quaternion)]
impl LiteQuaternion {
    pub fn face_towards(dir: LiteVector3, up: LiteVector3) -> LiteQuaternion {
        LiteQuaternion(UnitQuaternion::face_towards(&dir.into(), &up.into()))
    }
    pub fn from_axis_angle(axis: LiteVector3, angle: f32) -> LiteQuaternion {
        LiteQuaternion(UnitQuaternion::from_axis_angle(
            &Unit::new_normalize(Vector3::from(axis)),
            angle,
        ))
    }

    #[allow(non_snake_case)]
    pub fn mul__LiteVector(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0.mul(&o.0))
    }

    #[allow(non_snake_case)]
    pub fn mul__LiteQuaternion(&self, rot_delta: LiteQuaternion) -> LiteQuaternion {
        LiteQuaternion(self.0.mul(&rot_delta.0))
    }
}

impl Debug for LiteQuaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl From<LiteQuaternion> for UnitQuaternion<f32> {
    fn from(value: LiteQuaternion) -> Self {
        value.0
    }
}

impl From<UnitQuaternion<f32>> for LiteQuaternion {
    fn from(value: UnitQuaternion<f32>) -> Self {
        LiteQuaternion(value)
    }
}


impl From<LiteQuaternion> for PodQuaternion {
    fn from(v: LiteQuaternion) -> Self {
        UnitQuaternion::<f32>::from(v).into()
    }
}

impl From<PodQuaternion> for LiteQuaternion {
    fn from(v: PodQuaternion) -> Self {
        UnitQuaternion::<f32>::from(v).into()
    }
}
