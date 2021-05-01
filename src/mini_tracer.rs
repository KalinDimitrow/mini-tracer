use crate::renderers::{
    rendering_context::RenderingContext,
    software_ray_tracer::SoftwareRayTracer,
    renderer::Renderer
};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
    dpi::LogicalSize,
};

use crate::scene::scene_manager::SceneManager;

pub struct MiniTracer {
}

impl MiniTracer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MiniTracer{}
    }

    #[allow(dead_code)]
    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut ctx = RenderingContext::new(&window)?;
        let mut renderer : Box<dyn Renderer> = Box::new(SoftwareRayTracer::new());
        let scene_manager = SceneManager::new();
        let scene = scene_manager.load("path to scene not implemented");
        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !ctx.input(event) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::KeyboardInput { input, .. } => match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            },
                            WindowEvent::Resized(physical_size) => {
                                ctx.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &mut so w have to dereference it twice
                                ctx.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    ctx.update();
                    let image = renderer.render(&mut ctx, &scene);
                    match ctx.render(image) {
                        Ok(dimensions) => {
                            window.set_inner_size(LogicalSize::new(dimensions.0, dimensions.1));
                        }
                        // Recreate the swap_chain if lost
                        Err(wgpu::SwapChainError::Lost) => ctx.resize(ctx.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}