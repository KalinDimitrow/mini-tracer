mod texture;
mod renderer;
mod mini_tracer;
mod rendering_context;
mod auxiliary;
mod scene;

use crate::mini_tracer::MiniTracer;

fn main() {
    let mut tracer = MiniTracer::new();
    match tracer.run() {
        Err(error) => {
            print!("{}", error);
        }
        _ => {}
    }

}
