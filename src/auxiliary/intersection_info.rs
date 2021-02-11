use nalgebra::{Point3, Vector3, Matrix3, Rotation3};

pub struct IntersectionInfo {
    pub point : Point3<f32>,
    pub normal : Vector3<f32>,
    pub distance : f32,
    pub u : f32,
    pub v : f32,
}

impl IntersectionInfo {
    pub fn new() -> IntersectionInfo {
        IntersectionInfo {
            point : Point3::<f32>::new(0.0, 0.0, 0.0),
            normal : Vector3::<f32>::new(0.0, 0.0, 0.0),
            distance : std::f32::INFINITY,
            u : 0.0,
            v : 0.0
        }
    }

    pub fn compare(&mut self, other : Self) -> bool {
        if (self.distance > other.distance) {
            *self = other;
            return true;
        }

        false
    }
}