use nalgebra::{Point3, Vector3};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::{Geometry};

pub struct Plane {
    position : Point3<f32>,
    normal : Vector3<f32>
}

impl Plane {
    pub fn new( position : Point3<f32>, normal : Vector3<f32>) -> Self {
        Plane {position, normal}
    }
}

impl Geometry for Plane {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        // plane can be described by point (p0) belonging to the plane and normal vector
        // ray can be describe by starting point (l0) and
        let denominator = ray.direction.dot(&self.normal);
        if denominator < 1e-6 {
            let p0l0 = &self.position - &ray.start;
            let t = p0l0.dot(&self.normal) / denominator;


            if t > 0.0 {
                let point = &ray.start + &ray.direction*t;
                let normal = self.normal.clone();
                let offset = point - self.position;
                return Some(IntersectionInfo{point, normal, distance : t, u : offset.x, v : offset.z})
            }
            return None;
        }

        None
    }
}