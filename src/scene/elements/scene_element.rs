use crate::auxiliary::{ray::Ray, intersection_info::IntersectionInfo};
use crate::auxiliary::color::Color;
use crate::scene::light::Light;

pub trait Geometry {
    fn intersect(&self, _ray : &Ray) -> Option<IntersectionInfo> {
        None
    }
}

pub trait Material {
    fn color(&self, _ray : &Ray, _intersection_info : &IntersectionInfo, _lights : &[Light]) -> Color {
        Color {r : 0.0, g : 0.0, b : 0.0}
    }
}

pub trait SceneElement {
    fn geometry(&self) -> &'_ dyn Geometry;
    fn material(&self) -> &'_ dyn Material;
}