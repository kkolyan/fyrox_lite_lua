use std::fmt::{Debug, Display};

use fyrox::core::{
    algebra::{Rotation3, Unit, UnitQuaternion, Vector3},
    num_traits::Zero,
};

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

impl LiteVector3 {
    #[rustfmt::skip]    pub fn get_x(&self) -> f32 { self.0.x }
    #[rustfmt::skip]    pub fn get_y(&self) -> f32 { self.0.y }
    #[rustfmt::skip]    pub fn get_z(&self) -> f32 { self.0.z }
    #[rustfmt::skip]    pub fn set_x(&mut self, value: f32) { self.0.x = value; }
    #[rustfmt::skip]    pub fn set_y(&mut self, value: f32) { self.0.y = value; }
    #[rustfmt::skip]    pub fn set_z(&mut self, value: f32) { self.0.z = value; }

    #[rustfmt::skip]     pub fn x_axis() -> Self { Vector3::x_axis().into_inner().into() }
    #[rustfmt::skip]     pub fn y_axis() -> Self { Vector3::y_axis().into_inner().into() }
    #[rustfmt::skip]     pub fn z_axis() -> Self { Vector3::z_axis().into_inner().into() }
	
    pub fn zero() -> Self {
        Vector3::zero().into()
    }
    pub fn new(x: f32, y: f32, z: f32) -> LiteVector3 {
        LiteVector3(Vector3::new(x, y, z))
    }
    pub fn mul__f32(&self, o: f32) -> LiteVector3 {
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

#[derive(Clone, Copy)]
pub struct LiteUnitQuaternion(UnitQuaternion<f32>);

impl LiteUnitQuaternion {
    pub fn face_towards(dir: LiteVector3, up: LiteVector3) -> LiteUnitQuaternion {
        LiteUnitQuaternion(UnitQuaternion::face_towards(&dir.into(), &up.into()))
    }
    pub fn from_axis_angle(axis: LiteVector3, angle: f32) -> LiteUnitQuaternion {
        LiteUnitQuaternion(UnitQuaternion::from_axis_angle(
            &Unit::new_normalize(Vector3::from(axis)),
            angle,
        ))
    }

    pub fn transform_vector(&self, o: LiteVector3) -> LiteVector3 {
        LiteVector3(self.0.transform_vector(&o.0))
    }
	
	pub fn mul(&self, rot_delta: Rotation3<f32>) -> LiteUnitQuaternion {
		LiteUnitQuaternion(self.0.mul(&rot_delta))
	}
}

impl From<LiteUnitQuaternion> for UnitQuaternion<f32> {
    fn from(value: LiteUnitQuaternion) -> Self {
        value.0
    }
}

impl From<UnitQuaternion<f32>> for LiteUnitQuaternion {
    fn from(value: UnitQuaternion<f32>) -> Self {
        LiteUnitQuaternion(value)
    }
}
