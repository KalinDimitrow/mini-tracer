use crate::renderers::{
    rendering_context::RenderingContext,
    renderer::Renderer
};
use crate::auxiliary::{
    intersection_info::IntersectionInfo,
    texture::HardwareTexture,
    color::Color,
    ray::Ray,
};
use crate::scene::scene::Scene;

pub struct SoftwareRayTracer {
}

impl SoftwareRayTracer {
    pub fn new() -> Self {
        SoftwareRayTracer{}
    }
    fn reytrace(&mut self, ray : Ray, scene : &Scene) -> Color {
        let mut closest_intersection_info = IntersectionInfo::new();
        let mut closest_node = None;
        for element  in &scene.elements {
            if let Some(info) = element.geometry().intersect(&ray) {
                if closest_intersection_info.compare(info) {
                    closest_node = Some(element);
                }
            }
        }

        if let Some(node) = closest_node {
            return node.material().color(&ray, &closest_intersection_info, &scene.lights);
        }
        return Color::new(0.0,0.0,0.0);
    }
}

impl Renderer for SoftwareRayTracer {
    fn render(&mut self, ctx : &mut RenderingContext, scene : &Scene) -> Option<HardwareTexture> {

        let mut image = Vec::with_capacity((ctx.size.width * ctx.size.height * 4) as usize);
        for y in 0..(ctx.size.height) {
            for x in 0..(ctx.size.width) {
                let ray = scene.camera.get_screen_ray(x as f32 + 0.5, y as f32 + 0.5);
                let color = self.reytrace(ray, scene);
                image.extend_from_slice(&color.to_srgb());

            }
        }

        let diffuse_texture =
            HardwareTexture::from_raw(&ctx.device, &ctx.queue, &image, (ctx.size.width, ctx.size.height), None).unwrap();

        Some(diffuse_texture)
    }
}