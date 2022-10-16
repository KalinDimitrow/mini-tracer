use winit::dpi::PhysicalSize;

use crate::auxiliary::intersection_info::IntersectionInfo;
use crate::auxiliary::{color::Color, ray::Ray, texture::HardwareTexture};
use crate::renderers::{renderer::Renderer, rendering_context::RenderingContext};
use crate::scene::elements::scene_element::SceneElement;
use crate::scene::scene::Scene;

pub struct SoftwareRayTracer {
    cached: bool,
}

impl SoftwareRayTracer {
    pub fn new() -> Self {
        SoftwareRayTracer { cached: false }
    }
    fn reytrace(&mut self, ray: Ray, scene: &Scene) -> Color {
        let mut stack = vec![1f32];

        while let Some(current) = stack.pop() {
            // let current = stack.pop();
        }

        if let Some((node, info)) = self.find_closest_intersection(&ray, scene) {
            return node.material().color(&ray, &info, &scene.lights);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    fn find_closest_intersection<'a>(
        &self,
        ray: &Ray,
        scene: &'a Scene,
    ) -> Option<(&'a Box<dyn SceneElement>, IntersectionInfo)> {
        scene
            .elements
            .iter()
            .filter_map(|element| {
                element
                    .geometry()
                    .intersect(ray)
                    .map(|info| (element, info))
            })
            .min_by(|t1, t2| t1.1.distance.total_cmp(&t2.1.distance))
    }
}

impl Renderer for SoftwareRayTracer {
    fn render(&mut self, ctx: &mut RenderingContext, scene: &Scene) -> Option<HardwareTexture> {
        let resolution = scene.camera.resolution.clone();
        let mut image = Vec::with_capacity((resolution.0 * resolution.1 * 4) as usize);
        for y in 0..(resolution.1) {
            for x in 0..(resolution.0) {
                let ray = scene.camera.get_screen_ray(x as f32 + 0.5, y as f32 + 0.5);
                let color = self.reytrace(ray, scene);
                image.extend_from_slice(&color.to_srgb());
            }
        }

        let diffuse_texture = HardwareTexture::from_raw(
            &ctx.device,
            &ctx.queue,
            &image,
            (resolution.0, resolution.1),
            None,
        )
        .unwrap();

        ctx.resize(PhysicalSize::new(resolution.0, resolution.1));
        Some(diffuse_texture)
    }
}
