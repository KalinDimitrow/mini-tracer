use crate::scene::{
    elements::scene_element::Material,
    light::Light
};

use crate::auxiliary::{
    intersection_info::IntersectionInfo,
    color::Color,
    ray::Ray,
};

pub struct FixedColorMaterial {
    color : Color,
}

impl FixedColorMaterial {
    #[allow(dead_code)]
    pub fn new( color : Color) -> FixedColorMaterial {
        FixedColorMaterial{color}
    }
}

impl Material for FixedColorMaterial {
    fn color(&self, _ray : &Ray, intersection_info : &IntersectionInfo, lights : &[Light]) -> Color {
        let mut acumulator = Color::new(0.0, 0.0, 0.0);
        for light in lights {
            let light_direction = &light.location - &intersection_info.point;
            let light_distance = light_direction.magnitude();
            let light_intensity = light_direction.dot(intersection_info.normal.as_ref()).max(0f32) * light.intensity / (light_distance.powi(2));
            acumulator += light_intensity * light.color.clone();
        }

        self.color.clone() * acumulator
    }
}