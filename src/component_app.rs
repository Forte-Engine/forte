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
        CLEAR_COLOR = $color:expr,
        APP {$(
            $component:ident: $type:ty[$($param:ident),*]
        ),*},
        PASSES {$(
            $pass_idx:literal: {
                COMPONENTS: [$($to_render:ident),*]
            }
        ),*}
    ) => {
        use forte_engine::{EngineApp, start_render, end_render, pass, component_app::HasRenderEngine, render::{input::EngineInput, render_engine::RenderEngine, render_utils}};

        pub struct App {
            render_engine: RenderEngine,
            $($component: $type,)*
        }

        impl EngineApp for App {
            // Takes in a render engine and creates each component individually in the order listed, then saves them into a new instance of App.
            fn create(mut render_engine: RenderEngine) -> Self {
                $(let $component = <$type>::create(&mut render_engine);)*
                Self {
                    render_engine,
                    $($component,)*
                }
            }

            // Starts app components of the App.
            fn start(&mut self) {
                $(
                    <$type>::start(&mut self.$component, ($(&mut self.$param),*));
                )*
            }

            // Updates App components, then performs the render steps in the order given.
            fn update(&mut self) {
                // Run update
                $(
                    <$type>::update(&mut self.$component, ($(&mut self.$param),*));
                )*

                // start the render
                let resources = render_utils::prepare_render(&self.render_engine);
                let mut resources = if resources.is_ok() { resources.unwrap() } else { return };

                // run each render pass in the order given
                $(
                    {
                        // create the render pass
                        let pass_id = $pass_idx;
                        let color_attachment = wgpu::RenderPassColorAttachment {
                            view: &resources.view,
                            resolve_target: None,
                            ops: if pass_id == 0 {
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Clear($color),
                                    store: wgpu::StoreOp::Store,
                                }
                            } else {
                                wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: wgpu::StoreOp::Store,
                                }
                            },
                        };
                        let mut pass = resources.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(color_attachment)],
                            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                                view: &self.render_engine.depth_texture.view,
                                depth_ops: Some(wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(1.0),
                                    store: wgpu::StoreOp::Store
                                }),
                                stencil_ops: None
                            }),
                            occlusion_query_set: None,
                            timestamp_writes: None,
                        });

                        // call all members of this pass' render functions
                        $(
                            self.$to_render.render(&self.render_engine, &mut pass);
                        )*
                    }
                )*

                // end the render
                render_utils::finalize_render(&mut self.render_engine, resources);

                // call next frame, will be replaced later
                self.render_engine.next_frame();
            }

            // takes all input from the event loop, will be processed later
            fn input(&mut self, input: EngineInput) {}
            
            // passes all resize from the event loop to the render engine
            fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) { self.render_engine.resize(new_size); }

            // calls all the exit functions of the components in the order given
            fn exit(&mut self) {
                $(
                    <$type>::exit(&mut self.$component, ($(&mut self.$param),*));
                )*
            }
        }
    };
}
