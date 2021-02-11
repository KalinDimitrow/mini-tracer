use nalgebra::{Point3, Vector3, Matrix3, Rotation3};
use crate::auxiliary::color::Color;

pub struct Light {
    pub location : Point3<f32>,
    pub color : Color,
    pub intensity : f32,
}

impl Light {
    pub fn new( location : Point3<f32>, color : Color, intensity : f32) -> Light {
        Light {location, color, intensity}
    }
}