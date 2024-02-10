use forte_engine::{component_app::EngineComponent, create_app, run_app, ui::{elements::UIElement, style::{Color, PositionSetting, Sizing, Style}, UIEngine}};
use glyphon::{Attrs, Metrics};

pub struct TestComponent {}

impl EngineComponent<(&mut RenderEngine, &mut UIEngine)> for TestComponent {
    fn start(&mut self, (engine, ui): (&mut RenderEngine, &mut UIEngine)) {
        let mut a = UIElement::container(
            &engine, 
            Style { 
                width: Sizing::Px(200.0), 
                height: Sizing::Px(200.0), 
                color: Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 },
                border: Sizing::Px(5.0),
                round: Sizing::Px(15.0),
                ..Default::default() 
            }
        );
        a.children.push(UIElement::container(
            &engine, 
            Style {
                width: Sizing::Px(100.0), 
                height: Sizing::Px(100.0), 
                position_setting: PositionSetting::Parent,
                top: Sizing::Px(10.0),
                left: Sizing::Px(10.0),
                border: Sizing::Px(5.0),
                round: Sizing::Px(10.0),
                color: Color { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 },
                ..Default::default() 
            }
        ));
        ui.elements.push(a);

        let text = UIElement::text(
            &engine, 
            ui,
            Style {
                width: Sizing::Px(180.0), 
                height: Sizing::Px(50.0), 
                position_setting: PositionSetting::Parent,
                top: Sizing::Px(10.0),
                left: Sizing::Px(10.0),
                border: Sizing::Px(5.0),
                round: Sizing::Px(10.0),
                color: Color { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 },
                ..Default::default() 
            }, 
            "Hello world!",
            Attrs::new().family(glyphon::Family::SansSerif),
            glyphon::Color::rgb(255, 255, 255),
            Metrics::new(30.0, 42.0)
        );
        ui.elements.push(text);
    }

    fn create(_: &mut RenderEngine) -> Self { Self {} }
    fn update(&mut self, _: (&mut RenderEngine, &mut UIEngine)) {}
    fn render<'rpass>(&'rpass mut self, _: &'rpass RenderEngine, _: &mut wgpu::RenderPass<'rpass>) {}
    fn exit(&mut self, _: (&mut RenderEngine, &mut UIEngine)) {}
}

create_app!(
    CLEAR_COLOR = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },

    APP {
        ui_engine: UIEngine[render_engine],
        test: TestComponent[render_engine, ui_engine]
    },

    PASSES {
        0: {
            PIPELINE: "forte.ui",
            PREPARE: [],
            RENDER: ui_engine,
            DEPTH: false
        }
    }
);

fn main() { run_app::<App>() }