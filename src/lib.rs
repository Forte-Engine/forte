use inputs::winit_input::EngineInput;
use render::render_engine::RenderEngine;
use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}, dpi::PhysicalSize};

pub mod component_app;
pub mod egui;
pub mod inputs;
pub mod lights;
pub mod math;
pub mod models;
pub mod primitives;
pub mod render;
pub mod ui;
pub mod utils;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

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
pub async fn run_app<T: EngineApp + 'static>() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    
    // setup window and event loop
    log!("Creating window and event loop...");
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // If in wasm mode, direct window to a canvas
    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_min_inner_size(Some(PhysicalSize::new(450, 400)));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("forte")?;
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }


    // setup engine
    log!("Creating RenderEngine...");
    let engine = RenderEngine::new(window).await;

    // create app
    log!("Creating app...");
    let mut app = T::create(engine);
    app.start();

    log!("Starting event loop...");
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        log(&format_args!($($t)*).to_string());
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", format_args!($($t)*).to_string());
    }
}
