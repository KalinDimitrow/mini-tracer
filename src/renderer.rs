use nalgebra::{Point3, Vector3};
use rand::{distributions::Uniform, Rng};

use crate::auxiliary::{color::Color, ray::Ray};
use crate::auxiliary::intersection_info::IntersectionInfo;
use crate::rendering_context::RenderingContext;
use crate::scene::camera::Camera;
use crate::scene::elements::scene_element::SceneElement;
use crate::scene::geometries::plane::Plane;
use crate::scene::elements::checker_board::CheckerBoard;
use crate::scene::light::Light;
use crate::texture::Texture;

pub struct Scene {
    pub camera : Camera,
    pub elements : Vec<Box<dyn SceneElement>>,
    pub lights : Vec<Light>,
}

impl Scene {
    pub fn new() -> Scene {
        let mut elements : Vec<Box<dyn SceneElement>> = Vec::new();
        elements.push(Box::new(CheckerBoard::new()));
        let mut lights = vec![Light::new(Point3::new(-5.0, 1.0, 20.0), Color::new(1.0, 1.0, 1.0), 5.0)];
        Scene {camera : Camera::new(Point3::new(0.0, 8.0, 0.0), 20.0, 0.0, 0.0, 1.33333, 90.0, (800, 600)),
                elements,
                lights,
        }
    }
}

pub trait Renderer {
    fn render(&mut self, ctx : &mut RenderingContext, scene : &Scene) -> Option<Texture>;
}


pub struct SoftwareRayTracer {
    rng : rand::rngs::ThreadRng
}

impl SoftwareRayTracer {
    pub fn new() -> Self {
        SoftwareRayTracer{rng : rand::thread_rng()}
    }
    fn reytrace(&mut self, ray : Ray, scene : &Scene) -> Color {
        let mut closest_intersection_info = IntersectionInfo::new();
        let mut result_color = Color::new(0.0,0.0,0.0);
        for element  in &scene.elements {
            if let Some(info) = element.geometry().intersect(&ray) {
                if closest_intersection_info.compare(info) {
                    result_color = element.material().color(&ray, &closest_intersection_info, &scene.lights);
                }
            }
        }

        return result_color;
    }
}

impl Renderer for SoftwareRayTracer {
    fn render(&mut self, ctx : &mut RenderingContext, scene : &Scene) -> Option<Texture> {

        let mut image = Vec::with_capacity((ctx.size.width * ctx.size.height * 4) as usize);
        for y in 0..(ctx.size.height) {
            for x in 0..(ctx.size.width) {
                let ray = scene.camera.get_screen_ray(x as f32 + 0.5, y as f32 + 0.5);
                let color = self.reytrace(ray, scene);
                image.extend_from_slice(&color.to_srgb());

            }
        }

        let diffuse_texture =
            Texture::from_raw(&ctx.device, &ctx.queue, &image, (ctx.size.width, ctx.size.height), None).unwrap();

        Some(diffuse_texture)
    }
}