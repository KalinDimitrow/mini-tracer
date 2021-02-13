mod mini_tracer;
mod auxiliary;
mod renderers;
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
