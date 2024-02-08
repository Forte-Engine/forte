use cgmath::Point2;
use egui::{pos2, vec2, ClippedPrimitive, FontDefinitions, TexturesDelta};
use egui_wgpu::ScreenDescriptor;
use winit::keyboard::KeyCode;

use crate::{component_app::EngineComponent, inputs::Inputs, render::render_engine::RenderEngine};

pub mod helpers;

/// A `EngineComponent` that provides the necessary functionality to render Egui UI.
/// 
/// Example for initializing:
/// ```rust
/// create_app!(
///      CLEAR_COLOR = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
///
///     APP {
///         egui: EguiEngine[render_engine, inputs]
///     },
///
///     PASSES {
///         0: {
///             COMPONENTS: [egui],
///             DEPTH: false
///         }
///     }
/// );
/// ```
/// 
/// Example for drawing:
/// ```rust
/// egui::Window::new("Test")
///     .show(egui.context(), |ui| { 
///         ui.label("Hi from test window!");
///         ui.text_edit_singleline(&mut self.test); 
///         if ui.button("Search").clicked() {
///             println!("Search for {}", self.test);
///         }
///     });
/// ```
pub struct EguiEngine {
    context: egui::Context,
    renderer: egui_wgpu::Renderer,
    raw_input: egui::RawInput,
    info: EguiRenderInfo
}

/// Used by `EguiEngine` to draw Egui UI.  Contains necessary information for rendering.
struct EguiRenderInfo {
    desc: ScreenDescriptor,
    tdelta: TexturesDelta,
    paint_jobs: Vec<ClippedPrimitive>
}

impl EguiEngine {
    /// Returns a immutable reference to an `egui::Context` for rendering.
    pub fn context(&self) -> &egui::Context { &self.context }

    /// Returns a mutable referecne to an `egui::Context` for rendering.
    pub fn context_mut(&mut self) -> &mut egui::Context { &mut self.context }
}

