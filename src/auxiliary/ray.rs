use nalgebra::{Point3, Vector3, Matrix3, Rotation3};

pub struct Ray {
    pub start : Point3<f32>,
    pub direction : Vector3<f32>
}