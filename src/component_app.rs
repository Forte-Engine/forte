use crate::render::render_engine::RenderEngine;

pub trait EngineComponent<C> {
    fn create(engine: &mut RenderEngine) -> Self;
    fn start(components: &mut C);
    fn update(components: &mut C);
    fn render<'rpass>(&'rpass self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>);
    fn exit(component: &mut C);
}

pub trait HasRenderEngine {
    fn render_engine(&self) -> &RenderEngine;
    fn render_engine_mut(&mut self) -> &mut RenderEngine;
}

#[macro_export]
macro_rules! create_app {
    (
        COMPONENTS => [$(
            $component:ident => $type:ty
        ),*]
        PASSES => [$(
            $pass_idx:literal => [$($to_render:ident),*]
        ),*]
    ) => {
        use forte_engine::{EngineApp, start_render, end_render, pass, component_app::HasRenderEngine, render::{input::EngineInput, render_engine::RenderEngine}};

        pub struct App {
            render_engine: RenderEngine,
            $($component: $type,)*
        }

        // mark the app as has render engine
        impl HasRenderEngine for App {
            fn render_engine(&self) -> &RenderEngine { &self.render_engine }
            fn render_engine_mut(&mut self) -> &mut RenderEngine { &mut self.render_engine }
        }

        impl EngineApp for App {
            fn create(mut render_engine: RenderEngine) -> Self {
                $(let $component = <$type>::create(&mut render_engine);)*
                Self {
                    render_engine,
                    $($component,)*
                }
            }

            fn start(&mut self) {
                $(
                    <$type>::start(self);
                )*
            }

            fn update(&mut self) {
                $(
                    <$type>::update(self);
                )*

                let mut resources = start_render!(self.render_engine);
                $(
                    {
                        let pass_id = $pass_idx;
                        let mut pass = pass!(self.render_engine, resources);
                        $(
                            self.$to_render.render(&self.render_engine, &mut pass);
                        )*
                        pass;
                    }
                )*
                end_render!(self.render_engine, resources);

                self.render_engine.next_frame();
            }

            fn input(&mut self, input: EngineInput) {}
            
            fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }

            fn exit(&mut self) {
                $(
                    <$type>::exit(self);
                )*
            }
        }
    };
}
