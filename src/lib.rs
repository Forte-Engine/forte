use log::{warn, info};
use render::{input::EngineInput, render_engine::RenderEngine};
use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, event::{Event, WindowEvent}, dpi::PhysicalSize};

pub mod lights;
pub mod math;
pub mod render;
pub mod ui;

/// A trait implemented by any struct using a render engine.  The run_app function will automatically call all functions when appropriate.
pub trait EngineApp {
    /// The create function is used by the run_app function to create a new app.  This is to be used to initialize the app with a render engine.
    fn create(engine: RenderEngine) -> Self;

    /// The input function is called when an input is picked up by the event loop in the run_app function.  See RenderEngineInput documentation for more info.
    fn input(&mut self, input: EngineInput);

    /// The update function that will be called once per frame before rendering occurs.
    fn update(&mut self);

    /// Called when the window resizes, if you are keeping a render engine around, call the engines resize function now.
    fn resize(&mut self, new_size: PhysicalSize<u32>);

    /// Called when a redraw is requested by the event loop.  For basic functionallity, just call `render_engine.next_frame()`.
    fn events_cleared(&mut self);

    /// The exit function that is called when the program exits.
    fn exit(&mut self);
}

/// The run_app function effectively creates an runs the app given as a generic argument.
/// 
/// This function will automatically call the create function on startup just after the window is created and render engine initialized.
/// Then the once per frame, the apps update and render functions will be called.
/// When an exit is request the loop will stop and then the exit function will be called before cleaning up all resources used by the render engine and this function.
/// When an input is received through the event loop is first passed to the render engine for initial processing before the apps input function is called.
pub fn run_app<T: EngineApp + 'static>() {
    env_logger::init();

    // setup window and event loop
    info!("Creating window and event loop...");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // setup engine
    info!("Creating RenderEngine...");
    let engine = RenderEngine::new(window);

    // create app
    info!("Creating app...");
    let mut app = T::create(engine);

    info!("Starting event loop...");
    event_loop.run(move |event, _, flow| {
        match event {
            // window events
            Event::WindowEvent { ref event, .. } => {
                // match event
                match event {
                    // if close requested, exit
                    WindowEvent::CloseRequested => *flow = ControlFlow::Exit, 

                    // handle resizes
                    WindowEvent::Resized(size) => app.resize(*size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => app.resize(**new_inner_size),
                    
                    // otherwise, handle as an app input
                    _ => {
                        // convert the winit window event to a `EngineInput`
                        let input = EngineInput::from_winit_input(event);

                        // if the input is some, call the apps input function and return true, otherwise return false
                        if input.is_some() { app.input(input.unwrap()); }
                    }
                }
            },

            // on main events cleared, request redraw
            Event::MainEventsCleared => app.events_cleared(),

            // update the app when redraw requested
            Event::RedrawRequested(..) => app.update(),

            // when the loop stops, call the exit functions
            Event::LoopDestroyed => {
                info!("Exiting...");
                app.exit();
                info!("Goodbye :(");
            },

            // uh-oh, we missed something
            _ => warn!("Unhandled global event {:?}", event)
        }
    })
}
