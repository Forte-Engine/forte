// #[macro_export]
// macro_rules! setup_world_app {
//     (
//         $app:ident
//     ) => {
//         use forte_engine::{start_render, pass, end_render, EngineApp};

//         pub trait WorldApp {
//             fn create(render_engine: RenderEngine, root: &mut Node) -> Self;
//             fn render_engine(&self) -> &RenderEngine;
//             fn render_engine_mut(&mut self) -> &mut RenderEngine;
//             fn update(&mut self, root: &mut Node);
//             fn exit(&mut self, root: &mut Node);
//         }
        
//         pub struct WorldContainer {
//             pub app: $app,
//             pub root: Node
//         }

//         impl EngineApp for WorldContainer {
//             // Simply create a new root node and create the above world app
//             fn create(mut engine: RenderEngine) -> Self {
//                 let mut root = Node::default();
//                 Self {
//                     app: $app::create(engine, &mut root),
//                     root
//                 }
//             }

//             // Update the app, then its nodes
//             fn update(&mut self) {
//                 // call updates
//                 self.app.update(&mut self.root);
//                 self.root.update(&Transform::default());

//                 // do render
//                 let mut resources = start_render!(self.app.render_engine_mut());
//                 {
//                     let mut pass = pass!(self.app.render_engine(), resources);
//                     pass.draw_node(&self.app, &self.root);
//                 }
//                 end_render!(self.app.render_engine_mut(), resources);
//             }

//             fn input(&mut self, input: forte_engine::render::input::EngineInput) {}
//             fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.app.render_engine_mut().resize(new_size); }
//             fn events_cleared(&mut self) { self.app.render_engine_mut().next_frame(); }
//             fn exit(&mut self) { self.app.exit(&mut self.root); }
//         }

//         fn main() { run_app::<WorldContainer>(); }
//     };
// }
