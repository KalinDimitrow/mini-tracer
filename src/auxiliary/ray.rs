use nalgebra::{Point3, Unit, Vector3};

pub struct Ray {
    pub start : Point3<f32>,
    pub direction : Unit<Vector3<f32>>,
}