use fxhash::FxHashMap;

use crate::render::render_engine::RenderEngine;

pub trait EngineComponent {
    fn start(&mut self, components: &mut App);
    fn update(&mut self, components: &mut App);
    fn render<'rpass>(&'rpass self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>);
    fn exit(&mut self, component: &mut App);
}

pub struct App {
    render_engine: RenderEngine,
    components: FxHashMap<String, Box<dyn EngineComponent>>
}

// #[macro_export]
// macro_rules! create_app {
//     (
//         COMPONENTS => [$(
//             $component:ident => $type:ty
//         ),*]
//         PASSES => [$(
//             $pass_idx:literal => [$($to_render:ident),*]
//         ),*]
//     ) => {
//         use forte_engine::{EngineApp, start_render, end_render, pass, render::{input::EngineInput, render_engine::RenderEngine}};

//         pub struct App {
//             render_engine: RenderEngine,
//             $($component: $type,)*
//         }

//         impl EngineApp for App {
//             fn create(mut render_engine: RenderEngine) -> Self {
//                 $(let $component = <$type>::create(&mut render_engine);)*
//                 Self {
//                     render_engine,
//                     $($component,)*
//                 }
//             }

//             fn start(&mut self) {
//                 $(
//                     <$type>::start(self);
//                 )*
//             }

//             fn update(&mut self) {
//                 $(
//                     <$type>::update(self);
//                 )*

//                 let mut resources = start_render!(self.render_engine);
//                 $(
//                     {
//                         let pass_id = $pass_idx;
//                         let mut pass = pass!(self.render_engine, resources);
//                         $(
//                             self.$to_render.render(&self.render_engine, &mut pass);
//                         )*
//                         pass;
//                     }
//                 )*
//                 end_render!(self.render_engine, resources);

//                 self.render_engine.next_frame();
//             }

//             fn input(&mut self, input: EngineInput) {}
            
//             fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }

//             fn exit(&mut self) {
//                 $(
//                     <$type>::exit(self);
//                 )*
//             }
//         }
//     };
// }
