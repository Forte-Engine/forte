use render_engine::{RenderEngine, RenderEngineInput};
use log::warn;
use winit::{window::WindowBuilder, event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}};

pub mod files;
pub mod pipelines;
pub mod primitives;
pub mod textures;
pub mod render_engine;
pub mod resources;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub trait RenderEngineApp {
    fn create(engine: &mut RenderEngine) -> Self;
    fn input(&mut self, engine: &mut RenderEngine, input: RenderEngineInput);
    fn update(&mut self, engine: &mut RenderEngine);
    fn render(&mut self, engine: &mut RenderEngine, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder);
    fn exit(&mut self, engine: &mut RenderEngine);
}

pub async fn run_app<T: RenderEngineApp + 'static>() {
    env_logger::init();

    // setup window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // setup engine
    let mut engine = RenderEngine::new(window).await;

    // create app
    let mut app = Box::new(T::create(&mut engine));

    event_loop.run(move |event, _, flow| {
        match event {
            // window events
            Event::WindowEvent { window_id, ref event } => {
                // make sure it is our window
                if window_id != engine.window.id() { return }

                // if input is handled, stop here
                if engine.input(&mut app, event) { return }
                
                // if close requested, stop
                match event {
                    // exit
                    WindowEvent::CloseRequested => *flow = ControlFlow::Exit, 

                    // resizes
                    WindowEvent::Resized(size) => engine.resize(*size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => engine.resize(**new_inner_size),
                    
                    _ => {}
                }
            },

            Event::MainEventsCleared => engine.window().request_redraw(),
            Event::RedrawRequested(window_id) => {
                // make sure our window
                if window_id != engine.window.id() { return }

                // update app then egui
                app.update(&mut engine);

                // render the app via the engine
                match engine.render(&mut app) {
                    // good
                    Ok(_) => {}

                    // reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => engine.resize(engine.size),

                    // if out of memory, we should quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *flow = ControlFlow::Exit,

                    // other errors
                    Err(e) => eprintln!("{:?}", e),
                }
            },

            Event::LoopDestroyed => app.exit(&mut engine),

            _ => warn!("Unhandled global event {:?}", event)
        }
    })
}
