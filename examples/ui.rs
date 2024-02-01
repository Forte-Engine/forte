use cgmath::{Quaternion, Vector2, Zero};
use forte_engine::{math::quaternion::QuaternionExt, render::{primitives::transforms::TransformRaw, render_engine::RenderEngine, resources::Handle, textures::textures::Texture}, run_world, ui::{canvas::UICanvas, elements::{ElementInfo, UIElement}, style::{Color, PositionSetting, Sizing, Style}, uniforms::UIInstance, DrawUI, UIEngine}};

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

                // render
                match &node.component {
                    Component::Canvas(canvas) => {
                        render_ui(node, app.render_engine(), &UIRenderInfo { position: Vector2::zero(), size, display_size: size }, &canvas.blank_texture, 0.5);
                    }
                    _ => {}
                }
            },
            RENDER => |_: &mut wgpu::RenderPass<'a>, _: &'b TestWorldApp, _: &'b UICanvas| {},
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        },

        // component representation of a UI element
        Ui => {
            DATA => UIElement,
            ADDED => |_: &mut TestWorldApp, _: &mut Node| {},
            UPDATE => |_: &mut TestWorldApp, _: &mut Node| {},
            RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestWorldApp, element: &'b UIElement| {
                pass.prepare_ui(&app.ui_engine);
                let texture = match &element.info {
                    ElementInfo::Image(texture) => app.render_engine().texture(texture),
                    _ => app.render_engine().texture_from_path("ui.blank")
                };
                pass.set_bind_group(0, &texture.bind_group, &[]);
                pass.draw_element(app.render_engine(), &app.ui_engine, element);
            },
            REMOVED => |_: &mut TestWorldApp, _: &mut Node| {}
        }
    ]
);

#[derive(Debug)]
pub struct UIRenderInfo {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub display_size: Vector2<f32>
}

pub fn render_ui(node: &Node, engine: &RenderEngine, info: &UIRenderInfo, blank_texture: &Handle<Texture>, layer: f32) {
    node.children.iter().for_each(|child| {
        match &child.component {
            Component::Ui(element) => {
                // calculate size and position of this element
                let (position, size) = calculate_position_size(child, info);
                let new_info = UIRenderInfo { position, size, display_size: info.display_size };

                // generate transform of UI
                let transform = Transform {
                    position: Vector3 { 
                        x: 2.0 * ((position.x + (size.x * 0.5)) / info.display_size.x) - 1.0,
                        y: 2.0 * ((position.y + (size.y * 0.5)) / info.display_size.y) - 1.0,
                        z: layer
                    },
                    rotation: Quaternion::euler_deg_z(element.style.rotation),
                    scale: Vector3 {
                        x: size.x / info.display_size.x,
                        y: size.y / info.display_size.y,
                        z: 0.0
                    }
                };

                // create instance
                let raw_transform = TransformRaw::from_generic(&transform).model;
                let instance = UIInstance([
                    raw_transform[0],
                    raw_transform[1],
                    raw_transform[2],
                    raw_transform[3],
                    element.style.color.to_array(),
                    element.style.border_color.to_array(),
                    [
                        element.style.round.size(&info.display_size) / f32::max(size.x, size.y),
                        element.style.border.size(&info.display_size) / f32::max(size.x, size.y),
                        0.0,
                        0.0
                    ]
                ]);

                // save instance info
                engine.queue.write_buffer(&element.buffer, 0, bytemuck::cast_slice(&instance.0));

                // render next elements
                render_ui(child, engine, &new_info, blank_texture, layer - 0.05);
            },
            _ => {}
        }
    });
}

// calculates the position and size of the given element by taking in its own node and some render info about its parent and display size
pub fn calculate_position_size(node: &Node, info: &UIRenderInfo) -> (Vector2<f32>, Vector2<f32>) {
    match &node.component {
        Component::Ui(element) => {
            // calculate my size
            let size = element.min_size(&info.display_size);

            // calculate initial position
            let mut position =  Vector2 { 
                x: info.position.x + ((info.size.x - size.x) * 0.5), 
                y: info.position.y + ((info.size.y - size.y) * 0.5)
            };
        
            // if left positioning given, position based on above info, an offset, and the positioning type
            if element.style.left_set() {
                let offset = element.style.left.size(&info.display_size);
                match element.style.position_setting {
                    PositionSetting::Parent => {
                        position.x = info.position.x + offset;
                    },
                    PositionSetting::Absolute => {
                        position.x = offset;
                    }
                }
            } 
            // otherwise, do the same for the right
            else if element.style.right_set() {
                let offset = element.style.right.size(&info.display_size);
                match element.style.position_setting {
                    PositionSetting::Parent => {
                        position.x = info.position.x + info.size.x - size.x - offset;
                    },
                    PositionSetting::Absolute => {
                        position.x = info.display_size.x - size.x - offset;
                    }
                }
            }

            // do top bottom positioning
            if element.style.top_set() {
                let offset = element.style.top.size(&info.display_size);
                match element.style.position_setting {
                    PositionSetting::Parent => {
                        position.y = info.position.y + info.size.y - size.y - offset;
                    },
                    PositionSetting::Absolute => {
                        position.y = info.display_size.y - size.y - offset;
                    }
                }
            } else if element.style.bottom_set() {
                let offset = element.style.bottom.size(&info.display_size);
                position.y = offset;
                match element.style.position_setting {
                    PositionSetting::Parent => {
                        position.y = info.position.y + offset;
                    },
                    PositionSetting::Absolute => {
                        position.y = offset;
                    }
                }
            }

            (position, size)
        },
        _ => (Vector2::zero(), Vector2::zero()
    )
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
        let canvas = UICanvas::new(self.render_engine_mut());
        let texture = self.render_engine_mut().load_texture("./examples/rotating_cube.png");
        root.add_child(self, Node {
            component: Component::Canvas(canvas),
            children: vec![
                Node {
                    component: Component::Ui(
                        UIElement::image(
                            self.render_engine(),
                            Style { 
                                width: Sizing::Px(100.0), 
                                height: Sizing::Px(100.0), 
                                position_setting: PositionSetting::Parent,
                                top: Sizing::Px(10.0),
                                left: Sizing::Px(10.0),
                                border: Sizing::Px(5.0),
                                round: Sizing::Px(10.0),
                                ..Default::default() 
                            },
                            texture
                        )
                    ),
                    ..Default::default()
                },
                Node {
                    component: Component::Ui(
                        UIElement::container(
                            self.render_engine(), 
                            Style { 
                                width: Sizing::Px(200.0), 
                                height: Sizing::Px(200.0), 
                                color: Color { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 },
                                border: Sizing::Px(5.0),
                                round: Sizing::Px(15.0),
                                ..Default::default() 
                            }
                        )
                    ),
                    children: vec![
                        Node {
                            component: Component::Ui(
                                UIElement::container(
                                    self.render_engine(), 
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
                                )
                            ),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        });
    }

    fn update(&mut self, _root: &mut Node) {}
    fn exit(&mut self, _root: &mut Node) {}
}
