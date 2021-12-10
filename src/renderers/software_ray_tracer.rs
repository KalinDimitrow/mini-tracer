use winit::dpi::PhysicalSize;

use crate::renderers::{
    rendering_context::RenderingContext,
    renderer::Renderer
};
use crate::auxiliary::{
    texture::HardwareTexture,
    color::Color,
    ray::Ray,
};
use crate::scene::scene::Scene;

pub struct SoftwareRayTracer {
    cached : bool,
}

impl SoftwareRayTracer {
    pub fn new() -> Self {
        SoftwareRayTracer{cached : false}
    }
    fn reytrace(&mut self, ray : Ray, scene : &Scene) -> Color {
        let mut closest_intersection_info = None;
        let mut closest_node = None;
        for element  in &scene.elements {
            if let Some(mut info) = element.geometry().intersect(&ray) {
                if info.compare(&closest_intersection_info) {
                    closest_intersection_info = Some(info);
                    closest_node = Some(element);
                } else {
                    // closest_intersection_info = Some()
                }
            }
        }

        if let Some(node) = closest_node {
            return node.material().color(&ray, &closest_intersection_info.unwrap(), &scene.lights);
        }
        return Color::new(0.0,0.0,0.0);
    }
}

impl Renderer for SoftwareRayTracer {
    fn render(&mut self, ctx : &mut RenderingContext, scene : &Scene) -> Option<HardwareTexture> {
        let resolution = scene.camera.resolution.clone();
        let mut image = Vec::with_capacity((resolution.0 * resolution.1 * 4) as usize);
        for y in 0..(resolution.1) {
            for x in 0..(resolution.0) {
                let ray = scene.camera.get_screen_ray(x as f32 + 0.5, y as f32 + 0.5);
                let color = self.reytrace(ray, scene);
                image.extend_from_slice(&color.to_srgb());
            }
        }

        let diffuse_texture =
            HardwareTexture::from_raw(&ctx.device, &ctx.queue, &image, (resolution.0, resolution.1), None).unwrap();

        ctx.resize(PhysicalSize::new(resolution.0, resolution.1));
        Some(diffuse_texture)
    }
}