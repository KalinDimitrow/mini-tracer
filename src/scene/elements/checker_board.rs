use nalgebra::{Point3, Vector3, Unit};
use crate::auxiliary::{color::Color};
use crate::scene::{
    elements::scene_element::{SceneElement, Geometry, Material},
    materials::checker_board_material::CheckerMaterial,
    geometries::plane::Plane
};

pub struct CheckerBoard {
    plane : Plane,
    material : CheckerMaterial,
}

impl CheckerBoard {
    pub fn new() -> Self {
        let plane = Plane::new(Point3::new(0.0, 0.0, 18.0), 0f32, 0f32, 0f32);
        let material = CheckerMaterial::new(Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0), 1.0);
        Self {plane, material}
    }
}

impl SceneElement for CheckerBoard {
    fn geometry(&self)-> &dyn Geometry {
        &self.plane
    }

    fn material(&self) -> &dyn Material {
        &self.material
    }
}