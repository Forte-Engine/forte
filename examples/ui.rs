use cgmath::{Quaternion, Vector2, Zero};
use forte_engine::{render::{primitives::transforms::TransformRaw, render_engine::RenderEngine}, run_world, ui::{canvas::UICanvas, elements::{ElementInfo, UIElement}, style::{Sizing, Style}, uniforms::UIInstance, DrawUI, UIEngine}};

run_world!(
    TestWorldApp,
    [
        // add a canvas component that renders all of its child UI elements
        Canvas => {
            DATA => UICanvas,
            ADDED => |_: &mut TestWorldApp, _: &mut Node| {},
            UPDATE => |app: &mut TestWorldApp, node: &mut Node| {
                // setup
                let size = Vector2 { x: app.render_engine().size.width as f32, y: app.render_engine().size.height as f32 };
                let mut contents = Vec::<UIInstance>::new();

                // render
                render_ui(node, &mut contents, &UIRenderInfo { position: Vector2::zero(), size, display_size: size });

                // update canvas with contents
                match &mut node.component {
                    Component::Canvas(canvas) => {
                        canvas.update(app.render_engine(), &contents);
                    },
                    _ => {}
                }
            },
            RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestWorldApp, data: &'b UICanvas| { 
                pass.prepare_ui(&app.ui_engine);
                pass.draw_canvas(app.render_engine(), &app.ui_engine, data);
            },
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        },

        // component representation of a UI element
        Ui => {
            DATA => UIElement,
            ADDED => |_: &mut TestWorldApp, _: &mut Node| {},
            UPDATE => |_: &mut TestWorldApp, _: &mut Node| {},
            RENDER => |_: &mut wgpu::RenderPass<'a>, _: &'b TestWorldApp, _: &'b UIElement| {},
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        }
    ]
);

pub struct UIRenderInfo {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub display_size: Vector2<f32>
}

pub fn render_ui(node: &Node, contents: &mut Vec<UIInstance>, info: &UIRenderInfo) {
    node.children.iter().for_each(|child| {
        match &child.component {
            Component::Ui(element) => {
                // calculate size and position of this element
                let size = calculate_size(child, &info.display_size);
                let position = calculate_position(element, info, &size);
                let new_info = UIRenderInfo { position, size, display_size: info.display_size };

                // generate transform of UI
                let transform = Transform {
                    position: Vector3 { 
                        x: 2.0 * ((position.x + (size.x * 0.5)) / info.display_size.x) - 1.0,
                        y: 2.0 * ((position.y + (size.y * 0.5)) / info.display_size.y) - 1.0,
                        z: 0.0
                    },
                    rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
                    scale: Vector3 {
                        x: size.x / info.display_size.x,
                        y: size.y / info.display_size.y,
                        z: 0.0
                    }
                };

                // save instance
                let instance = UIInstance(TransformRaw::from_generic(&transform).model);
                contents.push(instance);

                // render next elements
                render_ui(child, contents, &new_info);
            },
            _ => {}
        }
    });
}

pub fn calculate_position(_: &UIElement, info: &UIRenderInfo, size: &Vector2<f32>) -> Vector2<f32> {
    return Vector2 { 
        x: info.position.x + ((info.size.x - size.x) * 0.5), 
        y: info.position.y + ((info.size.y - size.y) * 0.5)
    }
}

pub fn calculate_size(node: &Node, display_size: &Vector2<f32>) -> Vector2<f32> {
    match &node.component {
        Component::Ui(element) => {
            // calculate my size
            let mut size = element.min_size(display_size);

            // make sure our size is contains the inside dimensions
            node.children.iter().for_each(|child| {
                let child = calculate_size(child, display_size);
                if size.x < child.x { size.x = child.x; }
                if size.y < child.y { size.y = child.y; }
            });

            size
        },
        _ => Vector2::zero()
    }
}

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

    fn start(&mut self, root: &mut Node) {
        root.add_child(self, Node {
            component: Component::Canvas(UICanvas::new(self.render_engine())),
            children: vec![
                Node {
                    component: Component::Ui(
                        UIElement { 
                            style: Style { 
                                width: Sizing::Px(100.0), 
                                height: Sizing::Px(100.0), 
                                ..Default::default() 
                            }, 
                            info: ElementInfo::Container 
                        }
                    ),
                    ..Default::default()
                }
            ],
            ..Default::default()
        });
    }

    fn update(&mut self, _root: &mut Node) {}
    fn exit(&mut self, _root: &mut Node) {}
}
