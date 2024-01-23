// use forte_engine::{*, render::{primitives::transforms::TransformRaw, render_engine::RenderEngine}, ui::{canvas::UICanvas, elements::{ElementInfo, UIElement}, style::Style, uniforms::UIInstance, DrawUI, UIEngine}, EngineApp};

// define_world!(
//     TestApp, 
//     [
//         Canvas => {
//             DATA => UICanvas,
//             ADDED => |_: &mut Node| {},
//             UPDATE => |_: &mut Node| {},
//             RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestApp, data: &'b UICanvas| {
//                 pass.prepare_ui(&app.ui_engine);
//                 pass.draw_canvas(&app.render_engine, &app.ui_engine, &data);
//             },
//             REMOVED => |_: &mut Node| {}
//         },
//         UI => {
//             DATA => UIElement,
//             ADDED => |_: &mut Node| {},
//             UPDATE => |_: &mut Node| {},
//             RENDER => |pass: &mut wgpu::RenderPass<'a>, app: &'b TestApp, data: &'b UIElement| {},
//             REMOVED => |_: &mut Node| {}
//         }
//     ]
// );

// define_ui_functions!(Node, Component);

// pub struct TestApp {
//     render_engine: RenderEngine,
//     ui_engine: UIEngine,
//     root: Node
// }

// impl EngineApp for TestApp {
//     fn create(mut render_engine: RenderEngine) -> Self {
//         // create render engine
//         let ui_engine = UIEngine::new(&mut render_engine);

//         // create canvas
//         let mut root = Node::default();
//         root.component = Component::Canvas(UICanvas::new(&render_engine));
//         let mut rect = Node::default();
//         rect.component = Component::UI(UIElement { style: Style::default(), info: ElementInfo::Container });

//         // create new self
//         Self {
//             render_engine,
//             ui_engine,
//             root
//         }
//     }

//     fn update(&mut self) {
//         // draw
//         let mut resources = start_render!(self.render_engine);
//         {
//             let mut pass = pass!(self.render_engine, resources);
//             pass.draw_node(&self, &self.root);
//         }
//         end_render!(self.render_engine, resources);
//     }

//     fn input(&mut self, _input: forte_engine::render::input::EngineInput) {}
//     fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }
//     fn events_cleared(&mut self) { self.render_engine.next_frame(); }
//     fn exit(&mut self) {}
// }

// fn main() { run_app::<TestApp>() }
fn main() {}