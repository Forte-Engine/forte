use crate::render::render_engine::RenderEngine;

pub trait EngineComponent<T> {
    fn create(engine: &mut RenderEngine) -> Self;
    fn start(&mut self, other: T);
    fn update(&mut self, other: T);
    fn render<'rpass>(&'rpass self, render_engine: &'rpass RenderEngine, pass: &mut wgpu::RenderPass<'rpass>);
    fn exit(&mut self, other: T);
}

pub trait HasRenderEngine {
    fn render_engine(&self) -> &RenderEngine;
    fn render_engine_mut(&mut self) -> &mut RenderEngine;
}

#[macro_export]
macro_rules! create_app {
    (
        COMPONENTS => [$(
            $component:ident => $type:ty => [$($param:ident),*]
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
                    <$type>::start(&mut self.$component, ($(&mut self.$param),*));
                )*
            }

            fn update(&mut self) {
                $(
                    <$type>::update(&mut self.$component, ($(&mut self.$param),*));
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
                    <$type>::exit(&mut self.$component, ($(&mut self.$param),*));
                )*
            }
        }
    };
}
