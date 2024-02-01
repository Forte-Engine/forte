use forte_engine::{components::EngineComponent, create_app, run_app, ui::UIEngine};

pub struct TestComponent;
impl EngineComponent<App> for TestComponent {
    fn create(render_engine: &mut RenderEngine) -> Self {
        println!("create");
        Self
    }

    fn start(components: &mut App) {
        println!("start");
    }

    fn update(components: &mut App) {}

    fn render<'rpass>(&self, pass: &mut wgpu::RenderPass<'rpass>) {
        println!("render");
    }

    fn exit(component: &mut App) {
        println!("exit");
    }
}

create_app!(
    COMPONENTS => [
        test => TestComponent
    ]

    PASSES => [
        0 => [test]
    ]
);

fn main() { run_app::<App>() }