use fyrox::core::algebra::{Quaternion, UnitQuaternion, Vector2, Vector3};
use lite_macro::lite_api;

#[lite_api(Vector3)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PodVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[lite_api(Vector2)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PodVector2 {
    pub x: f32,
    pub y: f32,
}

#[lite_api(Vector2i)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PodVector2i {
    pub x: i64,
    pub y: i64,
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

impl From<Vector2<f32>> for PodVector2 {
    fn from(v: Vector2<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
        }
    }
}

impl From<PodVector2> for Vector2<f32> {
    fn from(v: PodVector2) -> Self {
        Self::new(v.x, v.y)
    }
}

#[lite_api(Quaternion)]
#[derive(Debug, Clone, Copy, PartialEq)]
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
