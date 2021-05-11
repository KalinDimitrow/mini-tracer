#[allow(unused_imports)]
use nalgebra::{Point3, Vector3};
use crate::auxiliary::color::Color;
use crate::scene::{
    elements::{scene_element::SceneElement, checker_board::CheckerBoard, generic_scene_element::GenericSceneElement},
    camera::Camera,
    light::Light,
    geometries::sphere::Sphere,
    materials::checker_board_material::CheckerMaterial,
};

pub struct Scene {
    pub camera : Camera,
    pub elements : Vec<Box<dyn SceneElement>>,
    pub lights : Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut elements : Vec<Box<dyn SceneElement>> = Vec::new();
        elements.push(Box::new(CheckerBoard::new()));
        // elements.push(Box::new(GenericSceneElement{
        //     geometry : Box::new(Sphere { position : Point3::new(-5f32, 3f32, 18f32), face : Vector3::new(0f32, 0f32, 0f32), radious : 2f32}),
        //     material : Box::new(FixedColorMaterial::new(Color::new(1.0, 1.0, 1.0))),
        // }));
        elements.push(Box::new(GenericSceneElement{
            // geometry : Box::new(Sphere { position : Point3::new(-5f32, 3f32, 18f32), face : Vector3::new(0f32, 0f32, 0f32), radious : 2f32}),
            geometry : Box::new(Sphere::new(Point3::new(-5f32, 3f32, 18f32), 2f32, std::f32::consts::PI/2f32, 0f32, 0f32)),
            material : Box::new(CheckerMaterial::new(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 0.0), 0.1)),
        }));
        // elements.push(Box::new(GenericSceneElement{
        //     geometry : Box::new(Sphere { position : Point3::new(0.0, 10.0, 20.0), face : Vector3::new(0f32, 0f32, 0f32), radious : 0.1f32}),
        //     material : Box::new(FixedColorMaterial::new(Color::new(1.0, 1.0, 1.0))),
        // }));
        let lights = vec![Light::new(Point3::new(0.0, 10.0, 20.0), Color::new(1.0, 1.0, 1.0), 50.0)];
        Scene {camera : Camera::new(Point3::new(0.0, 8.0, 0.0), 20.0, 0.0, 0.0, 1.33333, 90.0, (800, 600)),
            elements,
            lights,
        }
    }
}