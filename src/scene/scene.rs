#[allow(unused_imports)]
use nalgebra::{Point3, Unit, Vector3};
use crate::auxiliary::color::Color;
use crate::scene::{
    elements::{scene_element::SceneElement, generic_scene_element::GenericSceneElement},
    camera::Camera,
    light::Light,
    geometries::plane::Plane,
    geometries::sphere::Sphere,
    geometries::cube::Cube,
    geometries::triangle::Triangle,
    geometries::regular_polygon::RegularPolygon,
    materials::checker_board_material::CheckerMaterial,
    materials::mandelbrot_material::MandelbrotMaterial,
};

pub struct Scene {
    pub camera : Camera,
    pub elements : Vec<Box<dyn SceneElement>>,
    pub lights : Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut elements : Vec<Box<dyn SceneElement>> = Vec::new();
        elements.push(Box::new(GenericSceneElement{
            geometry : Box::new(Plane::new(Point3::new(0.0, 0.0, 0.0), 0f32, 0f32, 0f32)),
            material : Box::new(CheckerMaterial::new(Color::new(1.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0), 1.0)),
        }));
        elements.push(Box::new(GenericSceneElement{
            geometry : Box::new(Sphere::new(Point3::new(0f32, 0f32, -40f32), 10f32, 0f32, 0f32, 0f32)),
            material : Box::new(CheckerMaterial::new(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 0.0), 0.1)),
        }));
        elements.push(Box::new(GenericSceneElement{
            geometry : Box::new(Plane::new(Point3::new(0.0, 0f32, -40f32), std::f32::consts::PI/2f32, -std::f32::consts::PI/6f32, std::f32::consts::PI/2f32)),
            material : Box::new(MandelbrotMaterial::new(40f32, 255)),
        }));
        elements.push(Box::new(GenericSceneElement{
            geometry : Box::new(Cube::new(Point3::new(5f32, 4f32, -15f32), 0f32, -std::f32::consts::PI/4f32, -std::f32::consts::PI/4f32, 5f32)),
            material : Box::new(MandelbrotMaterial::new(2f32, 255)),
        }));
        elements.push(Box::new(        GenericSceneElement{
            geometry : Box::new(
                Triangle::new(
                    Point3::new(-3.0, 5f32, -8f32)
                , Point3::new(3f32, 5f32, -8f32)
                , Point3::new(0.0, 8.0, -8.0))),
            material : Box::new(CheckerMaterial::new(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 0.0), 1f32)),
        }));
        elements.push(Box::new(GenericSceneElement{
            geometry : Box::new(RegularPolygon::new(Point3::new(-4.5f32, 5f32, -8f32), std::f32::consts::PI/2f32, std::f32::consts::PI/3f32, std::f32::consts::PI/2f32, 5, 3f32)),
            material : Box::new(CheckerMaterial::new(Color::new(0.0, 0.0, 1.0), Color::new(0.0, 1.0, 0.0), 1f32)),
        }));

        let lights = vec![Light::new(Point3::new(0.0, 10.0, 0.0), Color::new(1.0, 1.0, 1.0), 20.0)];
        Scene {camera : Camera::new(Point3::new(0.0, 8.0, 0.0), 0.0, 0.0, 0.0, 1.33333, 90.0, (800, 600)),
            elements,
            lights,
        }
    }
}