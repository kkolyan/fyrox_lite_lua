
use std::fmt::{Debug, Display};

use fyrox_core::{
    algebra::{Unit, UnitQuaternion, Vector3},
    num_traits::Zero,
};
use fyrox_lite::lite_math::{PodQuaternion, PodVector3};
use fyrox_lite_macro::fyrox_lite;
use fyrox_lite::LiteDataType;

use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct LiteVector3(pub Vector3<f32>);

impl Debug for LiteVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for LiteVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(non_snake_case)]
#[fyrox_lite(Vector3)]
impl LiteVector3 {
    #[rustfmt::skip]    pub fn get_x(&self) -> f32 { self.0.x }
    #[rustfmt::skip]    pub fn get_y(&self) -> f32 { self.0.y }
    #[rustfmt::skip]    pub fn get_z(&self) -> f32 { self.0.z }
    #[rustfmt::skip]    pub fn set_x(&mut self, value: f32) { self.0.x = value; }
    #[rustfmt::skip]    pub fn set_y(&mut self, value: f32) { self.0.y = value; }
    #[rustfmt::skip]    pub fn set_z(&mut self, value: f32) { self.0.z = value; }

    #[rustfmt::skip]    pub fn get_X() -> LiteVector3 { Vector3::x_axis().into_inner().into() }
    #[rustfmt::skip]    pub fn get_Y() -> LiteVector3 { Vector3::y_axis().into_inner().into() }
    #[rustfmt::skip]    pub fn get_Z() -> LiteVector3 { Vector3::z_axis().into_inner().into() }

    pub fn zero() -> LiteVector3 {
        Vector3::zero().into()
    }
    pub fn new(x: f32, y: f32, z: f32) -> LiteVector3 {
        LiteVector3(Vector3::new(x, y, z))
    }
    pub fn mul(&self, o: f32) -> LiteVector3 {
        LiteVector3(self.0 * o)
    }

    pub fn add(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0 + o.0)
    }

    pub fn normalize(&self) -> LiteVector3 {
        LiteVector3(self.0.normalize())
    }

    pub fn sub(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0 - o.0)
    }

    pub fn magnitude(&self) -> f32 {
        self.0.magnitude()
    }

    pub fn normalize_inplace(&mut self) {
        self.0.normalize_mut();
    }
}

impl From<Vector3<f32>> for LiteVector3 {
    fn from(value: Vector3<f32>) -> Self {
        LiteVector3(value)
    }
}

impl From<LiteVector3> for Vector3<f32> {
    fn from(value: LiteVector3) -> Self {
        value.0
    }
}

impl From<LiteVector3> for PodVector3 {
    fn from(v: LiteVector3) -> Self {
        Vector3::<f32>::from(v).into()
    }
}

impl From<PodVector3> for LiteVector3 {
    fn from(v: PodVector3) -> Self {
        Vector3::<f32>::from(v).into()
    }
}
