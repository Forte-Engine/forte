use forte_engine::{component_app::EngineComponent, create_app, run_app, ui::{elements::UIElement, style::{Color, PositionSetting, Sizing, Style}, UIEngine}};

pub struct TestComponent {}

impl EngineComponent<App> for TestComponent {
    fn start(app: &mut App) {
        let mut a = UIElement::container(
            app.render_engine(), 
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
            app.render_engine(), 
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
        app.ui_engine.elements.push(a);
    }

    fn create(_: &mut RenderEngine) -> Self { Self {} }
    fn update(_: &mut App) {}
    fn render<'rpass>(&'rpass self, _: &'rpass RenderEngine, _: &mut wgpu::RenderPass<'rpass>) {}
    fn exit(_: &mut App) {}
}

create_app!(
    COMPONENTS => [
        ui_engine => UIEngine<App>,
        test => TestComponent
    ]

    PASSES => [
        0 => [ui_engine]
    ]
);

fn main() { run_app::<App>() }