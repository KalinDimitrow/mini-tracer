use nalgebra::{Point3, Vector3, Unit};

pub struct IntersectionInfo {
    pub point : Point3<f32>,
    pub normal : Unit<Vector3<f32>>,
    pub distance : f32,
    pub u : f32,
    pub v : f32,
}

impl IntersectionInfo {
    pub fn compare(&mut self, other : &Option<Self>) -> bool {
        if let Some(other) = other {
            if self.distance < other.distance {
                return true;
            }
        } else {
            return true;
        }

        false
    }
}