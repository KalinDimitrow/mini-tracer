use crate::texture::Texture;
use rand::{distributions::Uniform, Rng};
use crate::rendering_context::RenderingContext;

pub struct Scene {}

pub trait Renderer {
    fn render(&mut self, ctx : &mut RenderingContext, scene : &Scene) -> Option<Texture>;
}


pub struct SoftwareRayTracer {

}

impl SoftwareRayTracer {
    pub fn new() -> Self {
        SoftwareRayTracer{}
    }
}

impl Renderer for SoftwareRayTracer {
    fn render(&mut self, ctx : &mut RenderingContext, _scene : &Scene) -> Option<Texture> {
        let range = Uniform::from(0..u8::max_value());
        let img: Vec<u8> = rand::thread_rng().sample_iter(&range).take(256*256*4).collect();
        let diffuse_texture =
            Texture::from_raw(&ctx.device, &ctx.queue, &img, (256, 256), Some("random")).unwrap();

        Some(diffuse_texture)
    }
}