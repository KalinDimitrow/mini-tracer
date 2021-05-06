use nalgebra::{Point3};
use crate::auxiliary::color::Color;
use crate::scene::{
    elements::{scene_element::SceneElement, checker_board::CheckerBoard},
    camera::Camera,
    light::Light,
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
        let lights = vec![Light::new(Point3::new(0.0, 1.0, 20.0), Color::new(1.0, 1.0, 1.0), 25.0)];
        Scene {camera : Camera::new(Point3::new(0.0, 8.0, 0.0), 20.0, 0.0, 0.0, 1.33333, 90.0, (800, 600)),
            elements,
            lights,
        }
    }
}