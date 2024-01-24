use forte_engine::{render::render_engine::RenderEngine, run_world, ui::{canvas::UICanvas, elements::UIElement, DrawUI, UIEngine}};

run_world!(
    TestWorldApp,
    [
        Canvas => {
            DATA => UICanvas,
            ADDED => |_: &mut TestWorldApp, _: &mut Node| {},
            UPDATE => |_: &mut TestWorldApp, _: &mut Node| {},
            RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestWorldApp, data: &'b UICanvas| { pass.draw_canvas(app.render_engine(), &app.ui_engine, data); },
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        },
        Ui => {
            DATA => UIElement,
            ADDED => |_: &mut TestWorldApp, _: &mut Node| {},
            UPDATE => |_: &mut TestWorldApp, _: &mut Node| {},
            RENDER => |_: &mut wgpu::RenderPass<'a>, _: &'b TestWorldApp, _: &'b UIElement| {},
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        }
    ]
);

pub struct TestWorldApp {
    render_engine: RenderEngine,
    ui_engine: UIEngine
}

impl WorldApp for TestWorldApp {
    fn render_engine(&self) ->  &RenderEngine { &self.render_engine }
    fn render_engine_mut(&mut self) ->  &mut RenderEngine { &mut self.render_engine }

    fn create(mut render_engine: RenderEngine) -> Self {
        let ui_engine = UIEngine::new(&mut render_engine);
        Self { render_engine, ui_engine }
    }

    fn start(&mut self, _root: &mut Node) {}
    fn update(&mut self, _root: &mut Node) {}
    fn exit(&mut self, _root: &mut Node) {}
}
