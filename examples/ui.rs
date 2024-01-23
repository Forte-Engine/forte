use forte_engine::{*, math::transforms::Transform, render::{primitives::transforms::TransformRaw, render_engine::RenderEngine}, ui::{elements::canvas::UICanvas, uniforms::UIInstance, DrawUI, UIEngine}, EngineApp};

pub struct TestApp {
    render_engine: RenderEngine,
    ui_engine: UIEngine,
    canvas: UICanvas
}

impl EngineApp for TestApp {
    fn create(mut render_engine: RenderEngine) -> Self {
        let mut transform = Transform::default();
        transform.scale *= 0.5;
        let transform_raw = TransformRaw::from_generic(&transform);

        // create render engine
        let ui_engine = UIEngine::new(&mut render_engine);

        // create canvas
        let canvas = UICanvas::new_with_contents(&render_engine, &[UIInstance(transform_raw.model)]);

        // create new self
        Self {
            render_engine,
            ui_engine,
            canvas
        }
    }

    fn update(&mut self) {
        let mut resources = start_render!(self.render_engine);

        {
            let mut pass = pass!(self.render_engine, resources);
            pass.prepare_ui(&self.ui_engine);
            pass.draw_canvas(&self.render_engine, &self.ui_engine, &self.canvas);
        }

        end_render!(self.render_engine, resources);
    }

    fn input(&mut self, _input: forte_engine::render::input::EngineInput) {}
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
    fn events_cleared(&mut self) { self.render_engine.next_frame(); }
    fn exit(&mut self) {}
}

fn main() { run_app::<TestApp>() }