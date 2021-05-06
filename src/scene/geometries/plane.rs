use nalgebra::{Point3, Vector3, Rotation3, Unit};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::{Geometry};

pub struct Plane {
    position : Point3<f32>,
    normal : Unit<Vector3<f32>>,
    rotation : f32
}

impl Plane {
    pub fn new( position : Point3<f32>, normal : Unit<Vector3<f32>>, rotation : f32) -> Self {
        Plane {position, normal, rotation}
    }
}

impl Geometry for Plane {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        // plane can be described by point (P0) belonging to the plane and normal vector N
        // ray can be describe by starting point (L0) and direction vector S (P - L0)/|P - L0| where P is the intersection point
        // P = L0 + t*S where t is unkown parameter
        // (P - P0) should be ortogonal to N therefor (P - P0)*N = 0 therefore (L0 + t*S - P0)*N = 0
        // therefore t = (P0 - L0)*N/S*N
        let denominator = ray.direction.dot(&self.normal);
        if denominator < 1e-6 {
            let p0l0 = &self.position - &ray.start;
            let t = p0l0.dot(&self.normal) / denominator;


            if t > 0.0 {
                let point = &ray.start + &ray.direction*t;
                let normal = self.normal.clone();
                let offset = point - self.position;
                let rotation = Rotation3::from_axis_angle(&self.normal, self.rotation);
                let offset = rotation*offset;
                return Some(IntersectionInfo{point, normal, distance : t, u : offset.x, v : offset.z})
            }
            return None;
        }

        None
    }
}