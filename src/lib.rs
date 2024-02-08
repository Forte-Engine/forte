use log::info;
use inputs::winit_input::EngineInput;
use render::render_engine::RenderEngine;
use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}, dpi::PhysicalSize};

pub mod component_app;
pub mod egui;
pub mod inputs;
pub mod lights;
pub mod math;
pub mod primitives;
pub mod render;
pub mod ui;
pub mod utils;

/// A trait implemented by any struct using a render engine.  The run_app function will automatically call all functions when appropriate.
pub trait EngineApp {
    /// The create function is used by the run_app function to create a new app.  This is to be used to initialize the app with a render engine.
    fn create(engine: RenderEngine) -> Self;

    /// The start function is called directly after create.  This is useful for running code after the engine and app are completely initialized.
    fn start(&mut self);

    /// The input function is called when an input is picked up by the event loop in the run_app function.  See RenderEngineInput documentation for more info.
    fn input(&mut self, input: EngineInput);

    /// The update function that will be called once per frame before rendering occurs.
    fn update(&mut self);

    /// Called when the window resizes, if you are keeping a render engine around, call the engines resize function now.
    fn resize(&mut self, new_size: PhysicalSize<u32>);

    /// The exit function that is called when the program exits.
    fn exit(&mut self);
}

/// The run_app function effectively creates an runs the app given as a generic argument.
/// 
/// This function will automatically call the create function on startup just after the window is created and render engine initialized.
/// This function will automatically call the start function just after the above create step.
/// Then the once per frame, the apps update and render functions will be called.
/// When an exit is request the loop will stop and then the exit function will be called before cleaning up all resources used by the render engine and this function.
/// When an input is received through the event loop is first passed to the render engine for initial processing before the apps input function is called.
pub fn run_app<T: EngineApp + 'static>() {
    // setup window and event loop
    info!("Creating window and event loop...");
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // setup engine
    info!("Creating RenderEngine...");
    let engine = RenderEngine::new(window);

    // create app
    info!("Creating app...");
    let mut app = T::create(engine);
    app.start();

    info!("Starting event loop...");
    let _ = event_loop.run(move |event, target| {
        match event {
            // window events
            Event::WindowEvent { ref event, .. } => {
                // match event
                match event {
                    // if close requested, exit
                    WindowEvent::CloseRequested => target.exit(), 

                    // handle resizes
                    WindowEvent::Resized(size) => app.resize(*size),

                    // handle updates
                    WindowEvent::RedrawRequested => {
                        app.update();
                    },
                    
                    // otherwise, handle as an app input
                    _ => {
                        // convert the winit window event to a `EngineInput`
                        let input = EngineInput::from_winit_input(event);

                        // if the input is some, call the apps input function and return true, otherwise return false
                        if input.is_some() { app.input(input.unwrap()); }
                    }
                }
            },
            Event::NewEvents(_) => {},
            Event::DeviceEvent { device_id: _, event: _ } => {},
            Event::UserEvent(_) => {},
            Event::Suspended => {},
            Event::Resumed => {},
            Event::AboutToWait => {},
            Event::LoopExiting => app.exit(),
            Event::MemoryWarning => panic!("Out of memory!"),
        }
    });
}
