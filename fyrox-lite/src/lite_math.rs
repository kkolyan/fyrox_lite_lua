use fyrox::core::algebra::{Quaternion, UnitQuaternion, Vector3};
use fyrox_lite_macro::fyrox_lite_pod;

#[fyrox_lite_pod("Vector3")]
#[derive(Copy, PartialEq)]
pub struct PodVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector3<f32>> for PodVector3 {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<PodVector3> for Vector3<f32> {
    fn from(v: PodVector3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[fyrox_lite_pod("Quaternion")]
#[derive(Copy, PartialEq)]
pub struct PodQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<UnitQuaternion<f32>> for PodQuaternion {
    fn from(v: UnitQuaternion<f32>) -> Self {
        Self {
            x: v.i,
            y: v.j,
            z: v.k,
            w: v.w,
        }
    }
}

impl From<PodQuaternion> for UnitQuaternion<f32> {
    fn from(v: PodQuaternion) -> Self {
        UnitQuaternion::from_quaternion(Quaternion::new(v.w, v.x, v.y, v.z))
    }
}
