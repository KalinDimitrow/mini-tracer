use nalgebra::{Rotation3, Vector3};

pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> Rotation3<f32> {
    let x = Vector3::x_axis();
    let y = Vector3::y_axis();
    let z = Vector3::z_axis();
    let a = Rotation3::from_axis_angle(&x, pitch);
    let b = Rotation3::from_axis_angle(&y, yaw);
    let c = Rotation3::from_axis_angle(&z, roll);

    b*a*c
}