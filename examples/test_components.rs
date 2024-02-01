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

    fn update(components: &mut App) {
        println!("update");
    }

    fn render<'rpass>(components: &mut App, pass: &mut wgpu::RenderPass<'rpass>) {
        
    }

    fn exit(component: &mut App) {
        println!("exit");
    }
}

create_app!(
    COMPONENTS => [
        test => TestComponent
    ]
);

fn main() { run_app::<App>() }