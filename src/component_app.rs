use fxhash::FxHashMap;

use crate::render::render_engine::RenderEngine;

pub trait EngineComponentFactory {
    fn id() -> u128;
    fn create(engine: &mut App) -> Self;
}

pub trait EngineComponent {
    fn start(&mut self, components: &mut App);
    fn update(&mut self, components: &mut App);
    fn render<'rpass>(&'rpass self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>);
    fn exit(&mut self, component: &mut App);
}

pub struct App {
    render_engine: RenderEngine,
    components: FxHashMap<u128, Box<dyn EngineComponent>>
}

impl App {
    // quick getters
    pub fn render_engine(&self) -> &RenderEngine { &self.render_engine }
    pub fn render_engine_mjt(&mut self) -> &mut RenderEngine { &mut self.render_engine }
    
    // component manipulation
    pub fn get_component(&self, id: u128) -> Option<&Box<dyn EngineComponent>> { self.components.get(&id) }
    pub fn get_component_mut(&mut self, id: u128) -> Option<&mut Box<dyn EngineComponent>> { self.components.get_mut(&id) }
    pub fn add_component<C: EngineComponent + EngineComponentFactory + 'static>(&mut self) {
        let component = C::create(self);
        self.components.insert(C::id(), Box::new(component));
    }
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
