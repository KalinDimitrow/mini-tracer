use nalgebra::{Vector2, Point3, Vector3, Unit};

pub struct IntersectionInfo {
    pub point : Point3<f32>,
    pub normal : Unit<Vector3<f32>>,
    pub distance : f32,
    pub uv : Vector2<f32>,
}

impl IntersectionInfo {
    pub fn compare(&mut self, other : &Option<Self>) -> bool {
        if let Some(other) = other {
            if self.distance < other.distance {
                true
            } else {
                false
            }
        } else {
            true
        }
    }
}