impl EngineComponent<(&mut RenderEngine, &mut Inputs)> for EguiEngine {
    /// Creates a new instance of `EguiEngine` from the given mutable reference to a `RenderEngine`.
    fn create(engine: &mut RenderEngine) -> Self {
        // setup egui renderer
        let renderer = egui_wgpu::Renderer::new(&engine.device, wgpu::TextureFormat::Bgra8UnormSrgb, None, 1);

        // setup egui context
        let context = egui::Context::default();
        context.set_fonts(FontDefinitions::default());
        context.set_style(egui::Style::default());

        // generate raw input
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::default(),
                egui::vec2(
                    engine.size.width as f32,
                    engine.size.height as f32,
                ),
            )),
            ..Default::default()
        };

        // create unused egui render info
        let info = EguiRenderInfo {
            desc: ScreenDescriptor {
                size_in_pixels: [engine.size.width, engine.size.height],
                pixels_per_point: 1.0,
            },
            tdelta: TexturesDelta::default(), 
            paint_jobs: Vec::new()
        };

        Self { renderer, context, raw_input, info }
    }

    /// Starts this `EguiEngine` using mutable references to `RenderEngine` and `Inputs` using the standard `EngineComponent` methods.
    fn start(&mut self, _: (&mut RenderEngine, &mut Inputs)) {
        self.context.begin_frame(self.raw_input.take());
    }

    /// Updates this `EguiEngine` using mutable references to `RenderEngine` and `Inputs` using the standard `EngineComponent` methods.
    fn update(&mut self, (engine, inputs): (&mut RenderEngine, &mut Inputs)) {
        // end current frame
        let output = self.context.end_frame();

        // update cursor
        if let Some(cursor_icon) = helpers::egui_to_winit_cursor_icon(output.platform_output.cursor_icon)
        {
            engine.window.set_cursor_visible(true);
            if inputs.mouse_position().is_some() {
                engine.window.set_cursor_icon(cursor_icon);
            }
        } else {
            engine.window.set_cursor_visible(false);
        }

        // update mouse position
        let position = inputs.mouse_position().unwrap_or(&Point2 { x: 0.0, y: 0.0 });
        self.raw_input.events.push(egui::Event::PointerMoved(pos2(position.x, position.y)));

        // call mouse button press'
        inputs.mouse_buttons_pressed().iter().for_each(|button| {
            // convert winit mouse button to egui pointer button
            let button = match button {
                winit::event::MouseButton::Left => Some(egui::PointerButton::Primary),
                winit::event::MouseButton::Right => Some(egui::PointerButton::Secondary),
                winit::event::MouseButton::Middle => Some(egui::PointerButton::Middle),
                winit::event::MouseButton::Back => Some(egui::PointerButton::Extra1),
                winit::event::MouseButton::Forward => Some(egui::PointerButton::Extra2),
                winit::event::MouseButton::Other(_) => None,
            };
            let button = if button.is_some() { button.unwrap() } else { return };

            // pass pointer button event to egui
            self.raw_input.events.push(egui::Event::PointerButton {
                pos: pos2(position.x, position.y), 
                button, 
                pressed: true, 
                modifiers: self.raw_input.modifiers
            });
        });

        // call mouse button release's
        inputs.mouse_buttons_just_released().iter().for_each(|button| {
            // convert winit mouse button to egui pointer button
            let button = match button {
                winit::event::MouseButton::Left => Some(egui::PointerButton::Primary),
                winit::event::MouseButton::Right => Some(egui::PointerButton::Secondary),
                winit::event::MouseButton::Middle => Some(egui::PointerButton::Middle),
                winit::event::MouseButton::Back => Some(egui::PointerButton::Extra1),
                winit::event::MouseButton::Forward => Some(egui::PointerButton::Extra2),
                winit::event::MouseButton::Other(_) => None,
            };
            let button = if button.is_some() { button.unwrap() } else { return };

            // pass pointer button event to egui
            self.raw_input.events.push(egui::Event::PointerButton {
                pos: pos2(position.x, position.y), 
                button, 
                pressed: false, 
                modifiers: self.raw_input.modifiers
            });
        });

        // call mouse wheel inputs
        self.raw_input.events.push(egui::Event::MouseWheel { 
            unit: egui::MouseWheelUnit::Point, 
            delta: vec2(inputs.mouse_scroll_delta().x, inputs.mouse_scroll_delta().y), 
            modifiers: self.raw_input.modifiers 
        });

        // call key events
        inputs.keys_just_pressed().iter().for_each(|key_code| {
            // pass event ot egui
            let key = helpers::key_from_key_code(*key_code);
            let key = if key.is_some() { key.unwrap() } else { return };
            self.raw_input.events.push(egui::Event::Key { key, physical_key: helpers::key_from_key_code(*key_code), pressed: true, repeat: false, modifiers: self.raw_input.modifiers });
        });
        inputs.keys_just_released().iter().for_each(|key_code| {
            // pass event to egui
            let key = helpers::key_from_key_code(*key_code);
            let key = if key.is_some() { key.unwrap() } else { return };
            self.raw_input.events.push(egui::Event::Key { key, physical_key: helpers::key_from_key_code(*key_code), pressed: false, repeat: false, modifiers: self.raw_input.modifiers });

            // if input can be converted to text, pass that along to egui
            let text = helpers::key_code_to_text(key_code);
            if text.is_some() {
                let text = text.unwrap();
                let text = if inputs.is_key_down(&KeyCode::ShiftLeft) || inputs.is_key_down(&KeyCode::ShiftRight) { text.to_uppercase() } else { text.to_lowercase() };
                self.raw_input.events.push(egui::Event::Text(text.to_string()));
            }
        });

        // create paint jobs
        let paint_jobs = self.context.tessellate(output.shapes, 1.0);

        // create new encoder
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [engine.size.width, engine.size.height],
            pixels_per_point: 1.0,
        };

        self.info = EguiRenderInfo { desc: screen_descriptor, tdelta: output.textures_delta, paint_jobs };
        self.context.begin_frame(self.raw_input.take());
    }

    /// Renders this `EguiEngine` using mutable references to self, a `wgpu::RenderPass` and a immutable to a `RenderEngine` to render the Egui UI specified during the last update cycle.
    fn render<'rpass>(&'rpass mut self, engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
        // create a command encode for drawing
        let mut encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        // handle texture delta
        self.info.tdelta.set.iter().for_each(|(id, delta)| {
            self.renderer.update_texture(&engine.device, &engine.queue, *id, delta);
        });

        // update textures
        self.renderer.update_buffers(&engine.device, &engine.queue, &mut encoder, &self.info.paint_jobs, &self.info.desc);
        self.renderer.render(pass, &self.info.paint_jobs, &self.info.desc);

        // start next frame
        engine.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Calls necessary exit functions using mutable references to `RenderEngine` and `Inputs` using the standard `EngineComponent` methods.
    fn exit(&mut self, _: (&mut RenderEngine, &mut Inputs)) {}
}
