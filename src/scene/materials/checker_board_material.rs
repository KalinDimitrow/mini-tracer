use crate::scene::elements::scene_element::Material;
use crate::scene::light::Light;
use crate::auxiliary::{color::Color, ray::Ray, intersection_info::IntersectionInfo};
use num::{ToPrimitive, Integer};


pub struct CheckerMaterial {
    color1 : Color,
    color2 : Color,
    size : f32,
}

impl CheckerMaterial {
    pub fn new( color1 : Color, color2 : Color, size : f32) -> CheckerMaterial {
        CheckerMaterial{color1, color2, size}
    }
}

impl Material for CheckerMaterial {
    fn color(&self, ray : &Ray, intersection_info : &IntersectionInfo, lights : &[Light]) -> Color {
        let mut acumulator = Color::new(0.0, 0.0, 0.0);
        for light in lights {
            let light_direction = (&light.location - &intersection_info.point);
            let light_distance = light_direction.magnitude();
            let light_intensity = light_direction.dot(&intersection_info.normal).abs() * light.intensity / (light_distance*light_distance);
            acumulator += light_intensity * light.color.clone();
        }
        let value = (intersection_info.u / self.size).floor().to_i32().unwrap() + (intersection_info.v / self.size).floor().to_i32().unwrap();
        if value.is_even() {
            self.color1.clone() * acumulator
        } else {
            self.color2.clone() * acumulator
        }
    }
}