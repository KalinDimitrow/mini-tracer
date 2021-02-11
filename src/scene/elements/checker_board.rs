use nalgebra::{Point3, Vector3, Matrix3, Rotation3};
use crate::auxiliary::{ray::Ray, color::Color, intersection_info::IntersectionInfo};
use crate::scene::elements::scene_element::{SceneElement, Geometry, Material};
use crate::scene::geometries::plane::Plane;
use crate::scene::materials::checker_board_material::CheckerMaterial;

pub struct CheckerBoard {
    plane : Plane,
    material : CheckerMaterial,
}

impl CheckerBoard {
    pub fn new() -> Self {
        let plane = Plane::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let material = CheckerMaterial::new(Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0), 1.0);
        Self {plane, material}
    }
}

impl SceneElement for CheckerBoard {
    fn geometry(&self)-> &Geometry {
        &self.plane
    }

    fn material(&self) -> &Material {
        &self.material
    }
}