use crate::renderers::rendering_context::RenderingContext;
use crate::auxiliary::texture::HardwareTexture;
use crate::scene::scene::Scene;

pub trait Renderer {
    fn render(&mut self, ctx : &mut RenderingContext, scene : &Scene) -> Option<HardwareTexture>;
}
