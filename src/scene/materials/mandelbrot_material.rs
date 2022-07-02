use num::complex::Complex;

use crate::scene::{
    elements::scene_element::Material,
    light::Light
};

use crate::auxiliary::{
    intersection_info::IntersectionInfo,
    color::Color,
    ray::Ray,
};

use num::{ToPrimitive, Integer};


pub struct MandelbrotMaterial {
    scale : f32,
    limit : usize,
}

impl MandelbrotMaterial {
    pub fn new( scale : f32, limit : usize) -> MandelbrotMaterial {
        MandelbrotMaterial{scale, limit}
    }
}

impl Material for MandelbrotMaterial {
    fn color(&self, _ray : &Ray, intersection_info : &IntersectionInfo, lights : &[Light]) -> Color {
        let mut acumulator = Color::new(0.0, 0.0, 0.0);
        for light in lights {
            let light_direction = &light.location - &intersection_info.point;
            let light_distance = light_direction.magnitude();
            let light_intensity = light_direction.dot(intersection_info.normal.as_ref()).abs() * light.intensity / (light_distance*light_distance);
            acumulator += light_intensity * light.color.clone();
        }

        let mut zn = Complex{re : intersection_info.uv.x/self.scale, im : intersection_info.uv.y/self.scale};
        let mut count = 0;
        while count < self.limit {
            count += 1;
            if zn.norm_sqr() > 4f32 {
                return Color{r : count as f32 / self.limit as f32, g : 0f32, b : 0f32};
            }

            zn = zn.powu(2) + Complex::new(intersection_info.uv.x/self.scale, intersection_info.uv.y/self.scale);
        }

        Color{r : 0f32, g : 0f32, b : 0f32}
    }
}