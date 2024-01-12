use render_engine::{RenderEngine, RenderEngineInput};
use log::warn;
use winit::{window::WindowBuilder, event_loop::{EventLoop, ControlFlow}, event::{Event, WindowEvent}};

pub mod files;
pub mod pipelines;
pub mod primitives;
pub mod textures;
pub mod render_engine;
pub mod resources;

/// A useful matrix for converting opengl matrices to WGPU matrices.  Used in rendering to make our lives easy.
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

/// A trait implemented by any struct using a render engine.  The run_app function will automatically call all functions when appropriate.
pub trait RenderEngineApp {
    /// The create function is used by the run_app function to create a new app.  This is to be used to initialize the app with a render engine.
    fn create(engine: &mut RenderEngine) -> Self;

    /// The input function is called when an input is picked up by the event loop in the run_app function.  See RenderEngineInput documentation for more info.
    fn input(&mut self, engine: &mut RenderEngine, input: RenderEngineInput);

    /// The update function that will be called once per frame before rendering occurs.
    fn update(&mut self, engine: &mut RenderEngine);

    /// The render function that will be called once per frame after update with the necessary data to construct whatever render pass you wish with WGPU for rendering.
    fn render(&mut self, engine: &mut RenderEngine, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder);

    /// The exit function that is called when the program exits.
    fn exit(&mut self, engine: &mut RenderEngine);
}

/// The run_app function effectively creates an runs the app given as a generic argument.
/// 
/// This function will automatically call the create function on startup just after the window is created and render engine initialized.
/// Then the once per frame, the apps update and render functions will be called.
/// When an exit is request the loop will stop and then the exit function will be called before cleaning up all resources used by the render engine and this function.
/// When an input is received through the event loop is first passed to the render engine for initial processing before the apps input function is called.
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
