use nalgebra::Point3;
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::Geometry;
use crate::scene::geometries::plane::Plane;
pub struct Rectangle {
    pub plane : Plane,
    pub size : (f32, f32)
}

impl Rectangle {
    pub fn new(position : Point3<f32>, roll : f32, pitch : f32, yaw : f32, size : (f32, f32)) -> Self {
        Rectangle{ plane : Plane::new(position, roll, pitch, yaw), size}
    }
}

impl Geometry for Rectangle {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        let intersection = self.plane.intersect(ray);
        if let Some(intersection) = intersection {
            if -self.size.0*0.5f32 < intersection.u && 
                intersection.u < self.size.0*0.5f32 &&
                -self.size.1*0.5f32 < intersection.v && 
                intersection.v < self.size.1*0.5f32 {
                    return Some(intersection);
            } 
        }

        None
    }
}