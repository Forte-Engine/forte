use egui::{ClippedPrimitive, FontDefinitions, FullOutput, TexturesDelta};
use egui_wgpu::ScreenDescriptor;

use crate::{component_app::EngineComponent, render::render_engine::RenderEngine};

pub mod helpers;

pub struct EguiEngine {
    pub context: egui::Context,
    renderer: egui_wgpu::Renderer,
    raw_input: egui::RawInput,
    info: EguiRenderInfo
}

struct EguiRenderInfo {
    desc: ScreenDescriptor,
    tdelta: TexturesDelta,
    paint_jobs: Vec<ClippedPrimitive>
}

impl EngineComponent<&mut RenderEngine> for EguiEngine {
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
        let encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });
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

    fn start(&mut self, _: &mut RenderEngine) {
        self.context.begin_frame(self.raw_input.take());
    }

    fn update(&mut self, engine: &mut RenderEngine) {
        // end current frame
        let output = self.context.end_frame();

        // update cursor
        if let Some(cursor_icon) = helpers::egui_to_winit_cursor_icon(output.platform_output.cursor_icon)
        {
            engine.window.set_cursor_visible(true);
            // if self.pointer_pos.is_some() {
                engine.window.set_cursor_icon(cursor_icon);
            // }
        } else {
            engine.window.set_cursor_visible(false);
        }

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

    fn render<'rpass>(&'rpass mut self, engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>) {
        let mut encoder = engine.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        if !self.info.tdelta.set.is_empty() { println!("Tdelta"); }
        self.info.tdelta.set.iter().for_each(|(id, delta)| {
            self.renderer.update_texture(&engine.device, &engine.queue, *id, delta);
        });

        // update textures
        self.renderer.update_buffers(&engine.device, &engine.queue, &mut encoder, &self.info.paint_jobs, &self.info.desc);
        self.renderer.render(pass, &self.info.paint_jobs, &self.info.desc);

        // start next frame
        engine.queue.submit(std::iter::once(encoder.finish()));
    }

    fn exit(&mut self, other: &mut RenderEngine) {}
}
