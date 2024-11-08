use fyrox::core::algebra::UnitQuaternion;

fn main() {
    let a: UnitQuaternion<f32> = UnitQuaternion::identity();
    println!("{:?}", a);
}