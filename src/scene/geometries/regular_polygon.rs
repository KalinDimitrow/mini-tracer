use nalgebra::{Point3, Vector3, Rotation3};
use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::Geometry;
use crate::scene::geometries::plane::Plane;

pub struct RegularPolygon {
    plane: Plane,
    size : f32,
    sides: u32,
}

impl RegularPolygon {
    pub fn new(position : Point3<f32>, roll : f32, pitch : f32, yaw : f32, size : f32) -> Self {
        Self {
            plane: Plane::new(position, roll, pitch, yaw),
            size,
            sides: 5,
        }
    }
}

impl Geometry for RegularPolygon {
    fn intersect(&self, ray : &Ray) ->Option<IntersectionInfo> {
        if let Some(info) = self.plane.intersect(ray) {

            return Some(info);
        }

        None
    }
}