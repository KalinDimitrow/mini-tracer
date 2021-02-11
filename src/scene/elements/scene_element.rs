use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::auxiliary::color::Color;
use crate::scene::light::Light;

pub trait Geometry {
    fn intersect(&self, ray : &Ray) -> Option<IntersectionInfo> {
        None
    }
}

pub trait Material {
    fn color(&self, ray : &Ray, intersection_info : &IntersectionInfo, lights : &[Light]) -> Color {
        Color {r : 0.0, g : 0.0, b : 0.0}
    }
}

pub trait SceneElement {
    // fn intersect(&self, ray : &Ray) -> Option<IntersectionInfo> {
    //     None
    // }
    fn geometry(&self) -> &'_ Geometry;
    fn material(&self) -> &'_ Material;
}